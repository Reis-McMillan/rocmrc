//! Thin `Result`-wrapped rocBLAS FFI. Mirror layout: `cudarc::cublas::result`.

use std::ffi::c_void;

use super::{RocblasError, sys};

#[inline]
fn check(r: sys::rocblas_status) -> Result<(), RocblasError> {
    if r == sys::rocblas_status::rocblas_status_success {
        Ok(())
    } else {
        Err(RocblasError::Rocblas(r))
    }
}

// ----- handle lifecycle -----

pub fn create_handle() -> Result<sys::rocblas_handle, RocblasError> {
    let mut h: sys::rocblas_handle = std::ptr::null_mut();
    unsafe { check(sys::rocblas_create_handle(&mut h))? };
    Ok(h)
}

pub fn destroy_handle(h: sys::rocblas_handle) -> Result<(), RocblasError> {
    unsafe { check(sys::rocblas_destroy_handle(h)) }
}

pub fn set_stream(h: sys::rocblas_handle, s: sys::hipStream_t) -> Result<(), RocblasError> {
    unsafe { check(sys::rocblas_set_stream(h, s)) }
}

pub fn set_pointer_mode(
    h: sys::rocblas_handle,
    mode: sys::rocblas_pointer_mode,
) -> Result<(), RocblasError> {
    unsafe { check(sys::rocblas_set_pointer_mode(h, mode)) }
}

// ----- L3: GEMM (typed) -----

#[allow(clippy::too_many_arguments)]
pub unsafe fn sgemm(
    h: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: i32,
    n: i32,
    k: i32,
    alpha: &f32,
    a: u64,
    lda: i32,
    b: u64,
    ldb: i32,
    beta: &f32,
    c: u64,
    ldc: i32,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_sgemm(
            h,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a as *const f32,
            lda,
            b as *const f32,
            ldb,
            beta,
            c as *mut f32,
            ldc,
        ))
    }
}

#[allow(clippy::too_many_arguments)]
pub unsafe fn dgemm(
    h: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: i32,
    n: i32,
    k: i32,
    alpha: &f64,
    a: u64,
    lda: i32,
    b: u64,
    ldb: i32,
    beta: &f64,
    c: u64,
    ldc: i32,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_dgemm(
            h,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a as *const f64,
            lda,
            b as *const f64,
            ldb,
            beta,
            c as *mut f64,
            ldc,
        ))
    }
}

// ----- L3: GEMM (type-erased ex form) -----

#[allow(clippy::too_many_arguments)]
pub unsafe fn gemm_ex(
    h: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: i32,
    n: i32,
    k: i32,
    alpha: *const c_void,
    a: u64,
    a_type: sys::rocblas_datatype,
    lda: i32,
    b: u64,
    b_type: sys::rocblas_datatype,
    ldb: i32,
    beta: *const c_void,
    c: u64,
    c_type: sys::rocblas_datatype,
    ldc: i32,
    d: u64,
    d_type: sys::rocblas_datatype,
    ldd: i32,
    compute_type: sys::rocblas_datatype,
    algo: sys::rocblas_gemm_algo,
    solution_index: i32,
    flags: u32,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_gemm_ex(
            h,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a as *const c_void,
            a_type,
            lda,
            b as *const c_void,
            b_type,
            ldb,
            beta,
            c as *const c_void,
            c_type,
            ldc,
            d as *mut c_void,
            d_type,
            ldd,
            compute_type,
            algo,
            solution_index,
            flags,
        ))
    }
}

// ----- L3: strided-batched GEMM -----

#[allow(clippy::too_many_arguments)]
pub unsafe fn sgemm_strided_batched(
    h: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: i32,
    n: i32,
    k: i32,
    alpha: &f32,
    a: u64,
    lda: i32,
    stride_a: i64,
    b: u64,
    ldb: i32,
    stride_b: i64,
    beta: &f32,
    c: u64,
    ldc: i32,
    stride_c: i64,
    batch_count: i32,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_sgemm_strided_batched(
            h,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a as *const f32,
            lda,
            stride_a,
            b as *const f32,
            ldb,
            stride_b,
            beta,
            c as *mut f32,
            ldc,
            stride_c,
            batch_count,
        ))
    }
}

#[allow(clippy::too_many_arguments)]
pub unsafe fn dgemm_strided_batched(
    h: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: i32,
    n: i32,
    k: i32,
    alpha: &f64,
    a: u64,
    lda: i32,
    stride_a: i64,
    b: u64,
    ldb: i32,
    stride_b: i64,
    beta: &f64,
    c: u64,
    ldc: i32,
    stride_c: i64,
    batch_count: i32,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_dgemm_strided_batched(
            h,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a as *const f64,
            lda,
            stride_a,
            b as *const f64,
            ldb,
            stride_b,
            beta,
            c as *mut f64,
            ldc,
            stride_c,
            batch_count,
        ))
    }
}

#[allow(clippy::too_many_arguments)]
pub unsafe fn gemm_strided_batched_ex(
    h: sys::rocblas_handle,
    transa: sys::rocblas_operation,
    transb: sys::rocblas_operation,
    m: i32,
    n: i32,
    k: i32,
    alpha: *const c_void,
    a: u64,
    a_type: sys::rocblas_datatype,
    lda: i32,
    stride_a: i64,
    b: u64,
    b_type: sys::rocblas_datatype,
    ldb: i32,
    stride_b: i64,
    beta: *const c_void,
    c: u64,
    c_type: sys::rocblas_datatype,
    ldc: i32,
    stride_c: i64,
    d: u64,
    d_type: sys::rocblas_datatype,
    ldd: i32,
    stride_d: i64,
    batch_count: i32,
    compute_type: sys::rocblas_datatype,
    algo: sys::rocblas_gemm_algo,
    solution_index: i32,
    flags: u32,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_gemm_strided_batched_ex(
            h,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a as *const c_void,
            a_type,
            lda,
            stride_a,
            b as *const c_void,
            b_type,
            ldb,
            stride_b,
            beta,
            c as *const c_void,
            c_type,
            ldc,
            stride_c,
            d as *mut c_void,
            d_type,
            ldd,
            stride_d,
            batch_count,
            compute_type,
            algo,
            solution_index,
            flags,
        ))
    }
}

// ----- L2: GEMV -----

#[allow(clippy::too_many_arguments)]
pub unsafe fn sgemv(
    h: sys::rocblas_handle,
    trans: sys::rocblas_operation,
    m: i32,
    n: i32,
    alpha: &f32,
    a: u64,
    lda: i32,
    x: u64,
    incx: i32,
    beta: &f32,
    y: u64,
    incy: i32,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_sgemv(
            h,
            trans,
            m,
            n,
            alpha,
            a as *const f32,
            lda,
            x as *const f32,
            incx,
            beta,
            y as *mut f32,
            incy,
        ))
    }
}

#[allow(clippy::too_many_arguments)]
pub unsafe fn dgemv(
    h: sys::rocblas_handle,
    trans: sys::rocblas_operation,
    m: i32,
    n: i32,
    alpha: &f64,
    a: u64,
    lda: i32,
    x: u64,
    incx: i32,
    beta: &f64,
    y: u64,
    incy: i32,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_dgemv(
            h,
            trans,
            m,
            n,
            alpha,
            a as *const f64,
            lda,
            x as *const f64,
            incx,
            beta,
            y as *mut f64,
            incy,
        ))
    }
}

// ----- L1 -----

pub unsafe fn saxpy(
    h: sys::rocblas_handle,
    n: i32,
    alpha: &f32,
    x: u64,
    incx: i32,
    y: u64,
    incy: i32,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_saxpy(
            h,
            n,
            alpha,
            x as *const f32,
            incx,
            y as *mut f32,
            incy,
        ))
    }
}

pub unsafe fn daxpy(
    h: sys::rocblas_handle,
    n: i32,
    alpha: &f64,
    x: u64,
    incx: i32,
    y: u64,
    incy: i32,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_daxpy(
            h,
            n,
            alpha,
            x as *const f64,
            incx,
            y as *mut f64,
            incy,
        ))
    }
}

pub unsafe fn sscal(
    h: sys::rocblas_handle,
    n: i32,
    alpha: &f32,
    x: u64,
    incx: i32,
) -> Result<(), RocblasError> {
    unsafe { check(sys::rocblas_sscal(h, n, alpha, x as *mut f32, incx)) }
}

pub unsafe fn dscal(
    h: sys::rocblas_handle,
    n: i32,
    alpha: &f64,
    x: u64,
    incx: i32,
) -> Result<(), RocblasError> {
    unsafe { check(sys::rocblas_dscal(h, n, alpha, x as *mut f64, incx)) }
}

pub unsafe fn snrm2(
    h: sys::rocblas_handle,
    n: i32,
    x: u64,
    incx: i32,
    result: u64,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_snrm2(
            h,
            n,
            x as *const f32,
            incx,
            result as *mut f32,
        ))
    }
}

pub unsafe fn dnrm2(
    h: sys::rocblas_handle,
    n: i32,
    x: u64,
    incx: i32,
    result: u64,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_dnrm2(
            h,
            n,
            x as *const f64,
            incx,
            result as *mut f64,
        ))
    }
}

pub unsafe fn sdot(
    h: sys::rocblas_handle,
    n: i32,
    x: u64,
    incx: i32,
    y: u64,
    incy: i32,
    result: u64,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_sdot(
            h,
            n,
            x as *const f32,
            incx,
            y as *const f32,
            incy,
            result as *mut f32,
        ))
    }
}

pub unsafe fn ddot(
    h: sys::rocblas_handle,
    n: i32,
    x: u64,
    incx: i32,
    y: u64,
    incy: i32,
    result: u64,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_ddot(
            h,
            n,
            x as *const f64,
            incx,
            y as *const f64,
            incy,
            result as *mut f64,
        ))
    }
}

pub unsafe fn scopy(
    h: sys::rocblas_handle,
    n: i32,
    x: u64,
    incx: i32,
    y: u64,
    incy: i32,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_scopy(
            h,
            n,
            x as *const f32,
            incx,
            y as *mut f32,
            incy,
        ))
    }
}

pub unsafe fn dcopy(
    h: sys::rocblas_handle,
    n: i32,
    x: u64,
    incx: i32,
    y: u64,
    incy: i32,
) -> Result<(), RocblasError> {
    unsafe {
        check(sys::rocblas_dcopy(
            h,
            n,
            x as *const f64,
            incx,
            y as *mut f64,
            incy,
        ))
    }
}
