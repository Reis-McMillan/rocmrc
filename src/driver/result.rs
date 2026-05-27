//! Thin `Result`-wrapped FFI. Mirror layout: `cudarc::driver::result`.

pub use super::{DriverError, sys};
use std::ffi::{CString, c_void};

#[inline]
pub(crate) fn check(r: sys::hipError_t) -> Result<(), DriverError> {
    if r == sys::hipError_t::hipSuccess {
        Ok(())
    } else {
        Err(DriverError::Hip(r))
    }
}

/// Promote a raw FFI status to a `Result`. Mirrors cudarc's `.result()?`
/// idiom so call sites copied from `luminal_cuda_lite` can write
/// `unsafe { sys::hipFoo(...) }.result()?` unchanged.
pub trait HipResult {
    type Err;
    fn result(self) -> std::result::Result<(), Self::Err>;
}

impl HipResult for sys::hipError_t {
    type Err = DriverError;
    fn result(self) -> Result<(), DriverError> {
        check(self)
    }
}

pub fn set_device(ordinal: i32) -> Result<(), DriverError> {
    unsafe { check(sys::hipSetDevice(ordinal)) }
}

pub fn device_name(ordinal: i32) -> Result<String, DriverError> {
    let mut buf = [0i8; 256];
    unsafe { check(sys::hipDeviceGetName(buf.as_mut_ptr(), buf.len() as i32, ordinal))? };
    let cstr = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) };
    Ok(cstr.to_string_lossy().into_owned())
}

pub fn stream_create() -> Result<sys::hipStream_t, DriverError> {
    let mut s: sys::hipStream_t = std::ptr::null_mut();
    unsafe { check(sys::hipStreamCreate(&mut s))? };
    Ok(s)
}

pub fn stream_destroy(s: sys::hipStream_t) -> Result<(), DriverError> {
    unsafe { check(sys::hipStreamDestroy(s)) }
}

pub fn stream_synchronize(s: sys::hipStream_t) -> Result<(), DriverError> {
    unsafe { check(sys::hipStreamSynchronize(s)) }
}

pub fn malloc(n_bytes: usize) -> Result<u64, DriverError> {
    let mut p: *mut c_void = std::ptr::null_mut();
    unsafe { check(sys::hipMalloc(&mut p, n_bytes))? };
    Ok(p as u64)
}

/// Allocates from the stream device's mempool. Requires the device to advertise
/// `memoryPoolsSupported`; the legacy NULL stream is rejected on some backends.
/// Pair with [`free_async`] to stay stream-ordered — otherwise the caller must
/// [`stream_synchronize`] before [`free`].
pub fn malloc_async(n_bytes: usize, s: sys::hipStream_t) -> Result<u64, DriverError> {
    let mut p: *mut c_void = std::ptr::null_mut();
    unsafe { check(sys::hipMallocAsync(&mut p, n_bytes, s))? };
    Ok(p as u64)
}

pub fn free(ptr: u64) -> Result<(), DriverError> {
    unsafe { check(sys::hipFree(ptr as *mut c_void)) }
}

pub fn free_async(ptr: u64, s:sys::hipStream_t) -> Result<(), DriverError> {
    unsafe { check(sys::hipFreeAsync(ptr as *mut c_void, s)) }
}

/// # Safety
/// `dst` must point to at least `src.len()` bytes of device memory owned by
/// the caller and valid for writes through `stream`.
pub unsafe fn memcpy_htod_async(
    dst: u64,
    src: &[u8],
    stream: sys::hipStream_t,
) -> Result<(), DriverError> {
    unsafe {
        check(sys::hipMemcpyHtoDAsync(
            dst as sys::hipDeviceptr_t,
            src.as_ptr() as *const c_void,
            src.len(),
            stream,
        ))
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
) -> Result<(), DriverError> {
    unsafe {
        check(sys::hipMemcpyDtoHAsync(
            dst.as_mut_ptr() as *mut c_void,
            src as sys::hipDeviceptr_t,
            dst.len(),
            stream,
        ))
    }
}

pub unsafe fn memset_d8_async(
    ptr: u64, value: u8, bytes: usize, stream: sys::hipStream_t,
) -> Result<(), DriverError> {
    unsafe { check(sys::hipMemsetD8Async(ptr as *mut _, value, bytes, stream)) }
}

/// # Safety
/// Both `dst` and `src` must point to at least `bytes` of device memory in the
/// same context, reachable through `stream`.
pub unsafe fn memcpy_dtod_async(
    dst: u64,
    src: u64,
    bytes: usize,
    stream: sys::hipStream_t,
) -> Result<(), DriverError> {
    unsafe {
        check(sys::hipMemcpyDtoDAsync(
            dst as sys::hipDeviceptr_t,
            src as sys::hipDeviceptr_t,
            bytes,
            stream,
        ))
    }
}


pub fn module_load_data(image: &[u8]) -> Result<sys::hipModule_t, DriverError> {
    let mut m: sys::hipModule_t = std::ptr::null_mut();
    unsafe { check(sys::hipModuleLoadData(&mut m, image.as_ptr() as *const c_void))? };
    Ok(m)
}

pub fn module_unload(m: sys::hipModule_t) -> Result<(), DriverError> {
    unsafe { check(sys::hipModuleUnload(m)) }
}

pub fn module_get_function(
    m: sys::hipModule_t,
    name: &str,
) -> Result<sys::hipFunction_t, DriverError> {
    let c = CString::new(name).map_err(|_| DriverError::InvalidName)?;
    let mut f: sys::hipFunction_t = std::ptr::null_mut();
    unsafe { check(sys::hipModuleGetFunction(&mut f, m, c.as_ptr()))? };
    Ok(f)
}

/// # Safety
/// `params` must contain pointers to live argument values whose count and
/// types match the kernel signature for `f`.
pub unsafe fn module_launch_kernel(
    f: sys::hipFunction_t,
    grid: (u32, u32, u32),
    block: (u32, u32, u32),
    shared_mem: u32,
    stream: sys::hipStream_t,
    params: &mut [*mut c_void],
) -> Result<(), DriverError> {
    unsafe {
        check(sys::hipModuleLaunchKernel(
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
        ))
    }
}

pub fn device_gcn_arch_name(ordinal: i32) -> Result<String, DriverError> {
    let mut props: sys::hipDeviceProp_tR0600 = unsafe { std::mem::zeroed() };
    unsafe { check(sys::hipGetDevicePropertiesR0600(&mut props, ordinal))? };
    let cstr = unsafe { std::ffi::CStr::from_ptr(props.gcnArchName.as_ptr()) };
    let raw = cstr.to_string_lossy().into_owned();
    // gcnArchName can carry feature suffixes like "gfx1100:sramecc-:xnack-"
    // that hipRTC won't accept. Strip at the first ':'.
    Ok(raw.split(':').next().unwrap_or(&raw).to_string())
}

pub fn set_device_flags(flags: core::ffi::c_uint) -> Result<(), DriverError> {
    unsafe { check(sys::hipSetDeviceFlags(flags)) }
}
