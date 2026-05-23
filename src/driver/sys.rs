//! Raw FFI for the HIP runtime. Hand-rolled minimum surface.
//!
//! Replace with bindgen-generated output (committed) once we need wider coverage.
//! Mirror layout: `cudarc::driver::sys`.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::ffi::{c_char, c_int, c_uint, c_void};

pub type hipError_t = c_int;
pub const HIP_SUCCESS: hipError_t = 0;

// Opaque handles. HIP defines these as pointers to incomplete structs.
pub type hipStream_t = *mut c_void;
pub type hipModule_t = *mut c_void;
pub type hipFunction_t = *mut c_void;

// `hipDeviceProp_t` is large and version-dependent; rather than mirror it by
// hand we expose `hipDeviceGetName` for now and add a TODO to read
// `gcnArchName` via bindgen-generated props later. The arch string is the
// piece luminal actually needs (for hipRTC `--offload-arch`).

unsafe extern "C" {
    pub fn hipSetDevice(device: c_int) -> hipError_t;
    pub fn hipDeviceGetName(name: *mut c_char, len: c_int, device: c_int) -> hipError_t;

    pub fn hipStreamCreate(stream: *mut hipStream_t) -> hipError_t;
    pub fn hipStreamDestroy(stream: hipStream_t) -> hipError_t;
    pub fn hipStreamSynchronize(stream: hipStream_t) -> hipError_t;

    pub fn hipMalloc(ptr: *mut *mut c_void, size: usize) -> hipError_t;
    pub fn hipMallocAsync(ptr: *mut *mut c_void, size: usize, stream: hipStream_t) -> hipError_t;
    pub fn hipFree(ptr: *mut c_void) -> hipError_t;
    pub unsafe fn hipFreeAsync(ptr: *mut c_void, stream: hipStream_t) -> hipError_t;

    pub fn hipMemcpyHtoDAsync(
        dst: *mut c_void,
        src: *const c_void,
        sizeBytes: usize,
        stream: hipStream_t,
    ) -> hipError_t;
    pub fn hipMemcpyDtoHAsync(
        dst: *mut c_void,
        src: *const c_void,
        sizeBytes: usize,
        stream: hipStream_t,
    ) -> hipError_t;

    pub fn hipModuleLoadData(module: *mut hipModule_t, image: *const c_void) -> hipError_t;
    pub fn hipModuleUnload(module: hipModule_t) -> hipError_t;
    pub fn hipModuleGetFunction(
        function: *mut hipFunction_t,
        module: hipModule_t,
        name: *const c_char,
    ) -> hipError_t;

    pub fn hipModuleLaunchKernel(
        f: hipFunction_t,
        gridDimX: c_uint,
        gridDimY: c_uint,
        gridDimZ: c_uint,
        blockDimX: c_uint,
        blockDimY: c_uint,
        blockDimZ: c_uint,
        sharedMemBytes: c_uint,
        stream: hipStream_t,
        kernelParams: *mut *mut c_void,
        extra: *mut *mut c_void,
    ) -> hipError_t;
}
