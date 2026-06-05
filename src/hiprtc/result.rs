use std::ffi::{CStr, CString, c_char, c_int};

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

pub fn create_program(src: &CStr, name: Option<&CStr>) -> Result<sys::hiprtcProgram, HiprtcError> {
    let mut prog: sys::hiprtcProgram = unsafe { std::mem::zeroed() };
    unsafe {
        sys::hiprtcCreateProgram(
            &mut prog,
            src.as_ptr(),
            name.map(|n| n.as_ptr()).unwrap_or(std::ptr::null()),
            0,
            std::ptr::null(),
            std::ptr::null(),
        ).result()?
    };
    Ok(prog)
}

pub fn compile_program<O: Clone + Into<Vec<u8>>>(
    prog: sys::hiprtcProgram,
    options: &[O]
) -> Result<(), HiprtcError> {
    let c_strings: Vec<CString> = options
        .iter()
        .cloned()
        .map(|o| CString::new(o).unwrap())
        .collect();
    let c_strs: Vec<&CStr> = c_strings.iter().map(CString::as_c_str).collect();
    let opts: Vec<*const c_char> = c_strs.iter().cloned().map(CStr::as_ptr).collect();
    unsafe {
        sys::hiprtcCompileProgram(
            prog,
            opts.len() as c_int,
            opts.as_ptr()
        ).result()
    }
}

pub fn destroy_program(mut prog: sys::hiprtcProgram) -> Result<(), HiprtcError> {
    unsafe{ sys::hiprtcDestroyProgram(&mut prog).result() }
}

pub fn get_hsaco(prog: sys::hiprtcProgram) -> Result<Vec<c_char>, HiprtcError> {
    let mut size: usize = 0;
    unsafe { sys::hiprtcGetCodeSize(prog, &mut size).result()? };

    let mut hsaco_src: Vec<c_char> = Vec::with_capacity(size);
    hsaco_src.resize(size, 0);
    unsafe { sys::hiprtcGetCode(prog, hsaco_src.as_mut_ptr()).result()? };
    Ok(hsaco_src)
}

pub fn get_program_log(prog: sys::hiprtcProgram) -> Result<Vec<c_char>, HiprtcError> {
    let mut size: usize = 0;
    unsafe { sys::hiprtcGetProgramLogSize(prog, &mut size).result()? };

    let mut log_src: Vec<c_char> = Vec::with_capacity(size);
    log_src.resize(size, 0);
    unsafe { sys::hiprtcGetProgramLog(prog, log_src.as_mut_ptr()).result()? };
    Ok(log_src)
}

mod tests {}
