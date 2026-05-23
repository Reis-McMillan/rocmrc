//! Thin `Result`-wrapped hipRTC FFI. Mirror layout: `cudarc::nvrtc::result`.

use super::{HiprtcError, sys};
use std::ffi::CString;

#[inline]
fn check(r: sys::hiprtcResult) -> Result<(), HiprtcError> {
    if r == sys::HIPRTC_SUCCESS {
        Ok(())
    } else {
        Err(HiprtcError::Hiprtc(r))
    }
}

pub fn create_program(src: &str, name: &str) -> Result<sys::hiprtcProgram, HiprtcError> {
    let src_c = CString::new(src).map_err(|_| HiprtcError::NullInSource)?;
    let name_c = CString::new(name).map_err(|_| HiprtcError::NullInSource)?;
    let mut prog: sys::hiprtcProgram = std::ptr::null_mut();
    unsafe {
        check(sys::hiprtcCreateProgram(
            &mut prog,
            src_c.as_ptr(),
            name_c.as_ptr(),
            0,
            std::ptr::null(),
            std::ptr::null(),
        ))?
    };
    Ok(prog)
}

pub fn destroy_program(mut prog: sys::hiprtcProgram) -> Result<(), HiprtcError> {
    unsafe { check(sys::hiprtcDestroyProgram(&mut prog)) }
}

pub fn compile_program(prog: sys::hiprtcProgram, options: &[&str]) -> Result<(), HiprtcError> {
    let c_options: Vec<CString> = options
        .iter()
        .map(|s| CString::new(*s).map_err(|_| HiprtcError::NullInOption))
        .collect::<Result<_, _>>()?;
    let ptrs: Vec<*const std::ffi::c_char> = c_options.iter().map(|c| c.as_ptr()).collect();
    unsafe {
        check(sys::hiprtcCompileProgram(
            prog,
            ptrs.len() as i32,
            ptrs.as_ptr(),
        ))
    }
}

pub fn get_program_log(prog: sys::hiprtcProgram) -> Result<String, HiprtcError> {
    let mut size: usize = 0;
    unsafe { check(sys::hiprtcGetProgramLogSize(prog, &mut size))? };
    if size <= 1 {
        return Ok(String::new());
    }
    let mut buf = vec![0i8; size];
    unsafe { check(sys::hiprtcGetProgramLog(prog, buf.as_mut_ptr()))? };
    // Drop trailing NUL.
    let bytes: Vec<u8> = buf.into_iter().take_while(|b| *b != 0).map(|b| b as u8).collect();
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

pub fn get_code(prog: sys::hiprtcProgram) -> Result<Vec<u8>, HiprtcError> {
    let mut size: usize = 0;
    unsafe { check(sys::hiprtcGetCodeSize(prog, &mut size))? };
    let mut buf = vec![0u8; size];
    unsafe { check(sys::hiprtcGetCode(prog, buf.as_mut_ptr() as *mut i8))? };
    Ok(buf)
}
