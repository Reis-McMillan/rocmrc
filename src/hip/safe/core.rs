//! Safe wrappers over [`crate::hip::result`] — `HipContext`, `HipStream`,
//! `HipSlice`, and the surrounding type machinery. Mirrors
//! `cudarc::driver::safe` in structure, naming, and the read/write
//! event-tracking model that makes multi-stream concurrency safe.
//!
//! HIP-specific divergences from cudarc are documented inline on the affected
//! items (see [`HipContext::set_blocking_synchronize`], [`HipFunction`]'s
//! missing setters, [`EventWaitFlags`]'s manual flag values, etc.).

use crate::hip::{
    result::{self, HipError},
    sys::{self},
};

use core::ffi::{c_int, c_uint};
use std::{
    ffi::CString,
    marker::PhantomData,
    ops::{Bound, RangeBounds},
    sync::{
        atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering},
        Arc,
    },
};

// HIP `#define` flag values that bindgen drops. Re-declared here with the same
// hex constants used in `<hip/hip_runtime_api.h>`. (Stream flags live in the
// typed `result::stream::StreamKind` enum.)
const HIP_EVENT_DISABLE_TIMING: c_uint = 0x2;
const HIP_EVENT_BLOCKING_SYNC: c_uint = 0x1;
const HIP_HOST_MALLOC_WRITE_COMBINED: c_uint = 0x4;
const HIP_DEVICE_SCHEDULE_BLOCKING_SYNC: c_uint = 0x4;

/// Flags for [`HipStream::wait`].
///
/// `hipEventWaitDefault` (0x0) and `hipEventWaitExternal` (0x1) are
/// `#define`s in HIP and don't appear in [`crate::hip::sys`]; this enum
/// re-declares them with stable values. `External` only takes effect during
/// graph capture.
#[derive(Debug, Clone, Copy)]
pub enum EventWaitFlags {
    Default,
    External,
}

impl EventWaitFlags {
    #[inline]
    fn to_raw(self) -> c_uint {
        match self {
            Self::Default => 0x0,
            Self::External => 0x1,
        }
    }
}

// ----------------------------------------------------------------------------
// HipContext
// ----------------------------------------------------------------------------

/// A device-bound HIP session — the root that manufactures streams, events,
/// and modules. Analogue of `cudarc::driver::safe::CudaContext`, but backed by
/// the runtime device API (`hipSetDevice`) rather than the deprecated
/// `hipCtx*` API: on HIP there's one (process-lifetime) primary context per
/// device, so there's no context handle to own.
///
/// Constructed via [`HipContext::new`].
///
/// Holds an `AtomicU32` deferred-error slot (`error_state`) so methods that
/// run in `Drop` — where a `Result` can't bubble — can stash an error to be
/// retrieved later via [`HipContext::check_err`].
#[derive(Debug)]
pub struct HipContext {
    pub(crate) hip_device: sys::hipDevice_t,
    pub(crate) ordinal: usize,
    pub(crate) has_async_alloc: bool,
    pub(crate) num_streams: AtomicUsize,
    pub(crate) event_tracking: AtomicBool,
    pub(crate) error_state: AtomicU32,
}

unsafe impl Send for HipContext {}
unsafe impl Sync for HipContext {}

impl PartialEq for HipContext {
    fn eq(&self, other: &Self) -> bool {
        self.ordinal == other.ordinal
    }
}
impl Eq for HipContext {}

impl HipContext {
    /// Bind device `ordinal` to the calling thread. Mirror of cudarc's
    /// `CudaContext::new`, but HIP-flavored: there's no context handle to
    /// retain — `hipSetDevice` lazily uses the device's (process-lifetime)
    /// primary context, so this is just `hipInit` + `hipSetDevice`.
    pub fn new(ordinal: usize) -> Result<Arc<Self>, HipError> {
        result::init()?;
        let hip_device = result::device::get(ordinal as i32)?;
        let has_async_alloc = result::device::get_attribute(
            hip_device,
            sys::hipDeviceAttribute_t::hipDeviceAttributeMemoryPoolsSupported,
        )? > 0;
        let ctx = Arc::new(HipContext {
            hip_device,
            ordinal,
            has_async_alloc,
            num_streams: AtomicUsize::new(0),
            event_tracking: AtomicBool::new(true),
            error_state: AtomicU32::new(0),
        });
        ctx.bind_to_thread()?;
        Ok(ctx)
    }

    pub fn has_async_alloc(&self) -> bool {
        self.has_async_alloc
    }

    pub fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// Number of devices visible to HIP. Static — does not require a context.
    pub fn device_count() -> Result<i32, HipError> {
        Ok(result::device::get_count()?)
    }

    /// Ordinal of the device currently bound to the calling thread.
    /// Static; does not require a [`HipContext`]. Wraps `hipGetDevice`.
    pub fn current_device() -> Result<i32, HipError> {
        result::device::current()
    }

    /// Full `hipDeviceProp_tR0600` struct for this context's device.
    /// Prefer [`Self::attribute`] for single-field queries on hot paths.
    pub fn properties(&self) -> Result<sys::hipDeviceProp_tR0600, HipError> {
        result::device::get_properties(self.ordinal as core::ffi::c_int)
    }

    /// The device's gfx arch as a string (e.g. `"gfx1100"`). Reads
    /// `gcnArchName` from the device properties and strips HIP's feature
    /// suffix (`":sramecc-:xnack-"` etc.) at the first `:`.
    pub fn gfx_arch(&self) -> Result<String, HipError> {
        let props = self.properties()?;
        let cstr = unsafe { core::ffi::CStr::from_ptr(props.gcnArchName.as_ptr()) };
        let raw = cstr.to_string_lossy();
        Ok(raw.split(':').next().unwrap_or(&raw).to_string())
    }

    /// Typed counterpart to [`Self::gfx_arch`]. Returns
    /// `Err(HipError(hipErrorInvalidValue))` if the device reports an
    /// arch that isn't in [`crate::hiprtc::GfxVersion`].
    pub fn gfx_version(&self) -> Result<crate::hiprtc::GfxVersion, HipError> {
        use std::str::FromStr;
        let arch = self.gfx_arch()?;
        crate::hiprtc::GfxVersion::from_str(&arch)
            .map_err(|_| HipError(sys::hipError_t::hipErrorInvalidValue))
    }

    /// Destroy the current device's primary context and release every
    /// resource it holds — allocations, modules, streams, events. Any
    /// `Hip*` value that referenced this device becomes invalid.
    ///
    /// Subsequent HIP calls on the calling thread will lazily create a
    /// fresh primary context.
    ///
    /// # Safety
    /// The caller must ensure that **no live** [`HipSlice`] /
    /// [`HipStream`] / [`HipEvent`] / [`HipModule`] / [`HipFunction`] /
    /// [`HipUnifiedSlice`](crate::hip::HipUnifiedSlice) / [`PinnedHostSlice`] /
    /// etc. tied to this device is used after the reset.
    pub unsafe fn reset() -> Result<(), HipError> {
        unsafe { result::device::reset() }
    }

    pub fn name(&self) -> Result<String, HipError> {
        result::device::get_name(self.hip_device)
    }

    pub fn uuid(&self) -> Result<sys::hipUUID, HipError> {
        result::device::get_uuid(self.hip_device)
    }

    /// `(major, minor)` from `hipDeviceAttributeComputeCapability{Major,Minor}`.
    ///
    /// **HIP note:** these attributes mirror the CUDA SM version layout but
    /// reflect AMD's gfx architecture, so the numbers don't compare across
    /// vendors. Shape parity with cudarc is preserved.
    pub fn compute_capability(&self) -> Result<(i32, i32), HipError> {
        let major = result::device::get_attribute(
            self.hip_device,
            sys::hipDeviceAttribute_t::hipDeviceAttributeComputeCapabilityMajor,
        )?;
        let minor = result::device::get_attribute(
            self.hip_device,
            sys::hipDeviceAttribute_t::hipDeviceAttributeComputeCapabilityMinor,
        )?;
        Ok((major, minor))
    }

    pub fn total_mem(&self) -> Result<usize, HipError> {
        result::device::total_mem(self.hip_device)
    }

    /// `(free, total)` bytes on the device.
    pub fn mem_get_info(&self) -> Result<(usize, usize), HipError> {
        self.bind_to_thread()?;
        result::mem_get_info()
    }

    /// Raw `hipDevice_t`. Do not free/release.
    pub fn hip_device(&self) -> sys::hipDevice_t {
        self.hip_device
    }

    /// Bind this context's device (`self.ordinal`) to the calling thread via
    /// `hipSetDevice` (skipping the call if it's already current). Key for
    /// thread safety — every HIP call below this layer assumes the right
    /// device is bound.
    pub fn bind_to_thread(&self) -> Result<(), HipError> {
        self.check_err()?;
        let mut curr: i32 = -1;
        result::device::get_device(&mut curr)?;
        if curr != self.ordinal as _ {
            result::device::set_device(self.ordinal as _)?;
        }
        Ok(())
    }

    pub fn attribute(&self, attr: sys::hipDeviceAttribute_t) -> Result<i32, HipError> {
        self.check_err()?;
        result::device::get_attribute(self.hip_device, attr)
    }

    pub fn synchronize(&self) -> Result<(), HipError> {
        self.bind_to_thread()?;
        result::device::synchronize()
    }

    /// Make subsequent [`Self::synchronize`] calls block the CPU thread by
    /// setting `hipDeviceScheduleBlockingSync` via `hipSetDeviceFlags`.
    ///
    /// **HIP divergence:** scheduling flags in HIP are scoped to the *device*,
    /// not the *context* (CUDA's analog is `cuCtxSetFlags`). Setting them
    /// affects every context on this device. `result.rs` doesn't wrap
    /// `hipSetDeviceFlags`, so this reaches into `sys::*` directly.
    pub fn set_blocking_synchronize(&self) -> Result<(), HipError> {
        self.bind_to_thread()?;
        unsafe { sys::hipSetDeviceFlags(HIP_DEVICE_SCHEDULE_BLOCKING_SYNC).result() }
    }

    pub fn get_limit(&self, limit: sys::hipLimit_t) -> Result<usize, HipError> {
        self.bind_to_thread()?;
        result::device::get_limit(limit)
    }

    pub fn set_limit(&self, limit: sys::hipLimit_t, value: usize) -> Result<(), HipError> {
        self.bind_to_thread()?;
        result::device::set_limit(limit, value)
    }

    pub fn get_cache_config(&self) -> Result<sys::hipFuncCache_t, HipError> {
        self.bind_to_thread()?;
        result::device::get_cache_config()
    }

    pub fn set_cache_config(&self, config: sys::hipFuncCache_t) -> Result<(), HipError> {
        self.bind_to_thread()?;
        result::device::set_cache_config(config)
    }

    /// `true` once a non-default stream has been created in this context.
    /// Only [`Self::new_stream`] / [`Self::new_stream_with_priority`] /
    /// [`HipStream::fork`] flip this; [`Self::default_stream`] and
    /// [`Self::per_thread_stream`] do not.
    pub fn is_in_multi_stream_mode(&self) -> bool {
        self.num_streams.load(Ordering::Relaxed) > 0
    }

    /// Whether `HipSlice` allocations record read/write events for automatic
    /// cross-stream synchronization. Default: `true`.
    pub fn is_event_tracking(&self) -> bool {
        self.event_tracking.load(Ordering::Relaxed)
    }

    /// `true` only when both multi-stream mode is active AND event tracking
    /// is enabled. Gates the wait-inserting branches in
    /// [`DevicePtr::device_ptr`] and [`DevicePtrMut::device_ptr_mut`].
    pub fn is_managing_stream_synchronization(&self) -> bool {
        self.is_in_multi_stream_mode() && self.is_event_tracking()
    }

    /// Re-enables event tracking on slices created *after* this call.
    ///
    /// # Safety
    /// Slices allocated while event tracking was disabled remain unmanaged
    /// — they still won't sync across streams even after this call.
    pub unsafe fn enable_event_tracking(&self) {
        self.event_tracking.store(true, Ordering::Relaxed);
    }

    /// Disables event tracking on slices created *after* this call. Caller
    /// becomes responsible for cross-stream synchronization.
    ///
    /// # Safety
    /// The user must manually ensure:
    /// - A `HipSlice` is not freed while a use on another stream is in flight.
    /// - A `HipSlice` is not used on a stream before its allocating stream
    ///   has finished the allocation.
    /// - Two streams don't write to the same `HipSlice` concurrently.
    pub unsafe fn disable_event_tracking(&self) {
        self.event_tracking.store(false, Ordering::Relaxed);
    }

    /// Drains the deferred error slot. If a prior `Drop`/internal step
    /// recorded a non-success status, returns it and clears the slot.
    pub fn check_err(&self) -> Result<(), HipError> {
        let raw = self.error_state.swap(0, Ordering::Relaxed);
        if raw == 0 {
            Ok(())
        } else {
            // Safe: we only ever store `hipError_t as u32`, so the round-trip
            // is valid. `hipError_t` is `#[repr(u32)]`.
            Err(HipError(unsafe {
                std::mem::transmute::<u32, sys::hipError_t>(raw)
            }))
        }
    }

    /// Stash a `Result`'s error in the deferred slot. Used from contexts
    /// (e.g. `Drop`) where bubbling isn't possible.
    pub fn record_err<T>(&self, result: Result<T, HipError>) {
        if let Err(err) = result {
            self.error_state.store(err.0 as u32, Ordering::Relaxed);
        }
    }
}

// ----------------------------------------------------------------------------
// HipEvent
// ----------------------------------------------------------------------------

/// Synchronization point on a stream. Analogue of
/// `cudarc::driver::safe::CudaEvent`.
#[derive(Debug)]
pub struct HipEvent {
    pub(crate) hip_event: sys::hipEvent_t,
    pub(crate) ctx: Arc<HipContext>,
}

unsafe impl Send for HipEvent {}
unsafe impl Sync for HipEvent {}

impl Drop for HipEvent {
    fn drop(&mut self) {
        self.ctx.record_err(self.ctx.bind_to_thread());
        self.ctx.record_err(result::event::destroy(self.hip_event));
    }
}

impl HipContext {
    /// Create a new event in this context. Default flag is
    /// `hipEventDisableTiming` (no elapsed-time tracking) — pass
    /// `Some(0)` for a full event or other flag combinations.
    pub fn new_event(
        self: &Arc<Self>,
        flags: Option<c_uint>,
    ) -> Result<HipEvent, HipError> {
        let flags = flags.unwrap_or(HIP_EVENT_DISABLE_TIMING);
        self.bind_to_thread()?;
        let hip_event = result::event::create_with_flags(flags)?;
        Ok(HipEvent {
            hip_event,
            ctx: self.clone(),
        })
    }
}

impl HipEvent {
    pub fn hip_event(&self) -> sys::hipEvent_t {
        self.hip_event
    }

    pub fn context(&self) -> &Arc<HipContext> {
        &self.ctx
    }

    /// Record the current workload on `stream` into `self`. Does **not**
    /// affect prior `stream.wait(&self)` calls.
    pub fn record(&self, stream: &HipStream) -> Result<(), HipError> {
        assert!(
            Arc::ptr_eq(&self.ctx, &stream.ctx),
            "HipEvent::record: event and stream belong to different contexts",
        );
        self.ctx.bind_to_thread()?;
        result::event::record(self.hip_event, stream.hip_stream)
    }

    /// Block the calling thread until this event has been reached.
    pub fn synchronize(&self) -> Result<(), HipError> {
        self.ctx.bind_to_thread()?;
        result::event::synchronize(self.hip_event)
    }

    /// Elapsed time in milliseconds between `self` (start) and `end`.
    /// Synchronizes both events before measuring.
    pub fn elapsed_ms(&self, end: &Self) -> Result<f32, HipError> {
        self.synchronize()?;
        end.synchronize()?;
        result::event::elapsed(self.hip_event, end.hip_event)
    }

    /// `true` if every operation recorded into this event has completed.
    pub fn is_complete(&self) -> Result<bool, HipError> {
        self.ctx.bind_to_thread()?;
        result::event::query(self.hip_event)
    }
}

// ----------------------------------------------------------------------------
// HipStream
// ----------------------------------------------------------------------------

/// Ordered command queue on a HIP device. Analogue of
/// `cudarc::driver::safe::CudaStream`.
///
/// `PartialEq` / `Eq` compare the raw `hipStream_t` handles.
#[derive(Debug)]
pub struct HipStream {
    pub(crate) hip_stream: sys::hipStream_t,
    pub(crate) ctx: Arc<HipContext>,
}

impl PartialEq for HipStream {
    fn eq(&self, other: &Self) -> bool {
        self.hip_stream == other.hip_stream
    }
}
impl Eq for HipStream {}

unsafe impl Send for HipStream {}
unsafe impl Sync for HipStream {}

// Magic value for the per-thread default stream, hard-coded since the
// `hipStreamPerThread` macro is dropped by bindgen.
const HIP_STREAM_PER_THREAD: usize = 0x2;

impl Drop for HipStream {
    fn drop(&mut self) {
        self.ctx.record_err(self.ctx.bind_to_thread());
        let hip_stream = std::mem::replace(&mut self.hip_stream, std::ptr::null_mut());
        if !hip_stream.is_null() && hip_stream as usize != HIP_STREAM_PER_THREAD {
            self.ctx.num_streams.fetch_sub(1, Ordering::Relaxed);
            self.ctx.record_err(result::stream::destroy(hip_stream));
        }
    }
}

impl HipContext {
    /// The legacy default (NULL) stream. Shared across the device; **does
    /// not** count toward `num_streams`.
    pub fn default_stream(self: &Arc<Self>) -> Arc<HipStream> {
        Arc::new(HipStream {
            hip_stream: std::ptr::null_mut(),
            ctx: self.clone(),
        })
    }

    /// The per-thread default stream (`hipStreamPerThread`). Each calling
    /// thread gets its own serialization, with no synchronization against
    /// the NULL stream. Does **not** count toward `num_streams`.
    pub fn per_thread_stream(self: &Arc<Self>) -> Arc<HipStream> {
        Arc::new(HipStream {
            hip_stream: HIP_STREAM_PER_THREAD as sys::hipStream_t,
            ctx: self.clone(),
        })
    }

    /// Create a non-blocking stream. Bumps the context into
    /// multi-stream mode on the 0→1 transition (with a `synchronize()` to
    /// serialize the switch, if event tracking is on).
    pub fn new_stream(self: &Arc<Self>) -> Result<Arc<HipStream>, HipError> {
        self.bind_to_thread()?;
        let prev = self.num_streams.fetch_add(1, Ordering::Relaxed);
        if prev == 0 && self.is_event_tracking() {
            self.synchronize()?;
        }
        let hip_stream = result::stream::create(result::stream::StreamKind::NonBlocking)?;
        Ok(Arc::new(HipStream {
            hip_stream,
            ctx: self.clone(),
        }))
    }

    /// Create a non-blocking stream with the given priority (lower value =
    /// higher priority).
    pub fn new_stream_with_priority(
        self: &Arc<Self>,
        priority: i32,
    ) -> Result<Arc<HipStream>, HipError> {
        self.bind_to_thread()?;
        let prev = self.num_streams.fetch_add(1, Ordering::Relaxed);
        if prev == 0 && self.is_event_tracking() {
            self.synchronize()?;
        }
        let hip_stream =
            result::stream::create_with_priority(
                result::stream::StreamKind::NonBlocking,
                priority,
            )?;
        Ok(Arc::new(HipStream {
            hip_stream,
            ctx: self.clone(),
        }))
    }
}

impl HipStream {
    /// Spawn a new non-blocking stream that waits on `self` before its first
    /// op. Useful for splitting a pipeline.
    pub fn fork(&self) -> Result<Arc<Self>, HipError> {
        self.ctx.bind_to_thread()?;
        self.ctx.num_streams.fetch_add(1, Ordering::Relaxed);
        let hip_stream = result::stream::create(result::stream::StreamKind::NonBlocking)?;
        let stream = Arc::new(HipStream {
            hip_stream,
            ctx: self.ctx.clone(),
        });
        stream.join(self)?;
        Ok(stream)
    }

    pub fn hip_stream(&self) -> sys::hipStream_t {
        self.hip_stream
    }

    pub fn context(&self) -> &Arc<HipContext> {
        &self.ctx
    }

    /// Block until every op submitted on this stream has completed.
    pub fn synchronize(&self) -> Result<(), HipError> {
        self.ctx.bind_to_thread()?;
        result::stream::synchronize(self.hip_stream)
    }

    /// Create a fresh event and record the stream's current work into it.
    /// Returns the event.
    pub fn record_event(&self, flags: Option<c_uint>) -> Result<HipEvent, HipError> {
        let event = self.ctx.new_event(flags)?;
        event.record(self)?;
        Ok(event)
    }

    /// Make this stream wait for `event` before issuing further work. You
    /// can re-record `event` afterwards without affecting this wait.
    pub fn wait(&self, event: &HipEvent) -> Result<(), HipError> {
        self.ctx.bind_to_thread()?;
        unsafe {
            result::stream::wait_event(
                self.hip_stream,
                event.hip_event,
                EventWaitFlags::Default.to_raw(),
            )
        }
    }

    /// Make `self` wait for `other`'s current workload to complete. Shorthand
    /// for `self.wait(&other.record_event(None)?)`.
    pub fn join(&self, other: &HipStream) -> Result<(), HipError> {
        self.wait(&other.record_event(None)?)
    }

    /// Wrap a raw device pointer + element count as a [`HipSlice<T>`].
    /// Pairs with [`HipSlice::leak`].
    ///
    /// The returned slice has `read` / `write` events set per the
    /// owning context's current [`HipContext::is_event_tracking`] flag,
    /// matching what a fresh [`Self::alloc`] would produce.
    ///
    /// # Safety
    /// - `ptr` must be a live device allocation on this stream's
    ///   context of at least `len * size_of::<T>()` bytes.
    /// - Ownership transfers to the returned slice — caller must not
    ///   free `ptr` separately.
    pub unsafe fn upgrade_device_ptr<T>(
        self: &Arc<Self>,
        ptr: sys::hipDeviceptr_t,
        len: usize,
    ) -> Result<HipSlice<T>, HipError> {
        let (read, write) = if self.ctx.is_event_tracking() {
            (Some(self.ctx.new_event(None)?), Some(self.ctx.new_event(None)?))
        } else {
            (None, None)
        };
        Ok(HipSlice {
            hip_device_ptr: ptr,
            len,
            read,
            write,
            stream: self.clone(),
            marker: PhantomData,
        })
    }
}

// ----------------------------------------------------------------------------
// SyncOnDrop
// ----------------------------------------------------------------------------

/// RAII guard returned by [`DevicePtr::device_ptr`] /
/// [`DevicePtrMut::device_ptr_mut`] / [`HostSlice::stream_synced_slice`].
///
/// On drop:
/// - [`SyncOnDrop::Record`] — records the current stream workload into the
///   captured event. Cheap; no CPU block.
/// - [`SyncOnDrop::Sync`] — calls `stream.synchronize()`. Blocks the calling
///   thread until the stream drains. Used at multi-stream-mode transitions
///   and for non-pinned host borrows.
///
/// Any error encountered during drop is stashed via
/// [`HipContext::record_err`] for later inspection.
#[derive(Debug)]
#[must_use]
pub enum SyncOnDrop<'a> {
    Record(Option<(&'a HipEvent, &'a HipStream)>),
    Sync(Option<&'a HipStream>),
}

impl<'a> SyncOnDrop<'a> {
    /// If `event` is `Some`, capture it for recording at drop time.
    /// Otherwise the guard is a no-op (this happens when event tracking
    /// is disabled at allocation time).
    pub fn record_event(event: &'a Option<HipEvent>, stream: &'a HipStream) -> Self {
        match event {
            Some(e) => SyncOnDrop::Record(Some((e, stream))),
            None => SyncOnDrop::Record(None),
        }
    }

    /// Capture `stream` for a `synchronize()` call at drop time.
    pub fn sync_stream(stream: &'a HipStream) -> Self {
        SyncOnDrop::Sync(Some(stream))
    }
}

impl Drop for SyncOnDrop<'_> {
    fn drop(&mut self) {
        match self {
            SyncOnDrop::Record(target) => {
                if let Some((event, stream)) = std::mem::take(target) {
                    stream.ctx.record_err(event.record(stream));
                }
            }
            SyncOnDrop::Sync(target) => {
                if let Some(stream) = std::mem::take(target) {
                    stream.ctx.record_err(stream.synchronize());
                }
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Trait declarations
// ----------------------------------------------------------------------------

/// Common surface for any device-resident `[T]`-shaped value.
pub trait DeviceSlice<T> {
    fn len(&self) -> usize;
    fn num_bytes(&self) -> usize {
        self.len() * std::mem::size_of::<T>()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn stream(&self) -> &Arc<HipStream>;
}

/// Read access to a device pointer. Inserts an event wait for the slice's
/// last write (if tracking is on) before returning.
pub trait DevicePtr<T>: DeviceSlice<T> {
    fn device_ptr<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>);
}

/// Write access. Inserts event waits for the slice's last *read and write*
/// before returning.
pub trait DevicePtrMut<T>: DeviceSlice<T> {
    fn device_ptr_mut<'a>(
        &'a mut self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>);
}

/// A host-side `[T]` whose underlying storage can be borrowed for a
/// stream-synchronized copy. Implemented for plain arrays / slices / `Vec`
/// (which return a `SyncOnDrop::Sync` guard) and for [`PinnedHostSlice`]
/// (which uses event-based sync).
pub trait HostSlice<T> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// # Safety
    /// The returned slice must not outlive the underlying storage. The
    /// returned `SyncOnDrop` must be dropped before the host buffer is
    /// reused or freed.
    unsafe fn stream_synced_slice<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (&'a [T], SyncOnDrop<'a>);

    /// # Safety
    /// Same as [`Self::stream_synced_slice`]; additionally the caller must
    /// uphold mutable-borrow invariants on the returned slice.
    unsafe fn stream_synced_mut_slice<'a>(
        &'a mut self,
        stream: &'a HipStream,
    ) -> (&'a mut [T], SyncOnDrop<'a>);
}

// ----------------------------------------------------------------------------
// DeviceRepr / ValidAsZeroBits
// ----------------------------------------------------------------------------

/// Marker trait: type can be copied as raw bytes into kernel arg memory.
///
/// # Safety
/// Implementors should be `#[repr(C)]` (or transparent) with a stable
/// memory layout; bit patterns must be valid.
pub unsafe trait DeviceRepr {}
unsafe impl DeviceRepr for bool {}
unsafe impl DeviceRepr for i8 {}
unsafe impl DeviceRepr for i16 {}
unsafe impl DeviceRepr for i32 {}
unsafe impl DeviceRepr for i64 {}
unsafe impl DeviceRepr for i128 {}
unsafe impl DeviceRepr for isize {}
unsafe impl DeviceRepr for u8 {}
unsafe impl DeviceRepr for u16 {}
unsafe impl DeviceRepr for u32 {}
unsafe impl DeviceRepr for u64 {}
unsafe impl DeviceRepr for u128 {}
unsafe impl DeviceRepr for usize {}
unsafe impl DeviceRepr for f32 {}
unsafe impl DeviceRepr for f64 {}
unsafe impl DeviceRepr for half::f16 {}
unsafe impl DeviceRepr for half::bf16 {}
#[cfg(feature = "f8")]
unsafe impl DeviceRepr for float8::F8E4M3 {}
#[cfg(feature = "f8")]
unsafe impl DeviceRepr for float8::F8E5M2 {}
#[cfg(feature = "f4")]
unsafe impl DeviceRepr for float4::F4E2M1 {}
#[cfg(feature = "f4")]
unsafe impl DeviceRepr for float4::E8M0 {}
// Note: cudarc also impls for `float4::F4E2M1x2`, but that type isn't in
// `float4 = "0.1"`. Add back if the dep is bumped.
unsafe impl<const N: usize, T: DeviceRepr> DeviceRepr for [T; N] {}

/// Marker trait: type is safely zero-initializable.
///
/// # Safety
/// All-zero bit pattern must be a valid value of `Self`.
pub unsafe trait ValidAsZeroBits {}
unsafe impl ValidAsZeroBits for bool {}
unsafe impl ValidAsZeroBits for i8 {}
unsafe impl ValidAsZeroBits for i16 {}
unsafe impl ValidAsZeroBits for i32 {}
unsafe impl ValidAsZeroBits for i64 {}
unsafe impl ValidAsZeroBits for i128 {}
unsafe impl ValidAsZeroBits for isize {}
unsafe impl ValidAsZeroBits for u8 {}
unsafe impl ValidAsZeroBits for u16 {}
unsafe impl ValidAsZeroBits for u32 {}
unsafe impl ValidAsZeroBits for u64 {}
unsafe impl ValidAsZeroBits for u128 {}
unsafe impl ValidAsZeroBits for usize {}
unsafe impl ValidAsZeroBits for f32 {}
unsafe impl ValidAsZeroBits for f64 {}
unsafe impl ValidAsZeroBits for half::f16 {}
unsafe impl ValidAsZeroBits for half::bf16 {}
#[cfg(feature = "f8")]
unsafe impl ValidAsZeroBits for float8::F8E4M3 {}
#[cfg(feature = "f8")]
unsafe impl ValidAsZeroBits for float8::F8E5M2 {}
#[cfg(feature = "f4")]
unsafe impl ValidAsZeroBits for float4::F4E2M1 {}
#[cfg(feature = "f4")]
unsafe impl ValidAsZeroBits for float4::E8M0 {}
// See DeviceRepr note re: `float4::F4E2M1x2`.
unsafe impl<const N: usize, T: ValidAsZeroBits> ValidAsZeroBits for [T; N] {}

// Tuple impls — mirrors cudarc's `impl_tuples!` pattern.
macro_rules! impl_tuples_zero {
    ($t:tt) => {
        impl_tuples_zero!(@ $t);
    };
    ($l:tt $(,$t:tt)+) => {
        impl_tuples_zero!($($t),+);
        impl_tuples_zero!(@ $l $(,$t)+);
    };
    (@ $($t:tt),+) => {
        unsafe impl<$($t: ValidAsZeroBits,)+> ValidAsZeroBits for ($($t,)+) {}
    };
}
impl_tuples_zero!(A, B, C, D, E, F, G, H, I, J, K, L);

macro_rules! impl_tuples_repr {
    ($t:tt) => {
        impl_tuples_repr!(@ $t);
    };
    ($l:tt $(,$t:tt)+) => {
        impl_tuples_repr!($($t),+);
        impl_tuples_repr!(@ $l $(,$t)+);
    };
    (@ $($t:tt),+) => {
        unsafe impl<$($t: DeviceRepr,)+> DeviceRepr for ($($t,)+) {}
    };
}
impl_tuples_repr!(A, B, C, D, E, F, G, H, I, J, K, L);

// ----------------------------------------------------------------------------
// HostSlice impls for plain host types
// ----------------------------------------------------------------------------

impl<T, const N: usize> HostSlice<T> for [T; N] {
    fn len(&self) -> usize {
        N
    }
    unsafe fn stream_synced_slice<'a>(
        &'a self,
        _stream: &'a HipStream,
    ) -> (&'a [T], SyncOnDrop<'a>) {
        (self.as_slice(), SyncOnDrop::Sync(None))
    }
    unsafe fn stream_synced_mut_slice<'a>(
        &'a mut self,
        _stream: &'a HipStream,
    ) -> (&'a mut [T], SyncOnDrop<'a>) {
        (self.as_mut_slice(), SyncOnDrop::Sync(None))
    }
}

impl<T> HostSlice<T> for [T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
    unsafe fn stream_synced_slice<'a>(
        &'a self,
        _stream: &'a HipStream,
    ) -> (&'a [T], SyncOnDrop<'a>) {
        (self, SyncOnDrop::Sync(None))
    }
    unsafe fn stream_synced_mut_slice<'a>(
        &'a mut self,
        _stream: &'a HipStream,
    ) -> (&'a mut [T], SyncOnDrop<'a>) {
        (self, SyncOnDrop::Sync(None))
    }
}

impl<T> HostSlice<T> for Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
    unsafe fn stream_synced_slice<'a>(
        &'a self,
        _stream: &'a HipStream,
    ) -> (&'a [T], SyncOnDrop<'a>) {
        (self.as_slice(), SyncOnDrop::Sync(None))
    }
    unsafe fn stream_synced_mut_slice<'a>(
        &'a mut self,
        _stream: &'a HipStream,
    ) -> (&'a mut [T], SyncOnDrop<'a>) {
        (self.as_mut_slice(), SyncOnDrop::Sync(None))
    }
}

// ----------------------------------------------------------------------------
// HipSlice<T>
// ----------------------------------------------------------------------------

/// Owned RAII device buffer of `T`. Analogue of
/// `cudarc::driver::safe::CudaSlice`.
///
/// Holds optional `read` / `write` events used by the multi-stream
/// synchronization machinery — these are `Some` when the allocating
/// context had [`HipContext::is_event_tracking`] enabled at alloc time.
#[derive(Debug)]
pub struct HipSlice<T> {
    pub(crate) hip_device_ptr: sys::hipDeviceptr_t,
    pub(crate) len: usize,
    pub(crate) read: Option<HipEvent>,
    pub(crate) write: Option<HipEvent>,
    pub(crate) stream: Arc<HipStream>,
    pub(crate) marker: PhantomData<*const T>,
}

unsafe impl<T: Send> Send for HipSlice<T> {}
unsafe impl<T: Sync> Sync for HipSlice<T> {}

impl<T> Drop for HipSlice<T> {
    fn drop(&mut self) {
        if let Some(r) = &self.read {
            let _ = r.synchronize();
        }
        if let Some(w) = &self.write {
            let _ = w.synchronize();
        }
        if self.hip_device_ptr.is_null() {
            return;
        }
        let ptr = self.hip_device_ptr as u64;
        if self.stream.ctx.has_async_alloc {
            self.stream
                .ctx
                .record_err(result::free_async(ptr, self.stream.hip_stream));
        } else {
            self.stream.ctx.record_err(self.stream.synchronize());
            self.stream.ctx.record_err(result::free_sync(ptr));
        }
    }
}

impl<T> HipSlice<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn num_bytes(&self) -> usize {
        self.len * std::mem::size_of::<T>()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn ordinal(&self) -> usize {
        self.stream.ctx.ordinal
    }

    pub fn context(&self) -> &Arc<HipContext> {
        &self.stream.ctx
    }

    pub fn stream(&self) -> &Arc<HipStream> {
        &self.stream
    }

    /// Reinterpret the buffer as `[U]`. Returns `None` if the byte
    /// length isn't divisible by `size_of::<U>()`.
    ///
    /// Consumes `self`; the returned slice continues to own the
    /// underlying allocation and any read/write events.
    pub fn transmute<U>(self) -> Option<HipSlice<U>> {
        let bytes = self.num_bytes();
        let u_size = std::mem::size_of::<U>();
        if u_size == 0 || bytes % u_size != 0 {
            return None;
        }
        let new_len = bytes / u_size;
        // Move fields out of `self` without running Drop; the returned
        // slice takes ownership of the same allocation + events.
        let s = std::mem::ManuallyDrop::new(self);
        Some(HipSlice {
            hip_device_ptr: s.hip_device_ptr,
            len: new_len,
            read: unsafe { std::ptr::read(&s.read) },
            write: unsafe { std::ptr::read(&s.write) },
            stream: unsafe { std::ptr::read(&s.stream) },
            marker: PhantomData,
        })
    }

    /// Return the raw device pointer and element count without freeing
    /// the underlying allocation. Pair with
    /// [`HipStream::upgrade_device_ptr`] to take ownership back.
    ///
    /// The associated read/write events are synchronized and dropped
    /// (consistent with the contract that subsequent `upgrade_device_ptr`
    /// produces a fresh, unmanaged slice).
    pub fn leak(self) -> (sys::hipDeviceptr_t, usize) {
        let s = std::mem::ManuallyDrop::new(self);
        if let Some(r) = unsafe { std::ptr::read(&s.read) } {
            let _ = r.synchronize();
        }
        if let Some(w) = unsafe { std::ptr::read(&s.write) } {
            let _ = w.synchronize();
        }
        // Drop the stream Arc separately so the allocation is the only
        // resource leaked.
        let _stream: Arc<HipStream> = unsafe { std::ptr::read(&s.stream) };
        (s.hip_device_ptr, s.len)
    }
}

impl<'a, T> HipView<'a, T> {
    /// Reinterpret the view as `&[U]`. Returns `None` if the byte
    /// length isn't divisible by `size_of::<U>()`.
    pub fn transmute<U>(self) -> Option<HipView<'a, U>> {
        let bytes = self.len * std::mem::size_of::<T>();
        let u_size = std::mem::size_of::<U>();
        if u_size == 0 || bytes % u_size != 0 {
            return None;
        }
        Some(HipView {
            ptr: self.ptr,
            len: bytes / u_size,
            read: self.read,
            write: self.write,
            stream: self.stream,
            marker: PhantomData,
        })
    }
}

impl<'a, T> HipViewMut<'a, T> {
    /// Reinterpret the mutable view as `&mut [U]`. Returns `None` if
    /// the byte length isn't divisible by `size_of::<U>()`.
    pub fn transmute_mut<U>(self) -> Option<HipViewMut<'a, U>> {
        let bytes = self.len * std::mem::size_of::<T>();
        let u_size = std::mem::size_of::<U>();
        if u_size == 0 || bytes % u_size != 0 {
            return None;
        }
        Some(HipViewMut {
            ptr: self.ptr,
            len: bytes / u_size,
            read: self.read,
            write: self.write,
            stream: self.stream,
            marker: PhantomData,
        })
    }
}

impl<T: DeviceRepr> Clone for HipSlice<T> {
    fn clone(&self) -> Self {
        self.stream
            .clone_dtod(self)
            .expect("HipSlice::clone: device-to-device copy failed")
    }
}

impl<T: Clone + Default + DeviceRepr> TryFrom<HipSlice<T>> for Vec<T> {
    type Error = HipError;
    fn try_from(value: HipSlice<T>) -> Result<Self, Self::Error> {
        value.stream.clone().clone_dtoh(&value)
    }
}

// ----------------------------------------------------------------------------
// HipView / HipViewMut
// ----------------------------------------------------------------------------

/// Immutable borrowed view into a [`HipSlice`] (or a sub-range thereof).
/// Borrows the parent's `read` / `write` event references — the slice
/// cannot be dropped while a view exists.
#[derive(Debug)]
pub struct HipView<'a, T> {
    pub(crate) ptr: sys::hipDeviceptr_t,
    pub(crate) len: usize,
    pub(crate) read: &'a Option<HipEvent>,
    pub(crate) write: &'a Option<HipEvent>,
    pub(crate) stream: &'a Arc<HipStream>,
    pub(crate) marker: PhantomData<&'a [T]>,
}

/// Mutable borrowed view into a [`HipSlice`].
#[derive(Debug)]
pub struct HipViewMut<'a, T> {
    pub(crate) ptr: sys::hipDeviceptr_t,
    pub(crate) len: usize,
    pub(crate) read: &'a Option<HipEvent>,
    pub(crate) write: &'a Option<HipEvent>,
    pub(crate) stream: &'a Arc<HipStream>,
    pub(crate) marker: PhantomData<&'a mut [T]>,
}

impl<T> HipSlice<T> {
    /// Borrow the whole slice as a [`HipView`].
    pub fn as_view(&self) -> HipView<'_, T> {
        HipView {
            ptr: self.hip_device_ptr,
            len: self.len,
            read: &self.read,
            write: &self.write,
            stream: &self.stream,
            marker: PhantomData,
        }
    }

    /// Borrow the whole slice as a [`HipViewMut`].
    pub fn as_view_mut(&mut self) -> HipViewMut<'_, T> {
        HipViewMut {
            ptr: self.hip_device_ptr,
            len: self.len,
            read: &self.read,
            write: &self.write,
            stream: &self.stream,
            marker: PhantomData,
        }
    }

    /// Borrow a sub-range as a [`HipView`].
    pub fn slice<R: RangeBounds<usize>>(&self, range: R) -> HipView<'_, T> {
        let (start, end) = resolve_range(range, self.len);
        let ptr = unsafe {
            (self.hip_device_ptr as *mut u8).add(start * std::mem::size_of::<T>())
        } as sys::hipDeviceptr_t;
        HipView {
            ptr,
            len: end - start,
            read: &self.read,
            write: &self.write,
            stream: &self.stream,
            marker: PhantomData,
        }
    }

    /// Borrow a sub-range as a [`HipViewMut`].
    pub fn slice_mut<R: RangeBounds<usize>>(&mut self, range: R) -> HipViewMut<'_, T> {
        let (start, end) = resolve_range(range, self.len);
        let ptr = unsafe {
            (self.hip_device_ptr as *mut u8).add(start * std::mem::size_of::<T>())
        } as sys::hipDeviceptr_t;
        HipViewMut {
            ptr,
            len: end - start,
            read: &self.read,
            write: &self.write,
            stream: &self.stream,
            marker: PhantomData,
        }
    }
}

impl<'a, T> HipView<'a, T> {
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Sub-view of the current view.
    pub fn slice<R: RangeBounds<usize>>(&self, range: R) -> HipView<'_, T> {
        let (start, end) = resolve_range(range, self.len);
        let ptr =
            unsafe { (self.ptr as *mut u8).add(start * std::mem::size_of::<T>()) }
                as sys::hipDeviceptr_t;
        HipView {
            ptr,
            len: end - start,
            read: self.read,
            write: self.write,
            stream: self.stream,
            marker: PhantomData,
        }
    }
}

impl<'a, T> HipViewMut<'a, T> {
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Downgrade to an immutable view.
    pub fn as_view(&self) -> HipView<'_, T> {
        HipView {
            ptr: self.ptr,
            len: self.len,
            read: self.read,
            write: self.write,
            stream: self.stream,
            marker: PhantomData,
        }
    }

    /// Sub-view of the current mutable view (immutable).
    pub fn slice<R: RangeBounds<usize>>(&self, range: R) -> HipView<'_, T> {
        let (start, end) = resolve_range(range, self.len);
        let ptr =
            unsafe { (self.ptr as *mut u8).add(start * std::mem::size_of::<T>()) }
                as sys::hipDeviceptr_t;
        HipView {
            ptr,
            len: end - start,
            read: self.read,
            write: self.write,
            stream: self.stream,
            marker: PhantomData,
        }
    }

    /// Mutable sub-view of the current mutable view.
    pub fn slice_mut<R: RangeBounds<usize>>(&mut self, range: R) -> HipViewMut<'_, T> {
        let (start, end) = resolve_range(range, self.len);
        let ptr =
            unsafe { (self.ptr as *mut u8).add(start * std::mem::size_of::<T>()) }
                as sys::hipDeviceptr_t;
        HipViewMut {
            ptr,
            len: end - start,
            read: self.read,
            write: self.write,
            stream: self.stream,
            marker: PhantomData,
        }
    }
}

fn resolve_range<R: RangeBounds<usize>>(range: R, total_len: usize) -> (usize, usize) {
    let start = match range.start_bound() {
        Bound::Included(&n) => n,
        Bound::Excluded(&n) => n + 1,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(&n) => n + 1,
        Bound::Excluded(&n) => n,
        Bound::Unbounded => total_len,
    };
    assert!(start <= end && end <= total_len, "slice range out of bounds");
    (start, end)
}

// ----------------------------------------------------------------------------
// DeviceSlice / DevicePtr / DevicePtrMut impls
// ----------------------------------------------------------------------------

impl<T> DeviceSlice<T> for HipSlice<T> {
    fn len(&self) -> usize {
        self.len
    }
    fn stream(&self) -> &Arc<HipStream> {
        &self.stream
    }
}

impl<T> DeviceSlice<T> for HipView<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
    fn stream(&self) -> &Arc<HipStream> {
        self.stream
    }
}

impl<T> DeviceSlice<T> for HipViewMut<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
    fn stream(&self) -> &Arc<HipStream> {
        self.stream
    }
}

impl<T> DevicePtr<T> for HipSlice<T> {
    fn device_ptr<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>) {
        if self.stream.ctx.is_managing_stream_synchronization() {
            if let Some(write) = self.write.as_ref() {
                stream.ctx.record_err(stream.wait(write));
            }
        }
        (
            self.hip_device_ptr,
            SyncOnDrop::record_event(&self.read, stream),
        )
    }
}

impl<T> DevicePtr<T> for HipView<'_, T> {
    fn device_ptr<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>) {
        if self.stream.ctx.is_managing_stream_synchronization() {
            if let Some(write) = self.write.as_ref() {
                stream.ctx.record_err(stream.wait(write));
            }
        }
        (self.ptr, SyncOnDrop::record_event(self.read, stream))
    }
}

impl<T> DevicePtr<T> for HipViewMut<'_, T> {
    fn device_ptr<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>) {
        if self.stream.ctx.is_managing_stream_synchronization() {
            if let Some(write) = self.write.as_ref() {
                stream.ctx.record_err(stream.wait(write));
            }
        }
        (self.ptr, SyncOnDrop::record_event(self.read, stream))
    }
}

impl<T> DevicePtrMut<T> for HipSlice<T> {
    fn device_ptr_mut<'a>(
        &'a mut self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>) {
        if self.stream.ctx.is_managing_stream_synchronization() {
            if let Some(read) = self.read.as_ref() {
                stream.ctx.record_err(stream.wait(read));
            }
            if let Some(write) = self.write.as_ref() {
                stream.ctx.record_err(stream.wait(write));
            }
        }
        (
            self.hip_device_ptr,
            SyncOnDrop::record_event(&self.write, stream),
        )
    }
}

impl<T> DevicePtrMut<T> for HipViewMut<'_, T> {
    fn device_ptr_mut<'a>(
        &'a mut self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>) {
        if self.stream.ctx.is_managing_stream_synchronization() {
            if let Some(read) = self.read.as_ref() {
                stream.ctx.record_err(stream.wait(read));
            }
            if let Some(write) = self.write.as_ref() {
                stream.ctx.record_err(stream.wait(write));
            }
        }
        (self.ptr, SyncOnDrop::record_event(self.write, stream))
    }
}

// ----------------------------------------------------------------------------
// HipStream — allocation, fills, memcpy
// ----------------------------------------------------------------------------

impl HipStream {
    /// Allocate `len` elements of `T` on the device. Uses
    /// `hipMallocAsync` when the device advertises mempools, otherwise
    /// `hipMalloc`.
    ///
    /// # Safety
    /// The returned buffer is uninitialized; reading before a write is
    /// undefined behavior. Use [`Self::alloc_zeros`] for a safe variant
    /// when `T: ValidAsZeroBits`.
    pub unsafe fn alloc<T: DeviceRepr>(
        self: &Arc<Self>,
        len: usize,
    ) -> Result<HipSlice<T>, HipError> {
        self.ctx.bind_to_thread()?;
        let bytes = len.checked_mul(std::mem::size_of::<T>()).expect("size overflow");
        let raw = if self.ctx.has_async_alloc {
            result::malloc_async(bytes, self.hip_stream)?
        } else {
            result::malloc_sync(bytes)?
        };
        let (read, write) = if self.ctx.is_event_tracking() {
            (Some(self.ctx.new_event(None)?), Some(self.ctx.new_event(None)?))
        } else {
            (None, None)
        };
        Ok(HipSlice {
            hip_device_ptr: raw as sys::hipDeviceptr_t,
            len,
            read,
            write,
            stream: self.clone(),
            marker: PhantomData,
        })
    }

    /// Allocate `len` zeroed elements.
    pub fn alloc_zeros<T: DeviceRepr + ValidAsZeroBits>(
        self: &Arc<Self>,
        len: usize,
    ) -> Result<HipSlice<T>, HipError> {
        let mut dst = unsafe { self.alloc::<T>(len) }?;
        self.memset_zeros(&mut dst)?;
        Ok(dst)
    }

    /// Zero out a device buffer.
    pub fn memset_zeros<T: DeviceRepr + ValidAsZeroBits, Dst: DevicePtrMut<T>>(
        self: &Arc<Self>,
        dst: &mut Dst,
    ) -> Result<(), HipError> {
        self.ctx.bind_to_thread()?;
        let bytes = dst.num_bytes();
        let (dptr, _record) = dst.device_ptr_mut(self);
        unsafe { result::memset_d8_async(dptr as u64, 0, bytes, self.hip_stream) }
    }

    /// Allocate a new device buffer and copy `src` into it.
    pub fn clone_htod<T: DeviceRepr, Src: HostSlice<T> + ?Sized>(
        self: &Arc<Self>,
        src: &Src,
    ) -> Result<HipSlice<T>, HipError> {
        let mut dst = unsafe { self.alloc::<T>(src.len()) }?;
        self.memcpy_htod(src, &mut dst)?;
        Ok(dst)
    }

    /// Copy `src` into the host `Vec<T>` returned. `T` must implement
    /// `Clone + Default` so we can construct the destination Vec.
    pub fn clone_dtoh<T: DeviceRepr + Clone + Default, Src: DevicePtr<T>>(
        self: &Arc<Self>,
        src: &Src,
    ) -> Result<Vec<T>, HipError> {
        let mut dst: Vec<T> = vec![T::default(); src.len()];
        self.memcpy_dtoh(src, &mut dst[..])?;
        Ok(dst)
    }

    /// Allocate a new device buffer and copy `src` into it.
    pub fn clone_dtod<T: DeviceRepr, Src: DevicePtr<T>>(
        self: &Arc<Self>,
        src: &Src,
    ) -> Result<HipSlice<T>, HipError> {
        let mut dst = unsafe { self.alloc::<T>(src.len()) }?;
        self.memcpy_dtod(src, &mut dst)?;
        Ok(dst)
    }

    /// Copy `src` (host) into `dst` (device). `dst.len()` must be at least
    /// `src.len()`.
    pub fn memcpy_htod<T: DeviceRepr, Src: HostSlice<T> + ?Sized, Dst: DevicePtrMut<T>>(
        self: &Arc<Self>,
        src: &Src,
        dst: &mut Dst,
    ) -> Result<(), HipError> {
        assert!(dst.len() >= src.len(), "memcpy_htod: dst smaller than src");
        self.ctx.bind_to_thread()?;
        let bytes = src.len() * std::mem::size_of::<T>();
        let (src_slice, _record_src) = unsafe { src.stream_synced_slice(self) };
        let (dptr, _record_dst) = dst.device_ptr_mut(self);
        let src_bytes = unsafe {
            std::slice::from_raw_parts(src_slice.as_ptr() as *const u8, bytes)
        };
        unsafe { result::memcpy_htod_async(dptr as u64, src_bytes, self.hip_stream) }
    }

    /// Copy `src` (device) into `dst` (host). `dst.len()` must be at least
    /// `src.len()`.
    pub fn memcpy_dtoh<T: DeviceRepr, Src: DevicePtr<T>, Dst: HostSlice<T> + ?Sized>(
        self: &Arc<Self>,
        src: &Src,
        dst: &mut Dst,
    ) -> Result<(), HipError> {
        assert!(dst.len() >= src.len(), "memcpy_dtoh: dst smaller than src");
        self.ctx.bind_to_thread()?;
        let bytes = src.len() * std::mem::size_of::<T>();
        let (sptr, _record_src) = src.device_ptr(self);
        let (dst_slice, _record_dst) = unsafe { dst.stream_synced_mut_slice(self) };
        let dst_bytes = unsafe {
            std::slice::from_raw_parts_mut(dst_slice.as_mut_ptr() as *mut u8, bytes)
        };
        unsafe { result::memcpy_dtoh_async(dst_bytes, sptr as u64, self.hip_stream) }
    }

    /// Copy `src` (device) into `dst` (device).
    pub fn memcpy_dtod<T: DeviceRepr, Src: DevicePtr<T>, Dst: DevicePtrMut<T>>(
        self: &Arc<Self>,
        src: &Src,
        dst: &mut Dst,
    ) -> Result<(), HipError> {
        assert!(dst.len() >= src.len(), "memcpy_dtod: dst smaller than src");
        self.ctx.bind_to_thread()?;
        
        let bytes = src.len() * std::mem::size_of::<T>();

        let src_ctx = src.stream().context();
        let dst_ctx = self.context();

        if src_ctx == dst_ctx {
            let (sptr, _record_src) = src.device_ptr(self);
        let (dptr, _record_dst) = dst.device_ptr_mut(self);
        unsafe { result::memcpy_dtod_async(dptr as u64, sptr as u64, bytes, self.hip_stream) }
        } else {
            let (sptr, _record_src) = src.device_ptr(src.stream());
            let (dptr, _record_dsdt) = dst.device_ptr_mut(self);
            self.wait(&src.stream().record_event(None)?)?;
            unsafe {
                result::memcpy_peer_async(
                    dptr as u64,
                    dst_ctx.ordinal() as c_int,
                    sptr as u64,
                    src_ctx.ordinal() as c_int,
                    bytes,
                    self.hip_stream
                )
            }
        }
    }
}

// ----------------------------------------------------------------------------
// PinnedHostSlice
// ----------------------------------------------------------------------------

/// Page-locked host buffer suitable for async H2D / D2H transfers.
///
/// Carries its own [`HipEvent`] (blocking-sync) so reads of the host data
/// after a kernel scheduled against the buffer wait for completion.
pub struct PinnedHostSlice<T> {
    pub(crate) ptr: *mut T,
    pub(crate) len: usize,
    pub(crate) event: HipEvent,
}

impl<T> std::fmt::Debug for PinnedHostSlice<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PinnedHostSlice")
            .field("ptr", &self.ptr)
            .field("len", &self.len)
            .finish()
    }
}

unsafe impl<T: Send> Send for PinnedHostSlice<T> {}
unsafe impl<T: Sync> Sync for PinnedHostSlice<T> {}

impl<T> Drop for PinnedHostSlice<T> {
    fn drop(&mut self) {
        let ctx = self.event.ctx.clone();
        ctx.record_err(self.event.synchronize());
        ctx.record_err(result::free_host(self.ptr as *mut _));
    }
}

impl HipContext {
    /// Allocate `len` elements of pinned (page-locked) host memory.
    /// Uses `hipHostMallocWriteCombined` (write-combined memory: fastest
    /// for H2D, slow for host reads).
    ///
    /// # Safety
    /// The returned buffer is uninitialized. Use [`PinnedHostSlice::as_slice`]
    /// /  [`PinnedHostSlice::as_mut_slice`] on `T: ValidAsZeroBits` for safe
    /// access patterns.
    pub unsafe fn alloc_pinned<T: DeviceRepr>(
        self: &Arc<Self>,
        len: usize,
    ) -> Result<PinnedHostSlice<T>, HipError> {
        self.bind_to_thread()?;
        let bytes = len.checked_mul(std::mem::size_of::<T>()).expect("size overflow");
        let ptr = result::malloc_host(bytes, HIP_HOST_MALLOC_WRITE_COMBINED)? as *mut T;
        assert!(!ptr.is_null());
        let event = self.new_event(Some(HIP_EVENT_BLOCKING_SYNC))?;
        Ok(PinnedHostSlice { ptr, len, event })
    }
}

impl<T> PinnedHostSlice<T> {
    pub fn context(&self) -> &Arc<HipContext> {
        &self.event.ctx
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn num_bytes(&self) -> usize {
        self.len * std::mem::size_of::<T>()
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T: ValidAsZeroBits> PinnedHostSlice<T> {
    /// Block on pending device work, then return a read-only host pointer.
    pub fn as_ptr(&self) -> Result<*const T, HipError> {
        self.event.synchronize()?;
        Ok(self.ptr)
    }

    /// Block on pending device work, then return a writable host pointer.
    pub fn as_mut_ptr(&mut self) -> Result<*mut T, HipError> {
        self.event.synchronize()?;
        Ok(self.ptr)
    }

    /// Block, then return a `&[T]` view of the buffer.
    pub fn as_slice(&self) -> Result<&[T], HipError> {
        self.event.synchronize()?;
        Ok(unsafe { std::slice::from_raw_parts(self.ptr, self.len) })
    }

    /// Block, then return a `&mut [T]` view of the buffer.
    pub fn as_mut_slice(&mut self) -> Result<&mut [T], HipError> {
        self.event.synchronize()?;
        Ok(unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) })
    }
}

impl<T> HostSlice<T> for PinnedHostSlice<T> {
    fn len(&self) -> usize {
        self.len
    }

    unsafe fn stream_synced_slice<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (&'a [T], SyncOnDrop<'a>) {
        stream.ctx.record_err(stream.wait(&self.event));
        (
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) },
            SyncOnDrop::Record(Some((&self.event, stream))),
        )
    }

    unsafe fn stream_synced_mut_slice<'a>(
        &'a mut self,
        stream: &'a HipStream,
    ) -> (&'a mut [T], SyncOnDrop<'a>) {
        stream.ctx.record_err(stream.wait(&self.event));
        (
            unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) },
            SyncOnDrop::Record(Some((&self.event, stream))),
        )
    }
}

// ----------------------------------------------------------------------------
// HipModule, HipFunction
// ----------------------------------------------------------------------------

/// A loaded HIP code object (hsaco / fatbin). Analogue of
/// `cudarc::driver::safe::CudaModule`.
#[derive(Debug)]
pub struct HipModule {
    pub(crate) hip_module: sys::hipModule_t,
    pub(crate) ctx: Arc<HipContext>,
}

unsafe impl Send for HipModule {}
unsafe impl Sync for HipModule {}

impl Drop for HipModule {
    fn drop(&mut self) {
        self.ctx.record_err(self.ctx.bind_to_thread());
        self.ctx
            .record_err(unsafe { result::module::unload(self.hip_module) });
    }
}

impl HipContext {
    /// Load an hsaco / fatbin code object into this context.
    ///
    /// For [`crate::hiprtc::HsacoKind::File`] hsacos, the file is read
    /// here; I/O errors surface as `HipError(hipErrorFileNotFound)`.
    pub fn load_module(
        self: &Arc<Self>,
        hsaco: crate::hiprtc::Hsaco,
    ) -> Result<Arc<HipModule>, HipError> {
        self.bind_to_thread()?;
        let bytes = hsaco
            .as_bytes()
            .map_err(|_| HipError(sys::hipError_t::hipErrorFileNotFound))?;
        let hip_module = unsafe { result::module::load_data(bytes.as_ref()) }?;
        Ok(Arc::new(HipModule {
            hip_module,
            ctx: self.clone(),
        }))
    }
}

impl HipModule {
    /// Look up a kernel by name within this module.
    pub fn load_function(self: &Arc<Self>, name: &str) -> Result<HipFunction, HipError> {
        let cname = CString::new(name)
            .map_err(|_| HipError(sys::hipError_t::hipErrorInvalidValue))?;
        let hip_function = result::module::get_function(self.hip_module, cname.as_c_str())?;
        Ok(HipFunction {
            hip_function,
            module: self.clone(),
        })
    }

    /// Get a `__device__` global symbol by name as a typed view of device
    /// memory. The view is unmanaged (no read/write events) — caller
    /// synchronizes via the stream they pass to `device_ptr` / `device_ptr_mut`.
    pub fn get_global<'a>(
        self: &'a Arc<Self>,
        name: &str,
        stream: &'a Arc<HipStream>,
    ) -> Result<HipViewMut<'a, u8>, HipError> {
        let cname = CString::new(name)
            .map_err(|_| HipError(sys::hipError_t::hipErrorInvalidValue))?;
        let (raw_ptr, bytes) =
            result::module::get_global(self.hip_module, cname.as_c_str())?;
        Ok(HipViewMut {
            ptr: raw_ptr as sys::hipDeviceptr_t,
            len: bytes,
            read: &NO_EVENT,
            write: &NO_EVENT,
            stream,
            marker: PhantomData,
        })
    }
}

/// Static `None` reference used by [`HipModule::get_global`] for the
/// unmanaged read/write event slots. Globals don't participate in
/// allocation-time event tracking — callers handle synchronization at the
/// stream level.
static NO_EVENT: Option<HipEvent> = None;

/// A kernel function inside a loaded [`HipModule`]. Analogue of
/// `cudarc::driver::safe::CudaFunction`. Cheap to clone — the raw
/// function handle is a pointer and the `Arc<HipModule>` keeps the owning
/// module alive.
///
/// # Unsupported on HIP
/// `set_attribute` / `set_function_cache_config` are intentionally **not**
/// exposed: HIP only ships the runtime-API setters (`hipFuncSetAttribute`,
/// `hipFuncSetCacheConfig`), which take a host `__global__` symbol pointer
/// rather than a `hipFunction_t`. Kernels loaded via `hipModuleGetFunction`
/// (the rocmrc happy path) have no associated host symbol, so the setters
/// are unreachable. See the `function` module doc on
/// [`crate::hip::result`].
#[derive(Debug, Clone)]
pub struct HipFunction {
    pub(crate) hip_function: sys::hipFunction_t,
    #[allow(unused)]
    pub(crate) module: Arc<HipModule>,
}

unsafe impl Send for HipFunction {}
unsafe impl Sync for HipFunction {}

impl HipFunction {
    /// Raw `hipFunction_t`. Do not destroy.
    pub fn hip_function(&self) -> sys::hipFunction_t {
        self.hip_function
    }

    pub fn get_attribute(
        &self,
        attribute: sys::hipFunction_attribute,
    ) -> Result<i32, HipError> {
        result::function::get_function_attribute(self.hip_function, attribute)
    }

    pub fn num_regs(&self) -> Result<i32, HipError> {
        self.get_attribute(sys::hipFunction_attribute::HIP_FUNC_ATTRIBUTE_NUM_REGS)
    }

    pub fn shared_size_bytes(&self) -> Result<i32, HipError> {
        self.get_attribute(sys::hipFunction_attribute::HIP_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES)
    }

    pub fn const_size_bytes(&self) -> Result<i32, HipError> {
        self.get_attribute(sys::hipFunction_attribute::HIP_FUNC_ATTRIBUTE_CONST_SIZE_BYTES)
    }

    pub fn local_size_bytes(&self) -> Result<i32, HipError> {
        self.get_attribute(sys::hipFunction_attribute::HIP_FUNC_ATTRIBUTE_LOCAL_SIZE_BYTES)
    }

    pub fn max_threads_per_block(&self) -> Result<i32, HipError> {
        self.get_attribute(sys::hipFunction_attribute::HIP_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK)
    }

    /// **HIP note:** the attribute is named `PTX_VERSION` for CUDA-source-shape
    /// parity, but on HIP it returns the hsaco code-object version.
    pub fn ptx_version(&self) -> Result<i32, HipError> {
        self.get_attribute(sys::hipFunction_attribute::HIP_FUNC_ATTRIBUTE_PTX_VERSION)
    }

    pub fn binary_version(&self) -> Result<i32, HipError> {
        self.get_attribute(sys::hipFunction_attribute::HIP_FUNC_ATTRIBUTE_BINARY_VERSION)
    }

    /// Maximum number of resident blocks per multiprocessor for this kernel
    /// at the given block size and dynamic shared memory.
    pub fn occupancy_max_active_blocks_per_multiprocessor(
        &self,
        block_size: i32,
        dynamic_smem_bytes: usize,
        flags: Option<c_uint>,
    ) -> Result<i32, HipError> {
        match flags {
            None => result::occupancy::max_active_blocks_per_multiprocessor(
                self.hip_function,
                block_size,
                dynamic_smem_bytes,
            ),
            Some(f) => result::occupancy::max_active_blocks_per_multiprocessor_with_flags(
                self.hip_function,
                block_size,
                dynamic_smem_bytes,
                f,
            ),
        }
    }

    /// Returns `(min_grid_size, block_size)` — the smallest grid that
    /// saturates the device and the block size that achieves the
    /// saturation.
    pub fn occupancy_max_potential_block_size(
        &self,
        dynamic_smem_bytes: usize,
        block_size_limit: i32,
        flags: Option<c_uint>,
    ) -> Result<(i32, i32), HipError> {
        match flags {
            None => result::occupancy::max_potential_block_size(
                self.hip_function,
                dynamic_smem_bytes,
                block_size_limit,
            ),
            Some(f) => result::occupancy::max_potential_block_size_with_flags(
                self.hip_function,
                dynamic_smem_bytes,
                block_size_limit,
                f,
            ),
        }
    }
}

// ============================================================================
// Tests — ported from cudarc's `src/driver/safe/core.rs` test module.
// All tests assume a HIP runtime + GPU is available; without one they fail
// loudly (consistent with cudarc's convention).
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transmutes() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let buf: HipSlice<u32> = stream.clone_htod(&[0xDEADBEEF_u32, 0xCAFEBABE]).unwrap();
        // u32 → u8 transmute should produce 8 bytes.
        let as_u8: HipSlice<u8> = buf.transmute().unwrap();
        assert_eq!(as_u8.len(), 8);
        // Round-trip via host to confirm bytes survived.
        let host: Vec<u8> = stream.clone_dtoh(&as_u8).unwrap();
        assert_eq!(host.len(), 8);
    }

    #[test]
    fn test_threading() {
        let ctx = HipContext::new(0).unwrap();
        let threads: Vec<_> = (0..2)
            .map(|_| {
                let ctx = ctx.clone();
                std::thread::spawn(move || {
                    ctx.bind_to_thread().unwrap();
                    let stream = ctx.default_stream();
                    let _: HipSlice<f32> = stream.alloc_zeros(64).unwrap();
                })
            })
            .collect();
        for t in threads {
            t.join().unwrap();
        }
    }

    #[test]
    fn test_post_build_arc_count() {
        let ctx = HipContext::new(0).unwrap();
        assert_eq!(Arc::strong_count(&ctx), 1);
    }

    #[test]
    fn test_post_alloc_arc_counts() {
        let ctx = HipContext::new(0).unwrap();
        assert_eq!(Arc::strong_count(&ctx), 1);
        let stream = ctx.default_stream();
        // +1 for the `ctx` Arc inside the freshly-created stream.
        assert_eq!(Arc::strong_count(&ctx), 2);
        let t: HipSlice<f32> = stream.alloc_zeros(64).unwrap();
        // +2 for the `ctx` Arcs inside the slice's read/write events.
        // (`slice.stream` is a clone of `stream`, so they share the same
        // inner `ctx` field — that one doesn't add to the count.)
        assert_eq!(Arc::strong_count(&ctx), 4);
        // +1 for `slice.stream` (clone of the local `stream`).
        assert_eq!(Arc::strong_count(&stream), 2);
        drop(t);
        assert_eq!(Arc::strong_count(&ctx), 2);
        assert_eq!(Arc::strong_count(&stream), 1);
        drop(stream);
        assert_eq!(Arc::strong_count(&ctx), 1);
    }

    #[test]
    #[ignore = "must be run in isolation"]
    fn test_post_alloc_memory() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let (free0, _total) = ctx.mem_get_info().unwrap();
        // ~40 MB allocation should be visible in the free-mem delta.
        let buf: HipSlice<f32> = stream.alloc_zeros(10_000_000).unwrap();
        let (free1, _) = ctx.mem_get_info().unwrap();
        assert!(free0 > free1, "free memory did not decrease after alloc");
        drop(buf);
        ctx.synchronize().unwrap();
        let (free2, _) = ctx.mem_get_info().unwrap();
        assert!(free2 > free1, "free memory did not recover after drop");
        assert_eq!(free2, free0, "freed memory did not match pre-allocated memory")
    }

    #[test]
    fn test_ctx_copy_to_views() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let mut big: HipSlice<f32> = stream.alloc_zeros(50).unwrap();
        for i in 0..5 {
            let chunk: Vec<f32> = (0..10).map(|j| (i * 10 + j) as f32).collect();
            let src = stream.clone_htod(&chunk).unwrap();
            let mut view = big.slice_mut(i * 10..(i + 1) * 10);
            stream.memcpy_dtod(&src, &mut view).unwrap();
        }
        let out: Vec<f32> = stream.clone_dtoh(&big).unwrap();
        for (i, &v) in out.iter().enumerate() {
            assert_eq!(v, i as f32);
        }
    }

    #[test]
    fn test_leak_and_upgrade() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let buf: HipSlice<f32> = stream.clone_htod(&[1.0_f32, 2.0, 3.0, 4.0]).unwrap();
        let (raw, len) = buf.leak();
        assert_eq!(len, 4);
        let rebuilt: HipSlice<f32> =
            unsafe { stream.upgrade_device_ptr(raw, len) }.unwrap();
        let out: Vec<f32> = stream.clone_dtoh(&rebuilt).unwrap();
        assert_eq!(out, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_slice_is_freed_with_correct_context() {
        let ctx_a = HipContext::new(0).unwrap();
        let stream_a = ctx_a.default_stream();
        let buf: HipSlice<f32> = stream_a.alloc_zeros(1024).unwrap();

        // Bind a (notionally) different context on this thread — for a
        // single-GPU host, `new` binds the same device, but the
        // bind_to_thread plumbing exercises the same code path.
        let ctx_b = HipContext::new(0).unwrap();
        ctx_b.bind_to_thread().unwrap();

        // Dropping the slice should still free against ctx_a, not the
        // currently-bound ctx_b. Without that, hipFree would target the
        // wrong context and surface an error via record_err.
        drop(buf);
        ctx_a.check_err().unwrap();
    }

    #[test]
    fn test_copy_uses_correct_context() {
        let ctx_a = HipContext::new(0).unwrap();
        let stream_a = ctx_a.default_stream();
        let buf: HipSlice<f32> = stream_a.clone_htod(&[1.0_f32; 32]).unwrap();
        let ctx_b = HipContext::new(0).unwrap();
        ctx_b.bind_to_thread().unwrap();
        let out: Vec<f32> = stream_a.clone_dtoh(&buf).unwrap();
        assert!(out.iter().all(|&v| v == 1.0));
    }

    #[test]
    fn test_htod_copy_pinned() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let mut pinned: PinnedHostSlice<f32> = unsafe { ctx.alloc_pinned(64).unwrap() };
        for (i, v) in pinned.as_mut_slice().unwrap().iter_mut().enumerate() {
            *v = i as f32;
        }
        let dev: HipSlice<f32> = stream.clone_htod(&pinned).unwrap();
        let back: Vec<f32> = stream.clone_dtoh(&dev).unwrap();
        for (i, &v) in back.iter().enumerate() {
            assert_eq!(v, i as f32);
        }
    }

    #[test]
    fn test_pinned_copy_is_faster() {
        // Pinned host memory should make H→D transfers measurably faster
        // than pageable memory because HIP can DMA straight from the
        // physical pages without staging through a driver-internal copy.
        //
        // The 1.5x threshold is empirical and PCIe/topology-dependent;
        // on lower-end iGPUs (shared host memory) the speedup is small,
        // on discrete cards it's usually 2-5x. Adjust per hardware if
        // CI sees flakes.
        use std::time::Instant;
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.new_stream().unwrap();

        let n = 100_000;
        let n_samples = 5;
        let not_pinned = vec![0.0f32; n];

        let start = Instant::now();
        for _ in 0..n_samples {
            let _ = stream.clone_htod(&not_pinned).unwrap();
            stream.synchronize().unwrap();
        }
        let unpinned_elapsed = start.elapsed() / n_samples;

        let pinned = unsafe { ctx.alloc_pinned::<f32>(n) }.unwrap();

        let start = Instant::now();
        for _ in 0..n_samples {
            let _ = stream.clone_htod(&pinned).unwrap();
            stream.synchronize().unwrap();
        }
        let pinned_elapsed = start.elapsed() / n_samples;

        assert!(
            pinned_elapsed.as_secs_f32() * 1.5 < unpinned_elapsed.as_secs_f32(),
            "{unpinned_elapsed:?} vs {pinned_elapsed:?}",
        );
    }

    #[test]
    fn test_context_htod_dtoh() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let dev: HipSlice<f32> = stream.clone_htod(&[1.0_f32, 2.0, 3.0]).unwrap();
        let host: Vec<f32> = stream.clone_dtoh(&dev).unwrap();
        assert_eq!(host, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_context_cross_thread_bind() {
        let ctx = HipContext::new(0).unwrap();
        let ctx_for_thread = ctx.clone();
        std::thread::spawn(move || {
            ctx_for_thread.bind_to_thread().unwrap();
            let stream = ctx_for_thread.default_stream();
            let _: HipSlice<f32> = stream.alloc_zeros(32).unwrap();
        })
        .join()
        .unwrap();
    }
}
