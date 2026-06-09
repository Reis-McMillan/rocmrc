//! End-to-end "hello world" — proves the hipRTC → hipModule → launch path.
//!
//! Uses only hip + hiprtc, which are always-on modules. No rocm-XYYYY
//! feature is required to compile or run this example.
//!
//! Run:
//!   ROCM_PATH=/opt/rocm cargo run --example vec_add

use rocmrc::hiprtc::compile_hsaco;
use rocmrc::{HipContext, LaunchConfig, PushKernelArg};

const SRC: &str = r#"
extern "C" __global__
void vec_add(float* out, const float* a, const float* b, int n) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) out[i] = a[i] + b[i];
}
"#;

fn main() {
    let ctx = HipContext::new(0).expect("HipContext::new failed");
    let stream = ctx.default_stream();
    println!(
        "device  = {}",
        ctx.name().unwrap_or_else(|_| "<unknown>".into())
    );

    let gfx = ctx.gfx_version().expect("unsupported gfx arch");
    println!("gfx     = {gfx}");

    let hsaco = compile_hsaco(SRC, gfx).expect("hipRTC compile");
    let module = ctx.load_module(hsaco).expect("module load");
    let func = module.load_function("vec_add").expect("load_function");

    const N: usize = 1024;
    let a: Vec<f32> = (0..N).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..N).map(|i| (2 * i) as f32).collect();

    let d_a = stream.clone_htod(&a).unwrap();
    let d_b = stream.clone_htod(&b).unwrap();
    let mut d_out = stream.alloc_zeros::<f32>(N).unwrap();

    let n = N as i32;
    let mut builder = stream.launch_builder(&func);
    builder.arg(&mut d_out).arg(&d_a).arg(&d_b).arg(&n);
    unsafe {
        builder
            .launch(LaunchConfig::for_num_elems(N as u32))
            .expect("kernel launch");
    }

    let out = stream.clone_dtoh(&d_out).expect("dtoh");
    for i in 0..N {
        let expected = a[i] + b[i];
        assert!(
            (out[i] - expected).abs() < 1e-6,
            "mismatch at {i}: got {} want {expected}",
            out[i]
        );
    }
    println!("ok  ({N} elements, sum-check passed)");
}
