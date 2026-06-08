//! Import externally-allocated memory (Vulkan / D3D / file-backed
//! buffers exposed via an OS handle) and map device-accessible buffers
//! over it. Mirrors `cudarc::driver::safe::external_memory`.

use std::fs::File;
use std::mem::ManuallyDrop;
use std::ops::Range;
use std::sync::Arc;

use crate::hip::{result, sys};

use super::{DevicePtr, DeviceSlice, HipContext, HipError, HipEvent, HipStream, SyncOnDrop};

/// An imported external memory object. Destroyed when dropped.
///
/// Created via [`HipContext::import_external_memory`].
#[derive(Debug)]
pub struct ExternalMemory {
    external_memory: sys::hipExternalMemory_t,
    size: u64,
    ctx: Arc<HipContext>,
    _file: ManuallyDrop<File>,
}

impl Drop for ExternalMemory {
    fn drop(&mut self) {
        let ctx = &self.ctx;
        ctx.record_err(ctx.bind_to_thread());
        ctx.record_err(unsafe {
            result::external_memory::destroy_external_memory(self.external_memory)
        });

        // On Unix the FD ownership transferred to HIP at import time, so
        // we must NOT close it ourselves — leave the `ManuallyDrop<File>`
        // un-dropped. On Windows the handle stays the caller's, so drop
        // the `File` and let std close it.
        #[cfg(windows)]
        unsafe {
            ManuallyDrop::<File>::drop(&mut self._file);
        }
    }
}

impl HipContext {
    /// Import an external memory object from a [`File`].
    ///
    /// # Safety
    /// `size` must be the actual size of the external memory object in bytes.
    #[cfg(any(unix, windows))]
    pub unsafe fn import_external_memory(
        self: &Arc<Self>,
        file: File,
        size: u64,
    ) -> Result<ExternalMemory, HipError> {
        self.bind_to_thread()?;

        #[cfg(unix)]
        let external_memory = unsafe {
            use std::os::fd::AsRawFd;
            result::external_memory::import_external_memory_opaque_fd(file.as_raw_fd(), size)
        }?;
        #[cfg(windows)]
        let external_memory = unsafe {
            use std::os::windows::io::AsRawHandle;
            result::external_memory::import_external_memory_opaque_win32(
                file.as_raw_handle(),
                size,
            )
        }?;
        Ok(ExternalMemory {
            external_memory,
            size,
            ctx: self.clone(),
            _file: ManuallyDrop::new(file),
        })
    }
}

impl ExternalMemory {
    /// Map the whole external memory as a single buffer.
    pub fn map_all(self) -> Result<MappedBuffer, HipError> {
        let size = self.size as usize;
        self.map_range(0..size)
    }

    /// Map a byte-range as a buffer.
    ///
    /// Only one mapped buffer is allowed at a time — this method
    /// consumes `self`, transferring ownership into the [`MappedBuffer`].
    /// More restrictive than HIP itself requires, but keeps safety
    /// mechanical.
    ///
    /// # Panics
    /// If `range.start` or `range.end` exceeds the memory object's size.
    pub fn map_range(self, range: Range<usize>) -> Result<MappedBuffer, HipError> {
        assert!(range.start as u64 <= self.size);
        assert!(range.end as u64 <= self.size);
        let device_ptr_u64 = unsafe {
            result::external_memory::get_mapped_buffer(
                self.external_memory,
                range.start as u64,
                range.len() as u64,
            )
        }?;
        let event = self.ctx.new_event(None)?;
        let stream = self.ctx.default_stream();
        Ok(MappedBuffer {
            device_ptr: device_ptr_u64 as sys::hipDeviceptr_t,
            len: range.len(),
            external_memory: self,
            event,
            stream,
        })
    }
}

/// A device-accessible buffer mapped from an [`ExternalMemory`]. Frees
/// the mapping on drop.
#[derive(Debug)]
pub struct MappedBuffer {
    device_ptr: sys::hipDeviceptr_t,
    len: usize,
    #[allow(unused)]
    external_memory: ExternalMemory,
    event: HipEvent,
    stream: Arc<HipStream>,
}

impl Drop for MappedBuffer {
    fn drop(&mut self) {
        let ctx = &self.external_memory.ctx;
        ctx.record_err(ctx.bind_to_thread());
        ctx.record_err(self.stream.wait(&self.event));
        ctx.record_err(result::free_sync(self.device_ptr as u64));
    }
}

impl DeviceSlice<u8> for MappedBuffer {
    fn len(&self) -> usize {
        self.len
    }
    fn stream(&self) -> &Arc<HipStream> {
        &self.stream
    }
}

impl DevicePtr<u8> for MappedBuffer {
    fn device_ptr<'a>(
        &'a self,
        stream: &'a HipStream,
    ) -> (sys::hipDeviceptr_t, SyncOnDrop<'a>) {
        // Read-only mapping (no `DevicePtrMut` impl), so no pre-wait
        // is needed. Still record the read so downstream consumers
        // observe it.
        (
            self.device_ptr,
            SyncOnDrop::Record(Some((&self.event, stream))),
        )
    }
}