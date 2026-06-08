//! End-to-end hipBLASLt matmul with a bias + ReLU epilogue.
//!
//! Computes D = ReLU(A * B + bias) in column-major f32 and compares to a CPU
//! reference.
//!
//! Run (pick any rocm-XYYYY feature matching the installed ROCm version):
//!   ROCM_PATH=/opt/rocm cargo run --features hipblaslt,rocm-07021 --example matmul_lt

use rocmrc::hip::HipContext;
use rocmrc::hipblaslt::{Activation, HipBlasLT, Matmul, MatmulConfig};

fn main() {
    let ctx = HipContext::new(0).expect("HipContext");
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

    let d_a = stream.clone_htod(&a_host).unwrap();
    let d_b = stream.clone_htod(&b_host).unwrap();
    let d_bias = stream.clone_htod(&bias_host).unwrap();
    let mut d_out = stream.alloc_zeros::<f32>(M * N).unwrap();

    // ----- hipBLASLt setup (workspace + algo heuristic are handled internally) -----
    let blas = HipBlasLT::new(stream.clone()).expect("HipBlasLT");

    // The data is already column-major, so no arg-swap is needed: hipBLASLt is
    // column-major natively. D = ReLU(A * B + bias).
    let cfg = MatmulConfig {
        transa: false,
        transb: false,
        m: M as u64,
        n: N as u64,
        k: K as u64,
        alpha: 1.0,
        lda: M as i64,
        ldb: K as i64,
        beta: 0.0,
        ldc: M as i64,
        stride_a: None,
        stride_b: None,
        stride_c: None,
        batch_size: None,
    };
    unsafe {
        blas.matmul(
            cfg,
            &d_a,
            &d_b,
            &mut d_out,
            Some(&d_bias),
            Some(&Activation::Relu),
        )
        .expect("matmul");
    }

    // ----- pull D back -----
    let d_got = stream.clone_dtoh(&d_out).expect("dtoh");

    // ----- CPU reference: D[i,j] = ReLU( sum_k A[i,k] * B[k,j] + bias[i] ) -----
    let mut max_err = 0f32;
    for j in 0..N {
        for i in 0..M {
            let mut acc = 0.0f32;
            for k in 0..K {
                acc += a_host[k * M + i] * b_host[j * K + k];
            }
            let relu = (acc + bias_host[i]).max(0.0);
            let got = d_got[j * M + i];
            max_err = max_err.max((got - relu).abs());
        }
    }
    println!("matmul_lt ({M}x{K} * {K}x{N} + bias, ReLU) max abs err = {max_err:.3e}");
    assert!(max_err < 1e-3, "matmul_lt precision");

    println!("ok");
}
