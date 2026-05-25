//! Safe wrappers for hipBLASLt.
//!
//! Layout mirrors `cudarc::cublaslt`. The descriptor pattern is exposed via
//! RAII types ([`MatrixLayout`], [`MatmulDesc`], [`MatmulPref`]) that the
//! caller owns and feeds into [`HipBlasLt::get_heuristic`] and
//! [`HipBlasLt::matmul`]. The matmul workspace is caller-owned (typically a
//! `HipSlice<u8>` sized from the chosen heuristic).

use std::{ffi::c_void, sync::Arc};

use crate::driver::HipStream;

pub mod result;
pub mod sys;

/// Re-exported sys enums that callers need to construct descriptors and
/// matrix layouts. Lets downstream code use `rocmrc::hipblaslt::hipDataType`
/// (etc.) instead of reaching into the generated `sys::` module.
pub use sys::{hipDataType, hipblasComputeType_t, hipblasLtEpilogue_t, hipblasOperation_t};

#[derive(Debug, thiserror::Error)]
pub enum HipBlasLtError {
    #[error("hipBLASLt error: {0:?}")]
    HipBlasLt(sys::hipblasStatus_t),
}

// ---------------- handle ----------------

/// Top-level hipBLASLt handle. hipBLASLt handles are thread-safe per upstream
/// docs.
pub struct HipBlasLt {
    raw: sys::hipblasLtHandle_t,
}

impl HipBlasLt {
    pub fn new() -> Result<Arc<Self>, HipBlasLtError> {
        let raw = result::create()?;
        Ok(Arc::new(Self { raw }))
    }

    pub fn hipblaslt_handle(&self) -> sys::hipblasLtHandle_t {
        self.raw
    }
}

impl Drop for HipBlasLt {
    fn drop(&mut self) {
        let _ = result::destroy(self.raw);
    }
}

unsafe impl Send for HipBlasLt {}
unsafe impl Sync for HipBlasLt {}

// ---------------- matrix layout ----------------

pub struct MatrixLayout {
    raw: sys::hipblasLtMatrixLayout_t,
}

impl MatrixLayout {
    pub fn new(
        dtype: sys::hipDataType,
        rows: u64,
        cols: u64,
        ld: i64,
    ) -> Result<Self, HipBlasLtError> {
        let raw = result::matrix_layout_create(dtype, rows, cols, ld)?;
        Ok(Self { raw })
    }

    pub fn set_batch_count(&mut self, count: i32) -> Result<(), HipBlasLtError> {
        unsafe {
            result::matrix_layout_set_attribute(
                self.raw,
                sys::hipblasLtMatrixLayoutAttribute_t::HIPBLASLT_MATRIX_LAYOUT_BATCH_COUNT,
                &count as *const _ as *const c_void,
                std::mem::size_of::<i32>(),
            )
        }
    }

    pub fn set_strided_batch_offset(&mut self, stride_bytes: i64) -> Result<(), HipBlasLtError> {
        unsafe {
            result::matrix_layout_set_attribute(
                self.raw,
                sys::hipblasLtMatrixLayoutAttribute_t::HIPBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET,
                &stride_bytes as *const _ as *const c_void,
                std::mem::size_of::<i64>(),
            )
        }
    }

    pub fn raw(&self) -> sys::hipblasLtMatrixLayout_t {
        self.raw
    }
}

impl Drop for MatrixLayout {
    fn drop(&mut self) {
        let _ = result::matrix_layout_destroy(self.raw);
    }
}

unsafe impl Send for MatrixLayout {}
unsafe impl Sync for MatrixLayout {}

// ---------------- matmul descriptor ----------------

pub struct MatmulDesc {
    raw: sys::hipblasLtMatmulDesc_t,
}

impl MatmulDesc {
    pub fn new(
        compute: sys::hipblasComputeType_t,
        scale: sys::hipDataType,
    ) -> Result<Self, HipBlasLtError> {
        let raw = result::matmul_desc_create(compute, scale)?;
        Ok(Self { raw })
    }

    pub fn set_transa(&mut self, op: sys::hipblasOperation_t) -> Result<(), HipBlasLtError> {
        unsafe {
            result::matmul_desc_set_attribute(
                self.raw,
                sys::hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_TRANSA,
                &op as *const _ as *const c_void,
                std::mem::size_of::<sys::hipblasOperation_t>(),
            )
        }
    }

    pub fn set_transb(&mut self, op: sys::hipblasOperation_t) -> Result<(), HipBlasLtError> {
        unsafe {
            result::matmul_desc_set_attribute(
                self.raw,
                sys::hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_TRANSB,
                &op as *const _ as *const c_void,
                std::mem::size_of::<sys::hipblasOperation_t>(),
            )
        }
    }

    pub fn set_epilogue(&mut self, epi: sys::hipblasLtEpilogue_t) -> Result<(), HipBlasLtError> {
        unsafe {
            result::matmul_desc_set_attribute(
                self.raw,
                sys::hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_EPILOGUE,
                &epi as *const _ as *const c_void,
                std::mem::size_of::<sys::hipblasLtEpilogue_t>(),
            )
        }
    }

    /// Set the bias pointer. `ptr` is a raw device pointer (typically from
    /// `HipSlice::device_ptr()`).
    pub fn set_bias_pointer(&mut self, ptr: u64) -> Result<(), HipBlasLtError> {
        unsafe {
            result::matmul_desc_set_attribute(
                self.raw,
                sys::hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_BIAS_POINTER,
                &ptr as *const _ as *const c_void,
                std::mem::size_of::<u64>(),
            )
        }
    }

    pub fn set_bias_dtype(&mut self, dtype: sys::hipDataType) -> Result<(), HipBlasLtError> {
        unsafe {
            result::matmul_desc_set_attribute(
                self.raw,
                sys::hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_BIAS_DATA_TYPE,
                &dtype as *const _ as *const c_void,
                std::mem::size_of::<sys::hipDataType>(),
            )
        }
    }

    pub fn raw(&self) -> sys::hipblasLtMatmulDesc_t {
        self.raw
    }
}

impl Drop for MatmulDesc {
    fn drop(&mut self) {
        let _ = result::matmul_desc_destroy(self.raw);
    }
}

unsafe impl Send for MatmulDesc {}
unsafe impl Sync for MatmulDesc {}

// ---------------- preference ----------------

pub struct MatmulPref {
    raw: sys::hipblasLtMatmulPreference_t,
}

impl MatmulPref {
    pub fn new() -> Result<Self, HipBlasLtError> {
        let raw = result::matmul_preference_create()?;
        Ok(Self { raw })
    }

    pub fn set_max_workspace_bytes(&mut self, bytes: u64) -> Result<(), HipBlasLtError> {
        unsafe {
            result::matmul_preference_set_attribute(
                self.raw,
                sys::hipblasLtMatmulPreferenceAttributes_t::HIPBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES,
                &bytes as *const _ as *const c_void,
                std::mem::size_of::<u64>(),
            )
        }
    }

    pub fn raw(&self) -> sys::hipblasLtMatmulPreference_t {
        self.raw
    }
}

impl Drop for MatmulPref {
    fn drop(&mut self) {
        let _ = result::matmul_preference_destroy(self.raw);
    }
}

unsafe impl Send for MatmulPref {}
unsafe impl Sync for MatmulPref {}

// ---------------- heuristic result ----------------

/// Wraps a `hipblasLtMatmulHeuristicResult_t` returned by the heuristic search.
/// The contained algo is consumed by [`HipBlasLt::matmul`].
pub struct MatmulHeuristic {
    inner: sys::hipblasLtMatmulHeuristicResult_t,
}

impl MatmulHeuristic {
    pub fn workspace_required(&self) -> usize {
        self.inner.workspaceSize
    }

    pub fn waves_count(&self) -> f32 {
        self.inner.wavesCount
    }

    pub fn algo(&self) -> &sys::hipblasLtMatmulAlgo_t {
        &self.inner.algo
    }
}

// ---------------- HipBlasLt: heuristic + matmul ----------------

impl HipBlasLt {
    /// Ask hipBLASLt to rank up to `requested` algorithms for the given
    /// (`desc`, `A/B/C/D` layouts, `pref`) combination. Returns them best-first.
    pub fn get_heuristic(
        &self,
        desc: &MatmulDesc,
        a: &MatrixLayout,
        b: &MatrixLayout,
        c: &MatrixLayout,
        d: &MatrixLayout,
        pref: &MatmulPref,
        requested: u32,
    ) -> Result<Vec<MatmulHeuristic>, HipBlasLtError> {
        let raw = unsafe {
            result::matmul_algo_get_heuristic(
                self.raw,
                desc.raw(),
                a.raw(),
                b.raw(),
                c.raw(),
                d.raw(),
                pref.raw(),
                requested,
            )?
        };
        Ok(raw.into_iter().map(|inner| MatmulHeuristic { inner }).collect())
    }

    /// Execute D = epilogue(alpha * A·B + beta * C) using `heuristic`'s algo
    /// and the caller-owned workspace.
    ///
    /// # Safety
    /// All device pointers must be valid for the layouts; `workspace` must be
    /// at least `workspace_size` bytes; `workspace_size` must be ≥
    /// `heuristic.workspace_required()`.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn matmul(
        &self,
        desc: &MatmulDesc,
        alpha: *const c_void,
        a: u64,
        a_layout: &MatrixLayout,
        b: u64,
        b_layout: &MatrixLayout,
        beta: *const c_void,
        c: u64,
        c_layout: &MatrixLayout,
        d: u64,
        d_layout: &MatrixLayout,
        heuristic: &MatmulHeuristic,
        workspace: u64,
        workspace_size: usize,
        stream: &HipStream,
    ) -> Result<(), HipBlasLtError> {
        unsafe {
            result::matmul(
                self.raw,
                desc.raw(),
                alpha,
                a,
                a_layout.raw(),
                b,
                b_layout.raw(),
                beta,
                c,
                c_layout.raw(),
                d,
                d_layout.raw(),
                heuristic.algo(),
                workspace,
                workspace_size,
                // driver::sys and hipblaslt::sys each redeclare ihipStream_t;
                // rustc treats them as distinct nominal types. Cast at the bridge.
                stream.hip_stream().cast(),
            )
        }
    }
}
