//! Safe wrappers for rocBLAS.
//!
//! Layout mirrors `cudarc::cublas`. Handle is bound to a single [`HipStream`] at
//! construction time and keeps it alive via `Arc`. Ops are exposed via
//! per-scalar-type traits (`Gemm<T>`, `Gemv<T>`, `Axpy<T>` …) so call sites can
//! be generic over the precision.

use std::{ffi::c_void, sync::Arc};

use crate::driver::HipStream;

pub mod result;
pub mod sys;

#[derive(Debug, thiserror::Error)]
pub enum RocblasError {
    #[error("rocBLAS error: {0:?}")]
    Rocblas(sys::rocblas_status),
}

/// Transpose flag passed to BLAS ops. Maps to `sys::rocblas_operation`.
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    None,
    Transpose,
    ConjugateTranspose,
}

impl From<Operation> for sys::rocblas_operation {
    fn from(op: Operation) -> Self {
        match op {
            Operation::None => sys::rocblas_operation::rocblas_operation_none,
            Operation::Transpose => sys::rocblas_operation::rocblas_operation_transpose,
            Operation::ConjugateTranspose => {
                sys::rocblas_operation::rocblas_operation_conjugate_transpose
            }
        }
    }
}

/// RAII rocBLAS handle bound to a single [`HipStream`].
///
/// rocBLAS handles are *not* thread-safe by the upstream contract — one handle
/// per host thread — but we mark `Send + Sync` to match the convention used
/// elsewhere in this crate (see [`HipStream`]). Callers sharing a handle across
/// threads are responsible for external synchronization.
pub struct RocblasHandle {
    raw: sys::rocblas_handle,
    #[allow(dead_code)]
    stream: Arc<HipStream>,
}

impl RocblasHandle {
    pub fn new(stream: Arc<HipStream>) -> Result<Arc<Self>, RocblasError> {
        let raw = result::create_handle()?;
        // driver::sys and rocblas::sys each redeclare ihipStream_t, so rustc
        // sees them as distinct types despite identical layout. Cast at the bridge.
        result::set_stream(raw, stream.hip_stream().cast())?;
        Ok(Arc::new(Self { raw, stream }))
    }

    /// Switch pointer mode (host vs device) for scalar args like `alpha`/`beta`
    /// and reduction `result` arguments. Defaults to host on a fresh handle.
    pub fn set_pointer_mode(&self, mode: sys::rocblas_pointer_mode) -> Result<(), RocblasError> {
        result::set_pointer_mode(self.raw, mode)
    }

    /// Raw handle. Exposed so callers can hand it to functions in
    /// [`result`] that haven't been wrapped yet.
    pub fn rocblas_handle(&self) -> sys::rocblas_handle {
        self.raw
    }
}

impl Drop for RocblasHandle {
    fn drop(&mut self) {
        let _ = result::destroy_handle(self.raw);
    }
}

// rocBLAS handle is not Sync per AMD docs (single-thread use). We mark it
// anyway for ergonomic parity with the rest of the crate; document and uphold
// the constraint externally.
unsafe impl Send for RocblasHandle {}
unsafe impl Sync for RocblasHandle {}

// ----- Config structs -----

#[derive(Clone, Copy)]
pub struct GemmConfig<T> {
    pub transa: Operation,
    pub transb: Operation,
    pub m: i32,
    pub n: i32,
    pub k: i32,
    pub alpha: T,
    pub lda: i32,
    pub ldb: i32,
    pub beta: T,
    pub ldc: i32,
}

#[derive(Clone, Copy)]
pub struct StridedBatchedConfig<T> {
    pub gemm: GemmConfig<T>,
    pub stride_a: i64,
    pub stride_b: i64,
    pub stride_c: i64,
    pub batch_count: i32,
}

#[derive(Clone, Copy)]
pub struct GemvConfig<T> {
    pub trans: Operation,
    pub m: i32,
    pub n: i32,
    pub alpha: T,
    pub lda: i32,
    pub incx: i32,
    pub beta: T,
    pub incy: i32,
}

#[derive(Clone, Copy)]
pub struct AxpyConfig<T> {
    pub n: i32,
    pub alpha: T,
    pub incx: i32,
    pub incy: i32,
}

#[derive(Clone, Copy)]
pub struct ScalConfig<T> {
    pub n: i32,
    pub alpha: T,
    pub incx: i32,
}

#[derive(Clone, Copy)]
pub struct Nrm2Config {
    pub n: i32,
    pub incx: i32,
}

#[derive(Clone, Copy)]
pub struct DotConfig {
    pub n: i32,
    pub incx: i32,
    pub incy: i32,
}

#[derive(Clone, Copy)]
pub struct CopyConfig {
    pub n: i32,
    pub incx: i32,
    pub incy: i32,
}

// ----- Traits -----

pub trait Gemm<T> {
    /// # Safety
    /// `a`, `b`, `c` must be valid device pointers sized for `cfg`'s dimensions
    /// and leading dimensions. `c` is read for `beta != 0` and always written.
    unsafe fn gemm(
        &self,
        cfg: GemmConfig<T>,
        a: u64,
        b: u64,
        c: u64,
    ) -> Result<(), RocblasError>;

    /// # Safety: same as [`Gemm::gemm`], applied per batch element.
    unsafe fn gemm_strided_batched(
        &self,
        cfg: StridedBatchedConfig<T>,
        a: u64,
        b: u64,
        c: u64,
    ) -> Result<(), RocblasError>;
}

pub trait Gemv<T> {
    /// # Safety: device pointers must be valid for `cfg`'s dimensions.
    unsafe fn gemv(
        &self,
        cfg: GemvConfig<T>,
        a: u64,
        x: u64,
        y: u64,
    ) -> Result<(), RocblasError>;
}

pub trait Axpy<T> {
    /// y := alpha*x + y.
    ///
    /// # Safety: device pointers must be valid for `cfg.n` elements at the given strides.
    unsafe fn axpy(&self, cfg: AxpyConfig<T>, x: u64, y: u64) -> Result<(), RocblasError>;
}

pub trait Scal<T> {
    /// x := alpha*x.
    /// # Safety: device pointer must be valid for `cfg.n * cfg.incx` elements.
    unsafe fn scal(&self, cfg: ScalConfig<T>, x: u64) -> Result<(), RocblasError>;
}

pub trait Nrm2<T> {
    /// result := sqrt(sum(x[i]^2)). `result` is interpreted per the handle's pointer mode.
    /// # Safety: pointers valid; `result` points to a single `T`.
    unsafe fn nrm2(
        &self,
        cfg: Nrm2Config,
        x: u64,
        result: u64,
    ) -> Result<(), RocblasError>;
}

pub trait Dot<T> {
    /// result := sum(x[i] * y[i]). `result` is interpreted per the handle's pointer mode.
    /// # Safety: pointers valid; `result` points to a single `T`.
    unsafe fn dot(
        &self,
        cfg: DotConfig,
        x: u64,
        y: u64,
        result: u64,
    ) -> Result<(), RocblasError>;
}

pub trait Copy<T> {
    /// y := x.
    /// # Safety: pointers valid for `cfg.n` elements.
    unsafe fn copy(&self, cfg: CopyConfig, x: u64, y: u64) -> Result<(), RocblasError>;
}

// ----- Trait impls: f32 -----

impl Gemm<f32> for RocblasHandle {
    unsafe fn gemm(
        &self,
        cfg: GemmConfig<f32>,
        a: u64,
        b: u64,
        c: u64,
    ) -> Result<(), RocblasError> {
        unsafe {
            result::sgemm(
                self.raw,
                cfg.transa.into(),
                cfg.transb.into(),
                cfg.m,
                cfg.n,
                cfg.k,
                &cfg.alpha,
                a,
                cfg.lda,
                b,
                cfg.ldb,
                &cfg.beta,
                c,
                cfg.ldc,
            )
        }
    }

    unsafe fn gemm_strided_batched(
        &self,
        cfg: StridedBatchedConfig<f32>,
        a: u64,
        b: u64,
        c: u64,
    ) -> Result<(), RocblasError> {
        let g = cfg.gemm;
        unsafe {
            result::sgemm_strided_batched(
                self.raw,
                g.transa.into(),
                g.transb.into(),
                g.m,
                g.n,
                g.k,
                &g.alpha,
                a,
                g.lda,
                cfg.stride_a,
                b,
                g.ldb,
                cfg.stride_b,
                &g.beta,
                c,
                g.ldc,
                cfg.stride_c,
                cfg.batch_count,
            )
        }
    }
}

impl Gemm<f64> for RocblasHandle {
    unsafe fn gemm(
        &self,
        cfg: GemmConfig<f64>,
        a: u64,
        b: u64,
        c: u64,
    ) -> Result<(), RocblasError> {
        unsafe {
            result::dgemm(
                self.raw,
                cfg.transa.into(),
                cfg.transb.into(),
                cfg.m,
                cfg.n,
                cfg.k,
                &cfg.alpha,
                a,
                cfg.lda,
                b,
                cfg.ldb,
                &cfg.beta,
                c,
                cfg.ldc,
            )
        }
    }

    unsafe fn gemm_strided_batched(
        &self,
        cfg: StridedBatchedConfig<f64>,
        a: u64,
        b: u64,
        c: u64,
    ) -> Result<(), RocblasError> {
        let g = cfg.gemm;
        unsafe {
            result::dgemm_strided_batched(
                self.raw,
                g.transa.into(),
                g.transb.into(),
                g.m,
                g.n,
                g.k,
                &g.alpha,
                a,
                g.lda,
                cfg.stride_a,
                b,
                g.ldb,
                cfg.stride_b,
                &g.beta,
                c,
                g.ldc,
                cfg.stride_c,
                cfg.batch_count,
            )
        }
    }
}

// ----- Trait impls: f16 and bf16 via gemm_ex -----
//
// gemm_ex compute mode is f32 (HPA). Alpha/beta are promoted to f32 here.

impl Gemm<half::f16> for RocblasHandle {
    unsafe fn gemm(
        &self,
        cfg: GemmConfig<half::f16>,
        a: u64,
        b: u64,
        c: u64,
    ) -> Result<(), RocblasError> {
        let alpha = cfg.alpha.to_f32();
        let beta = cfg.beta.to_f32();
        unsafe {
            result::gemm_ex(
                self.raw,
                cfg.transa.into(),
                cfg.transb.into(),
                cfg.m,
                cfg.n,
                cfg.k,
                &alpha as *const _ as *const c_void,
                a,
                sys::rocblas_datatype::rocblas_datatype_f16_r,
                cfg.lda,
                b,
                sys::rocblas_datatype::rocblas_datatype_f16_r,
                cfg.ldb,
                &beta as *const _ as *const c_void,
                c,
                sys::rocblas_datatype::rocblas_datatype_f16_r,
                cfg.ldc,
                c,
                sys::rocblas_datatype::rocblas_datatype_f16_r,
                cfg.ldc,
                sys::rocblas_datatype::rocblas_datatype_f32_r,
                sys::rocblas_gemm_algo::rocblas_gemm_algo_standard,
                0,
                0,
            )
        }
    }

    unsafe fn gemm_strided_batched(
        &self,
        cfg: StridedBatchedConfig<half::f16>,
        a: u64,
        b: u64,
        c: u64,
    ) -> Result<(), RocblasError> {
        let g = cfg.gemm;
        let alpha = g.alpha.to_f32();
        let beta = g.beta.to_f32();
        unsafe {
            result::gemm_strided_batched_ex(
                self.raw,
                g.transa.into(),
                g.transb.into(),
                g.m,
                g.n,
                g.k,
                &alpha as *const _ as *const c_void,
                a,
                sys::rocblas_datatype::rocblas_datatype_f16_r,
                g.lda,
                cfg.stride_a,
                b,
                sys::rocblas_datatype::rocblas_datatype_f16_r,
                g.ldb,
                cfg.stride_b,
                &beta as *const _ as *const c_void,
                c,
                sys::rocblas_datatype::rocblas_datatype_f16_r,
                g.ldc,
                cfg.stride_c,
                c,
                sys::rocblas_datatype::rocblas_datatype_f16_r,
                g.ldc,
                cfg.stride_c,
                cfg.batch_count,
                sys::rocblas_datatype::rocblas_datatype_f32_r,
                sys::rocblas_gemm_algo::rocblas_gemm_algo_standard,
                0,
                0,
            )
        }
    }
}

impl Gemm<half::bf16> for RocblasHandle {
    unsafe fn gemm(
        &self,
        cfg: GemmConfig<half::bf16>,
        a: u64,
        b: u64,
        c: u64,
    ) -> Result<(), RocblasError> {
        let alpha = cfg.alpha.to_f32();
        let beta = cfg.beta.to_f32();
        unsafe {
            result::gemm_ex(
                self.raw,
                cfg.transa.into(),
                cfg.transb.into(),
                cfg.m,
                cfg.n,
                cfg.k,
                &alpha as *const _ as *const c_void,
                a,
                sys::rocblas_datatype::rocblas_datatype_bf16_r,
                cfg.lda,
                b,
                sys::rocblas_datatype::rocblas_datatype_bf16_r,
                cfg.ldb,
                &beta as *const _ as *const c_void,
                c,
                sys::rocblas_datatype::rocblas_datatype_bf16_r,
                cfg.ldc,
                c,
                sys::rocblas_datatype::rocblas_datatype_bf16_r,
                cfg.ldc,
                sys::rocblas_datatype::rocblas_datatype_f32_r,
                sys::rocblas_gemm_algo::rocblas_gemm_algo_standard,
                0,
                0,
            )
        }
    }

    unsafe fn gemm_strided_batched(
        &self,
        cfg: StridedBatchedConfig<half::bf16>,
        a: u64,
        b: u64,
        c: u64,
    ) -> Result<(), RocblasError> {
        let g = cfg.gemm;
        let alpha = g.alpha.to_f32();
        let beta = g.beta.to_f32();
        unsafe {
            result::gemm_strided_batched_ex(
                self.raw,
                g.transa.into(),
                g.transb.into(),
                g.m,
                g.n,
                g.k,
                &alpha as *const _ as *const c_void,
                a,
                sys::rocblas_datatype::rocblas_datatype_bf16_r,
                g.lda,
                cfg.stride_a,
                b,
                sys::rocblas_datatype::rocblas_datatype_bf16_r,
                g.ldb,
                cfg.stride_b,
                &beta as *const _ as *const c_void,
                c,
                sys::rocblas_datatype::rocblas_datatype_bf16_r,
                g.ldc,
                cfg.stride_c,
                c,
                sys::rocblas_datatype::rocblas_datatype_bf16_r,
                g.ldc,
                cfg.stride_c,
                cfg.batch_count,
                sys::rocblas_datatype::rocblas_datatype_f32_r,
                sys::rocblas_gemm_algo::rocblas_gemm_algo_standard,
                0,
                0,
            )
        }
    }
}

// ----- L2: GEMV (f32, f64) -----

impl Gemv<f32> for RocblasHandle {
    unsafe fn gemv(
        &self,
        cfg: GemvConfig<f32>,
        a: u64,
        x: u64,
        y: u64,
    ) -> Result<(), RocblasError> {
        unsafe {
            result::sgemv(
                self.raw,
                cfg.trans.into(),
                cfg.m,
                cfg.n,
                &cfg.alpha,
                a,
                cfg.lda,
                x,
                cfg.incx,
                &cfg.beta,
                y,
                cfg.incy,
            )
        }
    }
}

impl Gemv<f64> for RocblasHandle {
    unsafe fn gemv(
        &self,
        cfg: GemvConfig<f64>,
        a: u64,
        x: u64,
        y: u64,
    ) -> Result<(), RocblasError> {
        unsafe {
            result::dgemv(
                self.raw,
                cfg.trans.into(),
                cfg.m,
                cfg.n,
                &cfg.alpha,
                a,
                cfg.lda,
                x,
                cfg.incx,
                &cfg.beta,
                y,
                cfg.incy,
            )
        }
    }
}

// ----- L1 impls (f32 + f64) -----

impl Axpy<f32> for RocblasHandle {
    unsafe fn axpy(
        &self,
        cfg: AxpyConfig<f32>,
        x: u64,
        y: u64,
    ) -> Result<(), RocblasError> {
        unsafe { result::saxpy(self.raw, cfg.n, &cfg.alpha, x, cfg.incx, y, cfg.incy) }
    }
}

impl Axpy<f64> for RocblasHandle {
    unsafe fn axpy(
        &self,
        cfg: AxpyConfig<f64>,
        x: u64,
        y: u64,
    ) -> Result<(), RocblasError> {
        unsafe { result::daxpy(self.raw, cfg.n, &cfg.alpha, x, cfg.incx, y, cfg.incy) }
    }
}

impl Scal<f32> for RocblasHandle {
    unsafe fn scal(&self, cfg: ScalConfig<f32>, x: u64) -> Result<(), RocblasError> {
        unsafe { result::sscal(self.raw, cfg.n, &cfg.alpha, x, cfg.incx) }
    }
}

impl Scal<f64> for RocblasHandle {
    unsafe fn scal(&self, cfg: ScalConfig<f64>, x: u64) -> Result<(), RocblasError> {
        unsafe { result::dscal(self.raw, cfg.n, &cfg.alpha, x, cfg.incx) }
    }
}

impl Nrm2<f32> for RocblasHandle {
    unsafe fn nrm2(
        &self,
        cfg: Nrm2Config,
        x: u64,
        result_ptr: u64,
    ) -> Result<(), RocblasError> {
        unsafe { result::snrm2(self.raw, cfg.n, x, cfg.incx, result_ptr) }
    }
}

impl Nrm2<f64> for RocblasHandle {
    unsafe fn nrm2(
        &self,
        cfg: Nrm2Config,
        x: u64,
        result_ptr: u64,
    ) -> Result<(), RocblasError> {
        unsafe { result::dnrm2(self.raw, cfg.n, x, cfg.incx, result_ptr) }
    }
}

impl Dot<f32> for RocblasHandle {
    unsafe fn dot(
        &self,
        cfg: DotConfig,
        x: u64,
        y: u64,
        result_ptr: u64,
    ) -> Result<(), RocblasError> {
        unsafe { result::sdot(self.raw, cfg.n, x, cfg.incx, y, cfg.incy, result_ptr) }
    }
}

impl Dot<f64> for RocblasHandle {
    unsafe fn dot(
        &self,
        cfg: DotConfig,
        x: u64,
        y: u64,
        result_ptr: u64,
    ) -> Result<(), RocblasError> {
        unsafe { result::ddot(self.raw, cfg.n, x, cfg.incx, y, cfg.incy, result_ptr) }
    }
}

impl Copy<f32> for RocblasHandle {
    unsafe fn copy(&self, cfg: CopyConfig, x: u64, y: u64) -> Result<(), RocblasError> {
        unsafe { result::scopy(self.raw, cfg.n, x, cfg.incx, y, cfg.incy) }
    }
}

impl Copy<f64> for RocblasHandle {
    unsafe fn copy(&self, cfg: CopyConfig, x: u64, y: u64) -> Result<(), RocblasError> {
        unsafe { result::dcopy(self.raw, cfg.n, x, cfg.incx, y, cfg.incy) }
    }
}
