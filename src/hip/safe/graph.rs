//! Replay-able HIP graphs via stream capture. Mirrors
//! [`cudarc::driver::safe::graph`].

use std::sync::Arc;

use crate::hip::{result, sys};

use super::{HipError, HipStream};

/// A replay-able HIP graph. Created by capturing a sequence of ops on a
/// stream with [`HipStream::begin_capture`] / [`HipStream::end_capture`],
/// then replayed via [`HipGraph::launch`].
///
/// # Thread safety
///
/// HIP graph objects are **not** internally synchronized. API calls
/// touching the same graph must be serialized externally. From AMD's
/// docs: "Graph objects (`hipGraph_t`, `hipGraphExec_t`) are not
/// internally synchronized and must not be accessed concurrently from
/// multiple threads."
pub struct HipGraph {
    hip_graph: sys::hipGraph_t,
    hip_graph_exec: sys::hipGraphExec_t,
    stream: Arc<HipStream>,
}

impl Drop for HipGraph {
    fn drop(&mut self) {
        let ctx = self.stream.context().clone();

        let exec = std::mem::replace(&mut self.hip_graph_exec, std::ptr::null_mut());
        if !exec.is_null() {
            ctx.record_err(unsafe { result::graph::exec_destroy(exec) });
        }

        let graph = std::mem::replace(&mut self.hip_graph, std::ptr::null_mut());
        if !graph.is_null() {
            ctx.record_err(unsafe { result::graph::destroy(graph) });
        }
    }
}

impl HipStream {
    /// Start capturing operations enqueued on this stream into a graph.
    pub fn begin_capture(
        &self,
        mode: sys::hipStreamCaptureMode,
    ) -> Result<(), HipError> {
        self.context().bind_to_thread()?;
        unsafe { result::stream::begin_capture(self.hip_stream(), mode) }
    }

    /// Finish capture and instantiate the resulting graph. Returns
    /// `Ok(None)` if capture had been invalidated (for example because
    /// an error occurred during the captured region).
    ///
    /// `flags` is forwarded to `hipGraphInstantiateWithFlags`.
    pub fn end_capture(
        self: &Arc<Self>,
        flags: sys::hipGraphInstantiateFlags,
    ) -> Result<Option<HipGraph>, HipError> {
        self.context().bind_to_thread()?;
        let graph = unsafe { result::stream::end_capture(self.hip_stream()) }?;
        if graph.is_null() {
            return Ok(None);
        }
        let exec = unsafe { result::graph::instantiate(graph, flags) }?;
        Ok(Some(HipGraph {
            hip_graph: graph,
            hip_graph_exec: exec,
            stream: self.clone(),
        }))
    }

    /// Query whether this stream is currently capturing.
    pub fn capture_status(&self) -> Result<sys::hipStreamCaptureStatus, HipError> {
        self.context().bind_to_thread()?;
        unsafe { result::stream::is_capturing(self.hip_stream()) }
    }
}

impl HipGraph {
    /// Replay the captured graph on its owning stream.
    pub fn launch(&self) -> Result<(), HipError> {
        self.stream.context().bind_to_thread()?;
        unsafe { result::graph::launch(self.hip_graph_exec, self.stream.hip_stream()) }
    }

    /// Pre-upload the graph's resources to the device so the first
    /// [`Self::launch`] doesn't incur setup overhead.
    pub fn upload(&self) -> Result<(), HipError> {
        self.stream.context().bind_to_thread()?;
        unsafe { result::graph::upload(self.hip_graph_exec, self.stream.hip_stream()) }
    }

    /// Raw `hipGraph_t`. Do not destroy.
    pub fn hip_graph(&self) -> sys::hipGraph_t {
        self.hip_graph
    }

    /// Raw `hipGraphExec_t`. Do not destroy.
    pub fn hip_graph_exec(&self) -> sys::hipGraphExec_t {
        self.hip_graph_exec
    }
}
