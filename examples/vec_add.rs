//! End-to-end "hello world" — proves the hipRTC → hipModule → launch path.
//!
//! Uses only driver + hiprtc, which are always-on modules. No rocm-XYYYY
//! feature is required to compile or run this example.
//!
//! Run:
//!   ROCM_PATH=/opt/rocm cargo run --example vec_add

use rocmrc::{HipContext, HipModule};
use std::ffi::c_void;

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
    println!("device  = {}", ctx.name().unwrap_or_else(|_| "<unknown>".into()));
    println!("gfx     = {}", ctx.gfx_arch());

    let (hsaco, log) = rocmrc::hiprtc::compile(SRC, ctx.gfx_arch()).expect("hipRTC compile");
    if !log.trim().is_empty() {
        println!("hipRTC log:\n{log}");
    }

    let module = HipModule::from_hsaco(hsaco.as_bytes()).expect("module load");
    let func = module.get_function("vec_add").expect("get_function");

    const N: usize = 1024;
    let a: Vec<f32> = (0..N).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..N).map(|i| (2 * i) as f32).collect();

    let d_a = stream.clone_htod(&a).unwrap();
    let d_b = stream.clone_htod(&b).unwrap();
    let d_out = ctx.alloc::<f32>(N).unwrap();

    // Arg storage must outlive the launch call. Hold values in named locals
    // and take pointers to them.
    let d_out_ptr = d_out.device_ptr(&stream).0;
    let d_a_ptr = d_a.device_ptr(&stream).0;
    let d_b_ptr = d_b.device_ptr(&stream).0;
    let n_i32: i32 = N as i32;
    let mut params: [*mut c_void; 4] = [
        &d_out_ptr as *const _ as *mut c_void,
        &d_a_ptr as *const _ as *mut c_void,
        &d_b_ptr as *const _ as *mut c_void,
        &n_i32 as *const _ as *mut c_void,
    ];

    let grid = (((N + 255) / 256) as u32, 1, 1);
    let block = (256u32, 1, 1);
    unsafe {
        func.launch(grid, block, 0, &stream, &mut params)
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
    println!("ok  ({} elements, sum-check passed)", N);
}
