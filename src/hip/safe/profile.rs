//! Toggle HIP profile collection from within a process. Mirrors
//! [`cudarc::driver::safe::profile`]. Pair with `rocprof` (or any tool
//! consuming `hipProfilerStart` / `hipProfilerStop`) to bracket the hot
//! path you want recorded.

use crate::hip::{result::HipError, sys};

/// RAII profiler scope. Calls [`profiler_start`] in [`Profiler::new`] and
/// [`profiler_stop`] on drop.
///
/// ```no_run
/// use rocmrc::Profiler;
///
/// # fn run() -> Result<(), rocmrc::HipError> {
/// {
///     let _profiler = Profiler::new()?;
///     // Hot path — profiler stops automatically on drop.
/// }
/// # Ok(())
/// # }
/// // Then collect:
/// //   rocprof --hip-trace /path/to/bin
/// ```
#[derive(Default)]
pub struct Profiler;

impl Profiler {
    /// Enable profile collection for the current context. If profiling
    /// is already on, this is a no-op.
    pub fn new() -> Result<Self, HipError> {
        profiler_start()?;
        Ok(Self)
    }
}

impl Drop for Profiler {
    fn drop(&mut self) {
        // Swallow errors — `Drop` can't propagate them, and we don't want
        // to panic during cleanup.
        profiler_stop().ok();
    }
}

/// Enable profile collection by the active profiling tool for the
/// current context. If profiling is already on, this is a no-op. For an
/// RAII wrapper see [`Profiler::new`].
///
/// ```no_run
/// use rocmrc::{profiler_start, profiler_stop};
///
/// # fn run() -> Result<(), rocmrc::HipError> {
/// profiler_start()?;
/// // Hot path
/// profiler_stop()?;
/// # Ok(())
/// # }
/// ```
pub fn profiler_start() -> Result<(), HipError> {
    unsafe { sys::hipProfilerStart() }.result()
}

/// Disable profile collection for the current context. If profiling is
/// already off, this is a no-op.
pub fn profiler_stop() -> Result<(), HipError> {
    unsafe { sys::hipProfilerStop() }.result()
}
