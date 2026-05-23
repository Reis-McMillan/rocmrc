//! Safe wrappers over the HIP driver/runtime API.
//!
//! Layout mirrors `cudarc::driver` so call sites in `luminal_rocm_lite`
//! can be near-identical to the existing CUDA paths.

use std::{marker::PhantomData, sync::Arc};

pub mod result;
pub mod sys;

#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error("HIP error code {0}")]
    Hip(sys::hipError_t),
    #[error("kernel name contained a null byte")]
    InvalidName,
}

/// A HIP device + its default stream. Analogue of `cudarc::driver::CudaContext`.
///
/// HIP uses implicit per-thread contexts (like CUDA's primary context), so
/// this struct just bundles `(device_ordinal, default_stream, arch)`.
pub struct HipContext {
    ordinal: i32,
    default_stream: Arc<HipStream>,
    /// gfx arch string passed to hipRTC (e.g. `"gfx1100"`).
    ///
    /// Provided explicitly for now; future work: read `gcnArchName` from
    /// `hipGetDeviceProperties` via bindgen-generated `hipDeviceProp_t`.
    gfx_arch: String,
}

impl HipContext {
    /// Bind to the given device and create a default stream.
    ///
    /// `gfx_arch` is the target architecture string for hipRTC compilation
    /// (e.g. `"gfx1100"` for RDNA3, `"gfx942"` for MI300X). Until we wire
    /// up `hipGetDeviceProperties`, the caller passes it in. Strip any
    /// `:sramecc-:xnack-` suffix before passing.
    pub fn new(ordinal: i32, gfx_arch: impl Into<String>) -> Result<Arc<Self>, DriverError> {
        result::set_device(ordinal)?;
        let stream = Arc::new(HipStream::new_on_device(ordinal)?);
        Ok(Arc::new(Self {
            ordinal,
            default_stream: stream,
            gfx_arch: gfx_arch.into(),
        }))
    }

    pub fn ordinal(&self) -> i32 {
        self.ordinal
    }

    pub fn default_stream(&self) -> Arc<HipStream> {
        self.default_stream.clone()
    }

    pub fn gfx_arch(&self) -> &str {
        &self.gfx_arch
    }

    /// Best-effort device name from `hipDeviceGetName`.
    pub fn name(&self) -> Result<String, DriverError> {
        result::device_name(self.ordinal)
    }

    /// Allocate a typed device buffer of `len` elements, bound to the
    /// default stream for cleanup.
    pub fn alloc<T>(self: &Arc<Self>, len: usize) -> Result<HipSlice<T>, DriverError> {
        HipSlice::alloc(self.default_stream.clone(), len)
    }
}

/// A HIP stream. Analogue of `cudarc::driver::CudaStream`.
pub struct HipStream {
    raw: sys::hipStream_t,
    #[allow(dead_code)]
    device_ordinal: i32,
}

impl HipStream {
    fn new_on_device(ordinal: i32) -> Result<Self, DriverError> {
        result::set_device(ordinal)?;
        let raw = result::stream_create()?;
        Ok(Self {
            raw,
            device_ordinal: ordinal,
        })
    }

    pub fn synchronize(&self) -> Result<(), DriverError> {
        result::stream_synchronize(self.raw)
    }

    /// Raw `hipStream_t`. Exposed so call sites can hand it to functions in
    /// `driver::result::*` or pass into vendor libraries.
    pub fn hip_stream(&self) -> sys::hipStream_t {
        self.raw
    }
}

impl Drop for HipStream {
    fn drop(&mut self) {
        let _ = result::stream_destroy(self.raw);
    }
}

// HIP handles are usable from any thread once the device is set.
unsafe impl Send for HipStream {}
unsafe impl Sync for HipStream {}

/// Typed RAII device buffer. Analogue of `cudarc::driver::CudaSlice<T>`.
///
/// Holds an `Arc<HipStream>` so the stream outlives any buffer that schedules
/// cleanup against it.
pub struct HipSlice<T> {
    ptr: u64,
    len: usize,
    #[allow(dead_code)]
    stream: Arc<HipStream>,
    _marker: PhantomData<*const T>,
}

impl<T> HipSlice<T> {
    fn alloc(stream: Arc<HipStream>, len: usize) -> Result<Self, DriverError> {
        let bytes = len.checked_mul(std::mem::size_of::<T>()).expect("size overflow");
        let ptr = if bytes == 0 { 0 } else { result::malloc(bytes)? };
        Ok(Self {
            ptr,
            len,
            stream,
            _marker: PhantomData,
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn device_ptr(&self) -> u64 {
        self.ptr
    }
}

impl<T> Drop for HipSlice<T> {
    fn drop(&mut self) {
        if self.ptr != 0 {
            let _ = result::free(self.ptr);
        }
    }
}

unsafe impl<T: Send> Send for HipSlice<T> {}
unsafe impl<T: Sync> Sync for HipSlice<T> {}

/// Trait for "anything with a device pointer" — lets call sites in
/// `luminal_rocm_lite` be generic over `HipSlice<T>` / bare `u64` / etc.
/// Analogue of `cudarc::driver::DevicePtr`.
pub trait DevicePtr {
    fn device_ptr(&self) -> u64;
}

impl<T> DevicePtr for HipSlice<T> {
    fn device_ptr(&self) -> u64 {
        self.ptr
    }
}

/// A loaded code-object module. Analogue of `cudarc::driver::CudaModule`.
pub struct HipModule {
    raw: sys::hipModule_t,
}

impl HipModule {
    /// Load a compiled hsaco code-object blob (typically produced by
    /// `hiprtc::compile`).
    pub fn from_hsaco(bytes: &[u8]) -> Result<Arc<Self>, DriverError> {
        let raw = result::module_load_data(bytes)?;
        Ok(Arc::new(Self { raw }))
    }

    pub fn get_function(self: &Arc<Self>, name: &str) -> Result<HipFunction, DriverError> {
        let raw = result::module_get_function(self.raw, name)?;
        Ok(HipFunction {
            raw,
            _module: self.clone(),
        })
    }
}

impl Drop for HipModule {
    fn drop(&mut self) {
        let _ = result::module_unload(self.raw);
    }
}

unsafe impl Send for HipModule {}
unsafe impl Sync for HipModule {}

/// A kernel function inside a loaded module. Keeps the module alive via `Arc`.
/// Analogue of `cudarc::driver::CudaFunction`.
pub struct HipFunction {
    raw: sys::hipFunction_t,
    _module: Arc<HipModule>,
}

impl HipFunction {
    /// Launch the kernel.
    ///
    /// # Safety
    /// `params` must hold pointers to live argument values whose count and
    /// types match the kernel signature.
    pub unsafe fn launch(
        &self,
        grid: (u32, u32, u32),
        block: (u32, u32, u32),
        shared_mem_bytes: u32,
        stream: &HipStream,
        params: &mut [*mut std::ffi::c_void],
    ) -> Result<(), DriverError> {
        unsafe {
            result::module_launch_kernel(
                self.raw,
                grid,
                block,
                shared_mem_bytes,
                stream.hip_stream(),
                params,
            )
        }
    }
}

unsafe impl Send for HipFunction {}
unsafe impl Sync for HipFunction {}
