//! Safe wrappers over the HIP driver/runtime API.
//!
//! Layout mirrors `cudarc::driver` so call sites in `luminal_rocm_lite`
//! can be near-identical to the existing CUDA paths.

use std::{marker::PhantomData, sync::{Arc, Weak}};

pub mod result;
pub mod sys;

#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error("HIP error: {0:?}")]
    Hip(sys::hipError_t),
    #[error("kernel name contained a null byte")]
    InvalidName,
}

#[derive(Debug, Clone, Copy)]
pub enum DeviceFlags {
    Auto,
    Spin,
    Yield,
    BlockingSync
}

impl DeviceFlags {
    // Values from hip_runtime_api.h. Bindgen drops `#define` macros, so the
    // hipDeviceSchedule* names aren't in `sys`; redeclare the bits here.
    fn to_raw(self) -> core::ffi::c_uint {
        match self {
            Self::Auto         => 0x0,
            Self::Spin         => 0x1,
            Self::Yield        => 0x2,
            Self::BlockingSync => 0x4,
        }
    }
}

/// A HIP device + its default stream. Analogue of `cudarc::driver::CudaContext`.
///
/// HIP uses implicit per-thread contexts (like CUDA's primary context), so
/// this struct just bundles `(device_ordinal, default_stream, arch)`.
pub struct HipContext {
    ordinal: i32,
    default_stream: Arc<HipStream>,
    gfx_arch: String,
    flags: DeviceFlags,
}

impl HipContext {
    /// Bind to the given device and create a default stream.
    pub fn new(ordinal: i32) -> Result<Arc<Self>, DriverError> {
        Self::new_with_flags(ordinal, DeviceFlags::BlockingSync)
    }

    pub fn new_with_flags(ordinal: i32, flags: DeviceFlags) -> Result<Arc<Self>, DriverError> {
        result::set_device(ordinal)?;
        result::set_device_flags(flags.to_raw())?;   // MUST be before stream_create
        let gfx_arch = result::device_gcn_arch_name(ordinal)?;
        // Fallible resource (stream) before the cyclic construction so we can
        // bubble errors. The Arc::new_cyclic closure is infallible.
        let raw_stream = result::stream_create()?;
        let ctx = Arc::new_cyclic(|weak_ctx: &Weak<HipContext>| {
            let stream = Arc::new(HipStream {
                raw: raw_stream,
                device_ordinal: ordinal,
                ctx: weak_ctx.clone(),
            });
            HipContext { ordinal, default_stream: stream, gfx_arch, flags }
        });
        Ok(ctx)
    }

    /// Parse `gfx_arch` into a `(major, minor)` tuple mirroring cudarc's
    /// `CudaContext::compute_capability`. HIP has no real compute-capability
    /// concept — the gfx arch string is the analog — so this is a heuristic
    /// (consistent per arch, used by luminal for capability gating and kernel
    /// cache keys). Leading digits → major (1–2 digits), remainder → minor.
    /// Examples: `gfx1100` → `(11, 0)`, `gfx942` → `(9, 42)`, `gfx90a` → `(9, 10)`.
    pub fn compute_capability(&self) -> Result<(i32, i32), DriverError> {
        let arch = self.gfx_arch.strip_prefix("gfx").unwrap_or(&self.gfx_arch);
        let bytes = arch.as_bytes();
        // Major = leading 1 or 2 digits. Use 2 if both leading chars are digits.
        let major_len = if bytes.len() >= 2 && bytes[0].is_ascii_digit() && bytes[1].is_ascii_digit() {
            2
        } else if !bytes.is_empty() && bytes[0].is_ascii_digit() {
            1
        } else {
            0
        };
        let (major_str, minor_str) = arch.split_at(major_len);
        let major: i32 = major_str.parse().unwrap_or(0);
        // Minor may have a hex letter tail (e.g. "90a"); fall back to hex.
        let minor: i32 = minor_str
            .parse()
            .or_else(|_| i32::from_str_radix(minor_str, 16))
            .unwrap_or(0);
        Ok((major, minor))
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

    pub fn bind_to_thread(&self) -> Result<(), DriverError> {
        result::set_device(self.ordinal)?;
        // Best-effort: if the device is already initialized on this thread,
        // the flag set is a no-op per HIP docs. This keeps per-thread flag
        // settings consistent for threads that bind late.
        let _ = result::set_device_flags(self.flags.to_raw());
        Ok(())
    }

    /// Load a compiled hsaco code object as a [`HipModule`]. Mirror of
    /// cudarc's `CudaContext::load_module(Ptx)` so call sites can write
    /// `ctx.load_module(hsaco)`. Internally delegates to
    /// [`HipModule::from_hsaco`]; HIP module loading doesn't actually need
    /// a context handle, but the cudarc-shape API keeps the port mechanical.
    pub fn load_module(&self, hsaco: crate::hiprtc::Hsaco) -> Result<Arc<HipModule>, DriverError> {
        HipModule::from_hsaco(hsaco.as_bytes())
    }
}

/// A HIP stream. Analogue of `cudarc::driver::CudaStream`.
///
/// Holds a `Weak<HipContext>` back-reference so `context()` can return an
/// `Arc<HipContext>`. The owning `HipContext` keeps an `Arc<HipStream>`
/// for its default stream, so the Weak is what breaks the cycle.
pub struct HipStream {
    raw: sys::hipStream_t,
    #[allow(dead_code)]
    device_ordinal: i32,
    ctx: Weak<HipContext>,
}

impl HipStream {
    pub fn synchronize(&self) -> Result<(), DriverError> {
        result::stream_synchronize(self.raw)
    }

    /// Raw `hipStream_t`. Exposed so call sites can hand it to functions in
    /// `driver::result::*` or pass into vendor libraries.
    pub fn hip_stream(&self) -> sys::hipStream_t {
        self.raw
    }

    /// The `HipContext` that owns this stream. Panics if the context has
    /// been dropped, which shouldn't happen since the context owns the
    /// stream's `Arc` (so an `Arc<HipStream>` keeps the context alive
    /// transitively in normal use).
    pub fn context(&self) -> Arc<HipContext> {
        self.ctx.upgrade().expect("HipContext dropped before HipStream")
    }

    pub fn alloc<T>(self: &Arc<Self>, len: usize) -> Result<HipSlice<T>, DriverError> {
        HipSlice::alloc(self.clone(), len)
    }

    pub fn alloc_zeros<T>(self: &Arc<Self>, len: usize) -> Result<HipSlice<T>, DriverError> {
        let slice = self.alloc::<T>(len)?;
        if !slice.is_empty() {
            let bytes = slice.len * std::mem::size_of::<T>();
            unsafe { result::memset_d8_async(slice.device_ptr(self).0, 0, bytes, self.raw)?; }
        }
        Ok(slice)
    }

    /// Pass-through to [`HipSlice::clone_dtoh`] so call sites can write either
    /// `buf.clone_dtoh(&stream)` or `stream.clone_dtoh(&buf)`. cuda_lite uses
    /// both shapes.
    pub fn clone_dtoh<T: Copy>(
        self: &Arc<Self>,
        src: &HipSlice<T>,
    ) -> Result<Vec<T>, DriverError> {
        src.clone_dtoh(self)
    }

    /// Copy `src` into the existing device buffer `dst` on this stream.
    /// Does NOT synchronize — caller is responsible for ordering reads.
    /// Mirrors cudarc's `CudaStream::memcpy_htod`.
    pub fn memcpy_htod<T: Copy>(
        &self,
        src: &[T],
        dst: &mut HipSlice<T>,
    ) -> Result<(), DriverError> {
        if src.is_empty() { return Ok(()); }
        assert!(src.len() <= dst.len, "memcpy_htod: src.len > dst.len");
        let bytes = std::mem::size_of_val(src);
        let src_bytes = unsafe {
            std::slice::from_raw_parts(src.as_ptr() as *const u8, bytes)
        };
        unsafe { result::memcpy_htod_async(dst.ptr, src_bytes, self.raw) }
    }

    /// Copy from `src` device buffer into the existing host slice `dst` on
    /// this stream. Does NOT synchronize. Mirrors cudarc's
    /// `CudaStream::memcpy_dtoh`.
    pub fn memcpy_dtoh<T: Copy>(
        &self,
        src: &HipSlice<T>,
        dst: &mut [T],
    ) -> Result<(), DriverError> {
        if dst.is_empty() { return Ok(()); }
        assert!(dst.len() <= src.len, "memcpy_dtoh: dst.len > src.len");
        let bytes = std::mem::size_of_val(dst);
        let dst_bytes = unsafe {
            std::slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u8, bytes)
        };
        unsafe { result::memcpy_dtoh_async(dst_bytes, src.ptr, self.raw) }
    }

    /// Allocate a device buffer and copy `src` into it on this stream.
    /// Synchronizes the stream before returning so the buffer is safe to use.
    pub fn clone_htod<T: Copy>(
        self: &Arc<Self>,
        src: &[T],
    ) -> Result<HipSlice<T>, DriverError> {
        let slice = self.alloc::<T>(src.len())?;
        if !src.is_empty() {
            let bytes = std::mem::size_of_val(src);
            let src_bytes = unsafe {
                std::slice::from_raw_parts(src.as_ptr() as *const u8, bytes)
            };
            unsafe { result::memcpy_htod_async(slice.ptr, src_bytes, self.raw)?; }
            self.synchronize()?;
        }
        Ok(slice)
    }

    /// Wrap an externally-owned device pointer in a `HipSlice<T>` without
    /// taking ownership. The returned slice will *not* free `ptr` on drop —
    /// the original owner is responsible for that.
    ///
    /// # Safety
    /// Caller must ensure `ptr` points to at least `len * size_of::<T>()`
    /// bytes of valid device memory in this stream's context, and that the
    /// pointer outlives every use of the returned slice.
    pub unsafe fn upgrade_device_ptr<T>(
        self: &Arc<Self>,
        ptr: u64,
        len: usize,
    ) -> HipSlice<T> {
        HipSlice {
            ptr,
            len,
            stream: self.clone(),
            owned: false,
            _marker: PhantomData,
        }
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

// Manual Debug impls: HipContext ⇄ HipStream form a (strong, Weak) cycle, so
// `#[derive(Debug)]` would infinite-recurse. Printing the leaf fields is
// enough for log/error messages.
impl std::fmt::Debug for HipStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HipStream")
            .field("raw", &self.raw)
            .field("device_ordinal", &self.device_ordinal)
            .finish()
    }
}

impl std::fmt::Debug for HipContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HipContext")
            .field("ordinal", &self.ordinal)
            .field("gfx_arch", &self.gfx_arch)
            .field("flags", &self.flags)
            .finish()
    }
}

/// Typed RAII device buffer. Analogue of `cudarc::driver::CudaSlice<T>`.
///
/// Holds an `Arc<HipStream>` so the stream outlives any buffer that schedules
/// cleanup against it.
pub struct HipSlice<T> {
    pub(crate) ptr: u64,
    pub(crate) len: usize,
    #[allow(dead_code)]
    pub(crate) stream: Arc<HipStream>,
    /// `true` for buffers allocated by rocmrc (Drop frees them).
    /// `false` for buffers wrapped via [`HipStream::upgrade_device_ptr`]
    /// (Drop is a no-op — caller owns the lifetime).
    pub(crate) owned: bool,
    pub(crate) _marker: PhantomData<*const T>,
}

impl<T> HipSlice<T> {
    fn alloc(stream: Arc<HipStream>, len: usize) -> Result<Self, DriverError> {
        let bytes = len.checked_mul(std::mem::size_of::<T>()).expect("size overflow");
        let ptr = if bytes == 0 { 0 } else { result::malloc(bytes)? };
        Ok(Self {
            ptr,
            len,
            stream,
            owned: true,
            _marker: PhantomData,
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the device pointer of this buffer, scoped to the given stream.
    ///
    /// The stream argument is currently ignored — rocmrc does not yet enforce
    /// stream-ordered access. The unit-type second tuple element is a
    /// placeholder for a future `SyncOnDrop`-style guard. Mirrors cudarc's
    /// `CudaSlice::device_ptr` so call sites can use `.device_ptr(&s).0`.
    pub fn device_ptr(&self, _stream: &Arc<HipStream>) -> (u64, ()) {
        (self.ptr, ())
    }
}

impl<T: Copy> HipSlice<T> {
    /// Copy the contents back to a host `Vec<T>` on `stream`.
    /// Synchronizes the stream before returning so the host data is valid.
    pub fn clone_dtoh(&self, stream: &Arc<HipStream>) -> Result<Vec<T>, DriverError> {
        let mut out: Vec<T> = Vec::with_capacity(self.len);
        if self.len > 0 {
            let bytes = self.len * std::mem::size_of::<T>();
            let out_bytes = unsafe {
                std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, bytes)
            };
            unsafe { result::memcpy_dtoh_async(out_bytes, self.ptr, stream.raw)?; }
            stream.synchronize()?;
            unsafe { out.set_len(self.len); }
        }
        Ok(out)
    }
}

impl<T> Drop for HipSlice<T> {
    fn drop(&mut self) {
        if self.owned && self.ptr != 0 {
            let _ = result::free(self.ptr);
        }
    }
}

unsafe impl<T: Send> Send for HipSlice<T> {}
unsafe impl<T: Sync> Sync for HipSlice<T> {}

// Manual Debug to avoid requiring T: Debug (PhantomData<*const T> would force
// that bound under `#[derive(Debug)]`).
impl<T> std::fmt::Debug for HipSlice<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HipSlice")
            .field("ptr", &self.ptr)
            .field("len", &self.len)
            .field("owned", &self.owned)
            .finish()
    }
}

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

    /// Alias for [`Self::get_function`] matching cudarc's
    /// `CudaModule::load_function`.
    pub fn load_function(self: &Arc<Self>, name: &str) -> Result<HipFunction, DriverError> {
        self.get_function(name)
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
/// Analogue of `cudarc::driver::CudaFunction`. `Clone`-able — the raw handle
/// is just a pointer and the `Arc` bump keeps the owning module alive.
#[derive(Clone)]
pub struct HipFunction {
    raw: sys::hipFunction_t,
    _module: Arc<HipModule>,
}

impl HipFunction {
    /// Raw `hipFunction_t`. Exposed so call sites can hand the bare handle
    /// to HIP graph node-creation FFI (`hipGraphAddKernelNode` etc.) that
    /// rocmrc doesn't wrap yet. Mirrors cudarc's `CudaFunction::cu_function`.
    pub fn raw(&self) -> sys::hipFunction_t {
        self.raw
    }

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

impl std::fmt::Debug for HipFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HipFunction").field("raw", &self.raw).finish()
    }
}
