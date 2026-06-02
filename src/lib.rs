//! Minimal safe HIP/ROCm bindings, modeled after `cudarc`.
//!
//! Surface intentionally small: only what `luminal_rocm_lite` needs to host
//! a kernel-launch-based runtime. Grow this crate by following `cudarc`'s
//! API shape so call sites in luminal stay near-identical.

pub mod hip;
pub mod hiprtc;

#[cfg(feature = "rocblas")]
pub mod rocblas;
#[cfg(feature = "hipblaslt")]
pub mod hipblaslt;

pub use hip::{
    profiler_start, profiler_stop, DevicePtr, DevicePtrMut, DeviceRepr, DeviceSlice,
    EventWaitFlags, ExternalMemory, HipContext, HipError, HipEvent, HipFunction, HipGraph,
    HipModule, HipSlice, HipStream, HipUnifiedSlice, HipUnifiedView, HipUnifiedViewMut,
    HipView, HipViewMut, HostSlice, LaunchArgs, LaunchConfig, MappedBuffer, MemAttachFlags,
    PinnedHostSlice, Profiler, PushKernelArg, StreamKind, SyncOnDrop, ValidAsZeroBits,
};
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

#[cfg(feature = "dynamic-loading")]
pub fn get_lib_name_candidates(lib_name: &str) -> Vec<String> {
    use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};

    let major = env!("ROCM_MAJOR_VERSION");
    let minor = env!("ROCM_MINOR_VERSION");

    let rocm_path =
        std::env::var("ROCM_PATH").unwrap_or_else(|_| "/opt/rocm".to_string());

    let stems = [
        // Bare soname; resolved via LD_LIBRARY_PATH / ldconfig.
        format!("{DLL_PREFIX}{lib_name}{DLL_SUFFIX}"),
        // Major-versioned soname (e.g. librocblas.so.4).
        format!("{DLL_PREFIX}{lib_name}{DLL_SUFFIX}.{major}"),
        // Common alternative majors to try as fallbacks.
        format!("{DLL_PREFIX}{lib_name}{DLL_SUFFIX}.0"),
        format!("{DLL_PREFIX}{lib_name}{DLL_SUFFIX}.1"),
        // Exact patch soname.
        format!("{DLL_PREFIX}{lib_name}{DLL_SUFFIX}.{major}.{minor}"),
    ];

    let mut out = Vec::with_capacity(stems.len() * 2);
    for stem in &stems {
        out.push(stem.clone());                    // search via loader
        out.push(format!("{rocm_path}/lib/{stem}")); // explicit ROCm install
    }
    out
}

#[cfg(feature = "dynamic-loading")]
pub fn panic_no_lib_found(name: &str, tried: &[String]) -> ! {
    panic!(
        "could not load dynamic library {name}; tried {} candidates: {tried:?}",
        tried.len()
    );
}
