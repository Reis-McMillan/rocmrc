//! Thin `Result`-wrapped hipBLASLt FFI. Mirror layout: `cudarc::cublaslt::result`.

use std::ffi::c_void;

pub use super::{HipBlasLtError, sys};
use crate::driver::result::HipResult;

#[inline]
fn check(r: sys::hipblasStatus_t) -> Result<(), HipBlasLtError> {
    if r == sys::hipblasStatus_t::HIPBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(HipBlasLtError::HipBlasLt(r))
    }
}

impl HipResult for sys::hipblasStatus_t {
    type Err = HipBlasLtError;
    fn result(self) -> Result<(), HipBlasLtError> {
        check(self)
    }
}

// ----- handle lifecycle -----

pub fn create() -> Result<sys::hipblasLtHandle_t, HipBlasLtError> {
    let mut h: sys::hipblasLtHandle_t = std::ptr::null_mut();
    unsafe { check(sys::hipblasLtCreate(&mut h))? };
    Ok(h)
}

pub fn destroy(h: sys::hipblasLtHandle_t) -> Result<(), HipBlasLtError> {
    unsafe { check(sys::hipblasLtDestroy(h)) }
}

// ----- matrix layout -----

pub fn matrix_layout_create(
    dtype: sys::hipDataType,
    rows: u64,
    cols: u64,
    ld: i64,
) -> Result<sys::hipblasLtMatrixLayout_t, HipBlasLtError> {
    let mut layout: sys::hipblasLtMatrixLayout_t = std::ptr::null_mut();
    unsafe { check(sys::hipblasLtMatrixLayoutCreate(&mut layout, dtype, rows, cols, ld))? };
    Ok(layout)
}

pub fn matrix_layout_destroy(
    layout: sys::hipblasLtMatrixLayout_t,
) -> Result<(), HipBlasLtError> {
    unsafe { check(sys::hipblasLtMatrixLayoutDestroy(layout)) }
}

/// # Safety: `buf` must point to `size` bytes of the type expected by `attr`.
pub unsafe fn matrix_layout_set_attribute(
    layout: sys::hipblasLtMatrixLayout_t,
    attr: sys::hipblasLtMatrixLayoutAttribute_t,
    buf: *const c_void,
    size: usize,
) -> Result<(), HipBlasLtError> {
    unsafe { check(sys::hipblasLtMatrixLayoutSetAttribute(layout, attr, buf, size)) }
}

// ----- matmul descriptor -----

pub fn matmul_desc_create(
    compute: sys::hipblasComputeType_t,
    scale: sys::hipDataType,
) -> Result<sys::hipblasLtMatmulDesc_t, HipBlasLtError> {
    let mut desc: sys::hipblasLtMatmulDesc_t = std::ptr::null_mut();
    unsafe { check(sys::hipblasLtMatmulDescCreate(&mut desc, compute, scale))? };
    Ok(desc)
}

pub fn matmul_desc_destroy(
    desc: sys::hipblasLtMatmulDesc_t,
) -> Result<(), HipBlasLtError> {
    unsafe { check(sys::hipblasLtMatmulDescDestroy(desc)) }
}

/// # Safety: `buf` must point to `size` bytes of the type expected by `attr`.
pub unsafe fn matmul_desc_set_attribute(
    desc: sys::hipblasLtMatmulDesc_t,
    attr: sys::hipblasLtMatmulDescAttributes_t,
    buf: *const c_void,
    size: usize,
) -> Result<(), HipBlasLtError> {
    unsafe { check(sys::hipblasLtMatmulDescSetAttribute(desc, attr, buf, size)) }
}

// ----- preference -----

pub fn matmul_preference_create() -> Result<sys::hipblasLtMatmulPreference_t, HipBlasLtError> {
    let mut pref: sys::hipblasLtMatmulPreference_t = std::ptr::null_mut();
    unsafe { check(sys::hipblasLtMatmulPreferenceCreate(&mut pref))? };
    Ok(pref)
}

pub fn matmul_preference_destroy(
    pref: sys::hipblasLtMatmulPreference_t,
) -> Result<(), HipBlasLtError> {
    unsafe { check(sys::hipblasLtMatmulPreferenceDestroy(pref)) }
}

/// # Safety: `buf` must point to `size` bytes of the type expected by `attr`.
pub unsafe fn matmul_preference_set_attribute(
    pref: sys::hipblasLtMatmulPreference_t,
    attr: sys::hipblasLtMatmulPreferenceAttributes_t,
    buf: *const c_void,
    size: usize,
) -> Result<(), HipBlasLtError> {
    unsafe {
        check(sys::hipblasLtMatmulPreferenceSetAttribute(
            pref, attr, buf, size,
        ))
    }
}

// ----- heuristic -----

/// # Safety: handle/desc/layouts/pref must all be valid and alive.
pub unsafe fn matmul_algo_get_heuristic(
    handle: sys::hipblasLtHandle_t,
    desc: sys::hipblasLtMatmulDesc_t,
    a_layout: sys::hipblasLtMatrixLayout_t,
    b_layout: sys::hipblasLtMatrixLayout_t,
    c_layout: sys::hipblasLtMatrixLayout_t,
    d_layout: sys::hipblasLtMatrixLayout_t,
    pref: sys::hipblasLtMatmulPreference_t,
    requested: u32,
) -> Result<Vec<sys::hipblasLtMatmulHeuristicResult_t>, HipBlasLtError> {
    // SAFETY: we hand bindgen a freshly-allocated slice of MaybeUninit storage and
    // let it fill it; then we set_len to the count it reports back. The struct is
    // POD (no Drop), so any uninitialised tail is forgotten cheaply.
    let mut buf: Vec<sys::hipblasLtMatmulHeuristicResult_t> = Vec::with_capacity(requested as usize);
    let mut returned: ::core::ffi::c_int = 0;
    unsafe {
        check(sys::hipblasLtMatmulAlgoGetHeuristic(
            handle,
            desc,
            a_layout,
            b_layout,
            c_layout,
            d_layout,
            pref,
            requested as ::core::ffi::c_int,
            buf.as_mut_ptr(),
            &mut returned,
        ))?;
        buf.set_len(returned as usize);
    }
    Ok(buf)
}

// ----- matmul -----

/// # Safety: device pointers (A/B/C/D/workspace) must be valid for the layouts.
#[allow(clippy::too_many_arguments)]
pub unsafe fn matmul(
    handle: sys::hipblasLtHandle_t,
    desc: sys::hipblasLtMatmulDesc_t,
    alpha: *const c_void,
    a: u64,
    a_layout: sys::hipblasLtMatrixLayout_t,
    b: u64,
    b_layout: sys::hipblasLtMatrixLayout_t,
    beta: *const c_void,
    c: u64,
    c_layout: sys::hipblasLtMatrixLayout_t,
    d: u64,
    d_layout: sys::hipblasLtMatrixLayout_t,
    algo: &sys::hipblasLtMatmulAlgo_t,
    workspace: u64,
    workspace_size: usize,
    stream: sys::hipStream_t,
) -> Result<(), HipBlasLtError> {
    unsafe {
        check(sys::hipblasLtMatmul(
            handle,
            desc,
            alpha,
            a as *const c_void,
            a_layout,
            b as *const c_void,
            b_layout,
            beta,
            c as *const c_void,
            c_layout,
            d as *mut c_void,
            d_layout,
            algo,
            workspace as *mut c_void,
            workspace_size,
            stream,
        ))
    }
}
