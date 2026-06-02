//! Safe abstractions over [`crate::hip::result`] — `HipSlice`, `HipContext`,
//! `HipStream`, and the surrounding type machinery. Layout mirrors
//! `cudarc::driver::safe`.

pub(crate) mod core;
pub(crate) mod graph;
pub(crate) mod launch;
pub(crate) mod external_memory;
pub(crate) mod profile;
pub(crate) mod unified_memory;

pub use self::core::{
    DevicePtr, DevicePtrMut, DeviceRepr, DeviceSlice, EventWaitFlags, HipContext, HipEvent,
    HipFunction, HipModule, HipSlice, HipStream, HipView, HipViewMut, HostSlice,
    PinnedHostSlice, SyncOnDrop, ValidAsZeroBits,
};
pub use self::external_memory::{ExternalMemory, MappedBuffer};
pub use self::graph::HipGraph;
pub use self::launch::{LaunchArgs, LaunchConfig, PushKernelArg};
pub use self::profile::{profiler_start, profiler_stop, Profiler};
pub use self::unified_memory::{
    HipUnifiedSlice, HipUnifiedView, HipUnifiedViewMut, MemAttachFlags,
};
pub use crate::hip::result::HipError;
pub use crate::hip::result::stream::StreamKind;
