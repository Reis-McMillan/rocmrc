//! Thin `Result`-wrapped hipBLASLt FFI. Mirror layout:
//! [`cudarc::cublaslt::result`]. Function shapes and `unsafe`-markers
//! follow cudarc 1:1 — every handle/descriptor lifecycle (create, set,
//! destroy) is wrapped, plus the heuristic search and `matmul` itself.

use super::sys::{self};
use crate::hipblaslt::sys::hipblasLtMatmulAlgo_t;
use core::ffi::c_void;
use core::mem::MaybeUninit;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HipblasError(pub sys::hipblasStatus_t);

impl sys::hipblasStatus_t {
    pub fn result(self) -> Result<(), HipblasError> {
        match self {
            sys::hipblasStatus_t::HIPBLAS_STATUS_SUCCESS => Ok(()),
            _ => Err(HipblasError(self)),
        }
    }
}

impl std::fmt::Display for HipblasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for HipblasError {}

/// Creates a handle to the hipBLASLt library. Mirrors `cublasLtCreate`.
pub fn create_handle() -> Result<sys::hipblasLtHandle_t, HipblasError> {
    let mut handle = MaybeUninit::uninit();
    unsafe {
        sys::hipblasLtCreate(handle.as_mut_ptr()).result()?;
        Ok(handle.assume_init())
    }
}

/// Destroy a handle previously created with [`create_handle`].
///
/// # Safety
/// `handle` must not have been freed already.
pub fn destroy_handle(handle: sys::hipblasLtHandle_t) -> Result<(), HipblasError> {
    unsafe { sys::hipblasLtDestroy(handle).result() }
}

/// Creates a matrix layout descriptor.
pub fn create_matrix_layout(
    matrix_type: sys::hipDataType,
    rows: u64,
    cols: u64,
    ld: i64,
) -> Result<sys::hipblasLtMatrixLayout_t, HipblasError> {
    let mut matrix_layout = MaybeUninit::uninit();
    unsafe {
        sys::hipblasLtMatrixLayoutCreate(
            matrix_layout.as_mut_ptr(),
            matrix_type,
            rows,
            cols,
            ld,
        )
        .result()?;
        Ok(matrix_layout.assume_init())
    }
}

/// Sets the value of the specified attribute on a matrix layout descriptor.
///
/// # Safety
/// `matrix_layout` must not have been freed already.
pub fn set_matrix_layout_attribute(
    matrix_layout: sys::hipblasLtMatrixLayout_t,
    attr: sys::hipblasLtMatrixLayoutAttribute_t,
    buf: *const c_void,
    buf_size: usize,
) -> Result<(), HipblasError> {
    unsafe {
        sys::hipblasLtMatrixLayoutSetAttribute(matrix_layout, attr, buf, buf_size).result()
    }
}

/// Destroy a matrix layout descriptor.
///
/// # Safety
/// `matrix_layout` must not have been freed already.
pub fn destroy_matrix_layout(
    matrix_layout: sys::hipblasLtMatrixLayout_t,
) -> Result<(), HipblasError> {
    unsafe { sys::hipblasLtMatrixLayoutDestroy(matrix_layout).result() }
}

/// Creates a matrix multiply descriptor.
pub fn create_matmul_desc(
    compute_type: sys::hipblasComputeType_t,
    scale_type: sys::hipDataType,
) -> Result<sys::hipblasLtMatmulDesc_t, HipblasError> {
    let mut matmul_desc = MaybeUninit::uninit();
    unsafe {
        sys::hipblasLtMatmulDescCreate(matmul_desc.as_mut_ptr(), compute_type, scale_type)
            .result()?;
        Ok(matmul_desc.assume_init())
    }
}

/// Sets the value of the specified attribute on a matrix multiply descriptor.
///
/// # Safety
/// `matmul_desc` must not have been freed already.
pub fn set_matmul_desc_attribute(
    matmul_desc: sys::hipblasLtMatmulDesc_t,
    attr: sys::hipblasLtMatmulDescAttributes_t,
    buf: *const c_void,
    buf_size: usize,
) -> Result<(), HipblasError> {
    unsafe {
        sys::hipblasLtMatmulDescSetAttribute(matmul_desc, attr, buf, buf_size).result()
    }
}

/// Destroy a matrix multiply descriptor.
///
/// # Safety
/// `matmul_desc` must not have been freed already.
pub fn destroy_matmul_desc(
    matmul_desc: sys::hipblasLtMatmulDesc_t,
) -> Result<(), HipblasError> {
    unsafe { sys::hipblasLtMatmulDescDestroy(matmul_desc).result() }
}

/// Creates a matrix multiply heuristic-search preferences descriptor.
pub fn create_matmul_pref() -> Result<sys::hipblasLtMatmulPreference_t, HipblasError> {
    let mut matmul_pref = MaybeUninit::uninit();
    unsafe {
        sys::hipblasLtMatmulPreferenceCreate(matmul_pref.as_mut_ptr()).result()?;
        Ok(matmul_pref.assume_init())
    }
}

/// Sets the value of the specified attribute on a matmul preferences descriptor.
///
/// # Safety
/// `matmul_pref` must not have been freed already.
pub fn set_matmul_pref_attribute(
    matmul_pref: sys::hipblasLtMatmulPreference_t,
    attr: sys::hipblasLtMatmulPreferenceAttributes_t,
    buf: *const c_void,
    buf_size: usize,
) -> Result<(), HipblasError> {
    unsafe {
        sys::hipblasLtMatmulPreferenceSetAttribute(matmul_pref, attr, buf, buf_size).result()
    }
}

/// Destroy a matmul preferences descriptor previously created with
/// [`create_matmul_pref`].
///
/// # Safety
/// `matmul_pref` must not have been freed already.
pub fn destroy_matmul_pref(
    matmul_pref: sys::hipblasLtMatmulPreference_t,
) -> Result<(), HipblasError> {
    unsafe { sys::hipblasLtMatmulPreferenceDestroy(matmul_pref).result() }
}

/// Retrieves the fastest algorithm for the matrix-multiply configuration
/// described by the matmul descriptor, A/B/C/D layouts, and preferences.
///
/// Returns `Err(HipblasError(HIPBLAS_STATUS_NOT_SUPPORTED))` if hipBLASLt
/// can't find any algorithm matching the request.
///
/// # Safety
/// All sys objects must outlive this call and must not have been freed.
pub fn get_matmul_algo_heuristic(
    handle: sys::hipblasLtHandle_t,
    matmul_desc: sys::hipblasLtMatmulDesc_t,
    a_layout: sys::hipblasLtMatrixLayout_t,
    b_layout: sys::hipblasLtMatrixLayout_t,
    c_layout: sys::hipblasLtMatrixLayout_t,
    d_layout: sys::hipblasLtMatrixLayout_t,
    matmul_pref: sys::hipblasLtMatmulPreference_t,
) -> Result<sys::hipblasLtMatmulHeuristicResult_t, HipblasError> {
    let mut matmul_heuristic = MaybeUninit::uninit();
    let mut algo_count: core::ffi::c_int = 0;

    unsafe {
        sys::hipblasLtMatmulAlgoGetHeuristic(
            handle,
            matmul_desc,
            a_layout,
            b_layout,
            c_layout,
            d_layout,
            matmul_pref,
            1, // only select the fastest algo
            matmul_heuristic.as_mut_ptr(),
            &mut algo_count,
        )
        .result()?;
    }

    if algo_count == 0 {
        return Err(HipblasError(
            sys::hipblasStatus_t::HIPBLAS_STATUS_NOT_SUPPORTED,
        ));
    }

    let matmul_heuristic = unsafe { matmul_heuristic.assume_init() };
    matmul_heuristic.state.result()?;

    Ok(matmul_heuristic)
}

/// Computes `D = alpha * (A * B) + beta * C` where A, B, C are input
/// matrices and `alpha` / `beta` are input scalars.
///
/// # Safety
/// All sys objects must outlive this call and must not have been freed.
/// `workspace` must be at least `workspace_size` bytes.
#[allow(clippy::too_many_arguments)]
pub fn matmul(
    handle: sys::hipblasLtHandle_t,
    matmul_desc: sys::hipblasLtMatmulDesc_t,
    alpha: *const c_void,
    beta: *const c_void,
    a: *const c_void,
    a_layout: sys::hipblasLtMatrixLayout_t,
    b: *const c_void,
    b_layout: sys::hipblasLtMatrixLayout_t,
    c: *const c_void,
    c_layout: sys::hipblasLtMatrixLayout_t,
    d: *mut c_void,
    d_layout: sys::hipblasLtMatrixLayout_t,
    algo: *const hipblasLtMatmulAlgo_t,
    workspace: *mut c_void,
    workspace_size: usize,
    stream: sys::hipStream_t,
) -> Result<(), HipblasError> {
    unsafe {
        sys::hipblasLtMatmul(
            handle,
            matmul_desc,
            alpha,
            a,
            a_layout,
            b,
            b_layout,
            beta,
            c,
            c_layout,
            d,
            d_layout,
            algo,
            workspace,
            workspace_size,
            stream,
        )
        .result()
    }
}
