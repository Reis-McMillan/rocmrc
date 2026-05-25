#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::sync::OnceLock;
#[cfg(feature = "dynamic-loading")]
fn load<F: Copy>(name: &str) -> F {
    unsafe { *rocmlib().get::<F>(name.as_bytes()).unwrap_or_else(|e| panic!("Missing symbol {name}: {e}")) }
}
pub type hiprtcLinkState = *mut ihiprtcLinkState;
pub type hiprtcProgram = *mut _hiprtcProgram;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipJitInputType {
    hipJitInputCubin = 0,
    hipJitInputPtx = 1,
    hipJitInputFatBinary = 2,
    hipJitInputObject = 3,
    hipJitInputLibrary = 4,
    hipJitInputNvvm = 5,
    hipJitNumLegacyInputTypes = 6,
    hipJitInputLLVMBitcode = 100,
    hipJitInputLLVMBundledBitcode = 101,
    hipJitInputLLVMArchivesOfBundledBitcode = 102,
    hipJitInputSpirv = 103,
    hipJitNumInputTypes = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipJitOption {
    hipJitOptionMaxRegisters = 0,
    hipJitOptionThreadsPerBlock = 1,
    hipJitOptionWallTime = 2,
    hipJitOptionInfoLogBuffer = 3,
    hipJitOptionInfoLogBufferSizeBytes = 4,
    hipJitOptionErrorLogBuffer = 5,
    hipJitOptionErrorLogBufferSizeBytes = 6,
    hipJitOptionOptimizationLevel = 7,
    hipJitOptionTargetFromContext = 8,
    hipJitOptionTarget = 9,
    hipJitOptionFallbackStrategy = 10,
    hipJitOptionGenerateDebugInfo = 11,
    hipJitOptionLogVerbose = 12,
    hipJitOptionGenerateLineInfo = 13,
    hipJitOptionCacheMode = 14,
    hipJitOptionSm3xOpt = 15,
    hipJitOptionFastCompile = 16,
    hipJitOptionGlobalSymbolNames = 17,
    hipJitOptionGlobalSymbolAddresses = 18,
    hipJitOptionGlobalSymbolCount = 19,
    hipJitOptionLto = 20,
    hipJitOptionFtz = 21,
    hipJitOptionPrecDiv = 22,
    hipJitOptionPrecSqrt = 23,
    hipJitOptionFma = 24,
    hipJitOptionPositionIndependentCode = 25,
    hipJitOptionMinCTAPerSM = 26,
    hipJitOptionMaxThreadsPerBlock = 27,
    hipJitOptionOverrideDirectiveValues = 28,
    hipJitOptionNumOptions = 29,
    hipJitOptionIRtoISAOptExt = 10000,
    hipJitOptionIRtoISAOptCountExt = 10001,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hiprtcResult {
    HIPRTC_SUCCESS = 0,
    HIPRTC_ERROR_OUT_OF_MEMORY = 1,
    HIPRTC_ERROR_PROGRAM_CREATION_FAILURE = 2,
    HIPRTC_ERROR_INVALID_INPUT = 3,
    HIPRTC_ERROR_INVALID_PROGRAM = 4,
    HIPRTC_ERROR_INVALID_OPTION = 5,
    HIPRTC_ERROR_COMPILATION = 6,
    HIPRTC_ERROR_BUILTIN_OPERATION_FAILURE = 7,
    HIPRTC_ERROR_NO_NAME_EXPRESSIONS_AFTER_COMPILATION = 8,
    HIPRTC_ERROR_NO_LOWERED_NAMES_BEFORE_COMPILATION = 9,
    HIPRTC_ERROR_NAME_EXPRESSION_NOT_VALID = 10,
    HIPRTC_ERROR_INTERNAL_ERROR = 11,
    HIPRTC_ERROR_LINKING = 100,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _hiprtcProgram {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihiprtcLinkState {
    _unused: [u8; 0],
}
pub unsafe fn hiprtcAddNameExpression(prog: hiprtcProgram, name_expression: *const ::core::ffi::c_char) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcProgram, *const ::core::ffi::c_char) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcAddNameExpression") });
        unsafe { _f(prog, name_expression) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcAddNameExpression(prog: hiprtcProgram, name_expression: *const ::core::ffi::c_char) -> hiprtcResult;
        }
        unsafe { hiprtcAddNameExpression(prog, name_expression) }
    }
}
pub unsafe fn hiprtcCompileProgram(prog: hiprtcProgram, numOptions: ::core::ffi::c_int, options: *const *const ::core::ffi::c_char) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcProgram, ::core::ffi::c_int, *const *const ::core::ffi::c_char) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcCompileProgram") });
        unsafe { _f(prog, numOptions, options) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcCompileProgram(prog: hiprtcProgram, numOptions: ::core::ffi::c_int, options: *const *const ::core::ffi::c_char) -> hiprtcResult;
        }
        unsafe { hiprtcCompileProgram(prog, numOptions, options) }
    }
}
pub unsafe fn hiprtcCreateProgram(prog: *mut hiprtcProgram, src: *const ::core::ffi::c_char, name: *const ::core::ffi::c_char, numHeaders: ::core::ffi::c_int, headers: *const *const ::core::ffi::c_char, includeNames: *const *const ::core::ffi::c_char) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hiprtcProgram, *const ::core::ffi::c_char, *const ::core::ffi::c_char, ::core::ffi::c_int, *const *const ::core::ffi::c_char, *const *const ::core::ffi::c_char) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcCreateProgram") });
        unsafe { _f(prog, src, name, numHeaders, headers, includeNames) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcCreateProgram(prog: *mut hiprtcProgram, src: *const ::core::ffi::c_char, name: *const ::core::ffi::c_char, numHeaders: ::core::ffi::c_int, headers: *const *const ::core::ffi::c_char, includeNames: *const *const ::core::ffi::c_char) -> hiprtcResult;
        }
        unsafe { hiprtcCreateProgram(prog, src, name, numHeaders, headers, includeNames) }
    }
}
pub unsafe fn hiprtcDestroyProgram(prog: *mut hiprtcProgram) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hiprtcProgram) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcDestroyProgram") });
        unsafe { _f(prog) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcDestroyProgram(prog: *mut hiprtcProgram) -> hiprtcResult;
        }
        unsafe { hiprtcDestroyProgram(prog) }
    }
}
pub unsafe fn hiprtcGetBitcode(prog: hiprtcProgram, bitcode: *mut ::core::ffi::c_char) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcProgram, *mut ::core::ffi::c_char) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcGetBitcode") });
        unsafe { _f(prog, bitcode) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcGetBitcode(prog: hiprtcProgram, bitcode: *mut ::core::ffi::c_char) -> hiprtcResult;
        }
        unsafe { hiprtcGetBitcode(prog, bitcode) }
    }
}
pub unsafe fn hiprtcGetBitcodeSize(prog: hiprtcProgram, bitcode_size: *mut usize) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcProgram, *mut usize) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcGetBitcodeSize") });
        unsafe { _f(prog, bitcode_size) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcGetBitcodeSize(prog: hiprtcProgram, bitcode_size: *mut usize) -> hiprtcResult;
        }
        unsafe { hiprtcGetBitcodeSize(prog, bitcode_size) }
    }
}
pub unsafe fn hiprtcGetCode(prog: hiprtcProgram, code: *mut ::core::ffi::c_char) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcProgram, *mut ::core::ffi::c_char) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcGetCode") });
        unsafe { _f(prog, code) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcGetCode(prog: hiprtcProgram, code: *mut ::core::ffi::c_char) -> hiprtcResult;
        }
        unsafe { hiprtcGetCode(prog, code) }
    }
}
pub unsafe fn hiprtcGetCodeSize(prog: hiprtcProgram, codeSizeRet: *mut usize) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcProgram, *mut usize) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcGetCodeSize") });
        unsafe { _f(prog, codeSizeRet) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcGetCodeSize(prog: hiprtcProgram, codeSizeRet: *mut usize) -> hiprtcResult;
        }
        unsafe { hiprtcGetCodeSize(prog, codeSizeRet) }
    }
}
pub unsafe fn hiprtcGetErrorString(result: hiprtcResult) -> *const ::core::ffi::c_char {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcResult) -> *const ::core::ffi::c_char;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcGetErrorString") });
        unsafe { _f(result) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcGetErrorString(result: hiprtcResult) -> *const ::core::ffi::c_char;
        }
        unsafe { hiprtcGetErrorString(result) }
    }
}
pub unsafe fn hiprtcGetLoweredName(prog: hiprtcProgram, name_expression: *const ::core::ffi::c_char, lowered_name: *mut *const ::core::ffi::c_char) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcProgram, *const ::core::ffi::c_char, *mut *const ::core::ffi::c_char) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcGetLoweredName") });
        unsafe { _f(prog, name_expression, lowered_name) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcGetLoweredName(prog: hiprtcProgram, name_expression: *const ::core::ffi::c_char, lowered_name: *mut *const ::core::ffi::c_char) -> hiprtcResult;
        }
        unsafe { hiprtcGetLoweredName(prog, name_expression, lowered_name) }
    }
}
pub unsafe fn hiprtcGetProgramLog(prog: hiprtcProgram, log: *mut ::core::ffi::c_char) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcProgram, *mut ::core::ffi::c_char) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcGetProgramLog") });
        unsafe { _f(prog, log) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcGetProgramLog(prog: hiprtcProgram, log: *mut ::core::ffi::c_char) -> hiprtcResult;
        }
        unsafe { hiprtcGetProgramLog(prog, log) }
    }
}
pub unsafe fn hiprtcGetProgramLogSize(prog: hiprtcProgram, logSizeRet: *mut usize) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcProgram, *mut usize) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcGetProgramLogSize") });
        unsafe { _f(prog, logSizeRet) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcGetProgramLogSize(prog: hiprtcProgram, logSizeRet: *mut usize) -> hiprtcResult;
        }
        unsafe { hiprtcGetProgramLogSize(prog, logSizeRet) }
    }
}
pub unsafe fn hiprtcLinkAddData(hip_link_state: hiprtcLinkState, input_type: hipJitInputType, image: *mut ::core::ffi::c_void, image_size: usize, name: *const ::core::ffi::c_char, num_options: ::core::ffi::c_uint, options_ptr: *mut hipJitOption, option_values: *mut *mut ::core::ffi::c_void) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcLinkState, hipJitInputType, *mut ::core::ffi::c_void, usize, *const ::core::ffi::c_char, ::core::ffi::c_uint, *mut hipJitOption, *mut *mut ::core::ffi::c_void) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcLinkAddData") });
        unsafe { _f(hip_link_state, input_type, image, image_size, name, num_options, options_ptr, option_values) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcLinkAddData(hip_link_state: hiprtcLinkState, input_type: hipJitInputType, image: *mut ::core::ffi::c_void, image_size: usize, name: *const ::core::ffi::c_char, num_options: ::core::ffi::c_uint, options_ptr: *mut hipJitOption, option_values: *mut *mut ::core::ffi::c_void) -> hiprtcResult;
        }
        unsafe { hiprtcLinkAddData(hip_link_state, input_type, image, image_size, name, num_options, options_ptr, option_values) }
    }
}
pub unsafe fn hiprtcLinkAddFile(hip_link_state: hiprtcLinkState, input_type: hipJitInputType, file_path: *const ::core::ffi::c_char, num_options: ::core::ffi::c_uint, options_ptr: *mut hipJitOption, option_values: *mut *mut ::core::ffi::c_void) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcLinkState, hipJitInputType, *const ::core::ffi::c_char, ::core::ffi::c_uint, *mut hipJitOption, *mut *mut ::core::ffi::c_void) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcLinkAddFile") });
        unsafe { _f(hip_link_state, input_type, file_path, num_options, options_ptr, option_values) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcLinkAddFile(hip_link_state: hiprtcLinkState, input_type: hipJitInputType, file_path: *const ::core::ffi::c_char, num_options: ::core::ffi::c_uint, options_ptr: *mut hipJitOption, option_values: *mut *mut ::core::ffi::c_void) -> hiprtcResult;
        }
        unsafe { hiprtcLinkAddFile(hip_link_state, input_type, file_path, num_options, options_ptr, option_values) }
    }
}
pub unsafe fn hiprtcLinkComplete(hip_link_state: hiprtcLinkState, bin_out: *mut *mut ::core::ffi::c_void, size_out: *mut usize) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcLinkState, *mut *mut ::core::ffi::c_void, *mut usize) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcLinkComplete") });
        unsafe { _f(hip_link_state, bin_out, size_out) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcLinkComplete(hip_link_state: hiprtcLinkState, bin_out: *mut *mut ::core::ffi::c_void, size_out: *mut usize) -> hiprtcResult;
        }
        unsafe { hiprtcLinkComplete(hip_link_state, bin_out, size_out) }
    }
}
pub unsafe fn hiprtcLinkCreate(num_options: ::core::ffi::c_uint, option_ptr: *mut hipJitOption, option_vals_pptr: *mut *mut ::core::ffi::c_void, hip_link_state_ptr: *mut hiprtcLinkState) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_uint, *mut hipJitOption, *mut *mut ::core::ffi::c_void, *mut hiprtcLinkState) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcLinkCreate") });
        unsafe { _f(num_options, option_ptr, option_vals_pptr, hip_link_state_ptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcLinkCreate(num_options: ::core::ffi::c_uint, option_ptr: *mut hipJitOption, option_vals_pptr: *mut *mut ::core::ffi::c_void, hip_link_state_ptr: *mut hiprtcLinkState) -> hiprtcResult;
        }
        unsafe { hiprtcLinkCreate(num_options, option_ptr, option_vals_pptr, hip_link_state_ptr) }
    }
}
pub unsafe fn hiprtcLinkDestroy(hip_link_state: hiprtcLinkState) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hiprtcLinkState) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcLinkDestroy") });
        unsafe { _f(hip_link_state) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcLinkDestroy(hip_link_state: hiprtcLinkState) -> hiprtcResult;
        }
        unsafe { hiprtcLinkDestroy(hip_link_state) }
    }
}
pub unsafe fn hiprtcVersion(major: *mut ::core::ffi::c_int, minor: *mut ::core::ffi::c_int) -> hiprtcResult {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> hiprtcResult;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hiprtcVersion") });
        unsafe { _f(major, minor) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hiprtcVersion(major: *mut ::core::ffi::c_int, minor: *mut ::core::ffi::c_int) -> hiprtcResult;
        }
        unsafe { hiprtcVersion(major, minor) }
    }
}
#[cfg(feature = "dynamic-loading")]
pub unsafe fn is_rocmlib_present() -> bool {
    let lib_names = ["hiprtc"];
    let choices = lib_names.iter().map(|l| crate::get_lib_name_candidates(l)).flatten();
    for choice in choices {
        if ::libloading::Library::new(choice).is_ok() {
            return true;
        }
    }
    false
}
#[cfg(feature = "dynamic-loading")]
pub unsafe fn rocmlib() -> &'static ::libloading::Library {
    static LIB: OnceLock<::libloading::Library> = OnceLock::new();
    LIB.get_or_init(|| {
        let lib_names = std::vec!["hiprtc"];
        let choices: std::vec::Vec<_> = lib_names.iter().map(|l| crate::get_lib_name_candidates(l)).flatten().collect();
        for choice in choices.iter() {
            if let Ok(lib) = ::libloading::Library::new(choice) {
                return lib;
            }
        }
        crate::panic_no_lib_found(lib_names[0], &choices);
    })
}
