#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::sync::OnceLock;
#[cfg(feature = "dynamic-loading")]
fn load<F: Copy>(name: &str) -> F {
    unsafe { *rocmlib().get::<F>(name.as_bytes()).unwrap_or_else(|e| panic!("Missing symbol {name}: {e}")) }
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024", feature = "rocm-06033", feature = "rocm-06043"))]
pub const HIPBLASLT_VERSION_MAJOR: u32 = 0;
#[cfg(any(feature = "rocm-07002", feature = "rocm-07011", feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
pub const HIPBLASLT_VERSION_MAJOR: u32 = 1;
#[cfg(any(feature = "rocm-06002"))]
pub const HIPBLASLT_VERSION_MINOR: u32 = 6;
#[cfg(any(feature = "rocm-06015"))]
pub const HIPBLASLT_VERSION_MINOR: u32 = 7;
#[cfg(any(feature = "rocm-06024"))]
pub const HIPBLASLT_VERSION_MINOR: u32 = 8;
#[cfg(any(feature = "rocm-06033"))]
pub const HIPBLASLT_VERSION_MINOR: u32 = 10;
#[cfg(any(feature = "rocm-06043"))]
pub const HIPBLASLT_VERSION_MINOR: u32 = 12;
#[cfg(any(feature = "rocm-07002"))]
pub const HIPBLASLT_VERSION_MINOR: u32 = 0;
#[cfg(any(feature = "rocm-07011"))]
pub const HIPBLASLT_VERSION_MINOR: u32 = 1;
#[cfg(any(feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
pub const HIPBLASLT_VERSION_MINOR: u32 = 2;
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024", feature = "rocm-06033"))]
pub const HIPBLASLT_VERSION_PATCH: u32 = 0;
#[cfg(any(feature = "rocm-06043"))]
pub const HIPBLASLT_VERSION_PATCH: u32 = 1;
#[cfg(any(feature = "rocm-07002", feature = "rocm-07011"))]
pub const HIPBLASLT_VERSION_PATCH: u32 = 0;
#[cfg(any(feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
pub const HIPBLASLT_VERSION_PATCH: u32 = 2;
#[cfg(any(feature = "rocm-06002"))]
pub const HIPBLASLT_VERSION_TWEAK: f64 = 5925180000000.0;
#[cfg(any(feature = "rocm-06015"))]
pub const HIPBLASLT_VERSION_TWEAK: f64 = f64::INFINITY;
pub type hipStream_t = *mut ihipStream_t;
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
pub type hipblasHalf = u16;
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
pub type hipblasHandle_t = *mut ::core::ffi::c_void;
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
pub type hipblasInt8 = i8;
pub type hipblasLtBfloat16 = hip_bfloat16;
pub type hipblasLtFloat = f32;
pub type hipblasLtHalf = _hipblasLtHalf;
pub type hipblasLtHandle_t = *mut ::core::ffi::c_void;
pub type hipblasLtInt32 = i32;
pub type hipblasLtInt8 = i8;
pub type hipblasLtMatmulAlgo_t = _hipblasLtMatmulAlgo_t;
pub type hipblasLtMatmulDesc_t = *mut hipblasLtMatmulDescOpaque_t;
pub type hipblasLtMatmulHeuristicResult_t = _hipblasLtMatmulHeuristicResult_t;
pub type hipblasLtMatmulPreference_t = *mut hipblasLtMatmulPreferenceOpaque_t;
pub type hipblasLtMatrixLayout_t = *mut hipblasLtMatrixLayoutOpaque_t;
pub type hipblasLtMatrixTransformDesc_t = *mut hipblasLtMatrixTransformDescOpaque_t;
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
pub type hipblasStride = i64;
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipDataType {
    HIP_R_32F = 0,
    HIP_R_64F = 1,
    HIP_R_16F = 2,
    HIP_R_8I = 3,
    HIP_C_32F = 4,
    HIP_C_64F = 5,
    HIP_C_16F = 6,
    HIP_C_8I = 7,
    HIP_R_8U = 8,
    HIP_C_8U = 9,
    HIP_R_32I = 10,
    HIP_C_32I = 11,
    HIP_R_32U = 12,
    HIP_C_32U = 13,
    HIP_R_16BF = 14,
    HIP_C_16BF = 15,
    HIP_R_4I = 16,
    HIP_C_4I = 17,
    HIP_R_4U = 18,
    HIP_C_4U = 19,
    HIP_R_16I = 20,
    HIP_C_16I = 21,
    HIP_R_16U = 22,
    HIP_C_16U = 23,
    HIP_R_64I = 24,
    HIP_C_64I = 25,
    HIP_R_64U = 26,
    HIP_C_64U = 27,
    HIP_R_8F_E4M3_FNUZ = 1000,
    HIP_R_8F_E5M2_FNUZ = 1001,
}
#[cfg(any(feature = "rocm-06033", feature = "rocm-06043"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipDataType {
    HIP_R_32F = 0,
    HIP_R_64F = 1,
    HIP_R_16F = 2,
    HIP_R_8I = 3,
    HIP_C_32F = 4,
    HIP_C_64F = 5,
    HIP_C_16F = 6,
    HIP_C_8I = 7,
    HIP_R_8U = 8,
    HIP_C_8U = 9,
    HIP_R_32I = 10,
    HIP_C_32I = 11,
    HIP_R_32U = 12,
    HIP_C_32U = 13,
    HIP_R_16BF = 14,
    HIP_C_16BF = 15,
    HIP_R_4I = 16,
    HIP_C_4I = 17,
    HIP_R_4U = 18,
    HIP_C_4U = 19,
    HIP_R_16I = 20,
    HIP_C_16I = 21,
    HIP_R_16U = 22,
    HIP_C_16U = 23,
    HIP_R_64I = 24,
    HIP_C_64I = 25,
    HIP_R_64U = 26,
    HIP_C_64U = 27,
    HIP_R_8F_E4M3 = 28,
    HIP_R_8F_E5M2 = 29,
    HIP_R_8F_E4M3_FNUZ = 1000,
    HIP_R_8F_E5M2_FNUZ = 1001,
}
#[cfg(any(feature = "rocm-07002", feature = "rocm-07011", feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipDataType {
    HIP_R_32F = 0,
    HIP_R_64F = 1,
    HIP_R_16F = 2,
    HIP_R_8I = 3,
    HIP_C_32F = 4,
    HIP_C_64F = 5,
    HIP_C_16F = 6,
    HIP_C_8I = 7,
    HIP_R_8U = 8,
    HIP_C_8U = 9,
    HIP_R_32I = 10,
    HIP_C_32I = 11,
    HIP_R_32U = 12,
    HIP_C_32U = 13,
    HIP_R_16BF = 14,
    HIP_C_16BF = 15,
    HIP_R_4I = 16,
    HIP_C_4I = 17,
    HIP_R_4U = 18,
    HIP_C_4U = 19,
    HIP_R_16I = 20,
    HIP_C_16I = 21,
    HIP_R_16U = 22,
    HIP_C_16U = 23,
    HIP_R_64I = 24,
    HIP_C_64I = 25,
    HIP_R_64U = 26,
    HIP_C_64U = 27,
    HIP_R_8F_E4M3 = 28,
    HIP_R_8F_E5M2 = 29,
    HIP_R_8F_UE8M0 = 30,
    HIP_R_6F_E2M3 = 31,
    HIP_R_6F_E3M2 = 32,
    HIP_R_4F_E2M1 = 33,
    HIP_R_8F_E4M3_FNUZ = 1000,
    HIP_R_8F_E5M2_FNUZ = 1001,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasAtomicsMode_t {
    HIPBLAS_ATOMICS_NOT_ALLOWED = 0,
    HIPBLAS_ATOMICS_ALLOWED = 1,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024", feature = "rocm-06033", feature = "rocm-06043"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasComputeType_t {
    HIPBLAS_COMPUTE_16F = 0,
    HIPBLAS_COMPUTE_16F_PEDANTIC = 1,
    HIPBLAS_COMPUTE_32F = 2,
    HIPBLAS_COMPUTE_32F_PEDANTIC = 3,
    HIPBLAS_COMPUTE_32F_FAST_16F = 4,
    HIPBLAS_COMPUTE_32F_FAST_16BF = 5,
    HIPBLAS_COMPUTE_32F_FAST_TF32 = 6,
    HIPBLAS_COMPUTE_64F = 7,
    HIPBLAS_COMPUTE_64F_PEDANTIC = 8,
    HIPBLAS_COMPUTE_32I = 9,
    HIPBLAS_COMPUTE_32I_PEDANTIC = 10,
}
#[cfg(any(feature = "rocm-07002", feature = "rocm-07011", feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasComputeType_t {
    HIPBLAS_COMPUTE_16F = 0,
    HIPBLAS_COMPUTE_16F_PEDANTIC = 1,
    HIPBLAS_COMPUTE_32F = 2,
    HIPBLAS_COMPUTE_32F_PEDANTIC = 3,
    HIPBLAS_COMPUTE_32F_FAST_16F = 4,
    HIPBLAS_COMPUTE_32F_FAST_16BF = 5,
    HIPBLAS_COMPUTE_32F_FAST_TF32 = 6,
    HIPBLAS_COMPUTE_64F = 7,
    HIPBLAS_COMPUTE_64F_PEDANTIC = 8,
    HIPBLAS_COMPUTE_32I = 9,
    HIPBLAS_COMPUTE_32I_PEDANTIC = 10,
    HIPBLAS_COMPUTE_32F_FAST_8F_FNUZ = 100,
    HIPBLAS_COMPUTE_32F_FAST_8BF_FNUZ = 101,
    HIPBLAS_COMPUTE_32F_FAST_8F8BF_FNUZ = 102,
    HIPBLAS_COMPUTE_32F_FAST_8BF8F_FNUZ = 103,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasDatatype_t {
    HIPBLAS_R_16F = 150,
    HIPBLAS_R_32F = 151,
    HIPBLAS_R_64F = 152,
    HIPBLAS_C_16F = 153,
    HIPBLAS_C_32F = 154,
    HIPBLAS_C_64F = 155,
    HIPBLAS_R_8I = 160,
    HIPBLAS_R_8U = 161,
    HIPBLAS_R_32I = 162,
    HIPBLAS_R_32U = 163,
    HIPBLAS_C_8I = 164,
    HIPBLAS_C_8U = 165,
    HIPBLAS_C_32I = 166,
    HIPBLAS_C_32U = 167,
    HIPBLAS_R_16B = 168,
    HIPBLAS_C_16B = 169,
    HIPBLAS_DATATYPE_INVALID = 255,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasDiagType_t {
    HIPBLAS_DIAG_NON_UNIT = 131,
    HIPBLAS_DIAG_UNIT = 132,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasFillMode_t {
    HIPBLAS_FILL_MODE_UPPER = 121,
    HIPBLAS_FILL_MODE_LOWER = 122,
    HIPBLAS_FILL_MODE_FULL = 123,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasGemmAlgo_t {
    HIPBLAS_GEMM_DEFAULT = 160,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasGemmFlags_t {
    HIPBLAS_GEMM_FLAGS_NONE = 0,
    HIPBLAS_GEMM_FLAGS_USE_CU_EFFICIENCY = 2,
    HIPBLAS_GEMM_FLAGS_FP16_ALT_IMPL = 4,
    HIPBLAS_GEMM_FLAGS_CHECK_SOLUTION_INDEX = 8,
    HIPBLAS_GEMM_FLAGS_FP16_ALT_IMPL_RNZ = 16,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024", feature = "rocm-06033", feature = "rocm-06043"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtEpilogue_t {
    HIPBLASLT_EPILOGUE_DEFAULT = 1,
    HIPBLASLT_EPILOGUE_RELU = 2,
    HIPBLASLT_EPILOGUE_BIAS = 4,
    HIPBLASLT_EPILOGUE_RELU_BIAS = 6,
    HIPBLASLT_EPILOGUE_GELU = 32,
    HIPBLASLT_EPILOGUE_GELU_BIAS = 36,
    HIPBLASLT_EPILOGUE_GELU_AUX = 160,
    HIPBLASLT_EPILOGUE_GELU_AUX_BIAS = 164,
    HIPBLASLT_EPILOGUE_DGELU = 192,
    HIPBLASLT_EPILOGUE_DGELU_BGRAD = 208,
    HIPBLASLT_EPILOGUE_BGRADA = 256,
    HIPBLASLT_EPILOGUE_BGRADB = 512,
}
#[cfg(any(feature = "rocm-07002"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtEpilogue_t {
    HIPBLASLT_EPILOGUE_DEFAULT = 1,
    HIPBLASLT_EPILOGUE_RELU = 2,
    HIPBLASLT_EPILOGUE_BIAS = 4,
    HIPBLASLT_EPILOGUE_RELU_BIAS = 6,
    HIPBLASLT_EPILOGUE_GELU = 32,
    HIPBLASLT_EPILOGUE_GELU_BIAS = 36,
    HIPBLASLT_EPILOGUE_GELU_AUX = 160,
    HIPBLASLT_EPILOGUE_GELU_AUX_BIAS = 164,
    HIPBLASLT_EPILOGUE_DGELU = 192,
    HIPBLASLT_EPILOGUE_DGELU_BGRAD = 208,
    HIPBLASLT_EPILOGUE_BGRADA = 256,
    HIPBLASLT_EPILOGUE_BGRADB = 512,
    HIPBLASLT_EPILOGUE_SWISH_EXT = 65536,
    HIPBLASLT_EPILOGUE_SWISH_BIAS_EXT = 65540,
}
#[cfg(any(feature = "rocm-07011"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtEpilogue_t {
    HIPBLASLT_EPILOGUE_DEFAULT = 1,
    HIPBLASLT_EPILOGUE_RELU = 2,
    HIPBLASLT_EPILOGUE_BIAS = 4,
    HIPBLASLT_EPILOGUE_RELU_BIAS = 6,
    HIPBLASLT_EPILOGUE_GELU = 32,
    HIPBLASLT_EPILOGUE_GELU_BIAS = 36,
    HIPBLASLT_EPILOGUE_RELU_AUX = 130,
    HIPBLASLT_EPILOGUE_RELU_AUX_BIAS = 134,
    HIPBLASLT_EPILOGUE_GELU_AUX = 160,
    HIPBLASLT_EPILOGUE_GELU_AUX_BIAS = 164,
    HIPBLASLT_EPILOGUE_DGELU = 192,
    HIPBLASLT_EPILOGUE_DGELU_BGRAD = 208,
    HIPBLASLT_EPILOGUE_BGRADA = 256,
    HIPBLASLT_EPILOGUE_BGRADB = 512,
    HIPBLASLT_EPILOGUE_SWISH_EXT = 65536,
    HIPBLASLT_EPILOGUE_SWISH_BIAS_EXT = 65540,
    HIPBLASLT_EPILOGUE_CLAMP_EXT = 131072,
    HIPBLASLT_EPILOGUE_CLAMP_BIAS_EXT = 131076,
    HIPBLASLT_EPILOGUE_CLAMP_AUX_EXT = 131200,
    HIPBLASLT_EPILOGUE_CLAMP_AUX_BIAS_EXT = 131204,
}
#[cfg(any(feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtEpilogue_t {
    HIPBLASLT_EPILOGUE_DEFAULT = 1,
    HIPBLASLT_EPILOGUE_RELU = 2,
    HIPBLASLT_EPILOGUE_BIAS = 4,
    HIPBLASLT_EPILOGUE_RELU_BIAS = 6,
    HIPBLASLT_EPILOGUE_GELU = 32,
    HIPBLASLT_EPILOGUE_GELU_BIAS = 36,
    HIPBLASLT_EPILOGUE_RELU_AUX = 130,
    HIPBLASLT_EPILOGUE_RELU_AUX_BIAS = 134,
    HIPBLASLT_EPILOGUE_GELU_AUX = 160,
    HIPBLASLT_EPILOGUE_GELU_AUX_BIAS = 164,
    HIPBLASLT_EPILOGUE_DGELU = 192,
    HIPBLASLT_EPILOGUE_DGELU_BGRAD = 208,
    HIPBLASLT_EPILOGUE_BGRADA = 256,
    HIPBLASLT_EPILOGUE_BGRADB = 512,
    HIPBLASLT_EPILOGUE_SWISH_EXT = 65536,
    HIPBLASLT_EPILOGUE_SWISH_BIAS_EXT = 65540,
    HIPBLASLT_EPILOGUE_CLAMP_EXT = 131072,
    HIPBLASLT_EPILOGUE_CLAMP_BIAS_EXT = 131076,
    HIPBLASLT_EPILOGUE_CLAMP_AUX_EXT = 131200,
    HIPBLASLT_EPILOGUE_CLAMP_AUX_BIAS_EXT = 131204,
    HIPBLASLT_EPILOGUE_SIGMOID_EXT = 262144,
    HIPBLASLT_EPILOGUE_SIGMOID_BIAS_EXT = 262148,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtMatmulDescAttributes_t {
    HIPBLASLT_MATMUL_DESC_TRANSA = 0,
    HIPBLASLT_MATMUL_DESC_TRANSB = 1,
    HIPBLASLT_MATMUL_DESC_EPILOGUE = 2,
    HIPBLASLT_MATMUL_DESC_BIAS_POINTER = 3,
    HIPBLASLT_MATMUL_DESC_BIAS_DATA_TYPE = 4,
    HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER = 5,
    HIPBLASLT_MATMUL_DESC_B_SCALE_POINTER = 6,
    HIPBLASLT_MATMUL_DESC_C_SCALE_POINTER = 7,
    HIPBLASLT_MATMUL_DESC_D_SCALE_POINTER = 8,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER = 9,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER = 10,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD = 11,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE = 12,
    HIPBLASLT_MATMUL_DESC_POINTER_MODE = 13,
    HIPBLASLT_MATMUL_DESC_MAX = 101,
}
#[cfg(any(feature = "rocm-06024"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtMatmulDescAttributes_t {
    HIPBLASLT_MATMUL_DESC_TRANSA = 0,
    HIPBLASLT_MATMUL_DESC_TRANSB = 1,
    HIPBLASLT_MATMUL_DESC_EPILOGUE = 2,
    HIPBLASLT_MATMUL_DESC_BIAS_POINTER = 3,
    HIPBLASLT_MATMUL_DESC_BIAS_DATA_TYPE = 4,
    HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER = 5,
    HIPBLASLT_MATMUL_DESC_B_SCALE_POINTER = 6,
    HIPBLASLT_MATMUL_DESC_C_SCALE_POINTER = 7,
    HIPBLASLT_MATMUL_DESC_D_SCALE_POINTER = 8,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER = 9,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER = 10,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD = 11,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE = 12,
    HIPBLASLT_MATMUL_DESC_POINTER_MODE = 13,
    HIPBLASLT_MATMUL_DESC_AMAX_D_POINTER = 14,
    HIPBLASLT_MATMUL_DESC_COMPUTE_INPUT_TYPE_A_EXT = 100,
    HIPBLASLT_MATMUL_DESC_COMPUTE_INPUT_TYPE_B_EXT = 101,
    HIPBLASLT_MATMUL_DESC_MAX = 102,
}
#[cfg(any(feature = "rocm-06033", feature = "rocm-06043"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtMatmulDescAttributes_t {
    HIPBLASLT_MATMUL_DESC_TRANSA = 0,
    HIPBLASLT_MATMUL_DESC_TRANSB = 1,
    HIPBLASLT_MATMUL_DESC_EPILOGUE = 2,
    HIPBLASLT_MATMUL_DESC_BIAS_POINTER = 3,
    HIPBLASLT_MATMUL_DESC_BIAS_DATA_TYPE = 4,
    HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER = 5,
    HIPBLASLT_MATMUL_DESC_B_SCALE_POINTER = 6,
    HIPBLASLT_MATMUL_DESC_C_SCALE_POINTER = 7,
    HIPBLASLT_MATMUL_DESC_D_SCALE_POINTER = 8,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER = 9,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER = 10,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD = 11,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE = 12,
    HIPBLASLT_MATMUL_DESC_POINTER_MODE = 13,
    HIPBLASLT_MATMUL_DESC_AMAX_D_POINTER = 14,
    HIPBLASLT_MATMUL_DESC_COMPUTE_INPUT_TYPE_A_EXT = 100,
    HIPBLASLT_MATMUL_DESC_COMPUTE_INPUT_TYPE_B_EXT = 101,
    HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER_VEC_EXT = 102,
    HIPBLASLT_MATMUL_DESC_B_SCALE_POINTER_VEC_EXT = 103,
    HIPBLASLT_MATMUL_DESC_MAX = 104,
}
#[cfg(any(feature = "rocm-07002"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtMatmulDescAttributes_t {
    HIPBLASLT_MATMUL_DESC_TRANSA = 0,
    HIPBLASLT_MATMUL_DESC_TRANSB = 1,
    HIPBLASLT_MATMUL_DESC_EPILOGUE = 2,
    HIPBLASLT_MATMUL_DESC_BIAS_POINTER = 3,
    HIPBLASLT_MATMUL_DESC_BIAS_DATA_TYPE = 4,
    HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER = 5,
    HIPBLASLT_MATMUL_DESC_B_SCALE_POINTER = 6,
    HIPBLASLT_MATMUL_DESC_C_SCALE_POINTER = 7,
    HIPBLASLT_MATMUL_DESC_D_SCALE_POINTER = 8,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER = 9,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER = 10,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD = 11,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE = 12,
    HIPBLASLT_MATMUL_DESC_POINTER_MODE = 13,
    HIPBLASLT_MATMUL_DESC_AMAX_D_POINTER = 14,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_DATA_TYPE = 22,
    HIPBLASLT_MATMUL_DESC_A_SCALE_MODE = 31,
    HIPBLASLT_MATMUL_DESC_B_SCALE_MODE = 32,
    HIPBLASLT_MATMUL_DESC_COMPUTE_INPUT_TYPE_A_EXT = 100,
    HIPBLASLT_MATMUL_DESC_COMPUTE_INPUT_TYPE_B_EXT = 101,
    HIPBLASLT_MATMUL_DESC_MAX = 102,
}
#[cfg(any(feature = "rocm-07011", feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtMatmulDescAttributes_t {
    HIPBLASLT_MATMUL_DESC_TRANSA = 0,
    HIPBLASLT_MATMUL_DESC_TRANSB = 1,
    HIPBLASLT_MATMUL_DESC_EPILOGUE = 2,
    HIPBLASLT_MATMUL_DESC_BIAS_POINTER = 3,
    HIPBLASLT_MATMUL_DESC_BIAS_DATA_TYPE = 4,
    HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER = 5,
    HIPBLASLT_MATMUL_DESC_B_SCALE_POINTER = 6,
    HIPBLASLT_MATMUL_DESC_C_SCALE_POINTER = 7,
    HIPBLASLT_MATMUL_DESC_D_SCALE_POINTER = 8,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER = 9,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER = 10,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD = 11,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE = 12,
    HIPBLASLT_MATMUL_DESC_POINTER_MODE = 13,
    HIPBLASLT_MATMUL_DESC_AMAX_D_POINTER = 14,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_DATA_TYPE = 22,
    HIPBLASLT_MATMUL_DESC_A_SCALE_MODE = 31,
    HIPBLASLT_MATMUL_DESC_B_SCALE_MODE = 32,
    HIPBLASLT_MATMUL_DESC_COMPUTE_INPUT_TYPE_A_EXT = 100,
    HIPBLASLT_MATMUL_DESC_COMPUTE_INPUT_TYPE_B_EXT = 101,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_ACT_ARG0_EXT = 102,
    HIPBLASLT_MATMUL_DESC_EPILOGUE_ACT_ARG1_EXT = 103,
    HIPBLASLT_MATMUL_DESC_MAX = 104,
}
#[cfg(any(feature = "rocm-07002", feature = "rocm-07011", feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtMatmulMatrixScale_t {
    HIPBLASLT_MATMUL_MATRIX_SCALE_SCALAR_32F = 0,
    HIPBLASLT_MATMUL_MATRIX_SCALE_VEC16_UE4M3 = 1,
    HIPBLASLT_MATMUL_MATRIX_SCALE_VEC32_UE8M0 = 2,
    HIPBLASLT_MATMUL_MATRIX_SCALE_OUTER_VEC_32F = 3,
    HIPBLASLT_MATMUL_MATRIX_SCALE_VEC128_32F = 4,
    HIPBLASLT_MATMUL_MATRIX_SCALE_BLK128x128_32F = 5,
    HIPBLASLT_MATMUL_MATRIX_SCALE_END = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtMatmulPreferenceAttributes_t {
    HIPBLASLT_MATMUL_PREF_SEARCH_MODE = 0,
    HIPBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES = 1,
    HIPBLASLT_MATMUL_PREF_MAX = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtMatrixLayoutAttribute_t {
    HIPBLASLT_MATRIX_LAYOUT_BATCH_COUNT = 0,
    HIPBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET = 1,
    HIPBLASLT_MATRIX_LAYOUT_TYPE = 2,
    HIPBLASLT_MATRIX_LAYOUT_ORDER = 3,
    HIPBLASLT_MATRIX_LAYOUT_ROWS = 4,
    HIPBLASLT_MATRIX_LAYOUT_COLS = 5,
    HIPBLASLT_MATRIX_LAYOUT_LD = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtMatrixTransformDescAttributes_t {
    HIPBLASLT_MATRIX_TRANSFORM_DESC_SCALE_TYPE = 0,
    HIPBLASLT_MATRIX_TRANSFORM_DESC_POINTER_MODE = 1,
    HIPBLASLT_MATRIX_TRANSFORM_DESC_TRANSA = 2,
    HIPBLASLT_MATRIX_TRANSFORM_DESC_TRANSB = 3,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024", feature = "rocm-06033", feature = "rocm-06043"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtOrder_t {
    HIPBLASLT_ORDER_COL = 0,
    HIPBLASLT_ORDER_ROW = 1,
}
#[cfg(any(feature = "rocm-07002", feature = "rocm-07011", feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtOrder_t {
    HIPBLASLT_ORDER_COL = 0,
    HIPBLASLT_ORDER_ROW = 1,
    HIPBLASLT_ORDER_COL16_4R16 = 100,
    HIPBLASLT_ORDER_COL16_4R8 = 101,
    HIPBLASLT_ORDER_COL16_4R4 = 102,
    HIPBLASLT_ORDER_COL16_4R2 = 103,
}
#[cfg(any(feature = "rocm-06002"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtPointerMode_t {
    HIPBLASLT_POINTER_MODE_HOST = 0,
    HIPBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_HOST = 1,
}
#[cfg(any(feature = "rocm-06015", feature = "rocm-06024", feature = "rocm-06033", feature = "rocm-06043", feature = "rocm-07002", feature = "rocm-07011", feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasLtPointerMode_t {
    HIPBLASLT_POINTER_MODE_HOST = 0,
    HIPBLASLT_POINTER_MODE_DEVICE = 1,
    HIPBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_HOST = 4,
}
#[cfg(any(feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasMath_t {
    HIPBLAS_DEFAULT_MATH = 0,
    HIPBLAS_XF32_XDL_MATH = 1,
    HIPBLAS_PEDANTIC_MATH = 2,
    HIPBLAS_TF32_TENSOR_OP_MATH = 3,
    HIPBLAS_MATH_DISALLOW_REDUCED_PRECISION_REDUCTION = 4,
    HIPBLAS_TENSOR_OP_MATH = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasOperation_t {
    HIPBLAS_OP_N = 111,
    HIPBLAS_OP_T = 112,
    HIPBLAS_OP_C = 113,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasPointerMode_t {
    HIPBLAS_POINTER_MODE_HOST = 0,
    HIPBLAS_POINTER_MODE_DEVICE = 1,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasSideMode_t {
    HIPBLAS_SIDE_LEFT = 141,
    HIPBLAS_SIDE_RIGHT = 142,
    HIPBLAS_SIDE_BOTH = 143,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipblasStatus_t {
    HIPBLAS_STATUS_SUCCESS = 0,
    HIPBLAS_STATUS_NOT_INITIALIZED = 1,
    HIPBLAS_STATUS_ALLOC_FAILED = 2,
    HIPBLAS_STATUS_INVALID_VALUE = 3,
    HIPBLAS_STATUS_MAPPING_ERROR = 4,
    HIPBLAS_STATUS_EXECUTION_FAILED = 5,
    HIPBLAS_STATUS_INTERNAL_ERROR = 6,
    HIPBLAS_STATUS_NOT_SUPPORTED = 7,
    HIPBLAS_STATUS_ARCH_MISMATCH = 8,
    HIPBLAS_STATUS_HANDLE_IS_NULLPTR = 9,
    HIPBLAS_STATUS_INVALID_ENUM = 10,
    HIPBLAS_STATUS_UNKNOWN = 11,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _hipblasLtHalf {
    pub data: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _hipblasLtMatmulAlgo_t {
    pub data: [u8; 16usize],
    pub max_workspace_bytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct _hipblasLtMatmulHeuristicResult_t {
    pub algo: hipblasLtMatmulAlgo_t,
    pub workspaceSize: usize,
    pub state: hipblasStatus_t,
    pub wavesCount: f32,
    pub reserved: [::core::ffi::c_int; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hip_bfloat16 {
    pub data: u16,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblasBfloat16 {
    pub data: u16,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct hipblasComplex {
    pub x: f32,
    pub y: f32,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct hipblasDoubleComplex {
    pub x: f64,
    pub y: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblasLtMatmulDescOpaque_t {
    pub data: [u64; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblasLtMatmulPreferenceOpaque_t {
    pub data: [u64; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblasLtMatrixLayoutOpaque_t {
    pub data: [u64; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblasLtMatrixTransformDescOpaque_t {
    pub data: [u64; 8usize],
}
#[cfg(any(feature = "rocm-06033", feature = "rocm-06043"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblaslt_bf8 {
    pub data: u8,
}
#[cfg(any(feature = "rocm-07002", feature = "rocm-07011", feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblaslt_bf8 {
    pub __x: u8,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024", feature = "rocm-06033", feature = "rocm-06043"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblaslt_bf8_fnuz {
    pub data: u8,
}
#[cfg(any(feature = "rocm-07002", feature = "rocm-07011", feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblaslt_bf8_fnuz {
    pub __x: u8,
}
#[cfg(any(feature = "rocm-06033", feature = "rocm-06043"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblaslt_f8 {
    pub data: u8,
}
#[cfg(any(feature = "rocm-07002", feature = "rocm-07011", feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblaslt_f8 {
    pub __x: u8,
}
#[cfg(any(feature = "rocm-06002", feature = "rocm-06015", feature = "rocm-06024", feature = "rocm-06033", feature = "rocm-06043"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblaslt_f8_fnuz {
    pub data: u8,
}
#[cfg(any(feature = "rocm-07002", feature = "rocm-07011", feature = "rocm-07021", feature = "rocm-07022", feature = "rocm-07023"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipblaslt_f8_fnuz {
    pub __x: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipStream_t {
    _unused: [u8; 0],
}
pub unsafe fn hipblasLtCreate(handle: *mut hipblasLtHandle_t) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipblasLtHandle_t) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtCreate") });
        unsafe { _f(handle) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtCreate(handle: *mut hipblasLtHandle_t) -> hipblasStatus_t;
        }
        unsafe { hipblasLtCreate(handle) }
    }
}
pub unsafe fn hipblasLtDestroy(handle: hipblasLtHandle_t) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtHandle_t) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtDestroy") });
        unsafe { _f(handle) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtDestroy(handle: hipblasLtHandle_t) -> hipblasStatus_t;
        }
        unsafe { hipblasLtDestroy(handle) }
    }
}
pub unsafe fn hipblasLtGetArchName(archName: *mut *mut ::core::ffi::c_char) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_char) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtGetArchName") });
        unsafe { _f(archName) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtGetArchName(archName: *mut *mut ::core::ffi::c_char) -> hipblasStatus_t;
        }
        unsafe { hipblasLtGetArchName(archName) }
    }
}
pub unsafe fn hipblasLtGetGitRevision(handle: hipblasLtHandle_t, rev: *mut ::core::ffi::c_char) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtHandle_t, *mut ::core::ffi::c_char) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtGetGitRevision") });
        unsafe { _f(handle, rev) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtGetGitRevision(handle: hipblasLtHandle_t, rev: *mut ::core::ffi::c_char) -> hipblasStatus_t;
        }
        unsafe { hipblasLtGetGitRevision(handle, rev) }
    }
}
pub unsafe fn hipblasLtGetVersion(handle: hipblasLtHandle_t, version: *mut ::core::ffi::c_int) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtHandle_t, *mut ::core::ffi::c_int) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtGetVersion") });
        unsafe { _f(handle, version) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtGetVersion(handle: hipblasLtHandle_t, version: *mut ::core::ffi::c_int) -> hipblasStatus_t;
        }
        unsafe { hipblasLtGetVersion(handle, version) }
    }
}
pub unsafe fn hipblasLtMatmul(handle: hipblasLtHandle_t, matmulDesc: hipblasLtMatmulDesc_t, alpha: *const ::core::ffi::c_void, A: *const ::core::ffi::c_void, Adesc: hipblasLtMatrixLayout_t, B: *const ::core::ffi::c_void, Bdesc: hipblasLtMatrixLayout_t, beta: *const ::core::ffi::c_void, C: *const ::core::ffi::c_void, Cdesc: hipblasLtMatrixLayout_t, D: *mut ::core::ffi::c_void, Ddesc: hipblasLtMatrixLayout_t, algo: *const hipblasLtMatmulAlgo_t, workspace: *mut ::core::ffi::c_void, workspaceSizeInBytes: usize, stream: hipStream_t) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtHandle_t, hipblasLtMatmulDesc_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, hipblasLtMatrixLayout_t, *const ::core::ffi::c_void, hipblasLtMatrixLayout_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, hipblasLtMatrixLayout_t, *mut ::core::ffi::c_void, hipblasLtMatrixLayout_t, *const hipblasLtMatmulAlgo_t, *mut ::core::ffi::c_void, usize, hipStream_t) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatmul") });
        unsafe { _f(handle, matmulDesc, alpha, A, Adesc, B, Bdesc, beta, C, Cdesc, D, Ddesc, algo, workspace, workspaceSizeInBytes, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatmul(handle: hipblasLtHandle_t, matmulDesc: hipblasLtMatmulDesc_t, alpha: *const ::core::ffi::c_void, A: *const ::core::ffi::c_void, Adesc: hipblasLtMatrixLayout_t, B: *const ::core::ffi::c_void, Bdesc: hipblasLtMatrixLayout_t, beta: *const ::core::ffi::c_void, C: *const ::core::ffi::c_void, Cdesc: hipblasLtMatrixLayout_t, D: *mut ::core::ffi::c_void, Ddesc: hipblasLtMatrixLayout_t, algo: *const hipblasLtMatmulAlgo_t, workspace: *mut ::core::ffi::c_void, workspaceSizeInBytes: usize, stream: hipStream_t) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatmul(handle, matmulDesc, alpha, A, Adesc, B, Bdesc, beta, C, Cdesc, D, Ddesc, algo, workspace, workspaceSizeInBytes, stream) }
    }
}
pub unsafe fn hipblasLtMatmulAlgoGetHeuristic(handle: hipblasLtHandle_t, matmulDesc: hipblasLtMatmulDesc_t, Adesc: hipblasLtMatrixLayout_t, Bdesc: hipblasLtMatrixLayout_t, Cdesc: hipblasLtMatrixLayout_t, Ddesc: hipblasLtMatrixLayout_t, pref: hipblasLtMatmulPreference_t, requestedAlgoCount: ::core::ffi::c_int, heuristicResultsArray: *mut hipblasLtMatmulHeuristicResult_t, returnAlgoCount: *mut ::core::ffi::c_int) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtHandle_t, hipblasLtMatmulDesc_t, hipblasLtMatrixLayout_t, hipblasLtMatrixLayout_t, hipblasLtMatrixLayout_t, hipblasLtMatrixLayout_t, hipblasLtMatmulPreference_t, ::core::ffi::c_int, *mut hipblasLtMatmulHeuristicResult_t, *mut ::core::ffi::c_int) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatmulAlgoGetHeuristic") });
        unsafe { _f(handle, matmulDesc, Adesc, Bdesc, Cdesc, Ddesc, pref, requestedAlgoCount, heuristicResultsArray, returnAlgoCount) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatmulAlgoGetHeuristic(handle: hipblasLtHandle_t, matmulDesc: hipblasLtMatmulDesc_t, Adesc: hipblasLtMatrixLayout_t, Bdesc: hipblasLtMatrixLayout_t, Cdesc: hipblasLtMatrixLayout_t, Ddesc: hipblasLtMatrixLayout_t, pref: hipblasLtMatmulPreference_t, requestedAlgoCount: ::core::ffi::c_int, heuristicResultsArray: *mut hipblasLtMatmulHeuristicResult_t, returnAlgoCount: *mut ::core::ffi::c_int) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatmulAlgoGetHeuristic(handle, matmulDesc, Adesc, Bdesc, Cdesc, Ddesc, pref, requestedAlgoCount, heuristicResultsArray, returnAlgoCount) }
    }
}
pub unsafe fn hipblasLtMatmulDescCreate(matmulDesc: *mut hipblasLtMatmulDesc_t, computeType: hipblasComputeType_t, scaleType: hipDataType) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipblasLtMatmulDesc_t, hipblasComputeType_t, hipDataType) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatmulDescCreate") });
        unsafe { _f(matmulDesc, computeType, scaleType) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatmulDescCreate(matmulDesc: *mut hipblasLtMatmulDesc_t, computeType: hipblasComputeType_t, scaleType: hipDataType) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatmulDescCreate(matmulDesc, computeType, scaleType) }
    }
}
pub unsafe fn hipblasLtMatmulDescDestroy(matmulDesc: hipblasLtMatmulDesc_t) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtMatmulDesc_t) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatmulDescDestroy") });
        unsafe { _f(matmulDesc) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatmulDescDestroy(matmulDesc: hipblasLtMatmulDesc_t) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatmulDescDestroy(matmulDesc) }
    }
}
pub unsafe fn hipblasLtMatmulDescGetAttribute(matmulDesc: hipblasLtMatmulDesc_t, attr: hipblasLtMatmulDescAttributes_t, buf: *mut ::core::ffi::c_void, sizeInBytes: usize, sizeWritten: *mut usize) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtMatmulDesc_t, hipblasLtMatmulDescAttributes_t, *mut ::core::ffi::c_void, usize, *mut usize) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatmulDescGetAttribute") });
        unsafe { _f(matmulDesc, attr, buf, sizeInBytes, sizeWritten) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatmulDescGetAttribute(matmulDesc: hipblasLtMatmulDesc_t, attr: hipblasLtMatmulDescAttributes_t, buf: *mut ::core::ffi::c_void, sizeInBytes: usize, sizeWritten: *mut usize) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatmulDescGetAttribute(matmulDesc, attr, buf, sizeInBytes, sizeWritten) }
    }
}
pub unsafe fn hipblasLtMatmulDescSetAttribute(matmulDesc: hipblasLtMatmulDesc_t, attr: hipblasLtMatmulDescAttributes_t, buf: *const ::core::ffi::c_void, sizeInBytes: usize) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtMatmulDesc_t, hipblasLtMatmulDescAttributes_t, *const ::core::ffi::c_void, usize) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatmulDescSetAttribute") });
        unsafe { _f(matmulDesc, attr, buf, sizeInBytes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatmulDescSetAttribute(matmulDesc: hipblasLtMatmulDesc_t, attr: hipblasLtMatmulDescAttributes_t, buf: *const ::core::ffi::c_void, sizeInBytes: usize) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatmulDescSetAttribute(matmulDesc, attr, buf, sizeInBytes) }
    }
}
pub unsafe fn hipblasLtMatmulPreferenceCreate(pref: *mut hipblasLtMatmulPreference_t) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipblasLtMatmulPreference_t) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatmulPreferenceCreate") });
        unsafe { _f(pref) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatmulPreferenceCreate(pref: *mut hipblasLtMatmulPreference_t) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatmulPreferenceCreate(pref) }
    }
}
pub unsafe fn hipblasLtMatmulPreferenceDestroy(pref: hipblasLtMatmulPreference_t) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtMatmulPreference_t) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatmulPreferenceDestroy") });
        unsafe { _f(pref) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatmulPreferenceDestroy(pref: hipblasLtMatmulPreference_t) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatmulPreferenceDestroy(pref) }
    }
}
pub unsafe fn hipblasLtMatmulPreferenceGetAttribute(pref: hipblasLtMatmulPreference_t, attr: hipblasLtMatmulPreferenceAttributes_t, buf: *mut ::core::ffi::c_void, sizeInBytes: usize, sizeWritten: *mut usize) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtMatmulPreference_t, hipblasLtMatmulPreferenceAttributes_t, *mut ::core::ffi::c_void, usize, *mut usize) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatmulPreferenceGetAttribute") });
        unsafe { _f(pref, attr, buf, sizeInBytes, sizeWritten) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatmulPreferenceGetAttribute(pref: hipblasLtMatmulPreference_t, attr: hipblasLtMatmulPreferenceAttributes_t, buf: *mut ::core::ffi::c_void, sizeInBytes: usize, sizeWritten: *mut usize) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatmulPreferenceGetAttribute(pref, attr, buf, sizeInBytes, sizeWritten) }
    }
}
pub unsafe fn hipblasLtMatmulPreferenceSetAttribute(pref: hipblasLtMatmulPreference_t, attr: hipblasLtMatmulPreferenceAttributes_t, buf: *const ::core::ffi::c_void, sizeInBytes: usize) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtMatmulPreference_t, hipblasLtMatmulPreferenceAttributes_t, *const ::core::ffi::c_void, usize) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatmulPreferenceSetAttribute") });
        unsafe { _f(pref, attr, buf, sizeInBytes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatmulPreferenceSetAttribute(pref: hipblasLtMatmulPreference_t, attr: hipblasLtMatmulPreferenceAttributes_t, buf: *const ::core::ffi::c_void, sizeInBytes: usize) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatmulPreferenceSetAttribute(pref, attr, buf, sizeInBytes) }
    }
}
pub unsafe fn hipblasLtMatrixLayoutCreate(matLayout: *mut hipblasLtMatrixLayout_t, type_: hipDataType, rows: u64, cols: u64, ld: i64) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipblasLtMatrixLayout_t, hipDataType, u64, u64, i64) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatrixLayoutCreate") });
        unsafe { _f(matLayout, type_, rows, cols, ld) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatrixLayoutCreate(matLayout: *mut hipblasLtMatrixLayout_t, type_: hipDataType, rows: u64, cols: u64, ld: i64) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatrixLayoutCreate(matLayout, type_, rows, cols, ld) }
    }
}
pub unsafe fn hipblasLtMatrixLayoutDestroy(matLayout: hipblasLtMatrixLayout_t) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtMatrixLayout_t) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatrixLayoutDestroy") });
        unsafe { _f(matLayout) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatrixLayoutDestroy(matLayout: hipblasLtMatrixLayout_t) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatrixLayoutDestroy(matLayout) }
    }
}
pub unsafe fn hipblasLtMatrixLayoutGetAttribute(matLayout: hipblasLtMatrixLayout_t, attr: hipblasLtMatrixLayoutAttribute_t, buf: *mut ::core::ffi::c_void, sizeInBytes: usize, sizeWritten: *mut usize) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtMatrixLayout_t, hipblasLtMatrixLayoutAttribute_t, *mut ::core::ffi::c_void, usize, *mut usize) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatrixLayoutGetAttribute") });
        unsafe { _f(matLayout, attr, buf, sizeInBytes, sizeWritten) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatrixLayoutGetAttribute(matLayout: hipblasLtMatrixLayout_t, attr: hipblasLtMatrixLayoutAttribute_t, buf: *mut ::core::ffi::c_void, sizeInBytes: usize, sizeWritten: *mut usize) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatrixLayoutGetAttribute(matLayout, attr, buf, sizeInBytes, sizeWritten) }
    }
}
pub unsafe fn hipblasLtMatrixLayoutSetAttribute(matLayout: hipblasLtMatrixLayout_t, attr: hipblasLtMatrixLayoutAttribute_t, buf: *const ::core::ffi::c_void, sizeInBytes: usize) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtMatrixLayout_t, hipblasLtMatrixLayoutAttribute_t, *const ::core::ffi::c_void, usize) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatrixLayoutSetAttribute") });
        unsafe { _f(matLayout, attr, buf, sizeInBytes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatrixLayoutSetAttribute(matLayout: hipblasLtMatrixLayout_t, attr: hipblasLtMatrixLayoutAttribute_t, buf: *const ::core::ffi::c_void, sizeInBytes: usize) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatrixLayoutSetAttribute(matLayout, attr, buf, sizeInBytes) }
    }
}
pub unsafe fn hipblasLtMatrixTransform(lightHandle: hipblasLtHandle_t, transformDesc: hipblasLtMatrixTransformDesc_t, alpha: *const ::core::ffi::c_void, A: *const ::core::ffi::c_void, Adesc: hipblasLtMatrixLayout_t, beta: *const ::core::ffi::c_void, B: *const ::core::ffi::c_void, Bdesc: hipblasLtMatrixLayout_t, C: *mut ::core::ffi::c_void, Cdesc: hipblasLtMatrixLayout_t, stream: hipStream_t) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtHandle_t, hipblasLtMatrixTransformDesc_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, hipblasLtMatrixLayout_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, hipblasLtMatrixLayout_t, *mut ::core::ffi::c_void, hipblasLtMatrixLayout_t, hipStream_t) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatrixTransform") });
        unsafe { _f(lightHandle, transformDesc, alpha, A, Adesc, beta, B, Bdesc, C, Cdesc, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatrixTransform(lightHandle: hipblasLtHandle_t, transformDesc: hipblasLtMatrixTransformDesc_t, alpha: *const ::core::ffi::c_void, A: *const ::core::ffi::c_void, Adesc: hipblasLtMatrixLayout_t, beta: *const ::core::ffi::c_void, B: *const ::core::ffi::c_void, Bdesc: hipblasLtMatrixLayout_t, C: *mut ::core::ffi::c_void, Cdesc: hipblasLtMatrixLayout_t, stream: hipStream_t) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatrixTransform(lightHandle, transformDesc, alpha, A, Adesc, beta, B, Bdesc, C, Cdesc, stream) }
    }
}
pub unsafe fn hipblasLtMatrixTransformDescCreate(transformDesc: *mut hipblasLtMatrixTransformDesc_t, scaleType: hipDataType) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipblasLtMatrixTransformDesc_t, hipDataType) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatrixTransformDescCreate") });
        unsafe { _f(transformDesc, scaleType) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatrixTransformDescCreate(transformDesc: *mut hipblasLtMatrixTransformDesc_t, scaleType: hipDataType) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatrixTransformDescCreate(transformDesc, scaleType) }
    }
}
pub unsafe fn hipblasLtMatrixTransformDescDestroy(transformDesc: hipblasLtMatrixTransformDesc_t) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtMatrixTransformDesc_t) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatrixTransformDescDestroy") });
        unsafe { _f(transformDesc) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatrixTransformDescDestroy(transformDesc: hipblasLtMatrixTransformDesc_t) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatrixTransformDescDestroy(transformDesc) }
    }
}
pub unsafe fn hipblasLtMatrixTransformDescGetAttribute(transformDesc: hipblasLtMatrixTransformDesc_t, attr: hipblasLtMatrixTransformDescAttributes_t, buf: *mut ::core::ffi::c_void, sizeInBytes: usize, sizeWritten: *mut usize) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtMatrixTransformDesc_t, hipblasLtMatrixTransformDescAttributes_t, *mut ::core::ffi::c_void, usize, *mut usize) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatrixTransformDescGetAttribute") });
        unsafe { _f(transformDesc, attr, buf, sizeInBytes, sizeWritten) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatrixTransformDescGetAttribute(transformDesc: hipblasLtMatrixTransformDesc_t, attr: hipblasLtMatrixTransformDescAttributes_t, buf: *mut ::core::ffi::c_void, sizeInBytes: usize, sizeWritten: *mut usize) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatrixTransformDescGetAttribute(transformDesc, attr, buf, sizeInBytes, sizeWritten) }
    }
}
pub unsafe fn hipblasLtMatrixTransformDescSetAttribute(transformDesc: hipblasLtMatrixTransformDesc_t, attr: hipblasLtMatrixTransformDescAttributes_t, buf: *const ::core::ffi::c_void, sizeInBytes: usize) -> hipblasStatus_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipblasLtMatrixTransformDesc_t, hipblasLtMatrixTransformDescAttributes_t, *const ::core::ffi::c_void, usize) -> hipblasStatus_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipblasLtMatrixTransformDescSetAttribute") });
        unsafe { _f(transformDesc, attr, buf, sizeInBytes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipblasLtMatrixTransformDescSetAttribute(transformDesc: hipblasLtMatrixTransformDesc_t, attr: hipblasLtMatrixTransformDescAttributes_t, buf: *const ::core::ffi::c_void, sizeInBytes: usize) -> hipblasStatus_t;
        }
        unsafe { hipblasLtMatrixTransformDescSetAttribute(transformDesc, attr, buf, sizeInBytes) }
    }
}
#[cfg(feature = "dynamic-loading")]
pub unsafe fn is_rocmlib_present() -> bool {
    let lib_names = ["hipblaslt"];
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
        let lib_names = std::vec!["hipblaslt"];
        let choices: std::vec::Vec<_> = lib_names.iter().map(|l| crate::get_lib_name_candidates(l)).flatten().collect();
        for choice in choices.iter() {
            if let Ok(lib) = ::libloading::Library::new(choice) {
                return lib;
            }
        }
        crate::panic_no_lib_found(lib_names[0], &choices);
    })
}
