//! Minimal safe HIP/ROCm bindings, modeled after `cudarc`.
//!
//! Surface intentionally small: only what `luminal_rocm_lite` needs to host
//! a kernel-launch-based runtime. Grow this crate by following `cudarc`'s
//! API shape so call sites in luminal stay near-identical.

pub mod driver;
pub mod hiprtc;

pub use driver::{DevicePtr, DriverError, HipContext, HipFunction, HipModule, HipSlice, HipStream};
pub use hiprtc::{HiprtcError, Hsaco};
