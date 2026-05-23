//! Raw FFI for hipRTC. Hand-rolled minimum surface.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::ffi::{c_char, c_int, c_void};

pub type hiprtcResult = c_int;
pub const HIPRTC_SUCCESS: hiprtcResult = 0;

pub type hiprtcProgram = *mut c_void;

unsafe extern "C" {
    pub fn hiprtcCreateProgram(
        prog: *mut hiprtcProgram,
        src: *const c_char,
        name: *const c_char,
        numHeaders: c_int,
        headers: *const *const c_char,
        includeNames: *const *const c_char,
    ) -> hiprtcResult;

    pub fn hiprtcDestroyProgram(prog: *mut hiprtcProgram) -> hiprtcResult;

    pub fn hiprtcCompileProgram(
        prog: hiprtcProgram,
        numOptions: c_int,
        options: *const *const c_char,
    ) -> hiprtcResult;

    pub fn hiprtcGetProgramLogSize(prog: hiprtcProgram, logSizeRet: *mut usize) -> hiprtcResult;
    pub fn hiprtcGetProgramLog(prog: hiprtcProgram, log: *mut c_char) -> hiprtcResult;

    pub fn hiprtcGetCodeSize(prog: hiprtcProgram, codeSizeRet: *mut usize) -> hiprtcResult;
    pub fn hiprtcGetCode(prog: hiprtcProgram, code: *mut c_char) -> hiprtcResult;
}
