//! End-to-end f32 GEMM via `rocblas`, plus a quick L1 (axpy/scal/nrm2/dot/copy)
//! round trip to exercise the trait surface.
//!
//! Run (pick any rocm-XYYYY feature matching the installed ROCm version):
//!   ROCM_PATH=/opt/rocm cargo run --features rocblas,rocm-07021 --example sgemm

use rocmrc::driver::{HipContext, HipSlice};
use rocmrc::driver::result as drv;
use rocmrc::rocblas::{
    Axpy, AxpyConfig, Copy as BlasCopy, CopyConfig, Dot, DotConfig, Gemm, GemmConfig, Nrm2,
    Nrm2Config, Operation, RocblasHandle, Scal, ScalConfig, rocblas_pointer_mode,
};

fn main() {
    let arch = std::env::var("ROCMRC_GFX").unwrap_or_else(|_| "gfx1102".to_string());
    let ctx = HipContext::new(0, &arch).expect("HipContext");
    let stream = ctx.default_stream();
    println!("device  = {}", ctx.name().unwrap_or_else(|_| "<unknown>".into()));

    let handle = RocblasHandle::new(stream.clone()).expect("RocblasHandle");

    // ----- GEMM: C = A * B for square matrices in column-major -----
    const N: usize = 32;
    let a_host: Vec<f32> = (0..N * N).map(|i| (i % 7) as f32).collect();
    let b_host: Vec<f32> = (0..N * N).map(|i| (i % 5) as f32 - 2.0).collect();

    let d_a: HipSlice<f32> = ctx.alloc(N * N).unwrap();
    let d_b: HipSlice<f32> = ctx.alloc(N * N).unwrap();
    let d_c: HipSlice<f32> = ctx.alloc(N * N).unwrap();

    unsafe {
        drv::memcpy_htod_async(
            d_a.device_ptr(),
            bytemuck::cast_slice(&a_host),
            stream.hip_stream(),
        )
        .unwrap();
        drv::memcpy_htod_async(
            d_b.device_ptr(),
            bytemuck::cast_slice(&b_host),
            stream.hip_stream(),
        )
        .unwrap();
    }

    let cfg = GemmConfig::<f32> {
        transa: Operation::None,
        transb: Operation::None,
        m: N as i32,
        n: N as i32,
        k: N as i32,
        alpha: 1.0,
        lda: N as i32,
        ldb: N as i32,
        beta: 0.0,
        ldc: N as i32,
    };
    unsafe {
        handle
            .gemm(cfg, d_a.device_ptr(), d_b.device_ptr(), d_c.device_ptr())
            .expect("sgemm");
    }

    let mut c_bytes = vec![0u8; N * N * std::mem::size_of::<f32>()];
    unsafe {
        drv::memcpy_dtoh_async(&mut c_bytes, d_c.device_ptr(), stream.hip_stream()).unwrap();
    }
    stream.synchronize().expect("sync");

    let c: &[f32] = bytemuck::cast_slice(&c_bytes);
    // Column-major CPU reference: C[i,j] = sum_k A[i,k] * B[k,j]
    let mut max_err = 0f32;
    for j in 0..N {
        for i in 0..N {
            let mut acc = 0.0f32;
            for k in 0..N {
                acc += a_host[k * N + i] * b_host[j * N + k];
            }
            let got = c[j * N + i];
            max_err = max_err.max((got - acc).abs());
        }
    }
    println!("sgemm  ({N}x{N}, col-major) max abs err = {max_err:.3e}");
    assert!(max_err < 1e-3, "sgemm precision");

    // ----- L1 smoke: scal -> axpy -> copy -> dot -> nrm2 -----
    const M: usize = 1024;
    let x_host: Vec<f32> = (0..M).map(|i| 0.001 * i as f32).collect();
    let y_host: Vec<f32> = (0..M).map(|i| 0.5 - 0.001 * i as f32).collect();

    let d_x: HipSlice<f32> = ctx.alloc(M).unwrap();
    let d_y: HipSlice<f32> = ctx.alloc(M).unwrap();
    let d_z: HipSlice<f32> = ctx.alloc(M).unwrap(); // copy target
    let d_scratch: HipSlice<f32> = ctx.alloc(1).unwrap(); // device-side reduction result

    unsafe {
        drv::memcpy_htod_async(
            d_x.device_ptr(),
            bytemuck::cast_slice(&x_host),
            stream.hip_stream(),
        )
        .unwrap();
        drv::memcpy_htod_async(
            d_y.device_ptr(),
            bytemuck::cast_slice(&y_host),
            stream.hip_stream(),
        )
        .unwrap();
    }

    // scal: x := 2*x
    unsafe {
        handle
            .scal(ScalConfig { n: M as i32, alpha: 2.0f32, incx: 1 }, d_x.device_ptr())
            .expect("scal");
    }

    // axpy: y := 3*x + y
    unsafe {
        handle
            .axpy(
                AxpyConfig { n: M as i32, alpha: 3.0f32, incx: 1, incy: 1 },
                d_x.device_ptr(),
                d_y.device_ptr(),
            )
            .expect("axpy");
    }

    // copy: z := y
    // Copy/Dot/Nrm2 configs don't carry a `T`, so the trait dispatch needs an
    // explicit type via UFCS. (scal/axpy can infer T from `alpha: T` in cfg.)
    unsafe {
        BlasCopy::<f32>::copy(
            &*handle,
            CopyConfig { n: M as i32, incx: 1, incy: 1 },
            d_y.device_ptr(),
            d_z.device_ptr(),
        )
        .expect("copy");
    }

    // Switch to device pointer mode for the reduction; result lands in d_scratch.
    handle
        .set_pointer_mode(rocblas_pointer_mode::rocblas_pointer_mode_device)
        .expect("ptr mode");

    // dot: <x, z>
    unsafe {
        Dot::<f32>::dot(
            &*handle,
            DotConfig { n: M as i32, incx: 1, incy: 1 },
            d_x.device_ptr(),
            d_z.device_ptr(),
            d_scratch.device_ptr(),
        )
        .expect("dot");
    }

    let mut dot_buf = [0f32];
    unsafe {
        drv::memcpy_dtoh_async(
            bytemuck::cast_slice_mut(&mut dot_buf),
            d_scratch.device_ptr(),
            stream.hip_stream(),
        )
        .unwrap();
    }
    stream.synchronize().unwrap();

    // nrm2(x)
    unsafe {
        Nrm2::<f32>::nrm2(
            &*handle,
            Nrm2Config { n: M as i32, incx: 1 },
            d_x.device_ptr(),
            d_scratch.device_ptr(),
        )
        .expect("nrm2");
    }
    let mut nrm_buf = [0f32];
    unsafe {
        drv::memcpy_dtoh_async(
            bytemuck::cast_slice_mut(&mut nrm_buf),
            d_scratch.device_ptr(),
            stream.hip_stream(),
        )
        .unwrap();
    }
    stream.synchronize().unwrap();

    // CPU reference for the L1 chain.
    let x_ref: Vec<f32> = x_host.iter().map(|&v| 2.0 * v).collect();
    let y_ref: Vec<f32> = y_host
        .iter()
        .zip(&x_ref)
        .map(|(&y, &x)| 3.0 * x + y)
        .collect();
    // z == y after copy.
    let dot_ref: f32 = x_ref.iter().zip(&y_ref).map(|(a, b)| a * b).sum();
    let nrm_ref: f32 = x_ref.iter().map(|v| v * v).sum::<f32>().sqrt();

    let dot_err = (dot_buf[0] - dot_ref).abs() / dot_ref.abs().max(1e-6);
    let nrm_err = (nrm_buf[0] - nrm_ref).abs() / nrm_ref.abs().max(1e-6);
    println!("dot     = {} (ref {}), rel err {:.3e}", dot_buf[0], dot_ref, dot_err);
    println!("nrm2(x) = {} (ref {}), rel err {:.3e}", nrm_buf[0], nrm_ref, nrm_err);
    assert!(dot_err < 1e-4 && nrm_err < 1e-4, "L1 precision");

    println!("ok");
}
