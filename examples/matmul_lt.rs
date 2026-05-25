//! End-to-end hipBLASLt matmul with a bias + ReLU epilogue.
//!
//! Computes D = ReLU(A * B + bias) in column-major f32 and compares to a CPU
//! reference.
//!
//! Run (pick any rocm-XYYYY feature matching the installed ROCm version):
//!   ROCM_PATH=/opt/rocm cargo run --features hipblaslt,rocm-07021 --example matmul_lt

use rocmrc::driver::{HipContext, HipSlice, result as drv};
use rocmrc::hipblaslt::{
    HipBlasLt, MatmulDesc, MatmulPref, MatrixLayout,
    hipDataType, hipblasComputeType_t, hipblasLtEpilogue_t, hipblasOperation_t,
};

fn main() {
    let arch = std::env::var("ROCMRC_GFX").unwrap_or_else(|_| "gfx1102".to_string());
    let ctx = HipContext::new(0, &arch).expect("HipContext");
    let stream = ctx.default_stream();
    println!("device  = {}", ctx.name().unwrap_or_else(|_| "<unknown>".into()));

    // ----- shapes (column-major) -----
    const M: usize = 64;
    const N: usize = 32;
    const K: usize = 48;

    // CPU data, column-major:
    //   A: M x K  with stride lda = M
    //   B: K x N  with stride ldb = K
    //   D: M x N  with stride ldd = M
    //   bias: length M
    let a_host: Vec<f32> = (0..M * K).map(|i| ((i % 11) as f32) * 0.05 - 0.2).collect();
    let b_host: Vec<f32> = (0..K * N).map(|i| ((i % 13) as f32) * 0.07 - 0.3).collect();
    let bias_host: Vec<f32> = (0..M).map(|i| 0.1 * (i as f32) - 1.5).collect();

    let d_a: HipSlice<f32> = ctx.alloc(M * K).unwrap();
    let d_b: HipSlice<f32> = ctx.alloc(K * N).unwrap();
    let d_c: HipSlice<f32> = ctx.alloc(M * N).unwrap(); // unused (beta=0) but layout requires a valid ptr
    let d_d: HipSlice<f32> = ctx.alloc(M * N).unwrap();
    let d_bias: HipSlice<f32> = ctx.alloc(M).unwrap();

    unsafe {
        drv::memcpy_htod_async(d_a.device_ptr(), bytemuck::cast_slice(&a_host), stream.hip_stream()).unwrap();
        drv::memcpy_htod_async(d_b.device_ptr(), bytemuck::cast_slice(&b_host), stream.hip_stream()).unwrap();
        drv::memcpy_htod_async(d_bias.device_ptr(), bytemuck::cast_slice(&bias_host), stream.hip_stream()).unwrap();
    }

    // ----- hipBLASLt setup -----
    let lt = HipBlasLt::new().expect("HipBlasLt");

    let a_layout = MatrixLayout::new(hipDataType::HIP_R_32F, M as u64, K as u64, M as i64).unwrap();
    let b_layout = MatrixLayout::new(hipDataType::HIP_R_32F, K as u64, N as u64, K as i64).unwrap();
    let c_layout = MatrixLayout::new(hipDataType::HIP_R_32F, M as u64, N as u64, M as i64).unwrap();
    let d_layout = MatrixLayout::new(hipDataType::HIP_R_32F, M as u64, N as u64, M as i64).unwrap();

    let mut desc = MatmulDesc::new(
        hipblasComputeType_t::HIPBLAS_COMPUTE_32F,
        hipDataType::HIP_R_32F,
    )
    .unwrap();
    desc.set_transa(hipblasOperation_t::HIPBLAS_OP_N).unwrap();
    desc.set_transb(hipblasOperation_t::HIPBLAS_OP_N).unwrap();
    desc.set_epilogue(hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_RELU_BIAS).unwrap();
    desc.set_bias_pointer(d_bias.device_ptr()).unwrap();
    desc.set_bias_dtype(hipDataType::HIP_R_32F).unwrap();

    let mut pref = MatmulPref::new().unwrap();
    pref.set_max_workspace_bytes(32 * 1024 * 1024).unwrap(); // 32 MB

    let heuristics = lt
        .get_heuristic(&desc, &a_layout, &b_layout, &c_layout, &d_layout, &pref, 4)
        .expect("get_heuristic");
    assert!(!heuristics.is_empty(), "no algos returned");
    let best = &heuristics[0];
    println!("chose algo, workspace_required = {} bytes", best.workspace_required());

    let workspace: HipSlice<u8> = ctx.alloc(best.workspace_required().max(1)).unwrap();
    let alpha: f32 = 1.0;
    let beta: f32 = 0.0;

    unsafe {
        lt.matmul(
            &desc,
            &alpha as *const _ as *const std::ffi::c_void,
            d_a.device_ptr(),
            &a_layout,
            d_b.device_ptr(),
            &b_layout,
            &beta as *const _ as *const std::ffi::c_void,
            d_c.device_ptr(),
            &c_layout,
            d_d.device_ptr(),
            &d_layout,
            best,
            workspace.device_ptr(),
            best.workspace_required(),
            &stream,
        )
        .expect("matmul");
    }

    // ----- pull D back -----
    let mut d_bytes = vec![0u8; M * N * std::mem::size_of::<f32>()];
    unsafe {
        drv::memcpy_dtoh_async(&mut d_bytes, d_d.device_ptr(), stream.hip_stream()).unwrap();
    }
    stream.synchronize().expect("sync");
    let d_got: &[f32] = bytemuck::cast_slice(&d_bytes);

    // ----- CPU reference: D[i,j] = ReLU( sum_k A[i,k] * B[k,j] + bias[i] ) -----
    let mut max_err = 0f32;
    for j in 0..N {
        for i in 0..M {
            let mut acc = 0.0f32;
            for k in 0..K {
                acc += a_host[k * M + i] * b_host[j * K + k];
            }
            let with_bias = acc + bias_host[i];
            let relu = with_bias.max(0.0);
            let got = d_got[j * M + i];
            max_err = max_err.max((got - relu).abs());
        }
    }
    println!("matmul_lt ({M}x{K} * {K}x{N} + bias, ReLU) max abs err = {max_err:.3e}");
    assert!(max_err < 1e-3, "matmul_lt precision");

    println!("ok");
}
