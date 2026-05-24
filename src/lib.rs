//! Minimal safe HIP/ROCm bindings, modeled after `cudarc`.
//!
//! Surface intentionally small: only what `luminal_rocm_lite` needs to host
//! a kernel-launch-based runtime. Grow this crate by following `cudarc`'s
//! API shape so call sites in luminal stay near-identical.

pub mod driver;
pub mod hiprtc;

#[cfg(feature = "rocblas")]
pub mod rocblas;
#[cfg(feature = "hipblaslt")]
pub mod hipblaslt;

pub use driver::{DevicePtr, DriverError, HipContext, HipFunction, HipModule, HipSlice, HipStream};
pub use hiprtc::{HiprtcError, Hsaco};

#[cfg(feature = "rocblas")]
pub use rocblas::{
    Axpy, AxpyConfig, Copy, CopyConfig, Dot, DotConfig, Gemm, GemmConfig, Gemv, GemvConfig, Nrm2,
    Nrm2Config, Operation, RocblasError, RocblasHandle, Scal, ScalConfig, StridedBatchedConfig,
};

#[cfg(feature = "hipblaslt")]
pub use hipblaslt::{
    HipBlasLt, HipBlasLtError, MatmulDesc, MatmulHeuristic, MatmulPref, MatrixLayout,
};
