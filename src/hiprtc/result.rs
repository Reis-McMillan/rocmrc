//! `Result`-wrapped FFI over hipRTC (`hiprtc.h`), HIP's runtime kernel
//! compiler. Mirrors `cudarc::nvrtc::result`'s shape: program lifecycle
//! (`create` / `compile` / `destroy`) plus code/log retrieval.
//!
//! See the [HIP RTC docs](https://rocm.docs.amd.com/projects/HIP/en/latest/).

use core::ffi::{CStr, c_char, c_int};
use core::mem::MaybeUninit;
use std::ffi::CString;

use super::sys::{self};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HiprtcError(pub sys::hiprtcResult);

impl sys::hiprtcResult {
    pub fn result(self) -> Result<(), HiprtcError> {
        match self {
            sys::hiprtcResult::HIPRTC_SUCCESS => Ok(()),
            _ => Err(HiprtcError(self)),
        }
    }
}

impl std::fmt::Display for HiprtcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for HiprtcError {}

/// Wraps `hiprtcCreateProgram`. See the [hipRTC docs](https://rocm.docs.amd.com/projects/HIP/en/latest/).
pub fn create_program(src: &CStr, name: Option<&CStr>) -> Result<sys::hiprtcProgram, HiprtcError> {
    let mut prog = MaybeUninit::uninit();
    unsafe {
        sys::hiprtcCreateProgram(
            prog.as_mut_ptr(),
            src.as_ptr(),
            name.map(|n| n.as_ptr()).unwrap_or(std::ptr::null()),
            0,
            std::ptr::null(),
            std::ptr::null(),
        )
        .result()?
    };
    Ok(unsafe { prog.assume_init() })
}

/// Wraps `hiprtcCompileProgram`. See the [hipRTC docs](https://rocm.docs.amd.com/projects/HIP/en/latest/).
pub fn compile_program<O: Clone + Into<Vec<u8>>>(
    prog: sys::hiprtcProgram,
    options: &[O],
) -> Result<(), HiprtcError> {
    let c_strings: Vec<CString> = options
        .iter()
        .cloned()
        .map(|o| CString::new(o).unwrap())
        .collect();
    let c_strs: Vec<&CStr> = c_strings.iter().map(CString::as_c_str).collect();
    let opts: Vec<*const c_char> = c_strs.iter().cloned().map(CStr::as_ptr).collect();
    unsafe { sys::hiprtcCompileProgram(prog, opts.len() as c_int, opts.as_ptr()).result() }
}

/// Wraps `hiprtcDestroyProgram`. See the [hipRTC docs](https://rocm.docs.amd.com/projects/HIP/en/latest/).
pub fn destroy_program(mut prog: sys::hiprtcProgram) -> Result<(), HiprtcError> {
    unsafe { sys::hiprtcDestroyProgram(&mut prog).result() }
}

/// Wraps `hiprtcGetCode / hiprtcGetCodeSize`. See the [hipRTC docs](https://rocm.docs.amd.com/projects/HIP/en/latest/).
pub fn get_hsaco(prog: sys::hiprtcProgram) -> Result<Vec<c_char>, HiprtcError> {
    let mut size: usize = 0;
    unsafe { sys::hiprtcGetCodeSize(prog, &mut size).result()? };

    let mut hsaco_src: Vec<c_char> = Vec::with_capacity(size);
    hsaco_src.resize(size, 0);
    unsafe { sys::hiprtcGetCode(prog, hsaco_src.as_mut_ptr()).result()? };
    Ok(hsaco_src)
}

/// Wraps `hiprtcGetProgramLog / hiprtcGetProgramLogSize`. See the [hipRTC docs](https://rocm.docs.amd.com/projects/HIP/en/latest/).
pub fn get_program_log(prog: sys::hiprtcProgram) -> Result<Vec<c_char>, HiprtcError> {
    let mut size: usize = 0;
    unsafe { sys::hiprtcGetProgramLogSize(prog, &mut size).result()? };

    let mut log_src: Vec<c_char> = Vec::with_capacity(size);
    log_src.resize(size, 0);
    unsafe { sys::hiprtcGetProgramLog(prog, log_src.as_mut_ptr()).result()? };
    Ok(log_src)
}

mod tests {}
