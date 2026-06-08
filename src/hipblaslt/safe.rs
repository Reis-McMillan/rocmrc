//! Safe abstractions around [`crate::hipblaslt::result`] for doing matmul.
//! Mirrors `cudarc::cublaslt::safe` with HIP-specific divergences flagged
//! inline (workspace sizing, TF32 semantics, `f16` feature gating).

use super::{result, sys};
use crate::hipblaslt::result::set_matrix_layout_attribute;
use crate::hip::sys::{hipDeviceAttribute_t, hipDeviceptr_t};
use crate::hip::{DevicePtr, DevicePtrMut, HipError, HipSlice, HipStream};
use core::ffi::c_int;
use core::mem;
use std::sync::Arc;

pub use super::result::HipblasError;

/// Wrapper around [`sys::hipblasLtHandle_t`]. Analogue of
/// `cudarc::cublaslt::CudaBlasLT`.
///
/// 1. Create with [`HipBlasLT::new`].
/// 2. Execute matmul kernels via the [`Matmul`] trait. `f32` is always
///    available; `f16` / `bf16` are gated on the `f16` feature.
///
/// Keeps an `Arc<HipStream>`, so the stream's context will be retained for
/// the lifetime of this handle.
#[derive(Debug)]
pub struct HipBlasLT {
    handle: sys::hipblasLtHandle_t,
    workspace: Workspace,
    stream: Arc<HipStream>,
}

unsafe impl Send for HipBlasLT {}
unsafe impl Sync for HipBlasLT {}

impl HipBlasLT {
    /// Creates a new hipBLASLt handle bound to `stream`.
    pub fn new(stream: Arc<HipStream>) -> Result<Self, HipblasError> {
        let handle = result::create_handle()?;
        let workspace = Workspace::new(stream.clone()).unwrap();
        Ok(Self {
            handle,
            workspace,
            stream,
        })
    }
}

impl Drop for HipBlasLT {
    fn drop(&mut self) {
        let handle = mem::replace(&mut self.handle, std::ptr::null_mut());
        if !handle.is_null() {
            unsafe { result::destroy_handle(handle) }.unwrap();
        }
    }
}

/// User-owned hipBLASLt workspace buffer.
///
/// **rocmrc sizing policy:** branch on `hipDeviceAttributeComputeCapabilityMajor`,
/// which AMD overloads to encode the gfx family. `major >= 9` covers the
/// Instinct line (gfx9 — MI200 / MI300) where matmul workloads benefit
/// from the larger 32 MiB buffer; smaller Radeon parts (RDNA family,
/// gfx10/11/12) use 4 MiB. Mirrors cudarc's Hopper-vs-other split.
#[derive(Debug, Clone)]
pub struct Workspace {
    pub(crate) buffer: HipSlice<u8>,
    pub(crate) size: usize,
}

impl Workspace {
    /// Allocate a workspace buffer on the device backing `stream`.
    pub fn new(stream: Arc<HipStream>) -> Result<Self, HipError> {
        stream.context().bind_to_thread()?;

        let major = stream
            .context()
            .attribute(hipDeviceAttribute_t::hipDeviceAttributeComputeCapabilityMajor)?;
        let workspace_size = if major >= 9 { 33_554_432 } else { 4_194_304 };

        let buffer = unsafe { stream.alloc::<u8>(workspace_size) }?;
        Ok(Self {
            buffer,
            size: workspace_size,
        })
    }
}

/// Activation function for kernel-fused matmul epilogues.
#[derive(Debug, Clone)]
pub enum Activation {
    Relu,
    Gelu,
}

// ---------------------------------------------------------------------------
// MatrixLayout — private RAII wrapper for the hipBLASLt layout descriptor.
// ---------------------------------------------------------------------------

struct MatrixLayout {
    handle: sys::hipblasLtMatrixLayout_t,
}

impl MatrixLayout {
    fn new(
        matrix_type: sys::hipDataType,
        rows: u64,
        cols: u64,
        ld: i64,
    ) -> Result<Self, HipblasError> {
        let handle = result::create_matrix_layout(matrix_type, rows, cols, ld)?;
        Ok(Self { handle })
    }

    fn set_batch(&self, size: c_int, stride: i64) -> Result<(), HipblasError> {
        unsafe {
            // Set batch size
            set_matrix_layout_attribute(
                self.handle,
                sys::hipblasLtMatrixLayoutAttribute_t::HIPBLASLT_MATRIX_LAYOUT_BATCH_COUNT,
                (&size) as *const _ as *const _,
                mem::size_of::<c_int>(),
            )?;
            // Set batch stride
            set_matrix_layout_attribute(
                self.handle,
                sys::hipblasLtMatrixLayoutAttribute_t::HIPBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET,
                (&stride) as *const _ as *const _,
                mem::size_of::<i64>(),
            )?;
        }
        Ok(())
    }
}

impl Drop for MatrixLayout {
    fn drop(&mut self) {
        // panic on failure (mirrors cudarc)
        unsafe {
            result::destroy_matrix_layout(self.handle)
                .expect("Unable to destroy matrix layout")
        }
    }
}

enum Matrix {
    A,
    B,
}

// ---------------------------------------------------------------------------
// MatmulDesc — private RAII wrapper for the matmul descriptor.
// ---------------------------------------------------------------------------

struct MatmulDesc {
    handle: sys::hipblasLtMatmulDesc_t,
}

impl MatmulDesc {
    fn new(
        compute_type: sys::hipblasComputeType_t,
        scale_type: sys::hipDataType,
    ) -> Result<Self, HipblasError> {
        let handle = result::create_matmul_desc(compute_type, scale_type)?;
        Ok(Self { handle })
    }

    fn set_transpose(&self, transpose: bool, matrix: Matrix) -> Result<(), HipblasError> {
        // 1 == T, 0 == N (per hipBLASLt convention).
        let transpose = transpose as i32;
        let attr = match matrix {
            Matrix::A => sys::hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_TRANSA,
            Matrix::B => sys::hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_TRANSB,
        };

        unsafe {
            result::set_matmul_desc_attribute(
                self.handle,
                attr,
                (&transpose) as *const _ as *const _,
                mem::size_of::<u32>(),
            )?;
        }
        Ok(())
    }

    /// Fuse an activation + bias add into the matmul epilogue.
    ///
    /// **HIP divergence:** hipBLASLt has no `BIAS_BATCH_STRIDE`
    /// attribute (cuBLASLt does), so bias-per-batch isn't supported on
    /// HIP. The bias pointer is broadcast across all batches.
    fn set_epilogue(
        &self,
        act: Option<&Activation>,
        bias_ptr: Option<&hipDeviceptr_t>,
    ) -> Result<(), HipblasError> {
        let epilogue = if let Some(bias_ptr) = bias_ptr {
            let epilogue = act
                .map(|act| match act {
                    // Act + bias
                    Activation::Relu => {
                        sys::hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_RELU_BIAS
                    }
                    Activation::Gelu => {
                        sys::hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_GELU_BIAS
                    }
                })
                // Bias only
                .unwrap_or(sys::hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_BIAS);

            // Set bias device pointer on the matmul descriptor.
            unsafe {
                result::set_matmul_desc_attribute(
                    self.handle,
                    sys::hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_BIAS_POINTER,
                    bias_ptr as *const hipDeviceptr_t as *const _,
                    mem::size_of::<hipDeviceptr_t>(),
                )?;
            }

            epilogue
        } else if let Some(act) = act {
            // Activation only.
            match act {
                Activation::Relu => sys::hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_RELU,
                Activation::Gelu => sys::hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_GELU,
            }
        } else {
            // No epilogue.
            sys::hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_DEFAULT
        };

        unsafe {
            result::set_matmul_desc_attribute(
                self.handle,
                sys::hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_EPILOGUE,
                (&epilogue) as *const _ as *const _,
                mem::size_of::<sys::hipblasLtMatmulDescAttributes_t>(),
            )?;
        }
        Ok(())
    }
}

impl Drop for MatmulDesc {
    fn drop(&mut self) {
        unsafe {
            result::destroy_matmul_desc(self.handle)
                .expect("Unable to destroy matmul desc")
        }
    }
}

// ---------------------------------------------------------------------------
// MatmulPref — private RAII wrapper for the matmul preferences descriptor.
// ---------------------------------------------------------------------------

struct MatmulPref {
    handle: sys::hipblasLtMatmulPreference_t,
}

impl MatmulPref {
    fn new() -> Result<Self, HipblasError> {
        let handle = result::create_matmul_pref()?;
        Ok(Self { handle })
    }

    fn set_workspace_size(&self, size: usize) -> Result<(), HipblasError> {
        unsafe {
            result::set_matmul_pref_attribute(
                self.handle,
                sys::hipblasLtMatmulPreferenceAttributes_t::HIPBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES,
                (&size) as *const _ as *const _,
                mem::size_of::<usize>(),
            )?;
        }
        Ok(())
    }
}

impl Drop for MatmulPref {
    fn drop(&mut self) {
        unsafe {
            result::destroy_matmul_pref(self.handle)
                .expect("Unable to destroy matmul pref")
        }
    }
}

// ---------------------------------------------------------------------------
// Public traits: MatmulShared + Matmul
// ---------------------------------------------------------------------------

/// Super-trait providing read-only access to the hipBLASLt handle,
/// workspace, and stream.
pub trait MatmulShared {
    /// Underlying hipBLASLt handle.
    fn handle(&self) -> &sys::hipblasLtHandle_t;

    /// Workspace allocated for matmul use.
    fn workspace(&self) -> &Workspace;

    /// Stream this handle was bound to.
    fn stream(&self) -> &Arc<HipStream>;
}

/// Configuration for [`Matmul::matmul`].
///
/// **HIP divergence from cudarc's `cudarc::cublaslt::MatmulConfig`:**
/// `transc` (no `HIPBLASLT_MATMUL_DESC_TRANSC` enum) and `stride_bias`
/// (no `HIPBLASLT_MATMUL_DESC_BIAS_BATCH_STRIDE`) are absent from this
/// struct because hipBLASLt doesn't accept them — keeping them as
/// unused fields would silently mislead callers.
#[derive(Debug, Copy, Clone)]
pub struct MatmulConfig {
    pub transa: bool,
    pub transb: bool,
    pub m: u64,
    pub n: u64,
    pub k: u64,
    pub alpha: f32,
    pub lda: i64,
    pub ldb: i64,
    pub beta: f32,
    pub ldc: i64,
    pub stride_a: Option<i64>,
    pub stride_b: Option<i64>,
    pub stride_c: Option<i64>,
    pub batch_size: Option<c_int>,
}

/// Matrix-matrix multiplication with elements of type `T`.
pub trait Matmul<T>: MatmulShared {
    /// Underlying HIP data type for `T`.
    fn matrix_type() -> sys::hipDataType;

    /// Underlying hipBLASLt compute type for `T`.
    fn compute_type() -> sys::hipblasComputeType_t;

    /// Run the matmul `D = alpha * (A * B) + beta * C` with an optional
    /// fused bias + activation epilogue. `C` doubles as both the input
    /// for the beta blend and the output buffer (matches cudarc's
    /// signature; hipBLASLt accepts in-place blends).
    ///
    /// # Safety
    /// Improper layouts, lda/ldb/ldc, or non-overlapping buffer
    /// expectations can lead to invalid memory accesses on the device.
    unsafe fn matmul<I: DevicePtr<T>, O: DevicePtrMut<T>>(
        &self,
        cfg: MatmulConfig,
        a: &I,
        b: &I,
        c: &mut O,
        bias: Option<&I>,
        act: Option<&Activation>,
    ) -> Result<(), HipblasError> {
        let stream = self.stream();
        let workspace = self.workspace();

        let (a_rows, a_cols) = if cfg.transa {
            (cfg.k, cfg.m)
        } else {
            (cfg.m, cfg.k)
        };
        let (b_rows, b_cols) = if cfg.transb {
            (cfg.n, cfg.k)
        } else {
            (cfg.k, cfg.n)
        };

        // Matrix layouts (with optional batch attrs).
        let a_layout = MatrixLayout::new(Self::matrix_type(), a_rows, a_cols, cfg.lda)?;
        if let (Some(batch_size), Some(stride_a)) = (cfg.batch_size, cfg.stride_a) {
            a_layout.set_batch(batch_size, stride_a)?;
        }

        let b_layout = MatrixLayout::new(Self::matrix_type(), b_rows, b_cols, cfg.ldb)?;
        if let (Some(batch_size), Some(stride_b)) = (cfg.batch_size, cfg.stride_b) {
            b_layout.set_batch(batch_size, stride_b)?;
        }

        let c_layout = MatrixLayout::new(Self::matrix_type(), cfg.m, cfg.n, cfg.ldc)?;
        if let (Some(batch_size), Some(stride_c)) = (cfg.batch_size, cfg.stride_c) {
            c_layout.set_batch(batch_size, stride_c)?;
        }

        // Matmul descriptor + transpose flags.
        let matmul_desc = MatmulDesc::new(Self::compute_type(), sys::hipDataType::HIP_R_32F)?;
        matmul_desc.set_transpose(cfg.transa, Matrix::A)?;
        matmul_desc.set_transpose(cfg.transb, Matrix::B)?;

        // Optional fused bias + activation.
        let (bias, _record_bias) = bias.map(|b| b.device_ptr(stream)).unzip();
        matmul_desc.set_epilogue(act, bias.as_ref())?;

        // Heuristic search.
        let matmul_pref = MatmulPref::new()?;
        matmul_pref.set_workspace_size(self.workspace().size)?;
        let heuristic = unsafe {
            result::get_matmul_algo_heuristic(
                *self.handle(),
                matmul_desc.handle,
                a_layout.handle,
                b_layout.handle,
                c_layout.handle,
                c_layout.handle,
                matmul_pref.handle,
            )
        }?;

        // Launch the matmul.
        let (a, _record_a) = a.device_ptr(stream);
        let (b, _record_b) = b.device_ptr(stream);
        let (c, _record_c) = c.device_ptr_mut(stream);
        let (w, _record_w) = workspace.buffer.device_ptr(stream);
        unsafe {
            result::matmul(
                *self.handle(),
                matmul_desc.handle,
                (&cfg.alpha) as *const _ as *const _,
                (&cfg.beta) as *const _ as *const _,
                a as *const _,
                a_layout.handle,
                b as *const _,
                b_layout.handle,
                c as *const _,
                c_layout.handle,
                c as *mut _,
                c_layout.handle,
                (&heuristic.algo) as *const _,
                w as *mut _,
                workspace.size,
                // hip::sys and hipblaslt::sys each redeclare ihipStream_t;
                // rustc treats them as distinct nominal types. Cast at
                // the bridge (same pattern as rocblas).
                stream.hip_stream().cast(),
            )
        }
    }
}

impl MatmulShared for HipBlasLT {
    fn handle(&self) -> &sys::hipblasLtHandle_t {
        &self.handle
    }

    fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    fn stream(&self) -> &Arc<HipStream> {
        &self.stream
    }
}

impl Matmul<f32> for HipBlasLT {
    fn matrix_type() -> sys::hipDataType {
        sys::hipDataType::HIP_R_32F
    }

    /// **HIP note:** `HIPBLAS_COMPUTE_32F_FAST_TF32` is named for cudarc
    /// shape parity, but TF32 is an NVIDIA tensor-core format. On most
    /// AMD GPUs hipBLASLt will fall back to plain F32; CDNA3+ (gfx940+)
    /// may have TF32-equivalent matrix paths.
    fn compute_type() -> sys::hipblasComputeType_t {
        sys::hipblasComputeType_t::HIPBLAS_COMPUTE_32F_FAST_TF32
    }
}

#[cfg(feature = "f16")]
impl Matmul<half::f16> for HipBlasLT {
    fn matrix_type() -> sys::hipDataType {
        sys::hipDataType::HIP_R_16F
    }

    fn compute_type() -> sys::hipblasComputeType_t {
        sys::hipblasComputeType_t::HIPBLAS_COMPUTE_32F
    }
}

#[cfg(feature = "f16")]
impl Matmul<half::bf16> for HipBlasLT {
    fn matrix_type() -> sys::hipDataType {
        sys::hipDataType::HIP_R_16BF
    }

    fn compute_type() -> sys::hipblasComputeType_t {
        sys::hipblasComputeType_t::HIPBLAS_COMPUTE_32F
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_range_loop)]

    use super::*;
    use crate::hip::HipContext;

    fn matmul_truth<T, const M: usize, const N: usize, const K: usize>(
        alpha: T,
        a: &[[T; K]; M],
        b: &[[T; N]; K],
        beta: T,
        c: &mut [[T; N]; M],
    ) where
        T: Copy + Clone + std::ops::AddAssign + std::ops::MulAssign + std::ops::Mul<T, Output = T>,
    {
        for m in 0..M {
            for n in 0..N {
                c[m][n] *= beta;
            }
        }
        for m in 0..M {
            for n in 0..N {
                for k in 0..K {
                    c[m][n] += alpha * a[m][k] * b[k][n];
                }
            }
        }
    }

    #[test]
    fn test_matmul_f32() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = HipBlasLT::new(stream.clone()).unwrap();
        const M: usize = 3;
        const K: usize = 4;
        const N: usize = 5;
        let a: [[f32; K]; M] = [
            [-0.5944882, 1.8055636, 0.52204555, -0.00397902],
            [-0.38346434, -0.38013917, 0.4198623, -0.22479166],
            [-1.6661372, -0.4568837, -0.9043474, 0.39125723],
        ];
        let b: [[f32; N]; K] = [
            [1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938],
            [1.0585804, -0.39789402, 0.90205914, 0.989318, -0.3443096],
            [1.3412506, 0.3059701, -0.9714474, -0.36113533, -1.6809629],
            [3.4746711, -1.0930681, 0.16502666, -0.59988785, 0.41375792],
        ];
        let mut c: [[f32; N]; M] = [[0.0; N]; M];
        matmul_truth(1.0, &a, &b, 0.0, &mut c);

        #[rustfmt::skip]
        let a_dev = stream.clone_htod(&[
            -0.5944882, 1.8055636, 0.52204555, -0.00397902,
            -0.38346434, -0.38013917, 0.4198623, -0.22479166,
            -1.6661372, -0.4568837, -0.9043474, 0.39125723,
        ]).unwrap();
        #[rustfmt::skip]
        let b_dev = stream.clone_htod(&[
            1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938,
            1.0585804, -0.39789402, 0.90205914, 0.989318, -0.3443096,
            1.3412506, 0.3059701, -0.9714474, -0.36113533, -1.6809629,
            3.4746711, -1.0930681, 0.16502666, -0.59988785, 0.41375792,
        ]).unwrap();
        let bias = stream.alloc_zeros::<f32>(N).unwrap();
        let mut c_dev = stream.alloc_zeros::<f32>(M * N).unwrap();

        // hipBLASLt expects column-major layouts; we work around by
        // computing (B*A) which, in col-major, equals (A*B) in row-major.
        // The arg-swap and `m`/`n` swap mirror cudarc's test exactly.
        unsafe {
            blas.matmul(
                MatmulConfig {
                    transa: false,
                    transb: false,
                    m: N as u64,
                    n: M as u64,
                    k: K as u64,
                    alpha: 1.0,
                    lda: N as i64,
                    ldb: K as i64,
                    beta: 0.0,
                    ldc: N as i64,
                    stride_a: None,
                    stride_b: None,
                    stride_c: None,
                    batch_size: None,
                },
                &b_dev,
                &a_dev,
                &mut c_dev,
                Some(&bias),
                None,
            )
        }
        .unwrap();

        let c_host = stream.clone_dtoh(&c_dev).unwrap();
        for m in 0..M {
            for n in 0..N {
                let found = c_host[m * N + n];
                let expected = c[m][n];
                assert!(
                    (found - expected) <= 1e-6,
                    "found={found:?}, expected={expected:?}"
                );
            }
        }
    }

    #[cfg(feature = "f16")]
    #[test]
    fn test_matmul_half() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = HipBlasLT::new(stream.clone()).unwrap();
        const M: usize = 2;
        const K: usize = 4;
        const N: usize = 6;
        let a: [[half::f16; K]; M] = [
            [-0.5944882, 1.8055636, 0.52204555, -0.00397902],
            [-0.38346434, -0.38013917, 0.4198623, -0.22479166],
        ]
        .map(|r| r.map(half::f16::from_f32));
        let b: [[half::f16; N]; K] = [
            [
                1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938, -1.6661372,
            ],
            [
                1.0585804, -0.39789402, 0.90205914, 0.989318, -0.3443096, -0.4568837,
            ],
            [
                1.3412506, 0.3059701, -0.9714474, -0.36113533, -1.6809629, -0.9043474,
            ],
            [
                3.4746711, -1.0930681, 0.16502666, -0.59988785, 0.41375792, 0.39125723,
            ],
        ]
        .map(|r| r.map(half::f16::from_f32));
        let mut c: [[half::f16; N]; M] = [[0.0; N]; M].map(|r| r.map(half::f16::from_f32));
        matmul_truth(
            half::f16::from_f32(1.0),
            &a,
            &b,
            half::f16::from_f32(0.0),
            &mut c,
        );

        // ---- f16 leg ----
        #[rustfmt::skip]
        let a_dev = stream.clone_htod(&[
            -0.5944882, 1.8055636, 0.52204555, -0.00397902,
            -0.38346434, -0.38013917, 0.4198623, -0.22479166,
        ].map(half::f16::from_f32)).unwrap();
        #[rustfmt::skip]
        let b_dev = stream.clone_htod(&[
            1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938, -1.6661372,
            1.0585804, -0.39789402, 0.90205914, 0.989318, -0.3443096, -0.4568837,
            1.3412506, 0.3059701, -0.9714474, -0.36113533, -1.6809629, -0.9043474,
            3.4746711, -1.0930681, 0.16502666, -0.59988785, 0.41375792, 0.39125723,
        ].map(half::f16::from_f32)).unwrap();
        let bias = stream.alloc_zeros::<half::f16>(N).unwrap();
        let mut c_dev = stream.alloc_zeros::<half::f16>(M * N).unwrap();
        unsafe {
            blas.matmul(
                MatmulConfig {
                    transa: false,
                    transb: false,
                    m: N as u64,
                    n: M as u64,
                    k: K as u64,
                    alpha: 1.0,
                    lda: N as i64,
                    ldb: K as i64,
                    beta: 0.0,
                    ldc: N as i64,
                    stride_a: None,
                    stride_b: None,
                    stride_c: None,
                    batch_size: None,
                },
                &b_dev,
                &a_dev,
                &mut c_dev,
                Some(&bias),
                None,
            )
        }
        .unwrap();
        let c_host = stream.clone_dtoh(&c_dev).unwrap();
        for m in 0..M {
            for n in 0..N {
                let found = c_host[m * N + n];
                let expected = c[m][n];
                assert!(
                    (found - expected) <= half::f16::from_f32(1e-2),
                    "found={found:?}, expected={expected:?}"
                );
            }
        }

        // ---- bf16 leg ----
        // Same inputs as f16, cast through bf16::from_f32. Compares
        // against the f16 truth `c` with a looser tolerance — mirrors
        // cudarc's behavior verbatim.
        #[rustfmt::skip]
        let a_dev = stream.clone_htod(&[
            -0.5944882, 1.8055636, 0.52204555, -0.00397902,
            -0.38346434, -0.38013917, 0.4198623, -0.22479166,
        ].map(half::bf16::from_f32)).unwrap();
        #[rustfmt::skip]
        let b_dev = stream.clone_htod(&[
            1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938, -1.6661372,
            1.0585804, -0.39789402, 0.90205914, 0.989318, -0.3443096, -0.4568837,
            1.3412506, 0.3059701, -0.9714474, -0.36113533, -1.6809629, -0.9043474,
            3.4746711, -1.0930681, 0.16502666, -0.59988785, 0.41375792, 0.39125723,
        ].map(half::bf16::from_f32)).unwrap();
        let bias = stream.alloc_zeros::<half::bf16>(N).unwrap();
        let mut c_dev = stream.alloc_zeros::<half::bf16>(M * N).unwrap();
        unsafe {
            blas.matmul(
                MatmulConfig {
                    transa: false,
                    transb: false,
                    m: N as u64,
                    n: M as u64,
                    k: K as u64,
                    alpha: 1.0,
                    lda: N as i64,
                    ldb: K as i64,
                    beta: 0.0,
                    ldc: N as i64,
                    stride_a: None,
                    stride_b: None,
                    stride_c: None,
                    batch_size: None,
                },
                &b_dev,
                &a_dev,
                &mut c_dev,
                Some(&bias),
                None,
            )
        }
        .unwrap();
        let c_host = stream.clone_dtoh(&c_dev).unwrap();
        for m in 0..M {
            for n in 0..N {
                let found = c_host[m * N + n];
                let expected = c[m][n];
                assert!(
                    (half::bf16::to_f32(found) - half::f16::to_f32(expected)) <= 1e-2,
                    "found={found:?}, expected={expected:?}"
                );
            }
        }
    }
}
