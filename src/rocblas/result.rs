//! Thin `Result`-wrapped rocBLAS FFI. Mirror layout:
//! `cudarc::cublas::result`. Surface matches cudarc 1:1 — handle
//! lifecycle, set_stream, GEMV / GEMM (s/d/h) and their strided-batched
//! variants, the `_ex` mixed-precision GEMMs, and the L1 reductions
//! cudarc surfaces (`asum`). Other L1 helpers (axpy / scal / nrm2 / dot
//! / copy) are wrapped here too so the safe layer's per-T traits can
//! lower into them.
//!
//! **rocBLAS type idioms (not cuBLAS-shaped):**
//! - `rocblas_int` (i32) instead of `c_int` — same underlying width but
//!   the rocBLAS API spells it explicitly.
//! - `rocblas_stride` (i64) instead of `c_longlong` — same width but the
//!   rocBLAS-idiomatic name.
//! - `rocblas_half` (`#[repr(C)] struct { data: u16 }`) instead of
//!   `__half`. Layout-equivalent to `half::f16`; we accept
//!   `*const half::f16` at the wrapper boundary and cast at the FFI
//!   call.
//!
//! **ILP64 (`_64`) API:** rocBLAS additionally ships
//! `rocblas_sgemm_64` / `rocblas_sgemm_strided_batched_64` / etc. that
//! take `i64` index arguments (instead of `rocblas_int`). This module
//! wraps only the standard 32-bit API, matching cudarc's choice. Reach
//! into `sys::rocblas_*_64` directly if you need 64-bit indices for
//! oversized matrices.
//!
//! See the [rocBLAS API docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/).

use super::sys::{self};
use core::ffi::c_void;
use core::mem::MaybeUninit;

pub struct RocblasError(pub sys::rocblas_status);

impl sys::rocblas_status {
    pub fn result(self) -> Result<(), RocblasError> {
        match self {
            sys::rocblas_status::rocblas_status_success => Ok(()),
            _ => Err(RocblasError(self)),
        }
    }
}

impl std::fmt::Debug for RocblasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RocblasError").field(&self.0).finish()
    }
}

impl std::fmt::Display for RocblasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for RocblasError {}

// ---------------------------------------------------------------------------
// Handle lifecycle
// ---------------------------------------------------------------------------

/// Create a rocBLAS handle.
/// Wraps `rocblas_create_handle`. See the [rocBLAS API docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/).
pub fn create_handle() -> Result<sys::rocblas_handle, RocblasError> {
    let mut handle = MaybeUninit::uninit();
    unsafe {
        sys::rocblas_create_handle(handle.as_mut_ptr()).result()?;
        Ok(handle.assume_init())
    }
}

/// # Safety
/// `handle` must not have already been destroyed.
/// Wraps `rocblas_destroy_handle`. See the [rocBLAS API docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/).
pub unsafe fn destroy_handle(handle: sys::rocblas_handle) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_destroy_handle(handle).result() }
}

/// # Safety
/// `handle` must be live; `stream` must be a live `hipStream_t`.
/// Wraps `rocblas_set_stream`. See the [rocBLAS API docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/).
pub unsafe fn set_stream(
    handle: sys::rocblas_handle,
    stream: sys::hipStream_t,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_set_stream(handle, stream).result() }
}

/// Switch pointer mode (host vs device) for scalar `alpha` / `beta`
/// args and reduction `result` arguments. Defaults to host on a fresh
/// handle.
///
/// # Safety
/// `handle` must be live.
/// Wraps `rocblas_set_pointer_mode`. See the [rocBLAS API docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/).
pub unsafe fn set_pointer_mode(
    handle: sys::rocblas_handle,
    mode: sys::rocblas_pointer_mode,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_set_pointer_mode(handle, mode).result() }
}

// ---------------------------------------------------------------------------
// L2 — GEMV (single + double)
// ---------------------------------------------------------------------------

/// # Safety
/// All pointers must be valid for `m * n` / `n` / `m` elements respectively
/// at the given strides. `alpha` / `beta` follow the handle's pointer mode.
/// Wraps `rocblas_sgemv`. See [rocBLAS level-2 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-2.html).
pub unsafe fn sgemv(
    handle: sys::rocblas_handle,
    trans: sys::rocblas_operation,
    m: sys::rocblas_int,
    n: sys::rocblas_int,
    alpha: *const f32,
    a: *const f32,
    lda: sys::rocblas_int,
    x: *const f32,
    incx: sys::rocblas_int,
    beta: *const f32,
    y: *mut f32,
    incy: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe {
        sys::rocblas_sgemv(handle, trans, m, n, alpha, a, lda, x, incx, beta, y, incy).result()
    }
}

/// # Safety: see [`sgemv`].
/// Wraps `rocblas_dgemv`. See [rocBLAS level-2 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-2.html).
pub unsafe fn dgemv(
    handle: sys::rocblas_handle,
    trans: sys::rocblas_operation,
    m: sys::rocblas_int,
    n: sys::rocblas_int,
    alpha: *const f64,
    a: *const f64,
    lda: sys::rocblas_int,
    x: *const f64,
    incx: sys::rocblas_int,
    beta: *const f64,
    y: *mut f64,
    incy: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe {
        sys::rocblas_dgemv(handle, trans, m, n, alpha, a, lda, x, incx, beta, y, incy).result()
    }
}

// ---------------------------------------------------------------------------
// L3 — GEMM (half / single / double)
// ---------------------------------------------------------------------------

/// # Safety
/// `a`, `b`, `c` must be valid device buffers of appropriate sizes.
/// `alpha` / `beta` follow the handle's pointer mode.
///
/// Casts `*const half::f16` → `*const sys::rocblas_half` at the FFI
/// boundary. Both are `repr(C)` over `u16` and are layout-compatible.
/// Wraps `rocblas_hgemm`. See [rocBLAS level-3 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-3.html).
pub unsafe fn hgemm(
    handle: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: sys::rocblas_int,
    n: sys::rocblas_int,
    k: sys::rocblas_int,
    alpha: *const half::f16,
    a: *const half::f16,
    lda: sys::rocblas_int,
    b: *const half::f16,
    ldb: sys::rocblas_int,
    beta: *const half::f16,
    c: *mut half::f16,
    ldc: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe {
        sys::rocblas_hgemm(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha as *const sys::rocblas_half,
            a as *const sys::rocblas_half,
            lda,
            b as *const sys::rocblas_half,
            ldb,
            beta as *const sys::rocblas_half,
            c as *mut sys::rocblas_half,
            ldc,
        )
        .result()
    }
}

/// # Safety: see [`hgemm`].
/// Wraps `rocblas_sgemm`. See [rocBLAS level-3 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-3.html).
pub unsafe fn sgemm(
    handle: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: sys::rocblas_int,
    n: sys::rocblas_int,
    k: sys::rocblas_int,
    alpha: *const f32,
    a: *const f32,
    lda: sys::rocblas_int,
    b: *const f32,
    ldb: sys::rocblas_int,
    beta: *const f32,
    c: *mut f32,
    ldc: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe {
        sys::rocblas_sgemm(
            handle, transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
        )
        .result()
    }
}

/// # Safety: see [`hgemm`].
/// Wraps `rocblas_dgemm`. See [rocBLAS level-3 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-3.html).
pub unsafe fn dgemm(
    handle: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: sys::rocblas_int,
    n: sys::rocblas_int,
    k: sys::rocblas_int,
    alpha: *const f64,
    a: *const f64,
    lda: sys::rocblas_int,
    b: *const f64,
    ldb: sys::rocblas_int,
    beta: *const f64,
    c: *mut f64,
    ldc: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe {
        sys::rocblas_dgemm(
            handle, transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
        )
        .result()
    }
}

// ---------------------------------------------------------------------------
// L3 — strided-batched GEMM (half / single / double)
// ---------------------------------------------------------------------------

/// # Safety: same as [`hgemm`] applied per batch element.
#[allow(clippy::too_many_arguments)]
/// Wraps `rocblas_hgemm_strided_batched`. See [rocBLAS level-3 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-3.html).
pub unsafe fn hgemm_strided_batched(
    handle: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: sys::rocblas_int,
    n: sys::rocblas_int,
    k: sys::rocblas_int,
    alpha: *const half::f16,
    a: *const half::f16,
    lda: sys::rocblas_int,
    stride_a: sys::rocblas_stride,
    b: *const half::f16,
    ldb: sys::rocblas_int,
    stride_b: sys::rocblas_stride,
    beta: *const half::f16,
    c: *mut half::f16,
    ldc: sys::rocblas_int,
    stride_c: sys::rocblas_stride,
    batch_count: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe {
        sys::rocblas_hgemm_strided_batched(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha as *const sys::rocblas_half,
            a as *const sys::rocblas_half,
            lda,
            stride_a,
            b as *const sys::rocblas_half,
            ldb,
            stride_b,
            beta as *const sys::rocblas_half,
            c as *mut sys::rocblas_half,
            ldc,
            stride_c,
            batch_count,
        )
        .result()
    }
}

/// # Safety: same as [`sgemm`] applied per batch element.
#[allow(clippy::too_many_arguments)]
/// Wraps `rocblas_sgemm_strided_batched`. See [rocBLAS level-3 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-3.html).
pub unsafe fn sgemm_strided_batched(
    handle: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: sys::rocblas_int,
    n: sys::rocblas_int,
    k: sys::rocblas_int,
    alpha: *const f32,
    a: *const f32,
    lda: sys::rocblas_int,
    stride_a: sys::rocblas_stride,
    b: *const f32,
    ldb: sys::rocblas_int,
    stride_b: sys::rocblas_stride,
    beta: *const f32,
    c: *mut f32,
    ldc: sys::rocblas_int,
    stride_c: sys::rocblas_stride,
    batch_count: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe {
        sys::rocblas_sgemm_strided_batched(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a,
            lda,
            stride_a,
            b,
            ldb,
            stride_b,
            beta,
            c,
            ldc,
            stride_c,
            batch_count,
        )
        .result()
    }
}

/// # Safety: same as [`dgemm`] applied per batch element.
#[allow(clippy::too_many_arguments)]
/// Wraps `rocblas_dgemm_strided_batched`. See [rocBLAS level-3 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-3.html).
pub unsafe fn dgemm_strided_batched(
    handle: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: sys::rocblas_int,
    n: sys::rocblas_int,
    k: sys::rocblas_int,
    alpha: *const f64,
    a: *const f64,
    lda: sys::rocblas_int,
    stride_a: sys::rocblas_stride,
    b: *const f64,
    ldb: sys::rocblas_int,
    stride_b: sys::rocblas_stride,
    beta: *const f64,
    c: *mut f64,
    ldc: sys::rocblas_int,
    stride_c: sys::rocblas_stride,
    batch_count: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe {
        sys::rocblas_dgemm_strided_batched(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a,
            lda,
            stride_a,
            b,
            ldb,
            stride_b,
            beta,
            c,
            ldc,
            stride_c,
            batch_count,
        )
        .result()
    }
}

// ---------------------------------------------------------------------------
// `_ex` mixed-precision GEMMs
// ---------------------------------------------------------------------------

/// Mixed-precision GEMM. Mirrors cudarc's `gemm_ex`.
///
/// **rocBLAS divergence:** `rocblas_gemm_ex` takes a fourth output
/// buffer `d` (along with its layout `ldd` and type `d_type`), plus a
/// trailing `solution_index: i32` and `flags: u32`. cuBLAS's
/// `cublasGemmEx` doesn't have those — caller hands them in here.
///
/// Common usage: pass `c` and `d` to the same buffer with `c_type == d_type`
/// and `ldc == ldd` for an in-place result; `solution_index = 0` and
/// `flags = 0` to let rocBLAS pick.
///
/// # Safety
/// - All device pointers must be valid for their declared types and strides.
/// - `alpha` / `beta` follow the handle's pointer mode.
///
/// Wraps `rocblas_gemm_ex`. See [rocBLAS level-3 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-3.html).
#[allow(clippy::too_many_arguments)]
pub unsafe fn gemm_ex(
    handle: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: sys::rocblas_int,
    n: sys::rocblas_int,
    k: sys::rocblas_int,
    alpha: *const c_void,
    a: *const c_void,
    a_type: sys::rocblas_datatype,
    lda: sys::rocblas_int,
    b: *const c_void,
    b_type: sys::rocblas_datatype,
    ldb: sys::rocblas_int,
    beta: *const c_void,
    c: *const c_void,
    c_type: sys::rocblas_datatype,
    ldc: sys::rocblas_int,
    d: *mut c_void,
    d_type: sys::rocblas_datatype,
    ldd: sys::rocblas_int,
    compute_type: sys::rocblas_datatype,
    algo: sys::rocblas_gemm_algo,
    solution_index: i32,
    flags: u32,
) -> Result<(), RocblasError> {
    unsafe {
        sys::rocblas_gemm_ex(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a,
            a_type,
            lda,
            b,
            b_type,
            ldb,
            beta,
            c,
            c_type,
            ldc,
            d,
            d_type,
            ldd,
            compute_type,
            algo,
            solution_index,
            flags,
        )
        .result()
    }
}

/// Strided-batched mixed-precision GEMM. Same shape as [`gemm_ex`] with
/// per-buffer strides + a batch count.
///
/// # Safety: see [`gemm_ex`], applied per batch element.
#[allow(clippy::too_many_arguments)]
/// Wraps `rocblas_gemm_strided_batched_ex`. See [rocBLAS level-3 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-3.html).
pub unsafe fn gemm_strided_batched_ex(
    handle: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: sys::rocblas_int,
    n: sys::rocblas_int,
    k: sys::rocblas_int,
    alpha: *const c_void,
    a: *const c_void,
    a_type: sys::rocblas_datatype,
    lda: sys::rocblas_int,
    stride_a: sys::rocblas_stride,
    b: *const c_void,
    b_type: sys::rocblas_datatype,
    ldb: sys::rocblas_int,
    stride_b: sys::rocblas_stride,
    beta: *const c_void,
    c: *const c_void,
    c_type: sys::rocblas_datatype,
    ldc: sys::rocblas_int,
    stride_c: sys::rocblas_stride,
    d: *mut c_void,
    d_type: sys::rocblas_datatype,
    ldd: sys::rocblas_int,
    stride_d: sys::rocblas_stride,
    batch_count: sys::rocblas_int,
    compute_type: sys::rocblas_datatype,
    algo: sys::rocblas_gemm_algo,
    solution_index: i32,
    flags: u32,
) -> Result<(), RocblasError> {
    unsafe {
        sys::rocblas_gemm_strided_batched_ex(
            handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a,
            a_type,
            lda,
            stride_a,
            b,
            b_type,
            ldb,
            stride_b,
            beta,
            c,
            c_type,
            ldc,
            stride_c,
            d,
            d_type,
            ldd,
            stride_d,
            batch_count,
            compute_type,
            algo,
            solution_index,
            flags,
        )
        .result()
    }
}

// ---------------------------------------------------------------------------
// L1 — reductions (asum / nrm2 / dot) and updates (axpy / scal / copy)
// `result` for reductions follows the handle's pointer mode.
// ---------------------------------------------------------------------------

/// # Safety: `x` must be valid for `n * incx` elements; `result` must
/// point to one `f32` reachable per the handle's pointer mode.
/// Wraps `rocblas_sasum`. See [rocBLAS level-1 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-1.html).
pub unsafe fn sasum(
    handle: sys::rocblas_handle,
    n: sys::rocblas_int,
    x: *const f32,
    incx: sys::rocblas_int,
    result: *mut f32,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_sasum(handle, n, x, incx, result).result() }
}

/// # Safety: see [`sasum`].
/// Wraps `rocblas_dasum`. See [rocBLAS level-1 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-1.html).
pub unsafe fn dasum(
    handle: sys::rocblas_handle,
    n: sys::rocblas_int,
    x: *const f64,
    incx: sys::rocblas_int,
    result: *mut f64,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_dasum(handle, n, x, incx, result).result() }
}

/// # Safety: see [`sasum`].
/// Wraps `rocblas_snrm2`. See [rocBLAS level-1 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-1.html).
pub unsafe fn snrm2(
    handle: sys::rocblas_handle,
    n: sys::rocblas_int,
    x: *const f32,
    incx: sys::rocblas_int,
    result: *mut f32,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_snrm2(handle, n, x, incx, result).result() }
}

/// # Safety: see [`sasum`].
/// Wraps `rocblas_dnrm2`. See [rocBLAS level-1 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-1.html).
pub unsafe fn dnrm2(
    handle: sys::rocblas_handle,
    n: sys::rocblas_int,
    x: *const f64,
    incx: sys::rocblas_int,
    result: *mut f64,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_dnrm2(handle, n, x, incx, result).result() }
}

/// # Safety: `x` / `y` valid for `n` elements at the given strides;
/// `result` follows the handle's pointer mode.
/// Wraps `rocblas_sdot`. See [rocBLAS level-1 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-1.html).
pub unsafe fn sdot(
    handle: sys::rocblas_handle,
    n: sys::rocblas_int,
    x: *const f32,
    incx: sys::rocblas_int,
    y: *const f32,
    incy: sys::rocblas_int,
    result: *mut f32,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_sdot(handle, n, x, incx, y, incy, result).result() }
}

/// # Safety: see [`sdot`].
/// Wraps `rocblas_ddot`. See [rocBLAS level-1 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-1.html).
pub unsafe fn ddot(
    handle: sys::rocblas_handle,
    n: sys::rocblas_int,
    x: *const f64,
    incx: sys::rocblas_int,
    y: *const f64,
    incy: sys::rocblas_int,
    result: *mut f64,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_ddot(handle, n, x, incx, y, incy, result).result() }
}

/// `y := alpha*x + y`.
///
/// # Safety: device pointers valid for `n` elements at the given strides.
/// Wraps `rocblas_saxpy`. See [rocBLAS level-1 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-1.html).
pub unsafe fn saxpy(
    handle: sys::rocblas_handle,
    n: sys::rocblas_int,
    alpha: *const f32,
    x: *const f32,
    incx: sys::rocblas_int,
    y: *mut f32,
    incy: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_saxpy(handle, n, alpha, x, incx, y, incy).result() }
}

/// # Safety: see [`saxpy`].
/// Wraps `rocblas_daxpy`. See [rocBLAS level-1 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-1.html).
pub unsafe fn daxpy(
    handle: sys::rocblas_handle,
    n: sys::rocblas_int,
    alpha: *const f64,
    x: *const f64,
    incx: sys::rocblas_int,
    y: *mut f64,
    incy: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_daxpy(handle, n, alpha, x, incx, y, incy).result() }
}

/// `x := alpha*x`.
///
/// # Safety: `x` valid for `n * incx` elements.
/// Wraps `rocblas_sscal`. See [rocBLAS level-1 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-1.html).
pub unsafe fn sscal(
    handle: sys::rocblas_handle,
    n: sys::rocblas_int,
    alpha: *const f32,
    x: *mut f32,
    incx: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_sscal(handle, n, alpha, x, incx).result() }
}

/// # Safety: see [`sscal`].
/// Wraps `rocblas_dscal`. See [rocBLAS level-1 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-1.html).
pub unsafe fn dscal(
    handle: sys::rocblas_handle,
    n: sys::rocblas_int,
    alpha: *const f64,
    x: *mut f64,
    incx: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_dscal(handle, n, alpha, x, incx).result() }
}

/// `y := x`.
///
/// # Safety: `x` / `y` valid for `n` elements at the given strides.
/// Wraps `rocblas_scopy`. See [rocBLAS level-1 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-1.html).
pub unsafe fn scopy(
    handle: sys::rocblas_handle,
    n: sys::rocblas_int,
    x: *const f32,
    incx: sys::rocblas_int,
    y: *mut f32,
    incy: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_scopy(handle, n, x, incx, y, incy).result() }
}

/// # Safety: see [`scopy`].
/// Wraps `rocblas_dcopy`. See [rocBLAS level-1 docs](https://rocm.docs.amd.com/projects/rocBLAS/en/latest/reference/level-1.html).
pub unsafe fn dcopy(
    handle: sys::rocblas_handle,
    n: sys::rocblas_int,
    x: *const f64,
    incx: sys::rocblas_int,
    y: *mut f64,
    incy: sys::rocblas_int,
) -> Result<(), RocblasError> {
    unsafe { sys::rocblas_dcopy(handle, n, x, incx, y, incy).result() }
}
