//! `Result`-wrapped FFI over `hip_runtime_api.h`.
//!
//! HIP intentionally unifies what CUDA splits across two distinct libraries:
//! the **driver API** (`cu*`, opaque handles, `cuda.h` — wrapped by
//! [`cudarc::driver::result`]) and the **runtime API** (`cuda*`, int
//! ordinals, `cuda_runtime.h` — wrapped by [`cudarc::runtime::result`]).
//! HIP collapses both into one header with no usage rule preventing you from
//! mixing styles — `hipMalloc` (runtime-flavored) and `hipModuleLaunchKernel`
//! (driver-flavored) are first-class siblings. So this single `hip::result`
//! module covers both halves; there is no cleanly-separable "driver-only" or
//! "runtime-only" HIP subset to factor out.
//!
//! **Structure and naming mirror cudarc** wherever the parallel is exact —
//! submodule layout, function shapes, out-pointer → `Result<T, HipError>`
//! idiom, `.result()?` propagation. Driver-flavored entries follow
//! `cudarc::driver::result`'s shape; runtime-flavored entries follow
//! `cudarc::runtime::result`'s.
//!
//! **Divergences from cudarc are flagged inline.** Two kinds:
//! - *rocmrc-only entries* — HIP exposes a primitive cudarc doesn't,
//!   because neither CUDA half has one (e.g. [`device::set`] wrapping
//!   `hipSetDevice`). Documented at the function/module level.
//! - *Architectural mismatches* — HIP's runtime-only function-attribute
//!   setters take a host `__global__` symbol pointer rather than a
//!   `hipFunction_t`, and are unreachable from `hipModuleGetFunction`
//!   handles. See the [`function`] module doc.

pub use super::{sys};
use std::ffi::{CStr, c_int, c_uint, c_void};


pub struct HipError(pub sys::hipError_t);

impl sys::hipError_t {
    #[inline]
    pub fn result(self) -> Result<(), HipError> {
        match self {
           sys::hipError_t::hipSuccess => Ok(()),
           _ => Err(HipError(self)),
        }
    }
}

impl HipError {
    pub fn error_name(&self) -> &'static CStr {
        unsafe { CStr::from_ptr(sys::hipGetErrorName(self.0)) }
    }

    pub fn error_string(&self) -> &'static CStr {
        unsafe { CStr::from_ptr(sys::hipGetErrorString(self.0)) }
    }
}

impl std::fmt::Debug for HipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HipError")
        .field(&self.0)
        .field(&self.error_string())
        .finish()
    }
}

impl std::fmt::Display for HipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for HipError {}

pub fn init() -> Result<(), HipError> {
    unsafe { sys::hipInit(0).result() }
}

pub mod version {
    use super::{
        sys::{self},
        HipError,
    };
    use std::ffi::c_int;

    /// The HIP driver API version (e.g. `60304000` for 6.3.4).
    pub fn driver() -> Result<i32, HipError> {
        let mut v: c_int = 0;
        unsafe { sys::hipDriverGetVersion(&mut v).result()? };
        Ok(v)
    }

    /// The HIP runtime API version. On HIP these often equal [`driver`].
    pub fn runtime() -> Result<i32, HipError> {
        let mut v: c_int = 0;
        unsafe { sys::hipRuntimeGetVersion(&mut v).result()? };
        Ok(v)
    }
}

pub mod device {
    use super::{
        sys::{self},
        HipError,
    };
    use std::{
        ffi::{c_int, c_uint, CStr},
        string::String,
    };

    pub fn get(ordinal: c_int) -> Result<sys::hipDevice_t, HipError> {
        let mut dev: sys::hipDevice_t = 0;
        unsafe { sys::hipDeviceGet(&mut dev, ordinal).result()? };
        Ok(dev)
    }

    /// Bind `ordinal` as the calling thread's current device. **rocmrc-only:**
    /// cudarc has no equivalent; CUDA forces callers through
    /// `ctx::set_current(CUcontext)`. HIP's runtime-API `hipSetDevice(int)`
    /// is the idiomatic AMD path and avoids the primary-context ceremony on
    /// a backend where `hipCtx_t` is largely vestigial.
    pub fn set(ordinal: c_int) -> Result<(), HipError> {
        unsafe { sys::hipSetDevice(ordinal).result() }
    }

    pub fn get_count() -> Result<c_int, HipError> {
        let mut n: c_int = 0;
        unsafe { sys::hipGetDeviceCount(&mut n).result()? };
        Ok(n)
    }

    pub fn total_mem(dev: sys::hipDevice_t) -> Result<usize, HipError> {
        let mut bytes: usize = 0;
        unsafe { sys::hipDeviceTotalMem(&mut bytes, dev).result()? };
        Ok(bytes)
    }

    pub fn get_attribute(
        dev: sys::hipDevice_t,
        attr: sys::hipDeviceAttribute_t,
    ) -> Result<i32, HipError> {
        let mut val: c_int = 0;
        unsafe { sys::hipDeviceGetAttribute(&mut val, attr, dev).result()? };
        Ok(val)
    }

    pub fn get_name(dev: sys::hipDevice_t) -> Result<String, HipError> {
        let mut buf = [0 as core::ffi::c_char; 256];
        unsafe {
            sys::hipDeviceGetName(buf.as_mut_ptr(), buf.len() as c_int, dev).result()?
        };
        let cstr = unsafe { CStr::from_ptr(buf.as_ptr()) };
        Ok(cstr.to_string_lossy().into_owned())
    }

    pub fn get_uuid(dev: sys::hipDevice_t) -> Result<sys::hipUUID, HipError> {
        let mut uuid: sys::hipUUID = unsafe { std::mem::zeroed() };
        unsafe { sys::hipDeviceGetUuid(&mut uuid, dev).result()? };
        Ok(uuid)
    }

    pub fn get_default_mem_pool(dev: sys::hipDevice_t) -> Result<sys::hipMemPool_t, HipError> {
        let mut mem_pool: sys::hipMemPool_t = std::ptr::null_mut();
        unsafe { sys::hipDeviceGetDefaultMemPool(&mut mem_pool, dev).result()? };
        Ok(mem_pool)
    }

    pub fn get_mem_pool(dev: sys::hipDevice_t) -> Result<sys::hipMemPool_t, HipError> {
        let mut mem_pool: sys::hipMemPool_t = std::ptr::null_mut();
        unsafe { sys::hipDeviceGetMemPool(&mut mem_pool, dev).result()? };
        Ok(mem_pool)
    }

    pub fn set_mem_pool(
        dev: sys::hipDevice_t,
        mem_pool: sys::hipMemPool_t,
    ) -> Result<(), HipError> {
        unsafe { sys::hipDeviceSetMemPool(dev, mem_pool).result() }
    }

    /// HIP scopes limits to the device, not a context (cudarc's
    /// `ctx::get_limit` / `set_limit` live here instead).
    pub fn get_limit(limit: sys::hipLimit_t) -> Result<usize, HipError> {
        let mut value: usize = 0;
        unsafe { sys::hipDeviceGetLimit(&mut value, limit).result()? };
        Ok(value)
    }

    pub fn set_limit(limit: sys::hipLimit_t, value: usize) -> Result<(), HipError> {
        unsafe { sys::hipDeviceSetLimit(limit, value).result() }
    }

    /// The ordinal of the device currently bound to the calling thread.
    /// Wraps `hipGetDevice`. (Note the asymmetry with [`get`], which
    /// resolves an ordinal to a `hipDevice_t` handle.)
    pub fn current() -> Result<c_int, HipError> {
        let mut ordinal: c_int = 0;
        unsafe { sys::hipGetDevice(&mut ordinal).result()? };
        Ok(ordinal)
    }

    /// Destroy the current device's primary context, freeing all
    /// resources held by it. Subsequent HIP calls on the calling thread
    /// re-initialize a fresh primary context.
    ///
    /// # Safety
    /// Pending work, live allocations, modules, streams, and events on
    /// the current device are *invalidated*. Caller must ensure nothing
    /// downstream tries to use them after this returns.
    pub unsafe fn reset() -> Result<(), HipError> {
        unsafe { sys::hipDeviceReset().result() }
    }

    /// Full device property struct (versioned R0600 layout). Wraps
    /// `hipGetDevicePropertiesR0600`. The struct is large; prefer
    /// [`get_attribute`] for single-field queries on hot paths.
    pub fn get_properties(ordinal: c_int) -> Result<sys::hipDeviceProp_tR0600, HipError> {
        let mut prop: sys::hipDeviceProp_tR0600 = unsafe { std::mem::zeroed() };
        unsafe { sys::hipGetDevicePropertiesR0600(&mut prop, ordinal).result()? };
        Ok(prop)
    }

    pub fn enable_peer_access(peer_id: c_int, flags: c_uint) -> Result<(), HipError> {
        unsafe { sys::hipDeviceEnablePeerAccess(peer_id, flags).result()? };
        Ok(())
    }

    pub fn can_access_peer(
        device_id: c_int,
        peer_id: c_int
    ) -> Result<bool, HipError> {
        let mut can_access: i32 = 0;
        unsafe { sys::hipDeviceCanAccessPeer(
            &mut can_access,
            device_id,
            peer_id
        ).result()? };
        Ok(can_access != 0)
    } 
}

/// Function attributes — driver-API reads only.
///
/// **HIP doesn't expose driver-API setters.** The only HIP setter entries
/// (`hipFuncSetAttribute`, `hipFuncSetCacheConfig`) are *runtime-API* and
/// take a host-side `__global__` symbol pointer (`*const c_void`), not a
/// `hipFunction_t`. CUDA exposes both forms; HIP only the runtime form.
///
/// Since `driver/result.rs` is strictly the driver-API surface, those
/// setters are not wrapped here. The consequence is real: kernels loaded
/// via `hipModuleGetFunction` (the rocmrc happy path through hipRTC →
/// hsaco → module) have no associated host symbol, so even the runtime-API
/// setters are unreachable. The `HipFunction` safe wrapper in
/// `driver/mod.rs` therefore exposes only `get_attribute`.
///
/// If you genuinely have a host-linked `__global__` symbol and need to set
/// its max-dynamic-smem or shared-memory carveout, call
/// [`sys::hipFuncSetAttribute`] / [`sys::hipFuncSetCacheConfig`] directly
/// with the host pointer — those are the only two attributes the write
/// surface (`hipFuncAttribute`) supports.
pub mod function {
    use super::{
        sys::{self, hipFunction_attribute},
        HipError,
    };
    use std::ffi::c_int;

    /// Driver-API read against a launched `hipFunction_t`.
    pub fn get_function_attribute(
        f: sys::hipFunction_t,
        attribute: hipFunction_attribute,
    ) -> Result<i32, HipError> {
        let mut value: c_int = 0;
        unsafe { sys::hipFuncGetAttribute(&mut value, attribute, f).result()? };
        Ok(value)
    }
}

pub mod occupancy {
    use super::{
        sys::{self},
        HipError,
    };
    use std::ffi::{c_int, c_uint, c_void};

    pub fn max_active_blocks_per_multiprocessor(
        f: sys::hipFunction_t,
        block_size: c_int,
        dynamic_smem_bytes: usize,
    ) -> Result<i32, HipError> {
        let mut n: c_int = 0;
        unsafe {
            sys::hipModuleOccupancyMaxActiveBlocksPerMultiprocessor(
                &mut n,
                f,
                block_size,
                dynamic_smem_bytes,
            )
            .result()?
        };
        Ok(n)
    }

    pub fn max_active_blocks_per_multiprocessor_with_flags(
        f: sys::hipFunction_t,
        block_size: c_int,
        dynamic_smem_bytes: usize,
        flags: c_uint,
    ) -> Result<i32, HipError> {
        let mut n: c_int = 0;
        unsafe {
            sys::hipModuleOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
                &mut n,
                f,
                block_size,
                dynamic_smem_bytes,
                flags,
            )
            .result()?
        };
        Ok(n)
    }

    /// Returns `(min_grid_size, block_size)` — the smallest grid that saturates
    /// the device and the block size that achieves it.
    pub fn max_potential_block_size(
        f: sys::hipFunction_t,
        dynamic_smem_bytes: usize,
        block_size_limit: c_int,
    ) -> Result<(i32, i32), HipError> {
        let mut grid: c_int = 0;
        let mut block: c_int = 0;
        unsafe {
            sys::hipModuleOccupancyMaxPotentialBlockSize(
                &mut grid,
                &mut block,
                f,
                dynamic_smem_bytes,
                block_size_limit,
            )
            .result()?
        };
        Ok((grid, block))
    }

    pub fn max_potential_block_size_with_flags(
        f: sys::hipFunction_t,
        dynamic_smem_bytes: usize,
        block_size_limit: c_int,
        flags: c_uint,
    ) -> Result<(i32, i32), HipError> {
        let mut grid: c_int = 0;
        let mut block: c_int = 0;
        unsafe {
            sys::hipModuleOccupancyMaxPotentialBlockSizeWithFlags(
                &mut grid,
                &mut block,
                f,
                dynamic_smem_bytes,
                block_size_limit,
                flags,
            )
            .result()?
        };
        Ok((grid, block))
    }

    /// **HIP discrepancy:** there is no driver-API variant of
    /// `AvailableDynamicSMemPerBlock`. The only HIP export is runtime-API and
    /// takes a host `__global__` symbol pointer rather than a `hipFunction_t`.
    ///
    /// # Safety
    /// `f` must be a valid pointer to a HIP `__global__` symbol.
    pub unsafe fn available_dynamic_shared_mem_per_block(
        f: *const c_void,
        num_blocks: c_int,
        block_size: c_int,
    ) -> Result<usize, HipError> {
        let mut bytes: usize = 0;
        unsafe {
            sys::hipOccupancyAvailableDynamicSMemPerBlock(
                &mut bytes,
                f,
                num_blocks,
                block_size,
            )
            .result()?
        };
        Ok(bytes)
    }
}

pub mod primary_ctx {
    use super::{
        sys::{self},
        HipError,
    };

    pub fn retain(dev: sys::hipDevice_t) -> Result<sys::hipCtx_t, HipError> {
        let mut ctx: sys::hipCtx_t = std::ptr::null_mut();
        unsafe { sys::hipDevicePrimaryCtxRetain(&mut ctx, dev).result()? };
        Ok(ctx)
    }

    pub fn release(dev: sys::hipDevice_t) -> Result<(), HipError> {
        unsafe { sys::hipDevicePrimaryCtxRelease(dev).result() }
    }
}

pub mod ctx {
    use super::{
        sys::{self},
        HipError,
    };
    use std::ffi::c_uint;

    pub fn create(
        dev: sys::hipDevice_t,
        flags: c_uint,
    ) -> Result<sys::hipCtx_t, HipError> {
        let mut ctx: sys::hipCtx_t = std::ptr::null_mut();
        unsafe { sys::hipCtxCreate(&mut ctx, flags, dev).result()? };
        Ok(ctx)
    }

    pub fn set_current(ctx: sys::hipCtx_t) -> Result<(), HipError> {
        unsafe { sys::hipCtxSetCurrent(ctx).result() }
    }

    /// `None` when no context is current on the calling thread —
    /// `hipCtxGetCurrent` reports this by writing a NULL handle through `pctx`.
    pub fn get_current() -> Result<Option<sys::hipCtx_t>, HipError> {
        let mut ctx: sys::hipCtx_t = std::ptr::null_mut();
        unsafe { sys::hipCtxGetCurrent(&mut ctx).result()? };
        Ok(if ctx.is_null() { None } else { Some(ctx) })
    }

    pub fn synchronize() -> Result<(), HipError> {
        unsafe { sys::hipCtxSynchronize().result() }
    }

    pub fn get_cache_config() -> Result<sys::hipFuncCache_t, HipError> {
        let mut config: sys::hipFuncCache_t =
            sys::hipFuncCache_t::hipFuncCachePreferNone;
        unsafe { sys::hipCtxGetCacheConfig(&mut config).result()? };
        Ok(config)
    }

    pub fn set_cache_config(config: sys::hipFuncCache_t) -> Result<(), HipError> {
        unsafe { sys::hipCtxSetCacheConfig(config).result() }
    }
}

pub mod stream {
    use super::{
        sys::{self},
        HipError,
    };
    use std::ffi::{c_int, c_uint};

    /// Stream-creation flag. Mirrors [`cudarc::driver::result::stream::StreamKind`]
    /// (and the runtime parallel). HIP exposes the underlying values as
    /// `#define`s (`hipStreamDefault = 0x0`, `hipStreamNonBlocking = 0x1`)
    /// which bindgen drops — same precedent as `EventWaitFlags` /
    /// `MemAttachFlags`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StreamKind {
        /// `hipStreamDefault` — implicit synchronization with the NULL
        /// stream (legacy behavior).
        Default,
        /// `hipStreamNonBlocking` — no implicit synchronization with the
        /// NULL stream; work may run concurrently with stream 0.
        NonBlocking,
    }

    impl StreamKind {
        #[inline]
        pub fn to_raw(self) -> c_uint {
            match self {
                Self::Default => 0x0,
                Self::NonBlocking => 0x1,
            }
        }
    }

    pub fn create(kind: StreamKind) -> Result<sys::hipStream_t, HipError> {
        let mut s: sys::hipStream_t = std::ptr::null_mut();
        unsafe { sys::hipStreamCreateWithFlags(&mut s, kind.to_raw()).result()? };
        Ok(s)
    }

    pub fn create_with_priority(
        kind: StreamKind,
        priority: c_int,
    ) -> Result<sys::hipStream_t, HipError> {
        let mut s: sys::hipStream_t = std::ptr::null_mut();
        unsafe {
            sys::hipStreamCreateWithPriority(&mut s, kind.to_raw(), priority).result()?
        };
        Ok(s)
    }

    pub fn destroy(s: sys::hipStream_t) -> Result<(), HipError> {
        unsafe { sys::hipStreamDestroy(s).result() }
    }

    pub fn synchronize(s: sys::hipStream_t) -> Result<(), HipError> {
        unsafe { sys::hipStreamSynchronize(s).result() }
    }

    /// Make `s` wait on `event` before issuing further work.
    ///
    /// `flags` is `0` (`hipEventWaitDefault`) in the common case;
    /// `hipEventWaitExternal` (`0x1`) only takes effect during graph capture.
    /// The constants are `#define`s in HIP, so they don't appear in
    /// [`super::sys`] — see `EventWaitFlags` in `driver/mod.rs`.
    ///
    /// # Safety
    /// `s` and `event` must both be live (not destroyed).
    pub unsafe fn wait_event(
        s: sys::hipStream_t,
        event: sys::hipEvent_t,
        flags: c_uint,
    ) -> Result<(), HipError> {
        unsafe { sys::hipStreamWaitEvent(s, event, flags).result() }
    }

    /// `Ok(true)` if every prior op on `s` has completed; `Ok(false)` if work
    /// is still pending (HIP signals this via `hipErrorNotReady`, which we map
    /// to a boolean rather than a `HipError`).
    pub fn query(s: sys::hipStream_t) -> Result<bool, HipError> {
        match unsafe { sys::hipStreamQuery(s) } {
            sys::hipError_t::hipSuccess => Ok(true),
            sys::hipError_t::hipErrorNotReady => Ok(false),
            e => Err(HipError(e)),
        }
    }

    pub fn get_flags(s: sys::hipStream_t) -> Result<c_uint, HipError> {
        let mut flags: c_uint = 0;
        unsafe { sys::hipStreamGetFlags(s, &mut flags).result()? };
        Ok(flags)
    }

    pub fn get_priority(s: sys::hipStream_t) -> Result<c_int, HipError> {
        let mut p: c_int = 0;
        unsafe { sys::hipStreamGetPriority(s, &mut p).result()? };
        Ok(p)
    }

    /// Begin recording subsequent ops on `s` into a graph capture.
    ///
    /// # Safety
    /// `s` must be a live, non-default stream. `mode` controls how
    /// blocking sub-operations are diverted (Global / ThreadLocal / Relaxed).
    pub unsafe fn begin_capture(
        s: sys::hipStream_t,
        mode: sys::hipStreamCaptureMode,
    ) -> Result<(), HipError> {
        unsafe { sys::hipStreamBeginCapture(s, mode).result() }
    }

    /// Stop capturing on `s` and return the resulting graph handle. A
    /// null handle indicates capture was invalidated mid-flight.
    ///
    /// # Safety
    /// `s` must currently be capturing.
    pub unsafe fn end_capture(s: sys::hipStream_t) -> Result<sys::hipGraph_t, HipError> {
        let mut graph: sys::hipGraph_t = std::ptr::null_mut();
        unsafe { sys::hipStreamEndCapture(s, &mut graph).result()? };
        Ok(graph)
    }

    /// # Safety
    /// `s` must be a live stream handle.
    pub unsafe fn is_capturing(
        s: sys::hipStream_t,
    ) -> Result<sys::hipStreamCaptureStatus, HipError> {
        let mut status = sys::hipStreamCaptureStatus::hipStreamCaptureStatusNone;
        unsafe { sys::hipStreamIsCapturing(s, &mut status).result()? };
        Ok(status)
    }

    /// Attach a managed (unified) memory range to a stream.
    ///
    /// # Safety
    /// `dev_ptr` must point to a live managed allocation of at least
    /// `length` bytes. `s` must be a live stream (the legacy NULL stream
    /// is rejected by HIP). `flags` is one of
    /// `hipMemAttachGlobal` (0x1), `hipMemAttachHost` (0x2), or
    /// `hipMemAttachSingle` (0x4).
    pub unsafe fn attach_mem_async(
        s: sys::hipStream_t,
        dev_ptr: u64,
        length: usize,
        flags: c_uint,
    ) -> Result<(), HipError> {
        unsafe {
            sys::hipStreamAttachMemAsync(s, dev_ptr as *mut std::ffi::c_void, length, flags)
                .result()
        }
    }
}

// ----------------------------------------------------------------------------
// Flat memory ops. cudarc keeps malloc/free/memcpy at module root rather than
// in a submodule; this section follows that convention.
// ----------------------------------------------------------------------------

pub fn malloc_sync(num_bytes: usize) -> Result<u64, HipError> {
    let mut p: *mut c_void = std::ptr::null_mut();
    unsafe { sys::hipMalloc(&mut p, num_bytes).result()? };
    Ok(p as u64)
}

/// Allocates from the device's mempool. Requires `memoryPoolsSupported` to be
/// advertised on the device. Pair with [`free_async`] to stay stream-ordered.
pub fn malloc_async(num_bytes: usize, stream: sys::hipStream_t) -> Result<u64, HipError> {
    let mut p: *mut c_void = std::ptr::null_mut();
    unsafe { sys::hipMallocAsync(&mut p, num_bytes, stream).result()? };
    Ok(p as u64)
}

pub fn malloc_managed(num_bytes: usize, flags: c_uint) -> Result<u64, HipError> {
    let mut p: *mut c_void = std::ptr::null_mut();
    unsafe { sys::hipMallocManaged(&mut p, num_bytes, flags).result()? };
    Ok(p as u64)
}

/// Pinned host allocation. **HIP discrepancy:** the HIP entry is
/// `hipHostMalloc`, the symmetric counterpart of CUDA's `cuMemAllocHost`.
pub fn malloc_host(num_bytes: usize, flags: c_uint) -> Result<*mut c_void, HipError> {
    let mut p: *mut c_void = std::ptr::null_mut();
    unsafe { sys::hipHostMalloc(&mut p, num_bytes, flags).result()? };
    Ok(p)
}

pub fn free_host(ptr: *mut c_void) -> Result<(), HipError> {
    unsafe { sys::hipHostFree(ptr).result() }
}

pub fn free_sync(ptr: u64) -> Result<(), HipError> {
    unsafe { sys::hipFree(ptr as *mut c_void).result() }
}

pub fn free_async(ptr: u64, stream: sys::hipStream_t) -> Result<(), HipError> {
    unsafe { sys::hipFreeAsync(ptr as *mut c_void, stream).result() }
}

/// `device` here is the runtime-API ordinal (an `i32`), not a `hipDevice_t`
/// driver handle — `hipMemAdvise` is one of the few APIs that takes the int
/// form even on the driver side.
pub unsafe fn mem_advise(
    ptr: u64,
    bytes: usize,
    advice: sys::hipMemoryAdvise,
    device: c_int,
) -> Result<(), HipError> {
    unsafe {
        sys::hipMemAdvise(ptr as *const c_void, bytes, advice, device).result()
    }
}

pub unsafe fn mem_prefetch_async(
    ptr: u64,
    bytes: usize,
    device: c_int,
    stream: sys::hipStream_t,
) -> Result<(), HipError> {
    unsafe {
        sys::hipMemPrefetchAsync(ptr as *const c_void, bytes, device, stream).result()
    }
}

/// `(free, total)` bytes for the current device.
pub fn mem_get_info() -> Result<(usize, usize), HipError> {
    let mut free: usize = 0;
    let mut total: usize = 0;
    unsafe { sys::hipMemGetInfo(&mut free, &mut total).result()? };
    Ok((free, total))
}

pub unsafe fn memset_d8_sync(ptr: u64, value: u8, bytes: usize) -> Result<(), HipError> {
    unsafe { sys::hipMemsetD8(ptr as sys::hipDeviceptr_t, value, bytes).result() }
}

pub unsafe fn memset_d8_async(
    ptr: u64,
    value: u8,
    bytes: usize,
    stream: sys::hipStream_t,
) -> Result<(), HipError> {
    unsafe {
        sys::hipMemsetD8Async(ptr as sys::hipDeviceptr_t, value, bytes, stream).result()
    }
}

/// # Safety
/// `dst` must point to at least `src.len()` bytes of device memory owned by
/// the caller and valid for writes.
pub unsafe fn memcpy_htod_sync(dst: u64, src: &[u8]) -> Result<(), HipError> {
    unsafe {
        sys::hipMemcpyHtoD(
            dst as sys::hipDeviceptr_t,
            src.as_ptr() as *const c_void,
            src.len(),
        )
        .result()
    }
}

/// # Safety
/// `dst` must point to at least `src.len()` bytes of device memory owned by
/// the caller and valid for writes through `stream`.
pub unsafe fn memcpy_htod_async(
    dst: u64,
    src: &[u8],
    stream: sys::hipStream_t,
) -> Result<(), HipError> {
    unsafe {
        sys::hipMemcpyHtoDAsync(
            dst as sys::hipDeviceptr_t,
            src.as_ptr() as *const c_void,
            src.len(),
            stream,
        )
        .result()
    }
}

/// # Safety
/// `src` must point to at least `dst.len()` bytes of device memory readable
/// by the caller.
pub unsafe fn memcpy_dtoh_sync(dst: &mut [u8], src: u64) -> Result<(), HipError> {
    unsafe {
        sys::hipMemcpyDtoH(
            dst.as_mut_ptr() as *mut c_void,
            src as sys::hipDeviceptr_t,
            dst.len(),
        )
        .result()
    }
}

/// # Safety
/// `src` must point to at least `dst.len()` bytes of device memory readable
/// through `stream`. Caller is responsible for synchronizing before reading
/// `dst`.
pub unsafe fn memcpy_dtoh_async(
    dst: &mut [u8],
    src: u64,
    stream: sys::hipStream_t,
) -> Result<(), HipError> {
    unsafe {
        sys::hipMemcpyDtoHAsync(
            dst.as_mut_ptr() as *mut c_void,
            src as sys::hipDeviceptr_t,
            dst.len(),
            stream,
        )
        .result()
    }
}

/// # Safety
/// Both `dst` and `src` must point to at least `bytes` of device memory in the
/// same context.
pub unsafe fn memcpy_dtod_sync(dst: u64, src: u64, bytes: usize) -> Result<(), HipError> {
    unsafe {
        sys::hipMemcpyDtoD(
            dst as sys::hipDeviceptr_t,
            src as sys::hipDeviceptr_t,
            bytes,
        )
        .result()
    }
}

/// # Safety
/// Both `dst` and `src` must point to at least `bytes` of device memory in the
/// same context, reachable through `stream`.
pub unsafe fn memcpy_dtod_async(
    dst: u64,
    src: u64,
    bytes: usize,
    stream: sys::hipStream_t,
) -> Result<(), HipError> {
    unsafe {
        sys::hipMemcpyDtoDAsync(
            dst as sys::hipDeviceptr_t,
            src as sys::hipDeviceptr_t,
            bytes,
            stream,
        )
        .result()
    }
}

/// # Safety
/// `dst` / `src` must point to at least `bytes` of device memory on
/// `dst_device` / `src_device` respectively; both devices must have peer
/// access enabled (see [`crate::driver::sys::hipDeviceEnablePeerAccess`]).
pub unsafe fn memcpy_peer_async(
    dst: u64,
    dst_device: c_int,
    src: u64,
    src_device: c_int,
    bytes: usize,
    stream: sys::hipStream_t,
) -> Result<(), HipError> {
    unsafe {
        sys::hipMemcpyPeerAsync(
            dst as *mut c_void,
            dst_device,
            src as *const c_void,
            src_device,
            bytes,
            stream,
        )
        .result()
    }
}

pub mod module {
    use super::{
        sys::{self},
        HipError,
    };
    use std::ffi::{c_void, CStr};

    /// # Safety
    /// `image` must be a properly-formed hsaco / fatbin code-object blob.
    /// Rust can't verify this; passing arbitrary bytes is UB at the HIP layer.
    pub unsafe fn load_data(image: &[u8]) -> Result<sys::hipModule_t, HipError> {
        let mut m: sys::hipModule_t = std::ptr::null_mut();
        unsafe {
            sys::hipModuleLoadData(&mut m, image.as_ptr() as *const c_void).result()?
        };
        Ok(m)
    }

    /// # Safety
    /// `m` must not have already been unloaded.
    pub unsafe fn unload(m: sys::hipModule_t) -> Result<(), HipError> {
        unsafe { sys::hipModuleUnload(m).result() }
    }

    /// Take a `&CStr` because `HipError` no longer carries an `InvalidName`
    /// variant — callers with a `&str` should convert via `CString::new(...)`
    /// at the boundary.
    pub fn get_function(
        m: sys::hipModule_t,
        name: &CStr,
    ) -> Result<sys::hipFunction_t, HipError> {
        let mut f: sys::hipFunction_t = std::ptr::null_mut();
        unsafe { sys::hipModuleGetFunction(&mut f, m, name.as_ptr()).result()? };
        Ok(f)
    }

    /// Returns `(device_ptr, size_in_bytes)` for a `__device__` global by name.
    pub fn get_global(
        m: sys::hipModule_t,
        name: &CStr,
    ) -> Result<(u64, usize), HipError> {
        let mut ptr: sys::hipDeviceptr_t = 0 as sys::hipDeviceptr_t;
        let mut size: usize = 0;
        unsafe {
            sys::hipModuleGetGlobal(&mut ptr, &mut size, m, name.as_ptr()).result()?
        };
        Ok((ptr as u64, size))
    }
}

pub mod event {
    use super::{
        sys::{self},
        HipError,
    };
    use std::ffi::c_uint;

    pub fn create() -> Result<sys::hipEvent_t, HipError> {
        let mut e: sys::hipEvent_t = std::ptr::null_mut();
        unsafe { sys::hipEventCreate(&mut e).result()? };
        Ok(e)
    }

    pub fn create_with_flags(flags: c_uint) -> Result<sys::hipEvent_t, HipError> {
        let mut e: sys::hipEvent_t = std::ptr::null_mut();
        unsafe { sys::hipEventCreateWithFlags(&mut e, flags).result()? };
        Ok(e)
    }

    pub fn destroy(e: sys::hipEvent_t) -> Result<(), HipError> {
        unsafe { sys::hipEventDestroy(e).result() }
    }

    pub fn record(e: sys::hipEvent_t, stream: sys::hipStream_t) -> Result<(), HipError> {
        unsafe { sys::hipEventRecord(e, stream).result() }
    }

    pub fn record_with_flags(
        e: sys::hipEvent_t,
        stream: sys::hipStream_t,
        flags: c_uint,
    ) -> Result<(), HipError> {
        unsafe { sys::hipEventRecordWithFlags(e, stream, flags).result() }
    }

    pub fn synchronize(e: sys::hipEvent_t) -> Result<(), HipError> {
        unsafe { sys::hipEventSynchronize(e).result() }
    }

    /// Same `hipErrorNotReady → false` mapping as [`stream::query`](super::stream::query).
    pub fn query(e: sys::hipEvent_t) -> Result<bool, HipError> {
        match unsafe { sys::hipEventQuery(e) } {
            sys::hipError_t::hipSuccess => Ok(true),
            sys::hipError_t::hipErrorNotReady => Ok(false),
            err => Err(HipError(err)),
        }
    }

    /// Elapsed time between two events in milliseconds.
    pub fn elapsed(start: sys::hipEvent_t, stop: sys::hipEvent_t) -> Result<f32, HipError> {
        let mut ms: f32 = 0.0;
        unsafe { sys::hipEventElapsedTime(&mut ms, start, stop).result()? };
        Ok(ms)
    }
}

/// # Safety
/// `params` must contain pointers to live argument values whose count and
/// types match the kernel signature for `f`.
pub unsafe fn launch_kernel(
    f: sys::hipFunction_t,
    grid: (u32, u32, u32),
    block: (u32, u32, u32),
    shared_mem: u32,
    stream: sys::hipStream_t,
    params: &mut [*mut c_void],
) -> Result<(), HipError> {
    unsafe {
        sys::hipModuleLaunchKernel(
            f,
            grid.0,
            grid.1,
            grid.2,
            block.0,
            block.1,
            block.2,
            shared_mem,
            stream,
            params.as_mut_ptr(),
            std::ptr::null_mut(),
        )
        .result()
    }
}

/// Driver-API cooperative launch. Note the FFI signature reorders the trailing
/// args vs. [`launch_kernel`] (no `extra` slot).
///
/// # Safety
/// Same as [`launch_kernel`], plus the kernel must be compiled with
/// `--cooperative` and the device must report `cooperativeLaunch` support.
pub unsafe fn launch_cooperative_kernel(
    f: sys::hipFunction_t,
    grid: (u32, u32, u32),
    block: (u32, u32, u32),
    shared_mem: u32,
    stream: sys::hipStream_t,
    params: &mut [*mut c_void],
) -> Result<(), HipError> {
    unsafe {
        sys::hipModuleLaunchCooperativeKernel(
            f,
            grid.0,
            grid.1,
            grid.2,
            block.0,
            block.1,
            block.2,
            shared_mem,
            stream,
            params.as_mut_ptr(),
        )
        .result()
    }
}

pub mod external_memory {
    use super::{
        sys::{self},
        HipError,
    };
    use std::mem::MaybeUninit;

    /// Import an external memory object from a Unix file descriptor.
    /// Destroy via [`destroy_external_memory`].
    ///
    /// **HIP / CUDA contract:** ownership of `fd` transfers to the HIP
    /// runtime on success — do not close it from the caller side.
    ///
    /// # Safety
    /// `size` must be the size of the underlying memory object in bytes.
    #[cfg(unix)]
    pub unsafe fn import_external_memory_opaque_fd(
        fd: std::os::fd::RawFd,
        size: u64,
    ) -> Result<sys::hipExternalMemory_t, HipError> {
        let mut external_memory = MaybeUninit::uninit();
        let handle_description = sys::hipExternalMemoryHandleDesc_st {
            type_: sys::hipExternalMemoryHandleType::hipExternalMemoryHandleTypeOpaqueFd,
            handle: sys::hipExternalMemoryHandleDesc_st__bindgen_ty_1 { fd },
            size,
            flags: 0,
            reserved: [0; 16],
        };
        unsafe {
            sys::hipImportExternalMemory(external_memory.as_mut_ptr(), &handle_description)
                .result()?
        };
        Ok(unsafe { external_memory.assume_init() })
    }

    /// Import an external memory object from a Win32 handle.
    /// Destroy via [`destroy_external_memory`].
    ///
    /// On Windows the handle ownership is **not** transferred to HIP —
    /// the caller is still responsible for closing it.
    ///
    /// # Safety
    /// `size` must be the size of the underlying memory object in bytes.
    #[cfg(windows)]
    pub unsafe fn import_external_memory_opaque_win32(
        handle: std::os::windows::io::RawHandle,
        size: u64,
    ) -> Result<sys::hipExternalMemory_t, HipError> {
        let mut external_memory = MaybeUninit::uninit();
        let handle_description = sys::hipExternalMemoryHandleDesc_st {
            type_: sys::hipExternalMemoryHandleType::hipExternalMemoryHandleTypeOpaqueWin32,
            handle: sys::hipExternalMemoryHandleDesc_st__bindgen_ty_1 {
                win32: sys::hipExternalMemoryHandleDesc_st__bindgen_ty_1__bindgen_ty_1 {
                    handle,
                    name: std::ptr::null(),
                },
            },
            size,
            flags: 0,
            reserved: [0; 16],
        };
        unsafe {
            sys::hipImportExternalMemory(external_memory.as_mut_ptr(), &handle_description)
                .result()?
        };
        Ok(unsafe { external_memory.assume_init() })
    }

    /// # Safety
    /// 1. Any mapped buffers onto this object must already be freed.
    /// 2. The object must only be destroyed once.
    pub unsafe fn destroy_external_memory(
        external_memory: sys::hipExternalMemory_t,
    ) -> Result<(), HipError> {
        unsafe { sys::hipDestroyExternalMemory(external_memory).result() }
    }

    /// Map a buffer over an imported memory object. Free via
    /// [`super::free_sync`].
    ///
    /// # Safety
    /// Mapped buffers may overlap. Caller must ensure the underlying
    /// memory remains valid for the buffer's lifetime.
    pub unsafe fn get_mapped_buffer(
        external_memory: sys::hipExternalMemory_t,
        offset: u64,
        size: u64,
    ) -> Result<u64, HipError> {
        let mut device_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        let buffer_description = sys::hipExternalMemoryBufferDesc_st {
            offset,
            size,
            flags: 0,
            reserved: [0; 16],
        };
        unsafe {
            sys::hipExternalMemoryGetMappedBuffer(
                &mut device_ptr,
                external_memory,
                &buffer_description,
            )
            .result()?
        };
        Ok(device_ptr as u64)
    }
}

pub mod graph {
    use super::{
        sys::{self},
        HipError,
    };
    use std::ffi::c_ulonglong;

    /// # Safety
    /// `graph` must not have already been destroyed.
    pub unsafe fn destroy(graph: sys::hipGraph_t) -> Result<(), HipError> {
        unsafe { sys::hipGraphDestroy(graph).result() }
    }

    /// # Safety
    /// `exec` must not have already been destroyed.
    pub unsafe fn exec_destroy(exec: sys::hipGraphExec_t) -> Result<(), HipError> {
        unsafe { sys::hipGraphExecDestroy(exec).result() }
    }

    /// **HIP divergence:** wraps `hipGraphInstantiateWithFlags` (the
    /// flag-bearing form) so the shape matches `cuGraphInstantiate`. HIP
    /// also ships an older `hipGraphInstantiate` taking an error-node
    /// out-pointer and a log buffer; that variant is not wrapped here.
    ///
    /// # Safety
    /// `graph` must be a valid handle and not currently being captured.
    pub unsafe fn instantiate(
        graph: sys::hipGraph_t,
        flags: sys::hipGraphInstantiateFlags,
    ) -> Result<sys::hipGraphExec_t, HipError> {
        let mut exec: sys::hipGraphExec_t = std::ptr::null_mut();
        unsafe {
            sys::hipGraphInstantiateWithFlags(&mut exec, graph, flags as c_ulonglong)
                .result()?
        };
        Ok(exec)
    }

    /// # Safety
    /// `exec` must be a valid handle and `stream` must be live.
    pub unsafe fn launch(
        exec: sys::hipGraphExec_t,
        stream: sys::hipStream_t,
    ) -> Result<(), HipError> {
        unsafe { sys::hipGraphLaunch(exec, stream).result() }
    }

    /// Upload the graph's resources to the device so the first
    /// [`launch`] doesn't incur setup overhead.
    ///
    /// # Safety
    /// Same as [`launch`].
    pub unsafe fn upload(
        exec: sys::hipGraphExec_t,
        stream: sys::hipStream_t,
    ) -> Result<(), HipError> {
        unsafe { sys::hipGraphUpload(exec, stream).result() }
    }
}

pub mod mem_pool {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Device-to-device memcpy on the *same* context (no peer access
    /// required). Mirrors cudarc's `peer_transfer_self`.
    #[test]
    fn peer_transfer_self() {
        init().unwrap();
        device::set(0).unwrap();

        let bytes = 1024 * std::mem::size_of::<f32>();
        let src = malloc_sync(bytes).unwrap();
        let dst = malloc_sync(bytes).unwrap();

        // Seed src from host.
        let src_host: Vec<f32> = (0..1024).map(|i| i as f32).collect();
        let src_bytes = unsafe {
            std::slice::from_raw_parts(src_host.as_ptr() as *const u8, bytes)
        };
        unsafe { memcpy_htod_sync(src, src_bytes).unwrap() };

        // Device-to-device copy.
        unsafe { memcpy_dtod_sync(dst, src, bytes).unwrap() };

        // Read back via dst.
        let mut dst_host = vec![0f32; 1024];
        let dst_bytes = unsafe {
            std::slice::from_raw_parts_mut(dst_host.as_mut_ptr() as *mut u8, bytes)
        };
        unsafe { memcpy_dtoh_sync(dst_bytes, dst).unwrap() };

        for (i, &v) in dst_host.iter().enumerate() {
            assert_eq!(v, i as f32);
        }

        free_sync(src).unwrap();
        free_sync(dst).unwrap();
    }
}
