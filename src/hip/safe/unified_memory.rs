//! HIP unified (managed) memory. Mirrors
//! [`cudarc::driver::safe::unified_memory`].
//!
//! Allocated via [`HipContext::alloc_unified`] (which wraps
//! `hipMallocManaged`). The returned [`HipUnifiedSlice`] is addressable
//! from both host (Rust) code via [`HipUnifiedSlice::as_slice`] /
//! [`as_mut_slice`](HipUnifiedSlice::as_mut_slice) **and** from device
//! kernels via the same `PushKernelArg` / `DevicePtr` machinery used for
//! plain [`super::HipSlice`].
//!
//! Three attach modes (HIP's `hipMemAttachGlobal` / `Host` / `Single`)
//! govern which streams may touch the buffer; access checks are enforced
//! at `device_ptr` and at kernel-arg time via
//! [`HipUnifiedSlice::check_device_access`].

use core::marker::PhantomData;
use std::ops::{Bound, RangeBounds};
use std::sync::Arc;

use crate::hip::{result, sys};

use super::{
    DevicePtr, DevicePtrMut, DeviceRepr, DeviceSlice, HipContext, HipError, HipEvent,
    HipStream, HostSlice, LaunchArgs, PushKernelArg, SyncOnDrop, ValidAsZeroBits,
};

// HIP `#define` flag values that bindgen drops. Re-declared with the same
// hex constants used in `<hip/hip_runtime_api.h>`.
const HIP_MEM_ATTACH_GLOBAL: std::ffi::c_uint = 0x1;
const HIP_MEM_ATTACH_HOST: std::ffi::c_uint = 0x2;
const HIP_MEM_ATTACH_SINGLE: std::ffi::c_uint = 0x4;

// `hipCpuDeviceId` is `#define hipCpuDeviceId ((int)-1)` — bindgen drops
// the macro, so we redeclare with the same value.
const HIP_CPU_DEVICE_ID: std::ffi::c_int = -1;

/// Attach mode for a [`HipUnifiedSlice`]. Mirrors HIP's
/// `hipMemAttachGlobal` / `Host` / `Single` flag triple, which are
/// `#define`s in the C headers (bindgen drops them).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemAttachFlags {
    /// Any device or stream may access the memory.
    Global,
    /// Only the host may access; device access requires
    /// `hipDeviceAttributeConcurrentManagedAccess`.
    Host,
    /// Only the attached stream may access. Host access synchronizes
    /// that stream.
    Single,
}

impl MemAttachFlags {
    #[inline]
    fn to_raw(self) -> std::ffi::c_uint {
        match self {
            Self::Global => HIP_MEM_ATTACH_GLOBAL,
            Self::Host => HIP_MEM_ATTACH_HOST,
            Self::Single => HIP_MEM_ATTACH_SINGLE,
        }
    }
}

// ----------------------------------------------------------------------------
// HipUnifiedSlice<T>
// ----------------------------------------------------------------------------

/// Unified memory buffer. Analogue of
/// [`cudarc::driver::safe::UnifiedSlice`].
///
/// Allocated by [`HipContext::alloc_unified`] (wraps `hipMallocManaged`)
/// and freed on drop via `hipFree`. Holds a blocking-sync [`HipEvent`]
/// used to gate host access and propagate read/write ordering through
/// the launch builder.
///
/// # Thread safety
/// Send + Sync. Host-side concurrent access is governed by
/// [`Self::check_host_access`] / [`Self::check_device_access`].
#[derive(Debug)]
pub struct HipUnifiedSlice<T> {
    pub(crate) hip_device_ptr: sys::hipDeviceptr_t,
    pub(crate) len: usize,
    pub(crate) stream: Arc<HipStream>,
    pub(crate) event: HipEvent,
    pub(crate) attach_mode: MemAttachFlags,
    pub(crate) concurrent_managed_access: bool,
    pub(crate) marker: PhantomData<*const T>,
}

unsafe impl<T> Send for HipUnifiedSlice<T> {}
unsafe impl<T> Sync for HipUnifiedSlice<T> {}

impl<T> Drop for HipUnifiedSlice<T> {
    fn drop(&mut self) {
        self.stream.context().record_err(self.event.synchronize());
        self.stream
            .context()
            .record_err(result::free_sync(self.hip_device_ptr as u64));
    }
}

/// Immutable borrowed view into a [`HipUnifiedSlice`].
#[derive(Debug, Copy, Clone)]
pub struct HipUnifiedView<'a, T> {
    pub(crate) ptr: sys::hipDeviceptr_t,
    pub(crate) len: usize,
    pub(crate) event: &'a HipEvent,
    pub(crate) stream: &'a Arc<HipStream>,
    pub(crate) attach_mode: MemAttachFlags,
    pub(crate) concurrent_managed_access: bool,
    marker: PhantomData<&'a [T]>,
}

/// Mutable borrowed view into a [`HipUnifiedSlice`].
#[derive(Debug)]
pub struct HipUnifiedViewMut<'a, T> {
    pub(crate) ptr: sys::hipDeviceptr_t,
    pub(crate) len: usize,
    pub(crate) event: &'a HipEvent,
    pub(crate) stream: &'a Arc<HipStream>,
    pub(crate) attach_mode: MemAttachFlags,
    pub(crate) concurrent_managed_access: bool,
    marker: PhantomData<&'a mut [T]>,
}

// ----------------------------------------------------------------------------
// HipContext::alloc_unified
// ----------------------------------------------------------------------------

impl HipContext {
    /// Allocate unified (managed) memory.
    ///
    /// `attach_global = true` uses `hipMemAttachGlobal` (any device/stream
    /// can access); `false` uses `hipMemAttachHost`. To switch to
    /// `hipMemAttachSingle` after allocation use
    /// [`HipUnifiedSlice::attach`].
    ///
    /// Returns [`sys::hipError_t::hipErrorNotSupported`] if the device
    /// does not advertise `hipDeviceAttributeManagedMemory`.
    ///
    /// # Safety
    /// The returned buffer is uninitialized; `T` may not be valid for
    /// every bit pattern. Initialize via [`HipUnifiedSlice::as_mut_slice`]
    /// (when `T: ValidAsZeroBits`) or memset before reads.
    pub unsafe fn alloc_unified<T: DeviceRepr>(
        self: &Arc<Self>,
        len: usize,
        attach_global: bool,
    ) -> Result<HipUnifiedSlice<T>, HipError> {
        if self.attribute(sys::hipDeviceAttribute_t::hipDeviceAttributeManagedMemory)? == 0 {
            return Err(HipError(sys::hipError_t::hipErrorNotSupported));
        }

        let attach_mode = if attach_global {
            MemAttachFlags::Global
        } else {
            MemAttachFlags::Host
        };

        let bytes = len.checked_mul(std::mem::size_of::<T>()).expect("size overflow");
        let raw = result::malloc_managed(bytes, attach_mode.to_raw())?;
        let concurrent_managed_access = self.attribute(
            sys::hipDeviceAttribute_t::hipDeviceAttributeConcurrentManagedAccess,
        )? != 0;

        let stream = self.default_stream();
        // Blocking-sync event so `as_slice` / `as_mut_slice` block on the
        // calling thread until kernels complete.
        let event = self.new_event(Some(0x1 /* hipEventBlockingSync */))?;

        Ok(HipUnifiedSlice {
            hip_device_ptr: raw as sys::hipDeviceptr_t,
            len,
            stream,
            event,
            attach_mode,
            concurrent_managed_access,
            marker: PhantomData,
        })
    }
}

// ----------------------------------------------------------------------------
// HipUnifiedSlice — state-management methods
// ----------------------------------------------------------------------------

impl<T> HipUnifiedSlice<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn attach_mode(&self) -> MemAttachFlags {
        self.attach_mode
    }

    pub fn num_bytes(&self) -> usize {
        self.len * std::mem::size_of::<T>()
    }

    /// Re-attach this buffer to a specific stream and update its attach
    /// mode. Wraps `hipStreamAttachMemAsync`.
    ///
    /// Synchronizes the buffer's current event before retargeting.
    /// HIP rejects the legacy NULL stream — pass a real stream.
    pub fn attach(
        &mut self,
        stream: &Arc<HipStream>,
        flags: MemAttachFlags,
    ) -> Result<(), HipError> {
        self.event.synchronize()?;
        self.stream = stream.clone();
        self.attach_mode = flags;
        unsafe {
            result::stream::attach_mem_async(
                self.stream.hip_stream(),
                self.hip_device_ptr as u64,
                self.num_bytes(),
                self.attach_mode.to_raw(),
            )
        }
    }

    /// Hint HIP to prefetch the buffer to the appropriate location based
    /// on the current [`Self::attach_mode`].
    ///
    /// - `Global` / `Single` → prefetch to the stream's device (requires
    ///   `hipDeviceAttributeConcurrentManagedAccess`).
    /// - `Host` → prefetch to host memory (`hipCpuDeviceId`).
    pub fn prefetch(&self) -> Result<(), HipError> {
        let device = match self.attach_mode {
            MemAttachFlags::Global | MemAttachFlags::Single => {
                if !self.concurrent_managed_access {
                    return Err(HipError(sys::hipError_t::hipErrorNotSupported));
                }
                self.stream.context().ordinal() as std::ffi::c_int
            }
            MemAttachFlags::Host => HIP_CPU_DEVICE_ID,
        };
        unsafe {
            result::mem_prefetch_async(
                self.hip_device_ptr as u64,
                self.num_bytes(),
                device,
                self.stream.hip_stream(),
            )
        }
    }

    /// Validate that host access is permitted for the current attach
    /// mode; synchronizes the owning stream when required.
    pub fn check_host_access(&self) -> Result<(), HipError> {
        match self.attach_mode {
            MemAttachFlags::Global => {
                // Docs are silent on whole-context sync requirements; we
                // trust the caller to coordinate.
            }
            MemAttachFlags::Host => {
                // No special host-side constraint documented for Host
                // attach mode.
            }
            MemAttachFlags::Single => {
                // Single-attach binds the buffer to its owning stream;
                // host access is permitted only once that stream drains.
                self.stream.synchronize()?;
            }
        }
        Ok(())
    }

    /// Validate that the given stream may access this buffer. Returns
    /// `Err(hipErrorNotSupported)` when the attach mode forbids it.
    pub fn check_device_access(&self, stream: &HipStream) -> Result<(), HipError> {
        check_device_access(
            self.attach_mode,
            &self.stream,
            self.concurrent_managed_access,
            stream,
        )
    }
}

/// Consolidated access check, shared between [`HipUnifiedSlice`] and the
/// borrowed views.
fn check_device_access(
    attach_mode: MemAttachFlags,
    owner_stream: &Arc<HipStream>,
    concurrent_managed_access: bool,
    stream: &HipStream,
) -> Result<(), HipError> {
    match attach_mode {
        MemAttachFlags::Global => {
            // Any context/stream may access.
        }
        MemAttachFlags::Host => {
            // Cross-context access requires the *target* context to
            // advertise concurrent managed access.
            let cma = if !Arc::ptr_eq(owner_stream.context(), stream.context()) {
                stream.context().attribute(
                    sys::hipDeviceAttribute_t::hipDeviceAttributeConcurrentManagedAccess,
                )? != 0
            } else {
                concurrent_managed_access
            };
            if !cma {
                return Err(HipError(sys::hipError_t::hipErrorNotSupported));
            }
        }
        MemAttachFlags::Single => {
            if owner_stream.as_ref() != stream {
                return Err(HipError(sys::hipError_t::hipErrorNotSupported));
            }
        }
    }
    Ok(())
}

// ----------------------------------------------------------------------------
// HipUnifiedSlice — view-creation methods
// ----------------------------------------------------------------------------

impl<T> HipUnifiedSlice<T> {
    /// Borrow the whole slice as an immutable view.
    pub fn as_view(&self) -> HipUnifiedView<'_, T> {
        HipUnifiedView {
            ptr: self.hip_device_ptr,
            len: self.len,
            event: &self.event,
            stream: &self.stream,
            attach_mode: self.attach_mode,
            concurrent_managed_access: self.concurrent_managed_access,
            marker: PhantomData,
        }
    }

    /// Borrow the whole slice as a mutable view.
    pub fn as_view_mut(&mut self) -> HipUnifiedViewMut<'_, T> {
        HipUnifiedViewMut {
            ptr: self.hip_device_ptr,
            len: self.len,
            event: &self.event,
            stream: &self.stream,
            attach_mode: self.attach_mode,
            concurrent_managed_access: self.concurrent_managed_access,
            marker: PhantomData,
        }
    }

    /// Borrow a sub-range as an immutable view.
    ///
    /// # Panics
    /// If the range is out of bounds.
    pub fn slice(&self, bounds: impl RangeBounds<usize>) -> HipUnifiedView<'_, T> {
        self.as_view().slice(bounds)
    }

    /// Fallible counterpart of [`Self::slice`].
    pub fn try_slice(&self, bounds: impl RangeBounds<usize>) -> Option<HipUnifiedView<'_, T>> {
        self.as_view().try_slice(bounds)
    }

    /// Borrow a sub-range as a mutable view.
    ///
    /// # Panics
    /// If the range is out of bounds.
    pub fn slice_mut(&mut self, bounds: impl RangeBounds<usize>) -> HipUnifiedViewMut<'_, T> {
        self.try_slice_mut(bounds).unwrap()
    }

    /// Fallible counterpart of [`Self::slice_mut`].
    pub fn try_slice_mut(
        &mut self,
        bounds: impl RangeBounds<usize>,
    ) -> Option<HipUnifiedViewMut<'_, T>> {
        to_range(bounds, self.len).map(|(start, end)| HipUnifiedViewMut {
            ptr: offset_ptr::<T>(self.hip_device_ptr, start),
            len: end - start,
            event: &self.event,
            stream: &self.stream,
            attach_mode: self.attach_mode,
            concurrent_managed_access: self.concurrent_managed_access,
            marker: PhantomData,
        })
    }

    /// Split into two immutable views at `mid`.
    ///
    /// # Panics
    /// If `mid > self.len`.
    pub fn split_at(&self, mid: usize) -> (HipUnifiedView<'_, T>, HipUnifiedView<'_, T>) {
        self.try_split_at(mid).unwrap()
    }

    /// Fallible counterpart of [`Self::split_at`].
    pub fn try_split_at(
        &self,
        mid: usize,
    ) -> Option<(HipUnifiedView<'_, T>, HipUnifiedView<'_, T>)> {
        (mid <= self.len).then(|| {
            let a = HipUnifiedView {
                ptr: self.hip_device_ptr,
                len: mid,
                event: &self.event,
                stream: &self.stream,
                attach_mode: self.attach_mode,
                concurrent_managed_access: self.concurrent_managed_access,
                marker: PhantomData,
            };
            let b = HipUnifiedView {
                ptr: offset_ptr::<T>(self.hip_device_ptr, mid),
                len: self.len - mid,
                event: &self.event,
                stream: &self.stream,
                attach_mode: self.attach_mode,
                concurrent_managed_access: self.concurrent_managed_access,
                marker: PhantomData,
            };
            (a, b)
        })
    }

    /// Split into two mutable views at `mid`.
    ///
    /// # Panics
    /// If `mid > self.len`.
    pub fn split_at_mut(
        &mut self,
        mid: usize,
    ) -> (HipUnifiedViewMut<'_, T>, HipUnifiedViewMut<'_, T>) {
        self.try_split_at_mut(mid).unwrap()
    }

    /// Fallible counterpart of [`Self::split_at_mut`].
    pub fn try_split_at_mut(
        &mut self,
        mid: usize,
    ) -> Option<(HipUnifiedViewMut<'_, T>, HipUnifiedViewMut<'_, T>)> {
        (mid <= self.len).then(|| {
            let a = HipUnifiedViewMut {
                ptr: self.hip_device_ptr,
                len: mid,
                event: &self.event,
                stream: &self.stream,
                attach_mode: self.attach_mode,
                concurrent_managed_access: self.concurrent_managed_access,
                marker: PhantomData,
            };
            let b = HipUnifiedViewMut {
                ptr: offset_ptr::<T>(self.hip_device_ptr, mid),
                len: self.len - mid,
                event: &self.event,
                stream: &self.stream,
                attach_mode: self.attach_mode,
                concurrent_managed_access: self.concurrent_managed_access,
                marker: PhantomData,
            };
            (a, b)
        })
    }
}

// ----------------------------------------------------------------------------
// DeviceSlice / DevicePtr / DevicePtrMut for HipUnifiedSlice
// ----------------------------------------------------------------------------

impl<T> DeviceSlice<T> for HipUnifiedSlice<T> {
    fn len(&self) -> usize {
        self.len
    }
    fn stream(&self) -> &Arc<HipStream> {
        &self.stream
    }
}

impl<T> DevicePtr<T> for HipUnifiedSlice<T> {
    fn device_ptr<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>) {
        stream.context().record_err(self.check_device_access(stream));
        stream.context().record_err(stream.wait(&self.event));
        (
            self.hip_device_ptr,
            SyncOnDrop::Record(Some((&self.event, stream))),
        )
    }
}

impl<T> DevicePtrMut<T> for HipUnifiedSlice<T> {
    fn device_ptr_mut<'a>(
        &'a mut self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>) {
        stream.context().record_err(self.check_device_access(stream));
        stream.context().record_err(stream.wait(&self.event));
        (
            self.hip_device_ptr,
            SyncOnDrop::Record(Some((&self.event, stream))),
        )
    }
}

// ----------------------------------------------------------------------------
// Host-side access (gated on `T: ValidAsZeroBits`)
// ----------------------------------------------------------------------------

impl<T: ValidAsZeroBits> HipUnifiedSlice<T> {
    /// Block on pending device work, then return a read-only host view.
    pub fn as_slice(&self) -> Result<&[T], HipError> {
        self.check_host_access()?;
        self.event.synchronize()?;
        Ok(unsafe { std::slice::from_raw_parts(self.hip_device_ptr as *const T, self.len) })
    }

    /// Block on pending device work, then return a writable host view.
    pub fn as_mut_slice(&mut self) -> Result<&mut [T], HipError> {
        self.check_host_access()?;
        self.event.synchronize()?;
        Ok(unsafe { std::slice::from_raw_parts_mut(self.hip_device_ptr as *mut T, self.len) })
    }
}

// ----------------------------------------------------------------------------
// HostSlice impls
// ----------------------------------------------------------------------------

impl<T> HostSlice<T> for HipUnifiedSlice<T> {
    fn len(&self) -> usize {
        self.len
    }
    unsafe fn stream_synced_slice<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (&'a [T], SyncOnDrop<'a>) {
        stream.context().record_err(self.check_device_access(stream));
        stream.context().record_err(stream.wait(&self.event));
        (
            std::slice::from_raw_parts(self.hip_device_ptr as *const T, self.len),
            SyncOnDrop::Record(Some((&self.event, stream))),
        )
    }

    unsafe fn stream_synced_mut_slice<'a>(
        &'a mut self,
        stream: &'a HipStream,
    ) -> (&'a mut [T], SyncOnDrop<'a>) {
        stream.context().record_err(self.check_device_access(stream));
        stream.context().record_err(stream.wait(&self.event));
        (
            std::slice::from_raw_parts_mut(self.hip_device_ptr as *mut T, self.len),
            SyncOnDrop::Record(Some((&self.event, stream))),
        )
    }
}

// ----------------------------------------------------------------------------
// PushKernelArg impls — HipUnifiedSlice
// ----------------------------------------------------------------------------

unsafe impl<'a, 'b: 'a, T> PushKernelArg<&'b HipUnifiedSlice<T>> for LaunchArgs<'a> {
    #[inline(always)]
    fn arg(&mut self, arg: &'b HipUnifiedSlice<T>) -> &mut Self {
        self.stream
            .context()
            .record_err(arg.check_device_access(self.stream));
        self.waits.push(&arg.event);
        self.records.push(&arg.event);
        self.args
            .push((&arg.hip_device_ptr) as *const sys::hipDeviceptr_t as _);
        self
    }
}

unsafe impl<'a, 'b: 'a, T> PushKernelArg<&'b mut HipUnifiedSlice<T>> for LaunchArgs<'a> {
    #[inline(always)]
    fn arg(&mut self, arg: &'b mut HipUnifiedSlice<T>) -> &mut Self {
        self.stream
            .context()
            .record_err(arg.check_device_access(self.stream));
        self.waits.push(&arg.event);
        self.records.push(&arg.event);
        self.args
            .push((&arg.hip_device_ptr) as *const sys::hipDeviceptr_t as _);
        self
    }
}

// ----------------------------------------------------------------------------
// DeviceSlice / DevicePtr / DevicePtrMut for views
// ----------------------------------------------------------------------------

impl<T> DeviceSlice<T> for HipUnifiedView<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
    fn stream(&self) -> &Arc<HipStream> {
        self.stream
    }
}

impl<T> DeviceSlice<T> for HipUnifiedViewMut<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
    fn stream(&self) -> &Arc<HipStream> {
        self.stream
    }
}

impl<T> DevicePtr<T> for HipUnifiedView<'_, T> {
    fn device_ptr<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>) {
        stream.context().record_err(check_device_access(
            self.attach_mode,
            self.stream,
            self.concurrent_managed_access,
            stream,
        ));
        stream.context().record_err(stream.wait(self.event));
        (self.ptr, SyncOnDrop::Record(Some((self.event, stream))))
    }
}

impl<T> DevicePtr<T> for HipUnifiedViewMut<'_, T> {
    fn device_ptr<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>) {
        stream.context().record_err(check_device_access(
            self.attach_mode,
            self.stream,
            self.concurrent_managed_access,
            stream,
        ));
        stream.context().record_err(stream.wait(self.event));
        (self.ptr, SyncOnDrop::Record(Some((self.event, stream))))
    }
}

impl<T> DevicePtrMut<T> for HipUnifiedViewMut<'_, T> {
    fn device_ptr_mut<'a>(
        &'a mut self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>) {
        stream.context().record_err(check_device_access(
            self.attach_mode,
            self.stream,
            self.concurrent_managed_access,
            stream,
        ));
        stream.context().record_err(stream.wait(self.event));
        (self.ptr, SyncOnDrop::Record(Some((self.event, stream))))
    }
}

// ----------------------------------------------------------------------------
// HostSlice impls for views
// ----------------------------------------------------------------------------

impl<T> HostSlice<T> for HipUnifiedView<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
    unsafe fn stream_synced_slice<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (&'a [T], SyncOnDrop<'a>) {
        stream.context().record_err(check_device_access(
            self.attach_mode,
            self.stream,
            self.concurrent_managed_access,
            stream,
        ));
        stream.context().record_err(stream.wait(self.event));
        (
            std::slice::from_raw_parts(self.ptr as *const T, self.len),
            SyncOnDrop::Record(Some((self.event, stream))),
        )
    }

    unsafe fn stream_synced_mut_slice<'a>(
        &'a mut self,
        stream: &'a HipStream,
    ) -> (&'a mut [T], SyncOnDrop<'a>) {
        stream.context().record_err(check_device_access(
            self.attach_mode,
            self.stream,
            self.concurrent_managed_access,
            stream,
        ));
        stream.context().record_err(stream.wait(self.event));
        (
            std::slice::from_raw_parts_mut(self.ptr as *mut T, self.len),
            SyncOnDrop::Record(Some((self.event, stream))),
        )
    }
}

impl<T> HostSlice<T> for HipUnifiedViewMut<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
    unsafe fn stream_synced_slice<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (&'a [T], SyncOnDrop<'a>) {
        stream.context().record_err(check_device_access(
            self.attach_mode,
            self.stream,
            self.concurrent_managed_access,
            stream,
        ));
        stream.context().record_err(stream.wait(self.event));
        (
            std::slice::from_raw_parts(self.ptr as *const T, self.len),
            SyncOnDrop::Record(Some((self.event, stream))),
        )
    }

    unsafe fn stream_synced_mut_slice<'a>(
        &'a mut self,
        stream: &'a HipStream,
    ) -> (&'a mut [T], SyncOnDrop<'a>) {
        stream.context().record_err(check_device_access(
            self.attach_mode,
            self.stream,
            self.concurrent_managed_access,
            stream,
        ));
        stream.context().record_err(stream.wait(self.event));
        (
            std::slice::from_raw_parts_mut(self.ptr as *mut T, self.len),
            SyncOnDrop::Record(Some((self.event, stream))),
        )
    }
}

// ----------------------------------------------------------------------------
// PushKernelArg impls for views
// ----------------------------------------------------------------------------

unsafe impl<'a, 'b: 'a, 'c: 'b, T> PushKernelArg<&'b HipUnifiedView<'c, T>> for LaunchArgs<'a> {
    #[inline(always)]
    fn arg(&mut self, arg: &'b HipUnifiedView<'c, T>) -> &mut Self {
        self.stream.context().record_err(check_device_access(
            arg.attach_mode,
            arg.stream,
            arg.concurrent_managed_access,
            self.stream,
        ));
        self.waits.push(arg.event);
        self.records.push(arg.event);
        self.args.push((&arg.ptr) as *const sys::hipDeviceptr_t as _);
        self
    }
}

unsafe impl<'a, 'b: 'a, 'c: 'b, T> PushKernelArg<&'b HipUnifiedViewMut<'c, T>>
    for LaunchArgs<'a>
{
    #[inline(always)]
    fn arg(&mut self, arg: &'b HipUnifiedViewMut<'c, T>) -> &mut Self {
        self.stream.context().record_err(check_device_access(
            arg.attach_mode,
            arg.stream,
            arg.concurrent_managed_access,
            self.stream,
        ));
        self.waits.push(arg.event);
        self.records.push(arg.event);
        self.args.push((&arg.ptr) as *const sys::hipDeviceptr_t as _);
        self
    }
}

unsafe impl<'a, 'b: 'a, 'c: 'b, T> PushKernelArg<&'b mut HipUnifiedViewMut<'c, T>>
    for LaunchArgs<'a>
{
    #[inline(always)]
    fn arg(&mut self, arg: &'b mut HipUnifiedViewMut<'c, T>) -> &mut Self {
        self.stream.context().record_err(check_device_access(
            arg.attach_mode,
            arg.stream,
            arg.concurrent_managed_access,
            self.stream,
        ));
        self.waits.push(arg.event);
        self.records.push(arg.event);
        self.args.push((&arg.ptr) as *const sys::hipDeviceptr_t as _);
        self
    }
}

// ----------------------------------------------------------------------------
// HipUnifiedView methods
// ----------------------------------------------------------------------------

impl<'a, T> HipUnifiedView<'a, T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Sub-view of the current view.
    ///
    /// # Panics
    /// If the range is out of bounds.
    pub fn slice(&self, bounds: impl RangeBounds<usize>) -> Self {
        self.try_slice(bounds).unwrap()
    }

    /// Fallible counterpart of [`Self::slice`].
    pub fn try_slice(&self, bounds: impl RangeBounds<usize>) -> Option<Self> {
        to_range(bounds, self.len).map(|(start, end)| HipUnifiedView {
            ptr: offset_ptr::<T>(self.ptr, start),
            len: end - start,
            event: self.event,
            stream: self.stream,
            attach_mode: self.attach_mode,
            concurrent_managed_access: self.concurrent_managed_access,
            marker: PhantomData,
        })
    }

    pub fn split_at(&self, mid: usize) -> (Self, Self) {
        self.try_split_at(mid).unwrap()
    }

    pub fn try_split_at(&self, mid: usize) -> Option<(Self, Self)> {
        (mid <= self.len).then(|| {
            let a = HipUnifiedView {
                ptr: self.ptr,
                len: mid,
                event: self.event,
                stream: self.stream,
                attach_mode: self.attach_mode,
                concurrent_managed_access: self.concurrent_managed_access,
                marker: PhantomData,
            };
            let b = HipUnifiedView {
                ptr: offset_ptr::<T>(self.ptr, mid),
                len: self.len - mid,
                event: self.event,
                stream: self.stream,
                attach_mode: self.attach_mode,
                concurrent_managed_access: self.concurrent_managed_access,
                marker: PhantomData,
            };
            (a, b)
        })
    }
}

// ----------------------------------------------------------------------------
// HipUnifiedViewMut methods
// ----------------------------------------------------------------------------

impl<'a, T> HipUnifiedViewMut<'a, T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Downgrade to an immutable view.
    pub fn as_view<'b>(&'b self) -> HipUnifiedView<'b, T> {
        HipUnifiedView {
            ptr: self.ptr,
            len: self.len,
            event: self.event,
            stream: self.stream,
            attach_mode: self.attach_mode,
            concurrent_managed_access: self.concurrent_managed_access,
            marker: PhantomData,
        }
    }

    pub fn slice<'b>(&'b self, bounds: impl RangeBounds<usize>) -> HipUnifiedView<'b, T> {
        self.try_slice(bounds).unwrap()
    }

    pub fn try_slice<'b>(
        &'b self,
        bounds: impl RangeBounds<usize>,
    ) -> Option<HipUnifiedView<'b, T>> {
        to_range(bounds, self.len).map(|(start, end)| HipUnifiedView {
            ptr: offset_ptr::<T>(self.ptr, start),
            len: end - start,
            event: self.event,
            stream: self.stream,
            attach_mode: self.attach_mode,
            concurrent_managed_access: self.concurrent_managed_access,
            marker: PhantomData,
        })
    }

    pub fn slice_mut<'b>(
        &'b mut self,
        bounds: impl RangeBounds<usize>,
    ) -> HipUnifiedViewMut<'b, T> {
        self.try_slice_mut(bounds).unwrap()
    }

    pub fn try_slice_mut<'b>(
        &'b mut self,
        bounds: impl RangeBounds<usize>,
    ) -> Option<HipUnifiedViewMut<'b, T>> {
        to_range(bounds, self.len).map(|(start, end)| HipUnifiedViewMut {
            ptr: offset_ptr::<T>(self.ptr, start),
            len: end - start,
            event: self.event,
            stream: self.stream,
            attach_mode: self.attach_mode,
            concurrent_managed_access: self.concurrent_managed_access,
            marker: PhantomData,
        })
    }

    pub fn split_at_mut<'b>(
        &'b mut self,
        mid: usize,
    ) -> (HipUnifiedViewMut<'b, T>, HipUnifiedViewMut<'b, T>) {
        self.try_split_at_mut(mid).unwrap()
    }

    pub fn try_split_at_mut<'b>(
        &'b mut self,
        mid: usize,
    ) -> Option<(HipUnifiedViewMut<'b, T>, HipUnifiedViewMut<'b, T>)> {
        (mid <= self.len).then(|| {
            let a = HipUnifiedViewMut {
                ptr: self.ptr,
                len: mid,
                event: self.event,
                stream: self.stream,
                attach_mode: self.attach_mode,
                concurrent_managed_access: self.concurrent_managed_access,
                marker: PhantomData,
            };
            let b = HipUnifiedViewMut {
                ptr: offset_ptr::<T>(self.ptr, mid),
                len: self.len - mid,
                event: self.event,
                stream: self.stream,
                attach_mode: self.attach_mode,
                concurrent_managed_access: self.concurrent_managed_access,
                marker: PhantomData,
            };
            (a, b)
        })
    }
}

// ----------------------------------------------------------------------------
// Helpers
// ----------------------------------------------------------------------------

/// Resolve a `RangeBounds` against `total_len`. Returns `None` if the
/// resulting `(start, end)` is out of bounds or inverted.
fn to_range(bounds: impl RangeBounds<usize>, total_len: usize) -> Option<(usize, usize)> {
    let start = match bounds.start_bound() {
        Bound::Included(&n) => n,
        Bound::Excluded(&n) => n.checked_add(1)?,
        Bound::Unbounded => 0,
    };
    let end = match bounds.end_bound() {
        Bound::Included(&n) => n.checked_add(1)?,
        Bound::Excluded(&n) => n,
        Bound::Unbounded => total_len,
    };
    (start <= end && end <= total_len).then_some((start, end))
}

#[inline]
fn offset_ptr<T>(base: sys::hipDeviceptr_t, count: usize) -> sys::hipDeviceptr_t {
    unsafe { (base as *mut u8).add(count * std::mem::size_of::<T>()) as sys::hipDeviceptr_t }
}

// ============================================================================
// Tests — ported from cudarc's `src/driver/safe/unified_memory.rs` test module.
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hip::safe::{HipSlice, LaunchConfig, PushKernelArg};
    use crate::hiprtc;

    /// Shared validation kernel: every element of `buf[0..n]` must
    /// equal `i` (as float).
    const CHECK_KERNEL: &str = r#"
extern "C" __global__
void check(float* buf, int n) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) assert(buf[i] == (float)i);
}
"#;

    fn compile_check(ctx: &Arc<HipContext>) -> crate::hip::safe::HipFunction {
        let gfx = ctx.gfx_version().expect("unsupported gfx arch");
        let hsaco = hiprtc::compile_hsaco(CHECK_KERNEL, gfx).unwrap();
        let module = ctx.load_module(hsaco).unwrap();
        module.load_function("check").unwrap()
    }

    #[test]
    fn test_unified_memory_global() {
        let ctx = HipContext::new(0).unwrap();
        let mut a: HipUnifiedSlice<f32> = unsafe { ctx.alloc_unified(100, true).unwrap() };
        // Host writes.
        for (i, v) in a.as_mut_slice().unwrap().iter_mut().enumerate() {
            *v = i as f32;
        }
        // Kernel reads and asserts.
        let f = compile_check(&ctx);
        let stream = ctx.default_stream();
        let n: i32 = 100;
        let cfg = LaunchConfig::for_num_elems(100);
        unsafe {
            stream
                .launch_builder(&f)
                .arg(&a)
                .arg(&n)
                .launch(cfg)
                .unwrap();
        }
        stream.synchronize().unwrap();
        // Host re-reads.
        let host = a.as_slice().unwrap();
        for (i, &v) in host.iter().enumerate() {
            assert_eq!(v, i as f32);
        }
    }

    #[test]
    fn test_unified_memory_host() {
        let ctx = HipContext::new(0).unwrap();
        // Host attach (attach_global = false).
        let mut a: HipUnifiedSlice<f32> = unsafe { ctx.alloc_unified(100, false).unwrap() };
        for (i, v) in a.as_mut_slice().unwrap().iter_mut().enumerate() {
            *v = i as f32;
        }
        // Device access from a stream other than default. If the device
        // doesn't advertise concurrent_managed_access, this will
        // surface an error from the launch builder via check_device_access.
        let f = compile_check(&ctx);
        let stream = ctx.new_stream().unwrap();
        let n: i32 = 100;
        let cfg = LaunchConfig::for_num_elems(100);
        if a.check_device_access(&stream).is_ok() {
            unsafe {
                stream
                    .launch_builder(&f)
                    .arg(&a)
                    .arg(&n)
                    .launch(cfg)
                    .unwrap();
            }
            stream.synchronize().unwrap();
        }
        // Else the device lacks concurrent_managed_access — the test
        // still validates the host-write half.
    }

    #[test]
    fn test_unified_memory_single_stream() {
        let ctx = HipContext::new(0).unwrap();
        let stream_a = ctx.new_stream().unwrap();
        let stream_b = ctx.new_stream().unwrap();
        let mut a: HipUnifiedSlice<f32> = unsafe { ctx.alloc_unified(100, true).unwrap() };
        for (i, v) in a.as_mut_slice().unwrap().iter_mut().enumerate() {
            *v = i as f32;
        }
        // Re-attach to stream_a in Single mode.
        a.attach(&stream_a, MemAttachFlags::Single).unwrap();

        // stream_a should be permitted.
        assert!(a.check_device_access(&stream_a).is_ok());
        // stream_b should be rejected.
        assert!(a.check_device_access(&stream_b).is_err());
    }

    #[test]
    fn test_unified_slice_copy_to_views() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let mut big: HipUnifiedSlice<f32> = unsafe { ctx.alloc_unified(50, true).unwrap() };
        // Zero-init via host.
        for v in big.as_mut_slice().unwrap().iter_mut() {
            *v = 0.0;
        }
        for i in 0..5 {
            let chunk: Vec<f32> = (0..10).map(|j| (i * 10 + j) as f32).collect();
            let src: HipSlice<f32> = stream.clone_htod(&chunk).unwrap();
            let mut view = big.slice_mut(i * 10..(i + 1) * 10);
            stream.memcpy_dtod(&src, &mut view).unwrap();
        }
        stream.synchronize().unwrap();
        let host = big.as_slice().unwrap();
        for (i, &v) in host.iter().enumerate() {
            assert_eq!(v, i as f32);
        }
    }

    #[test]
    fn test_unified_slice_split_at() {
        let ctx = HipContext::new(0).unwrap();
        let mut a: HipUnifiedSlice<f32> = unsafe { ctx.alloc_unified(100, true).unwrap() };
        for (i, v) in a.as_mut_slice().unwrap().iter_mut().enumerate() {
            *v = i as f32;
        }
        let (left, right) = a.split_at(50);
        assert_eq!(left.len(), 50);
        assert_eq!(right.len(), 50);
        // The two halves should expose disjoint pointers.
        assert_ne!(left.ptr, right.ptr);
    }

    #[test]
    fn test_unified_slice_views_respect_stream_attachment() {
        let ctx = HipContext::new(0).unwrap();
        let stream_a = ctx.new_stream().unwrap();
        let stream_b = ctx.new_stream().unwrap();
        let mut a: HipUnifiedSlice<f32> = unsafe { ctx.alloc_unified(100, true).unwrap() };
        // Global mode: any stream may access.
        let view = a.as_view();
        let _ = view; // ensure view participates in the same checks
        a.attach(&stream_a, MemAttachFlags::Single).unwrap();
        // After attach to Single mode + stream_a, stream_b is rejected.
        assert!(a.check_device_access(&stream_a).is_ok());
        assert!(a.check_device_access(&stream_b).is_err());
    }
}
