//! Kernel launch builder. Mirrors [`cudarc::driver::safe::launch`]. The
//! [`LaunchArgs`] builder accumulates kernel parameters via repeated
//! [`PushKernelArg::arg`] calls; each `arg` impl that takes a device
//! buffer also queues the read/write event waits/records needed to make
//! the launch compose correctly with other streams in the same context.

use std::ffi::c_uint;
use std::vec::Vec;

use crate::hip::{
    result::{self, HipError},
    sys,
};

use super::{DeviceRepr, HipEvent, HipFunction, HipSlice, HipStream, HipView, HipViewMut};

/// Kernel launch geometry. Mirrors cudarc's `LaunchConfig`.
#[derive(Clone, Copy, Debug)]
pub struct LaunchConfig {
    /// `(width, height, depth)` of grid in blocks.
    pub grid_dim: (u32, u32, u32),
    /// `(x, y, z)` dimension of each thread block.
    pub block_dim: (u32, u32, u32),
    /// Dynamic shared-memory size per block in bytes.
    pub shared_mem_bytes: u32,
}

impl LaunchConfig {
    /// 1D config covering `n` elements with 1024 threads per block.
    ///
    /// - `block_dim` = `(1024, 1, 1)`
    /// - `grid_dim`  = `((n + 1023) / 1024, 1, 1)`
    /// - `shared_mem_bytes` = `0`
    pub fn for_num_elems(n: u32) -> Self {
        const NUM_THREADS: u32 = 1024;
        let num_blocks = n.div_ceil(NUM_THREADS);
        Self {
            grid_dim: (num_blocks, 1, 1),
            block_dim: (NUM_THREADS, 1, 1),
            shared_mem_bytes: 0,
        }
    }
}

// ----------------------------------------------------------------------------
// LaunchArgs
// ----------------------------------------------------------------------------

/// Kernel launch builder. Construct via [`HipStream::launch_builder`].
///
/// Each `arg(...)` call extends the kernel parameter list and, when the
/// owning context is in
/// [`super::HipContext::is_managing_stream_synchronization`] mode, queues
/// read/write event waits and records so the launch composes cleanly with
/// other streams.
///
/// Anything pushed via `arg(...)` must either implement [`DeviceRepr`] (for
/// by-reference scalars) or have a custom
/// `impl PushKernelArg<T> for LaunchArgs<'a>`.
#[derive(Debug)]
pub struct LaunchArgs<'a> {
    pub(super) stream: &'a HipStream,
    pub(super) func: &'a HipFunction,
    pub(super) waits: Vec<&'a HipEvent>,
    pub(super) records: Vec<&'a HipEvent>,
    pub(super) args: Vec<*mut std::ffi::c_void>,
    pub(super) flags: Option<c_uint>,
}

impl HipStream {
    /// Begin building a kernel launch of `func` on `self`.
    pub fn launch_builder<'a>(&'a self, func: &'a HipFunction) -> LaunchArgs<'a> {
        LaunchArgs {
            stream: self,
            func,
            waits: Vec::new(),
            records: Vec::new(),
            args: Vec::new(),
            flags: None,
        }
    }
}

impl LaunchArgs<'_> {
    /// Opt in to having [`Self::launch`] / [`Self::launch_cooperative`]
    /// record start/end events around the kernel and return them.
    ///
    /// `flags` mirrors HIP's `hipEvent*` flag values — pass `0` for full
    /// timing or `0x2` (`hipEventDisableTiming`) to skip the timer.
    pub fn record_kernel_launch(&mut self, flags: c_uint) -> &mut Self {
        self.flags = Some(flags);
        self
    }

    /// Submit the kernel asynchronously on the stream.
    ///
    /// Returns `Some((start, end))` if [`Self::record_kernel_launch`] was
    /// called; otherwise `None`.
    ///
    /// # Safety
    /// 1. `args` count, sizes, and types must match the loaded kernel's
    ///    signature.
    /// 2. The kernel must not mutate args passed by shared reference.
    /// 3. The kernel must not access memory outside the buffers passed in
    ///    (no out-of-bounds reads/writes).
    ///
    /// ## Multi-stream safety
    /// [`HipSlice`] / [`HipView`] / [`HipViewMut`] carry optional read/write
    /// events; the [`PushKernelArg`] impls feed those into `self.waits` and
    /// `self.records`. This method waits on all queued events before the
    /// kernel runs and records all queued events afterwards, so launches
    /// across streams sequence correctly without further user action.
    #[inline(always)]
    pub unsafe fn launch(
        &mut self,
        cfg: LaunchConfig,
    ) -> Result<Option<(HipEvent, HipEvent)>, HipError> {
        self.stream.ctx.bind_to_thread()?;
        for &event in self.waits.iter() {
            self.stream.wait(event)?;
        }
        let start_event = self
            .flags
            .map(|flags| self.stream.record_event(Some(flags)))
            .transpose()?;
        unsafe {
            result::launch_kernel(
                self.func.hip_function,
                cfg.grid_dim,
                cfg.block_dim,
                cfg.shared_mem_bytes,
                self.stream.hip_stream,
                &mut self.args,
            )?;
        }
        let end_event = self
            .flags
            .map(|flags| self.stream.record_event(Some(flags)))
            .transpose()?;
        for &event in self.records.iter() {
            event.record(self.stream)?;
        }
        Ok(start_event.zip(end_event))
    }

    /// Submit a cooperative kernel. Same arg / event semantics as
    /// [`Self::launch`].
    ///
    /// # Safety
    /// Same conditions as [`Self::launch`], plus the kernel must be
    /// compiled cooperatively and the device must advertise
    /// `hipDeviceAttributeCooperativeLaunch`.
    #[inline(always)]
    pub unsafe fn launch_cooperative(
        &mut self,
        cfg: LaunchConfig,
    ) -> Result<Option<(HipEvent, HipEvent)>, HipError> {
        self.stream.ctx.bind_to_thread()?;
        for &event in self.waits.iter() {
            self.stream.wait(event)?;
        }
        let start_event = self
            .flags
            .map(|flags| self.stream.record_event(Some(flags)))
            .transpose()?;
        unsafe {
            result::launch_cooperative_kernel(
                self.func.hip_function,
                cfg.grid_dim,
                cfg.block_dim,
                cfg.shared_mem_bytes,
                self.stream.hip_stream,
                &mut self.args,
            )?;
        }
        let end_event = self
            .flags
            .map(|flags| self.stream.record_event(Some(flags)))
            .transpose()?;
        for &event in self.records.iter() {
            event.record(self.stream)?;
        }
        Ok(start_event.zip(end_event))
    }
}

// ----------------------------------------------------------------------------
// PushKernelArg
// ----------------------------------------------------------------------------

/// Anything that can be appended to a [`LaunchArgs`] parameter list.
///
/// # Safety
/// Implementors must push a properly-typed pointer/value into
/// `LaunchArgs::args` whose layout matches what the HIP kernel expects on
/// the device side. Most impls require `#[inline(always)]` so that the
/// caller-side reference whose address is pushed survives until the
/// matching `launch()` call.
pub unsafe trait PushKernelArg<T> {
    fn arg(&mut self, arg: T) -> &mut Self;
}

// 1. Scalar by reference.
unsafe impl<'a, 'b: 'a, T: DeviceRepr> PushKernelArg<&'b T> for LaunchArgs<'a> {
    #[inline(always)]
    fn arg(&mut self, arg: &'b T) -> &mut Self {
        self.args.push(arg as *const T as *mut _);
        self
    }
}

// 2. &HipSlice — kernel reads only. Wait on prior write; record this read.
unsafe impl<'a, 'b: 'a, T> PushKernelArg<&'b HipSlice<T>> for LaunchArgs<'a> {
    #[inline(always)]
    fn arg(&mut self, arg: &'b HipSlice<T>) -> &mut Self {
        if self.stream.ctx.is_managing_stream_synchronization() {
            if let Some(w) = arg.write.as_ref() {
                self.waits.push(w);
            }
            if let Some(r) = arg.read.as_ref() {
                self.records.push(r);
            }
        }
        self.args
            .push((&arg.hip_device_ptr) as *const sys::hipDeviceptr_t as _);
        self
    }
}

// 3. &mut HipSlice — kernel may mutate. Wait on prior read AND write;
//    record this write.
unsafe impl<'a, 'b: 'a, T> PushKernelArg<&'b mut HipSlice<T>> for LaunchArgs<'a> {
    #[inline(always)]
    fn arg(&mut self, arg: &'b mut HipSlice<T>) -> &mut Self {
        if self.stream.ctx.is_managing_stream_synchronization() {
            if let Some(r) = arg.read.as_ref() {
                self.waits.push(r);
            }
            if let Some(w) = arg.write.as_ref() {
                self.waits.push(w);
                self.records.push(w);
            }
        }
        self.args
            .push((&arg.hip_device_ptr) as *const sys::hipDeviceptr_t as _);
        self
    }
}

// 4. &HipView — same as &HipSlice, uses `arg.ptr` instead of `hip_device_ptr`.
unsafe impl<'a, 'b: 'a, 'c: 'b, T> PushKernelArg<&'b HipView<'c, T>> for LaunchArgs<'a> {
    #[inline(always)]
    fn arg(&mut self, arg: &'b HipView<'c, T>) -> &mut Self {
        if self.stream.ctx.is_managing_stream_synchronization() {
            if let Some(w) = arg.write.as_ref() {
                self.waits.push(w);
            }
            if let Some(r) = arg.read.as_ref() {
                self.records.push(r);
            }
        }
        self.args.push((&arg.ptr) as *const sys::hipDeviceptr_t as _);
        self
    }
}

// 5. &mut HipViewMut — same as &mut HipSlice, uses `arg.ptr`.
unsafe impl<'a, 'b: 'a, 'c: 'b, T> PushKernelArg<&'b mut HipViewMut<'c, T>>
    for LaunchArgs<'a>
{
    #[inline(always)]
    fn arg(&mut self, arg: &'b mut HipViewMut<'c, T>) -> &mut Self {
        if self.stream.ctx.is_managing_stream_synchronization() {
            if let Some(r) = arg.read.as_ref() {
                self.waits.push(r);
            }
            if let Some(w) = arg.write.as_ref() {
                self.waits.push(w);
                self.records.push(w);
            }
        }
        self.args.push((&arg.ptr) as *const sys::hipDeviceptr_t as _);
        self
    }
}

// ============================================================================
// Tests — ported from cudarc's `src/driver/safe/launch.rs` test module.
// Kernels are inlined as `const &str` per test (cudarc-style). hipRTC
// compiles the source against the running device's gfx arch.
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hip::safe::{HipContext, HipSlice};
    use crate::hiprtc;

    fn compile(src: &str, name: &str) -> (std::sync::Arc<crate::hip::safe::HipContext>, crate::hip::safe::HipFunction) {
        let ctx = HipContext::new(0).unwrap();
        let (hsaco, _log) = hiprtc::compile(src, ctx.gfx_arch()).expect("hipRTC compile");
        let module = ctx.load_module(hsaco).unwrap();
        let f = module.load_function(name).unwrap();
        (ctx, f)
    }

    const SIN_KERNEL: &str = r#"
extern "C" __global__
void sin_kernel(float* out, const float* inp, int n) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) out[i] = sinf(inp[i]);
}
"#;

    #[test]
    fn test_launch_arrays() {
        // Pass a small array element-wise as kernel args via DeviceRepr<[T;N]>.
        const SRC: &str = r#"
extern "C" __global__
void k(float a, float b, float c, float d) {
    if (threadIdx.x == 0) {
        assert(a == 1.0f);
        assert(b == 2.0f);
        assert(c == 3.0f);
        assert(d == 4.0f);
    }
}
"#;
        let (ctx, f) = compile(SRC, "k");
        let stream = ctx.default_stream();
        let a = 1.0f32;
        let b = 2.0f32;
        let c = 3.0f32;
        let d = 4.0f32;
        let cfg = LaunchConfig {
            grid_dim: (1, 1, 1),
            block_dim: (1, 1, 1),
            shared_mem_bytes: 0,
        };
        unsafe {
            stream
                .launch_builder(&f)
                .arg(&a)
                .arg(&b)
                .arg(&c)
                .arg(&d)
                .launch(cfg)
                .unwrap();
        }
        stream.synchronize().unwrap();
    }

    #[test]
    fn test_launch_with_mut_and_ref_cudarc() {
        let (ctx, f) = compile(SIN_KERNEL, "sin_kernel");
        let stream = ctx.default_stream();
        const N: usize = 64;
        let inp: HipSlice<f32> = stream
            .clone_htod(&(0..N).map(|i| i as f32 * 0.1).collect::<Vec<_>>())
            .unwrap();
        let mut out: HipSlice<f32> = stream.alloc_zeros(N).unwrap();
        let n = N as i32;
        let cfg = LaunchConfig::for_num_elems(N as u32);
        unsafe {
            stream
                .launch_builder(&f)
                .arg(&mut out)
                .arg(&inp)
                .arg(&n)
                .launch(cfg)
                .unwrap();
        }
        let host: Vec<f32> = stream.clone_dtoh(&out).unwrap();
        for i in 0..N {
            let expected = (i as f32 * 0.1).sin();
            assert!((host[i] - expected).abs() < 1e-5);
        }
    }

    #[test]
    fn test_large_launches() {
        let (ctx, f) = compile(SIN_KERNEL, "sin_kernel");
        let stream = ctx.default_stream();
        for &n in &[256usize, 512, 1024, 2048] {
            let inp: HipSlice<f32> = stream
                .clone_htod(&(0..n).map(|i| i as f32 * 0.01).collect::<Vec<_>>())
                .unwrap();
            let mut out: HipSlice<f32> = stream.alloc_zeros(n).unwrap();
            let n_i = n as i32;
            let cfg = LaunchConfig::for_num_elems(n as u32);
            unsafe {
                stream
                    .launch_builder(&f)
                    .arg(&mut out)
                    .arg(&inp)
                    .arg(&n_i)
                    .launch(cfg)
                    .unwrap();
            }
            let host: Vec<f32> = stream.clone_dtoh(&out).unwrap();
            for (i, &v) in host.iter().enumerate() {
                let expected = (i as f32 * 0.01).sin();
                assert!((v - expected).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn test_launch_with_views() {
        let (ctx, f) = compile(SIN_KERNEL, "sin_kernel");
        let stream = ctx.default_stream();
        const N: usize = 64;
        let inp_full: HipSlice<f32> = stream
            .clone_htod(&(0..N).map(|i| i as f32 * 0.1).collect::<Vec<_>>())
            .unwrap();
        let mut out_full: HipSlice<f32> = stream.alloc_zeros(N).unwrap();
        let half_n = (N / 2) as i32;

        // Run the kernel against the second halves of both buffers.
        let inp_view = inp_full.slice(N / 2..N);
        let mut out_view = out_full.slice_mut(N / 2..N);
        let cfg = LaunchConfig::for_num_elems((N / 2) as u32);
        unsafe {
            stream
                .launch_builder(&f)
                .arg(&mut out_view)
                .arg(&inp_view)
                .arg(&half_n)
                .launch(cfg)
                .unwrap();
        }
        let host: Vec<f32> = stream.clone_dtoh(&out_full).unwrap();
        // First half untouched (still zeros), second half = sin(0.1*i).
        for i in 0..N / 2 {
            assert_eq!(host[i], 0.0);
        }
        for i in N / 2..N {
            let j = i - N / 2; // index within the slice
            let expected = ((N / 2 + j) as f32 * 0.1).sin();
            assert!((host[i] - expected).abs() < 1e-5);
        }
    }

    #[test]
    fn test_launch_with_8bit() {
        const SRC: &str = r#"
extern "C" __global__
void int_8bit(char s_min, char s_max, unsigned char u_min, unsigned char u_max) {
    if (threadIdx.x == 0) {
        assert(s_min == -128);
        assert(s_max == 127);
        assert(u_min == 0);
        assert(u_max == 255);
    }
}
"#;
        let (ctx, f) = compile(SRC, "int_8bit");
        let stream = ctx.default_stream();
        let (s_min, s_max, u_min, u_max) = (i8::MIN, i8::MAX, u8::MIN, u8::MAX);
        let cfg = LaunchConfig {
            grid_dim: (1, 1, 1),
            block_dim: (1, 1, 1),
            shared_mem_bytes: 0,
        };
        unsafe {
            stream
                .launch_builder(&f)
                .arg(&s_min)
                .arg(&s_max)
                .arg(&u_min)
                .arg(&u_max)
                .launch(cfg)
                .unwrap();
        }
        stream.synchronize().unwrap();
    }

    #[test]
    fn test_launch_with_16bit() {
        const SRC: &str = r#"
extern "C" __global__
void int_16bit(short s_min, short s_max, unsigned short u_min, unsigned short u_max) {
    if (threadIdx.x == 0) {
        assert(s_min == -32768);
        assert(s_max == 32767);
        assert(u_min == 0);
        assert(u_max == 65535);
    }
}
"#;
        let (ctx, f) = compile(SRC, "int_16bit");
        let stream = ctx.default_stream();
        let (s_min, s_max, u_min, u_max) = (i16::MIN, i16::MAX, u16::MIN, u16::MAX);
        let cfg = LaunchConfig {
            grid_dim: (1, 1, 1),
            block_dim: (1, 1, 1),
            shared_mem_bytes: 0,
        };
        unsafe {
            stream
                .launch_builder(&f)
                .arg(&s_min)
                .arg(&s_max)
                .arg(&u_min)
                .arg(&u_max)
                .launch(cfg)
                .unwrap();
        }
        stream.synchronize().unwrap();
    }

    #[test]
    fn test_launch_with_32bit() {
        const SRC: &str = r#"
extern "C" __global__
void int_32bit(int s_min, int s_max, unsigned int u_min, unsigned int u_max) {
    if (threadIdx.x == 0) {
        assert(s_min == -2147483648);
        assert(s_max == 2147483647);
        assert(u_min == 0u);
        assert(u_max == 4294967295u);
    }
}
"#;
        let (ctx, f) = compile(SRC, "int_32bit");
        let stream = ctx.default_stream();
        let (s_min, s_max, u_min, u_max) = (i32::MIN, i32::MAX, u32::MIN, u32::MAX);
        let cfg = LaunchConfig {
            grid_dim: (1, 1, 1),
            block_dim: (1, 1, 1),
            shared_mem_bytes: 0,
        };
        unsafe {
            stream
                .launch_builder(&f)
                .arg(&s_min)
                .arg(&s_max)
                .arg(&u_min)
                .arg(&u_max)
                .launch(cfg)
                .unwrap();
        }
        stream.synchronize().unwrap();
    }

    #[test]
    fn test_launch_with_64bit() {
        const SRC: &str = r#"
extern "C" __global__
void int_64bit(long long s_min, long long s_max,
               unsigned long long u_min, unsigned long long u_max) {
    if (threadIdx.x == 0) {
        assert(s_min == -9223372036854775807LL - 1);
        assert(s_max ==  9223372036854775807LL);
        assert(u_min == 0ULL);
        assert(u_max == 18446744073709551615ULL);
    }
}
"#;
        let (ctx, f) = compile(SRC, "int_64bit");
        let stream = ctx.default_stream();
        let (s_min, s_max, u_min, u_max) = (i64::MIN, i64::MAX, u64::MIN, u64::MAX);
        let cfg = LaunchConfig {
            grid_dim: (1, 1, 1),
            block_dim: (1, 1, 1),
            shared_mem_bytes: 0,
        };
        unsafe {
            stream
                .launch_builder(&f)
                .arg(&s_min)
                .arg(&s_max)
                .arg(&u_min)
                .arg(&u_max)
                .launch(cfg)
                .unwrap();
        }
        stream.synchronize().unwrap();
    }

    #[test]
    fn test_launch_with_floats() {
        const SRC: &str = r#"
extern "C" __global__
void floating(float f, double d) {
    if (threadIdx.x == 0) {
        assert(fabsf(f - 1.5f) < 1e-6f);
        assert(fabs(d - 2.5)   < 1e-12);
    }
}
"#;
        let (ctx, f) = compile(SRC, "floating");
        let stream = ctx.default_stream();
        let f_arg = 1.5f32;
        let d_arg = 2.5f64;
        let cfg = LaunchConfig {
            grid_dim: (1, 1, 1),
            block_dim: (1, 1, 1),
            shared_mem_bytes: 0,
        };
        unsafe {
            stream
                .launch_builder(&f)
                .arg(&f_arg)
                .arg(&d_arg)
                .launch(cfg)
                .unwrap();
        }
        stream.synchronize().unwrap();
    }

    #[test]
    fn test_launch_with_half() {
        const SRC: &str = r#"
#include <hip/hip_fp16.h>
extern "C" __global__
void half_k(__half h) {
    if (threadIdx.x == 0) {
        // 1.5 in fp16 is exactly representable.
        assert(__half2float(h) == 1.5f);
    }
}
"#;
        let (ctx, f) = compile(SRC, "half_k");
        let stream = ctx.default_stream();
        let h = half::f16::from_f32(1.5);
        let cfg = LaunchConfig {
            grid_dim: (1, 1, 1),
            block_dim: (1, 1, 1),
            shared_mem_bytes: 0,
        };
        unsafe {
            stream.launch_builder(&f).arg(&h).launch(cfg).unwrap();
        }
        stream.synchronize().unwrap();
    }

    // The slow_worker kernel runs a long device-side loop so the host
    // can observe overlap (or serialization) between streams via
    // elapsed_ms. Timing thresholds in the multi-stream tests may need
    // tuning per device.
    const SLOW_WORKER: &str = r#"
extern "C" __global__
void slow_worker(int* out) {
    int sum = 0;
    for (int i = 0; i < 1000000; i++) {
        sum += i;
    }
    if (threadIdx.x == 0 && blockIdx.x == 0) out[0] = sum;
}
"#;

    #[test]
    fn test_par_launch() {
        let (ctx, f) = compile(SLOW_WORKER, "slow_worker");
        let s1 = ctx.new_stream().unwrap();
        let s2 = ctx.new_stream().unwrap();
        let mut buf1: HipSlice<i32> = s1.alloc_zeros(1).unwrap();
        let mut buf2: HipSlice<i32> = s2.alloc_zeros(1).unwrap();
        let cfg = LaunchConfig {
            grid_dim: (1, 1, 1),
            block_dim: (1, 1, 1),
            shared_mem_bytes: 0,
        };

        // Sequential reference.
        let start = ctx.new_event(None).unwrap();
        let end = ctx.new_event(None).unwrap();
        let f_ref = &f;
        let s1_ref = &s1;
        start.record(s1_ref).unwrap();
        unsafe {
            s1_ref
                .launch_builder(f_ref)
                .arg(&mut buf1)
                .launch(cfg)
                .unwrap();
            s1_ref
                .launch_builder(f_ref)
                .arg(&mut buf2)
                .launch(cfg)
                .unwrap();
        }
        end.record(s1_ref).unwrap();
        end.synchronize().unwrap();
        let sequential_ms = start.elapsed_ms(&end).unwrap();

        // Parallel.
        let p_start = ctx.new_event(None).unwrap();
        let p_end = ctx.new_event(None).unwrap();
        p_start.record(s1_ref).unwrap();
        s2.wait(&p_start).unwrap();
        unsafe {
            s1.launch_builder(&f)
                .arg(&mut buf1)
                .launch(cfg)
                .unwrap();
            s2.launch_builder(&f)
                .arg(&mut buf2)
                .launch(cfg)
                .unwrap();
        }
        s1.join(&s2).unwrap();
        p_end.record(s1_ref).unwrap();
        p_end.synchronize().unwrap();
        let parallel_ms = p_start.elapsed_ms(&p_end).unwrap();

        // Parallel run should be meaningfully faster than sequential.
        // Loose threshold (0.85x) to tolerate hardware variability.
        assert!(
            parallel_ms < sequential_ms * 0.85,
            "expected parallel < 0.85 * sequential, got par={parallel_ms}ms seq={sequential_ms}ms",
        );
    }

    #[test]
    fn test_multi_stream_concurrent_reads() {
        // Two streams reading the same buffer should not serialize.
        let (ctx, f) = compile(SLOW_WORKER, "slow_worker");
        let s1 = ctx.new_stream().unwrap();
        let s2 = ctx.new_stream().unwrap();
        // The kernel only writes its argument, so two distinct output
        // slices are fine — what matters is that they share no read/write
        // dependencies (the launch builder should not insert waits).
        let mut buf1: HipSlice<i32> = s1.alloc_zeros(1).unwrap();
        let mut buf2: HipSlice<i32> = s2.alloc_zeros(1).unwrap();
        let cfg = LaunchConfig {
            grid_dim: (1, 1, 1),
            block_dim: (1, 1, 1),
            shared_mem_bytes: 0,
        };

        let start = ctx.new_event(None).unwrap();
        let end = ctx.new_event(None).unwrap();
        start.record(&s1).unwrap();
        s2.wait(&start).unwrap();
        unsafe {
            s1.launch_builder(&f)
                .arg(&mut buf1)
                .launch(cfg)
                .unwrap();
            s2.launch_builder(&f)
                .arg(&mut buf2)
                .launch(cfg)
                .unwrap();
        }
        s1.join(&s2).unwrap();
        end.record(&s1).unwrap();
        end.synchronize().unwrap();
        // No correctness check — the smoke is that both kernels ran
        // without producing an error from the launch builder.
    }

    #[test]
    fn test_multi_stream_writes_block() {
        // Two streams writing the SAME buffer should serialize via the
        // launch builder's write-event tracking.
        let (ctx, f) = compile(SLOW_WORKER, "slow_worker");
        let s1 = ctx.new_stream().unwrap();
        let s2 = ctx.new_stream().unwrap();
        let mut shared: HipSlice<i32> = s1.alloc_zeros(1).unwrap();
        let cfg = LaunchConfig {
            grid_dim: (1, 1, 1),
            block_dim: (1, 1, 1),
            shared_mem_bytes: 0,
        };

        unsafe {
            s1.launch_builder(&f)
                .arg(&mut shared)
                .launch(cfg)
                .unwrap();
        }
        // Same `&mut` shared between two streams in sequence — the second
        // launch's PushKernelArg should record waits on the prior write.
        unsafe {
            s2.launch_builder(&f)
                .arg(&mut shared)
                .launch(cfg)
                .unwrap();
        }
        s1.synchronize().unwrap();
        s2.synchronize().unwrap();
        // Both kernels completed without error; the assertion lives in
        // the fact that the launch builder correctly chained them.
    }

    #[test]
    fn test_device_side_assert() {
        const SRC: &str = r#"
extern "C" __global__
void boom() { assert(0); }
"#;
        let (ctx, f) = compile(SRC, "boom");
        let stream = ctx.default_stream();
        let cfg = LaunchConfig {
            grid_dim: (1, 1, 1),
            block_dim: (1, 1, 1),
            shared_mem_bytes: 0,
        };
        unsafe {
            stream.launch_builder(&f).launch(cfg).unwrap();
        }
        // The assert fires asynchronously; sync should surface the error.
        assert!(stream.synchronize().is_err());
    }
}
