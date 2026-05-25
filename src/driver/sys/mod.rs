#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::sync::OnceLock;
#[cfg(feature = "dynamic-loading")]
fn load<F: Copy>(name: &str) -> F {
    unsafe { *rocmlib().get::<F>(name.as_bytes()).unwrap_or_else(|e| panic!("Missing symbol {name}: {e}")) }
}
pub use self::HIPaddress_mode_enum as HIPaddress_mode;
pub use self::HIPfilter_mode_enum as HIPfilter_mode;
pub use self::HIPresourceViewFormat_enum as HIPresourceViewFormat;
pub use self::HIPresourcetype_enum as HIPresourcetype;
pub use self::HIPresourcetype_enum as hipResourcetype;
pub use self::hipExternalMemoryHandleType_enum as hipExternalMemoryHandleType;
pub use self::hipExternalSemaphoreHandleType_enum as hipExternalSemaphoreHandleType;
pub use self::hipLibraryOption_e as hipLibraryOption;
pub const HIP_DEPRECATED_MSG: &[u8; 189] = b"This API is marked as deprecated and might not be supported in future releases. For more details please refer https://github.com/ROCm/HIP/blob/develop/docs/reference/deprecated_api_list.md\0";
pub const HIP_ERROR_INVALID_VALUE: _bindgen_ty_1 = _bindgen_ty_1::HIP_ERROR_INVALID_VALUE;
pub const HIP_ERROR_LAUNCH_OUT_OF_RESOURCES: _bindgen_ty_1 = _bindgen_ty_1::HIP_ERROR_LAUNCH_OUT_OF_RESOURCES;
pub const HIP_ERROR_NOT_INITIALIZED: _bindgen_ty_1 = _bindgen_ty_1::HIP_ERROR_NOT_INITIALIZED;
pub const HIP_IMAGE_OBJECT_SIZE_DWORD: u32 = 12;
pub const HIP_IPC_HANDLE_SIZE: u32 = 64;
pub const HIP_SAMPLER_OBJECT_OFFSET_DWORD: u32 = 12;
pub const HIP_SAMPLER_OBJECT_SIZE_DWORD: u32 = 8;
pub const HIP_SUCCESS: _bindgen_ty_1 = _bindgen_ty_1::HIP_SUCCESS;
pub const HIP_TEXTURE_OBJECT_SIZE_DWORD: u32 = 20;
pub const HIP_TRSA_OVERRIDE_FORMAT: u32 = 1;
pub const HIP_TRSF_NORMALIZED_COORDINATES: u32 = 2;
pub const HIP_TRSF_READ_AS_INTEGER: u32 = 1;
pub const HIP_TRSF_SRGB: u32 = 16;
pub const HIP_VERSION: u32 = 70253211;
pub const HIP_VERSION_BUILD_ID: u32 = 0;
pub const HIP_VERSION_BUILD_NAME: &[u8; 1] = b"\0";
pub const HIP_VERSION_GITHASH: &[u8; 11] = b"c2d9476115\0";
pub const HIP_VERSION_MAJOR: u32 = 7;
pub const HIP_VERSION_MINOR: u32 = 2;
pub const HIP_VERSION_PATCH: u32 = 53211;
pub type HIP_LAUNCH_CONFIG = HIP_LAUNCH_CONFIG_st;
pub type HIP_RESOURCE_DESC = HIP_RESOURCE_DESC_st;
pub type HIP_RESOURCE_VIEW_DESC = HIP_RESOURCE_VIEW_DESC_st;
pub type HIP_TEXTURE_DESC = HIP_TEXTURE_DESC_st;
pub type hipArray_const_t = *const hipArray;
pub type hipArray_t = *mut hipArray;
pub type hipCtx_t = *mut ihipCtx_t;
pub type hipDevice_t = ::core::ffi::c_int;
pub type hipDeviceptr_t = *mut ::core::ffi::c_void;
pub type hipEvent_t = *mut ihipEvent_t;
pub type hipExternalMemoryBufferDesc = hipExternalMemoryBufferDesc_st;
pub type hipExternalMemoryHandleDesc = hipExternalMemoryHandleDesc_st;
pub type hipExternalMemoryMipmappedArrayDesc = hipExternalMemoryMipmappedArrayDesc_st;
pub type hipExternalMemory_t = *mut ::core::ffi::c_void;
pub type hipExternalSemaphoreHandleDesc = hipExternalSemaphoreHandleDesc_st;
pub type hipExternalSemaphoreSignalParams = hipExternalSemaphoreSignalParams_st;
pub type hipExternalSemaphoreWaitParams = hipExternalSemaphoreWaitParams_st;
pub type hipExternalSemaphore_t = *mut ::core::ffi::c_void;
pub type hipFunctionLaunchParams = hipFunctionLaunchParams_t;
pub type hipFunction_t = *mut ihipModuleSymbol_t;
pub type hipGraphExec_t = *mut hipGraphExec;
pub type hipGraphNode_t = *mut hipGraphNode;
pub type hipGraph_t = *mut ihipGraph;
pub type hipGraphicsResource = _hipGraphicsResource;
pub type hipGraphicsResource_t = *mut hipGraphicsResource;
pub type hipHostFn_t = ::core::option::Option<unsafe extern "C" fn(userData: *mut ::core::ffi::c_void)>;
pub type hipIpcEventHandle_t = hipIpcEventHandle_st;
pub type hipIpcMemHandle_t = hipIpcMemHandle_st;
pub type hipKernel_t = *mut ihipKernel_t;
pub type hipLaunchAttribute = hipLaunchAttribute_st;
pub type hipLaunchConfig_t = hipLaunchConfig_st;
pub type hipLaunchParams = hipLaunchParams_t;
pub type hipLibrary_t = *mut ihipLibrary_t;
pub type hipLinkState_t = *mut ihipLinkState_t;
pub type hipMemGenericAllocationHandle_t = *mut ihipMemGenericAllocationHandle;
pub type hipMemPool_t = *mut ihipMemPoolHandle_t;
pub type hipMipmappedArray_const_t = *const hipMipmappedArray;
pub type hipMipmappedArray_t = *mut hipMipmappedArray;
pub type hipModule_t = *mut ihipModule_t;
pub type hipStreamBatchMemOpParams = hipStreamBatchMemOpParams_union;
pub type hipStreamCallback_t = ::core::option::Option<unsafe extern "C" fn(stream: hipStream_t, status: hipError_t, userData: *mut ::core::ffi::c_void)>;
pub type hipStream_t = *mut ihipStream_t;
pub type hipSurfaceObject_t = *mut __hip_surface;
pub type hipTextureObject_t = *mut __hip_texture;
pub type hipUUID = hipUUID_t;
pub type hipUserObject_t = *mut hipUserObject;
pub type hipmipmappedArray = hipMipmappedArray_t;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum HIPaddress_mode_enum {
    HIP_TR_ADDRESS_MODE_WRAP = 0,
    HIP_TR_ADDRESS_MODE_CLAMP = 1,
    HIP_TR_ADDRESS_MODE_MIRROR = 2,
    HIP_TR_ADDRESS_MODE_BORDER = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum HIPfilter_mode_enum {
    HIP_TR_FILTER_MODE_POINT = 0,
    HIP_TR_FILTER_MODE_LINEAR = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum HIPresourceViewFormat_enum {
    HIP_RES_VIEW_FORMAT_NONE = 0,
    HIP_RES_VIEW_FORMAT_UINT_1X8 = 1,
    HIP_RES_VIEW_FORMAT_UINT_2X8 = 2,
    HIP_RES_VIEW_FORMAT_UINT_4X8 = 3,
    HIP_RES_VIEW_FORMAT_SINT_1X8 = 4,
    HIP_RES_VIEW_FORMAT_SINT_2X8 = 5,
    HIP_RES_VIEW_FORMAT_SINT_4X8 = 6,
    HIP_RES_VIEW_FORMAT_UINT_1X16 = 7,
    HIP_RES_VIEW_FORMAT_UINT_2X16 = 8,
    HIP_RES_VIEW_FORMAT_UINT_4X16 = 9,
    HIP_RES_VIEW_FORMAT_SINT_1X16 = 10,
    HIP_RES_VIEW_FORMAT_SINT_2X16 = 11,
    HIP_RES_VIEW_FORMAT_SINT_4X16 = 12,
    HIP_RES_VIEW_FORMAT_UINT_1X32 = 13,
    HIP_RES_VIEW_FORMAT_UINT_2X32 = 14,
    HIP_RES_VIEW_FORMAT_UINT_4X32 = 15,
    HIP_RES_VIEW_FORMAT_SINT_1X32 = 16,
    HIP_RES_VIEW_FORMAT_SINT_2X32 = 17,
    HIP_RES_VIEW_FORMAT_SINT_4X32 = 18,
    HIP_RES_VIEW_FORMAT_FLOAT_1X16 = 19,
    HIP_RES_VIEW_FORMAT_FLOAT_2X16 = 20,
    HIP_RES_VIEW_FORMAT_FLOAT_4X16 = 21,
    HIP_RES_VIEW_FORMAT_FLOAT_1X32 = 22,
    HIP_RES_VIEW_FORMAT_FLOAT_2X32 = 23,
    HIP_RES_VIEW_FORMAT_FLOAT_4X32 = 24,
    HIP_RES_VIEW_FORMAT_UNSIGNED_BC1 = 25,
    HIP_RES_VIEW_FORMAT_UNSIGNED_BC2 = 26,
    HIP_RES_VIEW_FORMAT_UNSIGNED_BC3 = 27,
    HIP_RES_VIEW_FORMAT_UNSIGNED_BC4 = 28,
    HIP_RES_VIEW_FORMAT_SIGNED_BC4 = 29,
    HIP_RES_VIEW_FORMAT_UNSIGNED_BC5 = 30,
    HIP_RES_VIEW_FORMAT_SIGNED_BC5 = 31,
    HIP_RES_VIEW_FORMAT_UNSIGNED_BC6H = 32,
    HIP_RES_VIEW_FORMAT_SIGNED_BC6H = 33,
    HIP_RES_VIEW_FORMAT_UNSIGNED_BC7 = 34,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum HIPresourcetype_enum {
    HIP_RESOURCE_TYPE_ARRAY = 0,
    HIP_RESOURCE_TYPE_MIPMAPPED_ARRAY = 1,
    HIP_RESOURCE_TYPE_LINEAR = 2,
    HIP_RESOURCE_TYPE_PITCH2D = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
    HIP_SUCCESS = 0,
    HIP_ERROR_INVALID_VALUE = 1,
    HIP_ERROR_NOT_INITIALIZED = 2,
    HIP_ERROR_LAUNCH_OUT_OF_RESOURCES = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipAccessProperty {
    hipAccessPropertyNormal = 0,
    hipAccessPropertyStreaming = 1,
    hipAccessPropertyPersisting = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipArraySparseSubresourceType {
    hipArraySparseSubresourceTypeSparseLevel = 0,
    hipArraySparseSubresourceTypeMiptail = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipArray_Format {
    HIP_AD_FORMAT_UNSIGNED_INT8 = 1,
    HIP_AD_FORMAT_UNSIGNED_INT16 = 2,
    HIP_AD_FORMAT_UNSIGNED_INT32 = 3,
    HIP_AD_FORMAT_SIGNED_INT8 = 8,
    HIP_AD_FORMAT_SIGNED_INT16 = 9,
    HIP_AD_FORMAT_SIGNED_INT32 = 10,
    HIP_AD_FORMAT_HALF = 16,
    HIP_AD_FORMAT_FLOAT = 32,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipChannelFormatKind {
    hipChannelFormatKindSigned = 0,
    hipChannelFormatKindUnsigned = 1,
    hipChannelFormatKindFloat = 2,
    hipChannelFormatKindNone = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipComputeMode {
    hipComputeModeDefault = 0,
    hipComputeModeExclusive = 1,
    hipComputeModeProhibited = 2,
    hipComputeModeExclusiveProcess = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipDeviceAttribute_t {
    hipDeviceAttributeCudaCompatibleBegin = 0,
    hipDeviceAttributeAccessPolicyMaxWindowSize = 1,
    hipDeviceAttributeAsyncEngineCount = 2,
    hipDeviceAttributeCanMapHostMemory = 3,
    hipDeviceAttributeCanUseHostPointerForRegisteredMem = 4,
    hipDeviceAttributeClockRate = 5,
    hipDeviceAttributeComputeMode = 6,
    hipDeviceAttributeComputePreemptionSupported = 7,
    hipDeviceAttributeConcurrentKernels = 8,
    hipDeviceAttributeConcurrentManagedAccess = 9,
    hipDeviceAttributeCooperativeLaunch = 10,
    hipDeviceAttributeCooperativeMultiDeviceLaunch = 11,
    hipDeviceAttributeDeviceOverlap = 12,
    hipDeviceAttributeDirectManagedMemAccessFromHost = 13,
    hipDeviceAttributeGlobalL1CacheSupported = 14,
    hipDeviceAttributeHostNativeAtomicSupported = 15,
    hipDeviceAttributeIntegrated = 16,
    hipDeviceAttributeIsMultiGpuBoard = 17,
    hipDeviceAttributeKernelExecTimeout = 18,
    hipDeviceAttributeL2CacheSize = 19,
    hipDeviceAttributeLocalL1CacheSupported = 20,
    hipDeviceAttributeLuid = 21,
    hipDeviceAttributeLuidDeviceNodeMask = 22,
    hipDeviceAttributeComputeCapabilityMajor = 23,
    hipDeviceAttributeManagedMemory = 24,
    hipDeviceAttributeMaxBlocksPerMultiProcessor = 25,
    hipDeviceAttributeMaxBlockDimX = 26,
    hipDeviceAttributeMaxBlockDimY = 27,
    hipDeviceAttributeMaxBlockDimZ = 28,
    hipDeviceAttributeMaxGridDimX = 29,
    hipDeviceAttributeMaxGridDimY = 30,
    hipDeviceAttributeMaxGridDimZ = 31,
    hipDeviceAttributeMaxSurface1D = 32,
    hipDeviceAttributeMaxSurface1DLayered = 33,
    hipDeviceAttributeMaxSurface2D = 34,
    hipDeviceAttributeMaxSurface2DLayered = 35,
    hipDeviceAttributeMaxSurface3D = 36,
    hipDeviceAttributeMaxSurfaceCubemap = 37,
    hipDeviceAttributeMaxSurfaceCubemapLayered = 38,
    hipDeviceAttributeMaxTexture1DWidth = 39,
    hipDeviceAttributeMaxTexture1DLayered = 40,
    hipDeviceAttributeMaxTexture1DLinear = 41,
    hipDeviceAttributeMaxTexture1DMipmap = 42,
    hipDeviceAttributeMaxTexture2DWidth = 43,
    hipDeviceAttributeMaxTexture2DHeight = 44,
    hipDeviceAttributeMaxTexture2DGather = 45,
    hipDeviceAttributeMaxTexture2DLayered = 46,
    hipDeviceAttributeMaxTexture2DLinear = 47,
    hipDeviceAttributeMaxTexture2DMipmap = 48,
    hipDeviceAttributeMaxTexture3DWidth = 49,
    hipDeviceAttributeMaxTexture3DHeight = 50,
    hipDeviceAttributeMaxTexture3DDepth = 51,
    hipDeviceAttributeMaxTexture3DAlt = 52,
    hipDeviceAttributeMaxTextureCubemap = 53,
    hipDeviceAttributeMaxTextureCubemapLayered = 54,
    hipDeviceAttributeMaxThreadsDim = 55,
    hipDeviceAttributeMaxThreadsPerBlock = 56,
    hipDeviceAttributeMaxThreadsPerMultiProcessor = 57,
    hipDeviceAttributeMaxPitch = 58,
    hipDeviceAttributeMemoryBusWidth = 59,
    hipDeviceAttributeMemoryClockRate = 60,
    hipDeviceAttributeComputeCapabilityMinor = 61,
    hipDeviceAttributeMultiGpuBoardGroupID = 62,
    hipDeviceAttributeMultiprocessorCount = 63,
    hipDeviceAttributeUnused1 = 64,
    hipDeviceAttributePageableMemoryAccess = 65,
    hipDeviceAttributePageableMemoryAccessUsesHostPageTables = 66,
    hipDeviceAttributePciBusId = 67,
    hipDeviceAttributePciDeviceId = 68,
    hipDeviceAttributePciDomainId = 69,
    hipDeviceAttributePersistingL2CacheMaxSize = 70,
    hipDeviceAttributeMaxRegistersPerBlock = 71,
    hipDeviceAttributeMaxRegistersPerMultiprocessor = 72,
    hipDeviceAttributeReservedSharedMemPerBlock = 73,
    hipDeviceAttributeMaxSharedMemoryPerBlock = 74,
    hipDeviceAttributeSharedMemPerBlockOptin = 75,
    hipDeviceAttributeSharedMemPerMultiprocessor = 76,
    hipDeviceAttributeSingleToDoublePrecisionPerfRatio = 77,
    hipDeviceAttributeStreamPrioritiesSupported = 78,
    hipDeviceAttributeSurfaceAlignment = 79,
    hipDeviceAttributeTccDriver = 80,
    hipDeviceAttributeTextureAlignment = 81,
    hipDeviceAttributeTexturePitchAlignment = 82,
    hipDeviceAttributeTotalConstantMemory = 83,
    hipDeviceAttributeTotalGlobalMem = 84,
    hipDeviceAttributeUnifiedAddressing = 85,
    hipDeviceAttributeUnused2 = 86,
    hipDeviceAttributeWarpSize = 87,
    hipDeviceAttributeMemoryPoolsSupported = 88,
    hipDeviceAttributeVirtualMemoryManagementSupported = 89,
    hipDeviceAttributeHostRegisterSupported = 90,
    hipDeviceAttributeMemoryPoolSupportedHandleTypes = 91,
    hipDeviceAttributeHostNumaId = 92,
    hipDeviceAttributeCudaCompatibleEnd = 9999,
    hipDeviceAttributeAmdSpecificBegin = 10000,
    hipDeviceAttributeUnused3 = 10001,
    hipDeviceAttributeMaxSharedMemoryPerMultiprocessor = 10002,
    hipDeviceAttributeUnused4 = 10003,
    hipDeviceAttributeUnused5 = 10004,
    hipDeviceAttributeHdpMemFlushCntl = 10005,
    hipDeviceAttributeHdpRegFlushCntl = 10006,
    hipDeviceAttributeCooperativeMultiDeviceUnmatchedFunc = 10007,
    hipDeviceAttributeCooperativeMultiDeviceUnmatchedGridDim = 10008,
    hipDeviceAttributeCooperativeMultiDeviceUnmatchedBlockDim = 10009,
    hipDeviceAttributeCooperativeMultiDeviceUnmatchedSharedMem = 10010,
    hipDeviceAttributeIsLargeBar = 10011,
    hipDeviceAttributeAsicRevision = 10012,
    hipDeviceAttributeCanUseStreamWaitValue = 10013,
    hipDeviceAttributeImageSupport = 10014,
    hipDeviceAttributePhysicalMultiProcessorCount = 10015,
    hipDeviceAttributeFineGrainSupport = 10016,
    hipDeviceAttributeWallClockRate = 10017,
    hipDeviceAttributeNumberOfXccs = 10018,
    hipDeviceAttributeMaxAvailableVgprsPerThread = 10019,
    hipDeviceAttributePciChipId = 10020,
    hipDeviceAttributeAmdSpecificEnd = 19999,
    hipDeviceAttributeVendorSpecificBegin = 20000,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipDeviceP2PAttr {
    hipDevP2PAttrPerformanceRank = 0,
    hipDevP2PAttrAccessSupported = 1,
    hipDevP2PAttrNativeAtomicSupported = 2,
    hipDevP2PAttrHipArrayAccessSupported = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipDriverEntryPointQueryResult {
    hipDriverEntryPointSuccess = 0,
    hipDriverEntryPointSymbolNotFound = 1,
    hipDriverEntryPointVersionNotSufficent = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipDriverProcAddressQueryResult {
    HIP_GET_PROC_ADDRESS_SUCCESS = 0,
    HIP_GET_PROC_ADDRESS_SYMBOL_NOT_FOUND = 1,
    HIP_GET_PROC_ADDRESS_VERSION_NOT_SUFFICIENT = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipError_t {
    hipSuccess = 0,
    hipErrorInvalidValue = 1,
    hipErrorOutOfMemory = 2,
    hipErrorNotInitialized = 3,
    hipErrorDeinitialized = 4,
    hipErrorProfilerDisabled = 5,
    hipErrorProfilerNotInitialized = 6,
    hipErrorProfilerAlreadyStarted = 7,
    hipErrorProfilerAlreadyStopped = 8,
    hipErrorInvalidConfiguration = 9,
    hipErrorInvalidPitchValue = 12,
    hipErrorInvalidSymbol = 13,
    hipErrorInvalidDevicePointer = 17,
    hipErrorInvalidMemcpyDirection = 21,
    hipErrorInsufficientDriver = 35,
    hipErrorMissingConfiguration = 52,
    hipErrorPriorLaunchFailure = 53,
    hipErrorInvalidDeviceFunction = 98,
    hipErrorNoDevice = 100,
    hipErrorInvalidDevice = 101,
    hipErrorInvalidImage = 200,
    hipErrorInvalidContext = 201,
    hipErrorContextAlreadyCurrent = 202,
    hipErrorMapFailed = 205,
    hipErrorUnmapFailed = 206,
    hipErrorArrayIsMapped = 207,
    hipErrorAlreadyMapped = 208,
    hipErrorNoBinaryForGpu = 209,
    hipErrorAlreadyAcquired = 210,
    hipErrorNotMapped = 211,
    hipErrorNotMappedAsArray = 212,
    hipErrorNotMappedAsPointer = 213,
    hipErrorECCNotCorrectable = 214,
    hipErrorUnsupportedLimit = 215,
    hipErrorContextAlreadyInUse = 216,
    hipErrorPeerAccessUnsupported = 217,
    hipErrorInvalidKernelFile = 218,
    hipErrorInvalidGraphicsContext = 219,
    hipErrorInvalidSource = 300,
    hipErrorFileNotFound = 301,
    hipErrorSharedObjectSymbolNotFound = 302,
    hipErrorSharedObjectInitFailed = 303,
    hipErrorOperatingSystem = 304,
    hipErrorInvalidHandle = 400,
    hipErrorIllegalState = 401,
    hipErrorNotFound = 500,
    hipErrorNotReady = 600,
    hipErrorIllegalAddress = 700,
    hipErrorLaunchOutOfResources = 701,
    hipErrorLaunchTimeOut = 702,
    hipErrorPeerAccessAlreadyEnabled = 704,
    hipErrorPeerAccessNotEnabled = 705,
    hipErrorSetOnActiveProcess = 708,
    hipErrorContextIsDestroyed = 709,
    hipErrorAssert = 710,
    hipErrorHostMemoryAlreadyRegistered = 712,
    hipErrorHostMemoryNotRegistered = 713,
    hipErrorLaunchFailure = 719,
    hipErrorCooperativeLaunchTooLarge = 720,
    hipErrorNotSupported = 801,
    hipErrorStreamCaptureUnsupported = 900,
    hipErrorStreamCaptureInvalidated = 901,
    hipErrorStreamCaptureMerge = 902,
    hipErrorStreamCaptureUnmatched = 903,
    hipErrorStreamCaptureUnjoined = 904,
    hipErrorStreamCaptureIsolation = 905,
    hipErrorStreamCaptureImplicit = 906,
    hipErrorCapturedEvent = 907,
    hipErrorStreamCaptureWrongThread = 908,
    hipErrorGraphExecUpdateFailure = 910,
    hipErrorInvalidChannelDescriptor = 911,
    hipErrorInvalidTexture = 912,
    hipErrorUnknown = 999,
    hipErrorRuntimeMemory = 1052,
    hipErrorRuntimeOther = 1053,
    hipErrorTbd = 1054,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipExternalMemoryHandleType_enum {
    hipExternalMemoryHandleTypeOpaqueFd = 1,
    hipExternalMemoryHandleTypeOpaqueWin32 = 2,
    hipExternalMemoryHandleTypeOpaqueWin32Kmt = 3,
    hipExternalMemoryHandleTypeD3D12Heap = 4,
    hipExternalMemoryHandleTypeD3D12Resource = 5,
    hipExternalMemoryHandleTypeD3D11Resource = 6,
    hipExternalMemoryHandleTypeD3D11ResourceKmt = 7,
    hipExternalMemoryHandleTypeNvSciBuf = 8,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipExternalSemaphoreHandleType_enum {
    hipExternalSemaphoreHandleTypeOpaqueFd = 1,
    hipExternalSemaphoreHandleTypeOpaqueWin32 = 2,
    hipExternalSemaphoreHandleTypeOpaqueWin32Kmt = 3,
    hipExternalSemaphoreHandleTypeD3D12Fence = 4,
    hipExternalSemaphoreHandleTypeD3D11Fence = 5,
    hipExternalSemaphoreHandleTypeNvSciSync = 6,
    hipExternalSemaphoreHandleTypeKeyedMutex = 7,
    hipExternalSemaphoreHandleTypeKeyedMutexKmt = 8,
    hipExternalSemaphoreHandleTypeTimelineSemaphoreFd = 9,
    hipExternalSemaphoreHandleTypeTimelineSemaphoreWin32 = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipFlushGPUDirectRDMAWritesOptions {
    hipFlushGPUDirectRDMAWritesOptionHost = 1,
    hipFlushGPUDirectRDMAWritesOptionMemOps = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipFuncAttribute {
    hipFuncAttributeMaxDynamicSharedMemorySize = 8,
    hipFuncAttributePreferredSharedMemoryCarveout = 9,
    hipFuncAttributeMax = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipFuncCache_t {
    hipFuncCachePreferNone = 0,
    hipFuncCachePreferShared = 1,
    hipFuncCachePreferL1 = 2,
    hipFuncCachePreferEqual = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipFunction_attribute {
    HIP_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK = 0,
    HIP_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES = 1,
    HIP_FUNC_ATTRIBUTE_CONST_SIZE_BYTES = 2,
    HIP_FUNC_ATTRIBUTE_LOCAL_SIZE_BYTES = 3,
    HIP_FUNC_ATTRIBUTE_NUM_REGS = 4,
    HIP_FUNC_ATTRIBUTE_PTX_VERSION = 5,
    HIP_FUNC_ATTRIBUTE_BINARY_VERSION = 6,
    HIP_FUNC_ATTRIBUTE_CACHE_MODE_CA = 7,
    HIP_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES = 8,
    HIP_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT = 9,
    HIP_FUNC_ATTRIBUTE_MAX = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipGPUDirectRDMAWritesOrdering {
    hipGPUDirectRDMAWritesOrderingNone = 0,
    hipGPUDirectRDMAWritesOrderingOwner = 100,
    hipGPUDirectRDMAWritesOrderingAllDevices = 200,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipGraphDebugDotFlags {
    hipGraphDebugDotFlagsVerbose = 1,
    hipGraphDebugDotFlagsKernelNodeParams = 4,
    hipGraphDebugDotFlagsMemcpyNodeParams = 8,
    hipGraphDebugDotFlagsMemsetNodeParams = 16,
    hipGraphDebugDotFlagsHostNodeParams = 32,
    hipGraphDebugDotFlagsEventNodeParams = 64,
    hipGraphDebugDotFlagsExtSemasSignalNodeParams = 128,
    hipGraphDebugDotFlagsExtSemasWaitNodeParams = 256,
    hipGraphDebugDotFlagsKernelNodeAttributes = 512,
    hipGraphDebugDotFlagsHandles = 1024,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipGraphDependencyType {
    hipGraphDependencyTypeDefault = 0,
    hipGraphDependencyTypeProgrammatic = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipGraphExecUpdateResult {
    hipGraphExecUpdateSuccess = 0,
    hipGraphExecUpdateError = 1,
    hipGraphExecUpdateErrorTopologyChanged = 2,
    hipGraphExecUpdateErrorNodeTypeChanged = 3,
    hipGraphExecUpdateErrorFunctionChanged = 4,
    hipGraphExecUpdateErrorParametersChanged = 5,
    hipGraphExecUpdateErrorNotSupported = 6,
    hipGraphExecUpdateErrorUnsupportedFunctionChange = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipGraphInstantiateFlags {
    hipGraphInstantiateFlagAutoFreeOnLaunch = 1,
    hipGraphInstantiateFlagUpload = 2,
    hipGraphInstantiateFlagDeviceLaunch = 4,
    hipGraphInstantiateFlagUseNodePriority = 8,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipGraphInstantiateResult {
    hipGraphInstantiateSuccess = 0,
    hipGraphInstantiateError = 1,
    hipGraphInstantiateInvalidStructure = 2,
    hipGraphInstantiateNodeOperationNotSupported = 3,
    hipGraphInstantiateMultipleDevicesNotSupported = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipGraphMemAttributeType {
    hipGraphMemAttrUsedMemCurrent = 0,
    hipGraphMemAttrUsedMemHigh = 1,
    hipGraphMemAttrReservedMemCurrent = 2,
    hipGraphMemAttrReservedMemHigh = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipGraphNodeType {
    hipGraphNodeTypeKernel = 0,
    hipGraphNodeTypeMemcpy = 1,
    hipGraphNodeTypeMemset = 2,
    hipGraphNodeTypeHost = 3,
    hipGraphNodeTypeGraph = 4,
    hipGraphNodeTypeEmpty = 5,
    hipGraphNodeTypeWaitEvent = 6,
    hipGraphNodeTypeEventRecord = 7,
    hipGraphNodeTypeExtSemaphoreSignal = 8,
    hipGraphNodeTypeExtSemaphoreWait = 9,
    hipGraphNodeTypeMemAlloc = 10,
    hipGraphNodeTypeMemFree = 11,
    hipGraphNodeTypeMemcpyFromSymbol = 12,
    hipGraphNodeTypeMemcpyToSymbol = 13,
    hipGraphNodeTypeBatchMemOp = 14,
    hipGraphNodeTypeCount = 15,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipGraphicsRegisterFlags {
    hipGraphicsRegisterFlagsNone = 0,
    hipGraphicsRegisterFlagsReadOnly = 1,
    hipGraphicsRegisterFlagsWriteDiscard = 2,
    hipGraphicsRegisterFlagsSurfaceLoadStore = 4,
    hipGraphicsRegisterFlagsTextureGather = 8,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipJitCacheMode {
    hipJitCacheOptionNone = 0,
    hipJitCacheOptionCG = 1,
    hipJitCacheOptionCA = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipJitFallback {
    hipJitPreferPTX = 0,
    hipJitPreferBinary = 1,
}
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
pub enum hipLaunchAttributeID {
    hipLaunchAttributeAccessPolicyWindow = 1,
    hipLaunchAttributeCooperative = 2,
    hipLaunchAttributeSynchronizationPolicy = 3,
    hipLaunchAttributePriority = 8,
    hipLaunchAttributeMemSyncDomainMap = 9,
    hipLaunchAttributeMemSyncDomain = 10,
    hipLaunchAttributeMax = 11,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipLaunchMemSyncDomain {
    hipLaunchMemSyncDomainDefault = 0,
    hipLaunchMemSyncDomainRemote = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipLibraryOption_e {
    hipLibraryHostUniversalFunctionAndDataTable = 0,
    hipLibraryBinaryIsPreserved = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipLimit_t {
    hipLimitStackSize = 0,
    hipLimitPrintfFifoSize = 1,
    hipLimitMallocHeapSize = 2,
    hipExtLimitScratchMin = 4096,
    hipExtLimitScratchMax = 4097,
    hipExtLimitScratchCurrent = 4098,
    hipLimitRange = 4099,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemAccessFlags {
    hipMemAccessFlagsProtNone = 0,
    hipMemAccessFlagsProtRead = 1,
    hipMemAccessFlagsProtReadWrite = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemAllocationGranularity_flags {
    hipMemAllocationGranularityMinimum = 0,
    hipMemAllocationGranularityRecommended = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemAllocationHandleType {
    hipMemHandleTypeNone = 0,
    hipMemHandleTypePosixFileDescriptor = 1,
    hipMemHandleTypeWin32 = 2,
    hipMemHandleTypeWin32Kmt = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemAllocationType {
    hipMemAllocationTypeInvalid = 0,
    hipMemAllocationTypePinned = 1,
    hipMemAllocationTypeUncached = 1073741824,
    hipMemAllocationTypeMax = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemHandleType {
    hipMemHandleTypeGeneric = 0,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemLocationType {
    hipMemLocationTypeInvalid = 0,
    hipMemLocationTypeDevice = 1,
    hipMemLocationTypeHost = 2,
    hipMemLocationTypeHostNuma = 3,
    hipMemLocationTypeHostNumaCurrent = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemOperationType {
    hipMemOperationTypeMap = 1,
    hipMemOperationTypeUnmap = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemPoolAttr {
    hipMemPoolReuseFollowEventDependencies = 1,
    hipMemPoolReuseAllowOpportunistic = 2,
    hipMemPoolReuseAllowInternalDependencies = 3,
    hipMemPoolAttrReleaseThreshold = 4,
    hipMemPoolAttrReservedMemCurrent = 5,
    hipMemPoolAttrReservedMemHigh = 6,
    hipMemPoolAttrUsedMemCurrent = 7,
    hipMemPoolAttrUsedMemHigh = 8,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemRangeAttribute {
    hipMemRangeAttributeReadMostly = 1,
    hipMemRangeAttributePreferredLocation = 2,
    hipMemRangeAttributeAccessedBy = 3,
    hipMemRangeAttributeLastPrefetchLocation = 4,
    hipMemRangeAttributeCoherencyMode = 100,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemRangeCoherencyMode {
    hipMemRangeCoherencyModeFineGrain = 0,
    hipMemRangeCoherencyModeCoarseGrain = 1,
    hipMemRangeCoherencyModeIndeterminate = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemRangeFlags {
    hipMemRangeFlagDmaBufMappingTypePcie = 1,
    hipMemRangeFlagsMax = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemRangeHandleType {
    hipMemRangeHandleTypeDmaBufFd = 1,
    hipMemRangeHandleTypeMax = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemcpy3DOperandType {
    hipMemcpyOperandTypePointer = 1,
    hipMemcpyOperandTypeArray = 2,
    hipMemcpyOperandTypeMax = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemcpyFlags {
    hipMemcpyFlagDefault = 0,
    hipMemcpyFlagPreferOverlapWithCompute = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemcpyKind {
    hipMemcpyHostToHost = 0,
    hipMemcpyHostToDevice = 1,
    hipMemcpyDeviceToHost = 2,
    hipMemcpyDeviceToDevice = 3,
    hipMemcpyDefault = 4,
    hipMemcpyDeviceToDeviceNoCU = 1024,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemcpySrcAccessOrder {
    hipMemcpySrcAccessOrderInvalid = 0,
    hipMemcpySrcAccessOrderStream = 1,
    hipMemcpySrcAccessOrderDuringApiCall = 2,
    hipMemcpySrcAccessOrderAny = 3,
    hipMemcpySrcAccessOrderMax = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemoryAdvise {
    hipMemAdviseSetReadMostly = 1,
    hipMemAdviseUnsetReadMostly = 2,
    hipMemAdviseSetPreferredLocation = 3,
    hipMemAdviseUnsetPreferredLocation = 4,
    hipMemAdviseSetAccessedBy = 5,
    hipMemAdviseUnsetAccessedBy = 6,
    hipMemAdviseSetCoarseGrain = 100,
    hipMemAdviseUnsetCoarseGrain = 101,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipMemoryType {
    hipMemoryTypeUnregistered = 0,
    hipMemoryTypeHost = 1,
    hipMemoryTypeDevice = 2,
    hipMemoryTypeManaged = 3,
    hipMemoryTypeArray = 10,
    hipMemoryTypeUnified = 11,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipPointer_attribute {
    HIP_POINTER_ATTRIBUTE_CONTEXT = 1,
    HIP_POINTER_ATTRIBUTE_MEMORY_TYPE = 2,
    HIP_POINTER_ATTRIBUTE_DEVICE_POINTER = 3,
    HIP_POINTER_ATTRIBUTE_HOST_POINTER = 4,
    HIP_POINTER_ATTRIBUTE_P2P_TOKENS = 5,
    HIP_POINTER_ATTRIBUTE_SYNC_MEMOPS = 6,
    HIP_POINTER_ATTRIBUTE_BUFFER_ID = 7,
    HIP_POINTER_ATTRIBUTE_IS_MANAGED = 8,
    HIP_POINTER_ATTRIBUTE_DEVICE_ORDINAL = 9,
    HIP_POINTER_ATTRIBUTE_IS_LEGACY_HIP_IPC_CAPABLE = 10,
    HIP_POINTER_ATTRIBUTE_RANGE_START_ADDR = 11,
    HIP_POINTER_ATTRIBUTE_RANGE_SIZE = 12,
    HIP_POINTER_ATTRIBUTE_MAPPED = 13,
    HIP_POINTER_ATTRIBUTE_ALLOWED_HANDLE_TYPES = 14,
    HIP_POINTER_ATTRIBUTE_IS_GPU_DIRECT_RDMA_CAPABLE = 15,
    HIP_POINTER_ATTRIBUTE_ACCESS_FLAGS = 16,
    HIP_POINTER_ATTRIBUTE_MEMPOOL_HANDLE = 17,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipResourceType {
    hipResourceTypeArray = 0,
    hipResourceTypeMipmappedArray = 1,
    hipResourceTypeLinear = 2,
    hipResourceTypePitch2D = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipResourceViewFormat {
    hipResViewFormatNone = 0,
    hipResViewFormatUnsignedChar1 = 1,
    hipResViewFormatUnsignedChar2 = 2,
    hipResViewFormatUnsignedChar4 = 3,
    hipResViewFormatSignedChar1 = 4,
    hipResViewFormatSignedChar2 = 5,
    hipResViewFormatSignedChar4 = 6,
    hipResViewFormatUnsignedShort1 = 7,
    hipResViewFormatUnsignedShort2 = 8,
    hipResViewFormatUnsignedShort4 = 9,
    hipResViewFormatSignedShort1 = 10,
    hipResViewFormatSignedShort2 = 11,
    hipResViewFormatSignedShort4 = 12,
    hipResViewFormatUnsignedInt1 = 13,
    hipResViewFormatUnsignedInt2 = 14,
    hipResViewFormatUnsignedInt4 = 15,
    hipResViewFormatSignedInt1 = 16,
    hipResViewFormatSignedInt2 = 17,
    hipResViewFormatSignedInt4 = 18,
    hipResViewFormatHalf1 = 19,
    hipResViewFormatHalf2 = 20,
    hipResViewFormatHalf4 = 21,
    hipResViewFormatFloat1 = 22,
    hipResViewFormatFloat2 = 23,
    hipResViewFormatFloat4 = 24,
    hipResViewFormatUnsignedBlockCompressed1 = 25,
    hipResViewFormatUnsignedBlockCompressed2 = 26,
    hipResViewFormatUnsignedBlockCompressed3 = 27,
    hipResViewFormatUnsignedBlockCompressed4 = 28,
    hipResViewFormatSignedBlockCompressed4 = 29,
    hipResViewFormatUnsignedBlockCompressed5 = 30,
    hipResViewFormatSignedBlockCompressed5 = 31,
    hipResViewFormatUnsignedBlockCompressed6H = 32,
    hipResViewFormatSignedBlockCompressed6H = 33,
    hipResViewFormatUnsignedBlockCompressed7 = 34,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipSharedMemConfig {
    hipSharedMemBankSizeDefault = 0,
    hipSharedMemBankSizeFourByte = 1,
    hipSharedMemBankSizeEightByte = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipStreamBatchMemOpType {
    hipStreamMemOpWaitValue32 = 1,
    hipStreamMemOpWriteValue32 = 2,
    hipStreamMemOpWaitValue64 = 4,
    hipStreamMemOpWriteValue64 = 5,
    hipStreamMemOpBarrier = 6,
    hipStreamMemOpFlushRemoteWrites = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipStreamCaptureMode {
    hipStreamCaptureModeGlobal = 0,
    hipStreamCaptureModeThreadLocal = 1,
    hipStreamCaptureModeRelaxed = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipStreamCaptureStatus {
    hipStreamCaptureStatusNone = 0,
    hipStreamCaptureStatusActive = 1,
    hipStreamCaptureStatusInvalidated = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipStreamUpdateCaptureDependenciesFlags {
    hipStreamAddCaptureDependencies = 0,
    hipStreamSetCaptureDependencies = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipSurfaceBoundaryMode {
    hipBoundaryModeZero = 0,
    hipBoundaryModeTrap = 1,
    hipBoundaryModeClamp = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipSynchronizationPolicy {
    hipSyncPolicyAuto = 1,
    hipSyncPolicySpin = 2,
    hipSyncPolicyYield = 3,
    hipSyncPolicyBlockingSync = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipTextureAddressMode {
    hipAddressModeWrap = 0,
    hipAddressModeClamp = 1,
    hipAddressModeMirror = 2,
    hipAddressModeBorder = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipTextureFilterMode {
    hipFilterModePoint = 0,
    hipFilterModeLinear = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipTextureReadMode {
    hipReadModeElementType = 0,
    hipReadModeNormalizedFloat = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipUserObjectFlags {
    hipUserObjectNoDestructorSync = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum hipUserObjectRetainFlags {
    hipGraphUserObjectMove = 1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct HIP_ARRAY3D_DESCRIPTOR {
    pub Width: usize,
    pub Height: usize,
    pub Depth: usize,
    pub Format: hipArray_Format,
    pub NumChannels: ::core::ffi::c_uint,
    pub Flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct HIP_ARRAY_DESCRIPTOR {
    pub Width: usize,
    pub Height: usize,
    pub Format: hipArray_Format,
    pub NumChannels: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct HIP_LAUNCH_CONFIG_st {
    pub gridDimX: ::core::ffi::c_uint,
    pub gridDimY: ::core::ffi::c_uint,
    pub gridDimZ: ::core::ffi::c_uint,
    pub blockDimX: ::core::ffi::c_uint,
    pub blockDimY: ::core::ffi::c_uint,
    pub blockDimZ: ::core::ffi::c_uint,
    pub sharedMemBytes: ::core::ffi::c_uint,
    pub hStream: hipStream_t,
    pub attrs: *mut hipLaunchAttribute,
    pub numAttrs: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct HIP_MEMCPY3D {
    pub srcXInBytes: usize,
    pub srcY: usize,
    pub srcZ: usize,
    pub srcLOD: usize,
    pub srcMemoryType: hipMemoryType,
    pub srcHost: *const ::core::ffi::c_void,
    pub srcDevice: hipDeviceptr_t,
    pub srcArray: hipArray_t,
    pub srcPitch: usize,
    pub srcHeight: usize,
    pub dstXInBytes: usize,
    pub dstY: usize,
    pub dstZ: usize,
    pub dstLOD: usize,
    pub dstMemoryType: hipMemoryType,
    pub dstHost: *mut ::core::ffi::c_void,
    pub dstDevice: hipDeviceptr_t,
    pub dstArray: hipArray_t,
    pub dstPitch: usize,
    pub dstHeight: usize,
    pub WidthInBytes: usize,
    pub Height: usize,
    pub Depth: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HIP_RESOURCE_DESC_st {
    pub resType: HIPresourcetype,
    pub res: HIP_RESOURCE_DESC_st__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1 {
    pub hArray: hipArray_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_2 {
    pub hMipmappedArray: hipMipmappedArray_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_3 {
    pub devPtr: hipDeviceptr_t,
    pub format: hipArray_Format,
    pub numChannels: ::core::ffi::c_uint,
    pub sizeInBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_4 {
    pub devPtr: hipDeviceptr_t,
    pub format: hipArray_Format,
    pub numChannels: ::core::ffi::c_uint,
    pub width: usize,
    pub height: usize,
    pub pitchInBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_5 {
    pub reserved: [::core::ffi::c_int; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct HIP_RESOURCE_VIEW_DESC_st {
    pub format: HIPresourceViewFormat,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub firstMipmapLevel: ::core::ffi::c_uint,
    pub lastMipmapLevel: ::core::ffi::c_uint,
    pub firstLayer: ::core::ffi::c_uint,
    pub lastLayer: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct HIP_TEXTURE_DESC_st {
    pub addressMode: [HIPaddress_mode; 3usize],
    pub filterMode: HIPfilter_mode,
    pub flags: ::core::ffi::c_uint,
    pub maxAnisotropy: ::core::ffi::c_uint,
    pub mipmapFilterMode: HIPfilter_mode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
    pub borderColor: [f32; 4usize],
    pub reserved: [::core::ffi::c_int; 12usize],
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __hip_surface {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __hip_texture {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _hipGraphicsResource {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct dim3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct hipAccessPolicyWindow {
    pub base_ptr: *mut ::core::ffi::c_void,
    pub hitProp: hipAccessProperty,
    pub hitRatio: f32,
    pub missProp: hipAccessProperty,
    pub num_bytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipArray {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipArrayMapInfo {
    pub resourceType: hipResourceType,
    pub resource: hipArrayMapInfo__bindgen_ty_1,
    pub subresourceType: hipArraySparseSubresourceType,
    pub subresource: hipArrayMapInfo__bindgen_ty_2,
    pub memOperationType: hipMemOperationType,
    pub memHandleType: hipMemHandleType,
    pub memHandle: hipArrayMapInfo__bindgen_ty_3,
    pub offset: ::core::ffi::c_ulonglong,
    pub deviceBitMask: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipArrayMapInfo__bindgen_ty_2__bindgen_ty_1 {
    pub level: ::core::ffi::c_uint,
    pub layer: ::core::ffi::c_uint,
    pub offsetX: ::core::ffi::c_uint,
    pub offsetY: ::core::ffi::c_uint,
    pub offsetZ: ::core::ffi::c_uint,
    pub extentWidth: ::core::ffi::c_uint,
    pub extentHeight: ::core::ffi::c_uint,
    pub extentDepth: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipArrayMapInfo__bindgen_ty_2__bindgen_ty_2 {
    pub layer: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_ulonglong,
    pub size: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipBatchMemOpNodeParams {
    pub ctx: hipCtx_t,
    pub count: ::core::ffi::c_uint,
    pub paramArray: *mut hipStreamBatchMemOpParams,
    pub flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipChannelFormatDesc {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub z: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub f: hipChannelFormatKind,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipChildGraphNodeParams {
    pub graph: hipGraph_t,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipDeviceArch_t {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
    pub __bindgen_padding_0: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipDeviceProp_tR0600 {
    pub name: [::core::ffi::c_char; 256usize],
    pub uuid: hipUUID,
    pub luid: [::core::ffi::c_char; 8usize],
    pub luidDeviceNodeMask: ::core::ffi::c_uint,
    pub totalGlobalMem: usize,
    pub sharedMemPerBlock: usize,
    pub regsPerBlock: ::core::ffi::c_int,
    pub warpSize: ::core::ffi::c_int,
    pub memPitch: usize,
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    pub maxThreadsDim: [::core::ffi::c_int; 3usize],
    pub maxGridSize: [::core::ffi::c_int; 3usize],
    pub clockRate: ::core::ffi::c_int,
    pub totalConstMem: usize,
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
    pub textureAlignment: usize,
    pub texturePitchAlignment: usize,
    pub deviceOverlap: ::core::ffi::c_int,
    pub multiProcessorCount: ::core::ffi::c_int,
    pub kernelExecTimeoutEnabled: ::core::ffi::c_int,
    pub integrated: ::core::ffi::c_int,
    pub canMapHostMemory: ::core::ffi::c_int,
    pub computeMode: ::core::ffi::c_int,
    pub maxTexture1D: ::core::ffi::c_int,
    pub maxTexture1DMipmap: ::core::ffi::c_int,
    pub maxTexture1DLinear: ::core::ffi::c_int,
    pub maxTexture2D: [::core::ffi::c_int; 2usize],
    pub maxTexture2DMipmap: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLinear: [::core::ffi::c_int; 3usize],
    pub maxTexture2DGather: [::core::ffi::c_int; 2usize],
    pub maxTexture3D: [::core::ffi::c_int; 3usize],
    pub maxTexture3DAlt: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemap: ::core::ffi::c_int,
    pub maxTexture1DLayered: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLayered: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemapLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface1D: ::core::ffi::c_int,
    pub maxSurface2D: [::core::ffi::c_int; 2usize],
    pub maxSurface3D: [::core::ffi::c_int; 3usize],
    pub maxSurface1DLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface2DLayered: [::core::ffi::c_int; 3usize],
    pub maxSurfaceCubemap: ::core::ffi::c_int,
    pub maxSurfaceCubemapLayered: [::core::ffi::c_int; 2usize],
    pub surfaceAlignment: usize,
    pub concurrentKernels: ::core::ffi::c_int,
    pub ECCEnabled: ::core::ffi::c_int,
    pub pciBusID: ::core::ffi::c_int,
    pub pciDeviceID: ::core::ffi::c_int,
    pub pciDomainID: ::core::ffi::c_int,
    pub tccDriver: ::core::ffi::c_int,
    pub asyncEngineCount: ::core::ffi::c_int,
    pub unifiedAddressing: ::core::ffi::c_int,
    pub memoryClockRate: ::core::ffi::c_int,
    pub memoryBusWidth: ::core::ffi::c_int,
    pub l2CacheSize: ::core::ffi::c_int,
    pub persistingL2CacheMaxSize: ::core::ffi::c_int,
    pub maxThreadsPerMultiProcessor: ::core::ffi::c_int,
    pub streamPrioritiesSupported: ::core::ffi::c_int,
    pub globalL1CacheSupported: ::core::ffi::c_int,
    pub localL1CacheSupported: ::core::ffi::c_int,
    pub sharedMemPerMultiprocessor: usize,
    pub regsPerMultiprocessor: ::core::ffi::c_int,
    pub managedMemory: ::core::ffi::c_int,
    pub isMultiGpuBoard: ::core::ffi::c_int,
    pub multiGpuBoardGroupID: ::core::ffi::c_int,
    pub hostNativeAtomicSupported: ::core::ffi::c_int,
    pub singleToDoublePrecisionPerfRatio: ::core::ffi::c_int,
    pub pageableMemoryAccess: ::core::ffi::c_int,
    pub concurrentManagedAccess: ::core::ffi::c_int,
    pub computePreemptionSupported: ::core::ffi::c_int,
    pub canUseHostPointerForRegisteredMem: ::core::ffi::c_int,
    pub cooperativeLaunch: ::core::ffi::c_int,
    pub cooperativeMultiDeviceLaunch: ::core::ffi::c_int,
    pub sharedMemPerBlockOptin: usize,
    pub pageableMemoryAccessUsesHostPageTables: ::core::ffi::c_int,
    pub directManagedMemAccessFromHost: ::core::ffi::c_int,
    pub maxBlocksPerMultiProcessor: ::core::ffi::c_int,
    pub accessPolicyMaxWindowSize: ::core::ffi::c_int,
    pub reservedSharedMemPerBlock: usize,
    pub hostRegisterSupported: ::core::ffi::c_int,
    pub sparseHipArraySupported: ::core::ffi::c_int,
    pub hostRegisterReadOnlySupported: ::core::ffi::c_int,
    pub timelineSemaphoreInteropSupported: ::core::ffi::c_int,
    pub memoryPoolsSupported: ::core::ffi::c_int,
    pub gpuDirectRDMASupported: ::core::ffi::c_int,
    pub gpuDirectRDMAFlushWritesOptions: ::core::ffi::c_uint,
    pub gpuDirectRDMAWritesOrdering: ::core::ffi::c_int,
    pub memoryPoolSupportedHandleTypes: ::core::ffi::c_uint,
    pub deferredMappingHipArraySupported: ::core::ffi::c_int,
    pub ipcEventSupported: ::core::ffi::c_int,
    pub clusterLaunch: ::core::ffi::c_int,
    pub unifiedFunctionPointers: ::core::ffi::c_int,
    pub reserved: [::core::ffi::c_int; 63usize],
    pub hipReserved: [::core::ffi::c_int; 32usize],
    pub gcnArchName: [::core::ffi::c_char; 256usize],
    pub maxSharedMemoryPerMultiProcessor: usize,
    pub clockInstructionRate: ::core::ffi::c_int,
    pub arch: hipDeviceArch_t,
    pub hdpMemFlushCntl: *mut ::core::ffi::c_uint,
    pub hdpRegFlushCntl: *mut ::core::ffi::c_uint,
    pub cooperativeMultiDeviceUnmatchedFunc: ::core::ffi::c_int,
    pub cooperativeMultiDeviceUnmatchedGridDim: ::core::ffi::c_int,
    pub cooperativeMultiDeviceUnmatchedBlockDim: ::core::ffi::c_int,
    pub cooperativeMultiDeviceUnmatchedSharedMem: ::core::ffi::c_int,
    pub isLargeBar: ::core::ffi::c_int,
    pub asicRevision: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipEventRecordNodeParams {
    pub event: hipEvent_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipEventWaitNodeParams {
    pub event: hipEvent_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipExtent {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipExternalMemoryBufferDesc_st {
    pub offset: ::core::ffi::c_ulonglong,
    pub size: ::core::ffi::c_ulonglong,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipExternalMemoryHandleDesc_st {
    pub type_: hipExternalMemoryHandleType,
    pub handle: hipExternalMemoryHandleDesc_st__bindgen_ty_1,
    pub size: ::core::ffi::c_ulonglong,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipExternalMemoryHandleDesc_st__bindgen_ty_1__bindgen_ty_1 {
    pub handle: *mut ::core::ffi::c_void,
    pub name: *const ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipExternalMemoryMipmappedArrayDesc_st {
    pub offset: ::core::ffi::c_ulonglong,
    pub formatDesc: hipChannelFormatDesc,
    pub extent: hipExtent,
    pub flags: ::core::ffi::c_uint,
    pub numLevels: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipExternalSemaphoreHandleDesc_st {
    pub type_: hipExternalSemaphoreHandleType,
    pub handle: hipExternalSemaphoreHandleDesc_st__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipExternalSemaphoreHandleDesc_st__bindgen_ty_1__bindgen_ty_1 {
    pub handle: *mut ::core::ffi::c_void,
    pub name: *const ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipExternalSemaphoreSignalNodeParams {
    pub extSemArray: *mut hipExternalSemaphore_t,
    pub paramsArray: *const hipExternalSemaphoreSignalParams,
    pub numExtSems: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipExternalSemaphoreSignalParams_st {
    pub params: hipExternalSemaphoreSignalParams_st__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipExternalSemaphoreSignalParams_st__bindgen_ty_1 {
    pub fence: hipExternalSemaphoreSignalParams_st__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSync: hipExternalSemaphoreSignalParams_st__bindgen_ty_1__bindgen_ty_2,
    pub keyedMutex: hipExternalSemaphoreSignalParams_st__bindgen_ty_1__bindgen_ty_3,
    pub reserved: [::core::ffi::c_uint; 12usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipExternalSemaphoreSignalParams_st__bindgen_ty_1__bindgen_ty_1 {
    pub value: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipExternalSemaphoreSignalParams_st__bindgen_ty_1__bindgen_ty_3 {
    pub key: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipExternalSemaphoreWaitNodeParams {
    pub extSemArray: *mut hipExternalSemaphore_t,
    pub paramsArray: *const hipExternalSemaphoreWaitParams,
    pub numExtSems: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipExternalSemaphoreWaitParams_st {
    pub params: hipExternalSemaphoreWaitParams_st__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipExternalSemaphoreWaitParams_st__bindgen_ty_1 {
    pub fence: hipExternalSemaphoreWaitParams_st__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSync: hipExternalSemaphoreWaitParams_st__bindgen_ty_1__bindgen_ty_2,
    pub keyedMutex: hipExternalSemaphoreWaitParams_st__bindgen_ty_1__bindgen_ty_3,
    pub reserved: [::core::ffi::c_uint; 10usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipExternalSemaphoreWaitParams_st__bindgen_ty_1__bindgen_ty_1 {
    pub value: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipExternalSemaphoreWaitParams_st__bindgen_ty_1__bindgen_ty_3 {
    pub key: ::core::ffi::c_ulonglong,
    pub timeoutMs: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipFuncAttributes {
    pub binaryVersion: ::core::ffi::c_int,
    pub cacheModeCA: ::core::ffi::c_int,
    pub constSizeBytes: usize,
    pub localSizeBytes: usize,
    pub maxDynamicSharedSizeBytes: ::core::ffi::c_int,
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    pub numRegs: ::core::ffi::c_int,
    pub preferredShmemCarveout: ::core::ffi::c_int,
    pub ptxVersion: ::core::ffi::c_int,
    pub sharedSizeBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipFunctionLaunchParams_t {
    pub function: hipFunction_t,
    pub gridDimX: ::core::ffi::c_uint,
    pub gridDimY: ::core::ffi::c_uint,
    pub gridDimZ: ::core::ffi::c_uint,
    pub blockDimX: ::core::ffi::c_uint,
    pub blockDimY: ::core::ffi::c_uint,
    pub blockDimZ: ::core::ffi::c_uint,
    pub sharedMemBytes: ::core::ffi::c_uint,
    pub hStream: hipStream_t,
    pub kernelParams: *mut *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipGraphEdgeData {
    pub from_port: ::core::ffi::c_uchar,
    pub reserved: [::core::ffi::c_uchar; 5usize],
    pub to_port: ::core::ffi::c_uchar,
    pub type_: ::core::ffi::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipGraphExec {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipGraphInstantiateParams {
    pub errNode_out: hipGraphNode_t,
    pub flags: ::core::ffi::c_ulonglong,
    pub result_out: hipGraphInstantiateResult,
    pub uploadStream: hipStream_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipGraphNode {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipGraphNodeParams {
    pub type_: hipGraphNodeType,
    pub reserved0: [::core::ffi::c_int; 3usize],
    pub __bindgen_anon_1: hipGraphNodeParams__bindgen_ty_1,
    pub reserved2: ::core::ffi::c_longlong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipHostNodeParams {
    pub fn_: hipHostFn_t,
    pub userData: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipIpcEventHandle_st {
    pub reserved: [::core::ffi::c_char; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipIpcMemHandle_st {
    pub reserved: [::core::ffi::c_char; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipKernelNodeParams {
    pub blockDim: dim3,
    pub extra: *mut *mut ::core::ffi::c_void,
    pub func: *mut ::core::ffi::c_void,
    pub gridDim: dim3,
    pub kernelParams: *mut *mut ::core::ffi::c_void,
    pub sharedMemBytes: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipLaunchAttribute_st {
    pub id: hipLaunchAttributeID,
    pub pad: [::core::ffi::c_char; 4usize],
    pub __bindgen_anon_1: hipLaunchAttribute_st__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipLaunchConfig_st {
    pub gridDim: dim3,
    pub blockDim: dim3,
    pub dynamicSmemBytes: usize,
    pub stream: hipStream_t,
    pub attrs: *mut hipLaunchAttribute,
    pub numAttrs: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipLaunchMemSyncDomainMap {
    pub default_: ::core::ffi::c_uchar,
    pub remote: ::core::ffi::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipLaunchParams_t {
    pub func: *mut ::core::ffi::c_void,
    pub gridDim: dim3,
    pub blockDim: dim3,
    pub args: *mut *mut ::core::ffi::c_void,
    pub sharedMem: usize,
    pub stream: hipStream_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemAccessDesc {
    pub location: hipMemLocation,
    pub flags: hipMemAccessFlags,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemAllocNodeParams {
    pub poolProps: hipMemPoolProps,
    pub accessDescs: *const hipMemAccessDesc,
    pub accessDescCount: usize,
    pub bytesize: usize,
    pub dptr: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipMemAllocationProp {
    pub type_: hipMemAllocationType,
    pub __bindgen_anon_1: hipMemAllocationProp__bindgen_ty_1,
    pub location: hipMemLocation,
    pub win32HandleMetaData: *mut ::core::ffi::c_void,
    pub allocFlags: hipMemAllocationProp__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemAllocationProp__bindgen_ty_2 {
    pub compressionType: ::core::ffi::c_uchar,
    pub gpuDirectRDMACapable: ::core::ffi::c_uchar,
    pub usage: ::core::ffi::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemFreeNodeParams {
    pub dptr: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemLocation {
    pub type_: hipMemLocationType,
    pub id: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemPoolProps {
    pub allocType: hipMemAllocationType,
    pub handleTypes: hipMemAllocationHandleType,
    pub location: hipMemLocation,
    pub win32SecurityAttributes: *mut ::core::ffi::c_void,
    pub maxSize: usize,
    pub reserved: [::core::ffi::c_uchar; 56usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemPoolPtrExportData {
    pub reserved: [::core::ffi::c_uchar; 64usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipMemcpy3DBatchOp {
    pub src: hipMemcpy3DOperand,
    pub dst: hipMemcpy3DOperand,
    pub extent: hipExtent,
    pub srcAccessOrder: hipMemcpySrcAccessOrder,
    pub flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipMemcpy3DOperand {
    pub type_: hipMemcpy3DOperandType,
    pub op: hipMemcpy3DOperand__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemcpy3DOperand__bindgen_ty_1__bindgen_ty_1 {
    pub ptr: *mut ::core::ffi::c_void,
    pub rowLength: usize,
    pub layerHeight: usize,
    pub locHint: hipMemLocation,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemcpy3DOperand__bindgen_ty_1__bindgen_ty_2 {
    pub array: hipArray_t,
    pub offset: hipOffset3D,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemcpy3DParms {
    pub srcArray: hipArray_t,
    pub srcPos: hipPos,
    pub srcPtr: hipPitchedPtr,
    pub dstArray: hipArray_t,
    pub dstPos: hipPos,
    pub dstPtr: hipPitchedPtr,
    pub extent: hipExtent,
    pub kind: hipMemcpyKind,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemcpy3DPeerParms {
    pub srcArray: hipArray_t,
    pub srcPos: hipPos,
    pub srcPtr: hipPitchedPtr,
    pub srcDevice: ::core::ffi::c_int,
    pub dstArray: hipArray_t,
    pub dstPos: hipPos,
    pub dstPtr: hipPitchedPtr,
    pub dstDevice: ::core::ffi::c_int,
    pub extent: hipExtent,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemcpyAttributes {
    pub srcAccessOrder: hipMemcpySrcAccessOrder,
    pub srcLocHint: hipMemLocation,
    pub dstLocHint: hipMemLocation,
    pub flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemcpyNodeParams {
    pub flags: ::core::ffi::c_int,
    pub reserved: [::core::ffi::c_int; 3usize],
    pub copyParams: hipMemcpy3DParms,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMemsetParams {
    pub dst: *mut ::core::ffi::c_void,
    pub elementSize: ::core::ffi::c_uint,
    pub height: usize,
    pub pitch: usize,
    pub value: ::core::ffi::c_uint,
    pub width: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipMipmappedArray {
    pub data: *mut ::core::ffi::c_void,
    pub desc: hipChannelFormatDesc,
    pub type_: ::core::ffi::c_uint,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub depth: ::core::ffi::c_uint,
    pub min_mipmap_level: ::core::ffi::c_uint,
    pub max_mipmap_level: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
    pub format: hipArray_Format,
    pub num_channels: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipOffset3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipPitchedPtr {
    pub ptr: *mut ::core::ffi::c_void,
    pub pitch: usize,
    pub xsize: usize,
    pub ysize: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipPointerAttribute_t {
    pub type_: hipMemoryType,
    pub device: ::core::ffi::c_int,
    pub devicePointer: *mut ::core::ffi::c_void,
    pub hostPointer: *mut ::core::ffi::c_void,
    pub isManaged: ::core::ffi::c_int,
    pub allocationFlags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipPos {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipResourceDesc {
    pub resType: hipResourceType,
    pub res: hipResourceDesc__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipResourceDesc__bindgen_ty_1__bindgen_ty_1 {
    pub array: hipArray_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipResourceDesc__bindgen_ty_1__bindgen_ty_2 {
    pub mipmap: hipMipmappedArray_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipResourceDesc__bindgen_ty_1__bindgen_ty_3 {
    pub devPtr: *mut ::core::ffi::c_void,
    pub desc: hipChannelFormatDesc,
    pub sizeInBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipResourceDesc__bindgen_ty_1__bindgen_ty_4 {
    pub devPtr: *mut ::core::ffi::c_void,
    pub desc: hipChannelFormatDesc,
    pub width: usize,
    pub height: usize,
    pub pitchInBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipResourceViewDesc {
    pub format: hipResourceViewFormat,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub firstMipmapLevel: ::core::ffi::c_uint,
    pub lastMipmapLevel: ::core::ffi::c_uint,
    pub firstLayer: ::core::ffi::c_uint,
    pub lastLayer: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipStreamBatchMemOpParams_union_hipStreamMemOpFlushRemoteWritesParams_t {
    pub operation: hipStreamBatchMemOpType,
    pub flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipStreamBatchMemOpParams_union_hipStreamMemOpMemoryBarrierParams_t {
    pub operation: hipStreamBatchMemOpType,
    pub flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipStreamBatchMemOpParams_union_hipStreamMemOpWaitValueParams_t {
    pub operation: hipStreamBatchMemOpType,
    pub address: hipDeviceptr_t,
    pub __bindgen_anon_1: hipStreamBatchMemOpParams_union_hipStreamMemOpWaitValueParams_t__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    pub alias: hipDeviceptr_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipStreamBatchMemOpParams_union_hipStreamMemOpWriteValueParams_t {
    pub operation: hipStreamBatchMemOpType,
    pub address: hipDeviceptr_t,
    pub __bindgen_anon_1: hipStreamBatchMemOpParams_union_hipStreamMemOpWriteValueParams_t__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    pub alias: hipDeviceptr_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct hipTextureDesc {
    pub addressMode: [hipTextureAddressMode; 3usize],
    pub filterMode: hipTextureFilterMode,
    pub readMode: hipTextureReadMode,
    pub sRGB: ::core::ffi::c_int,
    pub borderColor: [f32; 4usize],
    pub normalizedCoords: ::core::ffi::c_int,
    pub maxAnisotropy: ::core::ffi::c_uint,
    pub mipmapFilterMode: hipTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipUUID_t {
    pub bytes: [::core::ffi::c_char; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipUserObject {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hip_Memcpy2D {
    pub srcXInBytes: usize,
    pub srcY: usize,
    pub srcMemoryType: hipMemoryType,
    pub srcHost: *const ::core::ffi::c_void,
    pub srcDevice: hipDeviceptr_t,
    pub srcArray: hipArray_t,
    pub srcPitch: usize,
    pub dstXInBytes: usize,
    pub dstY: usize,
    pub dstMemoryType: hipMemoryType,
    pub dstHost: *mut ::core::ffi::c_void,
    pub dstDevice: hipDeviceptr_t,
    pub dstArray: hipArray_t,
    pub dstPitch: usize,
    pub WidthInBytes: usize,
    pub Height: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipCtx_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipEvent_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipGraph {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipKernel_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipLibrary_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipLinkState_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipMemGenericAllocationHandle {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipMemPoolHandle_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipModuleSymbol_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipModule_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipStream_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct textureReference {
    pub normalized: ::core::ffi::c_int,
    pub readMode: hipTextureReadMode,
    pub filterMode: hipTextureFilterMode,
    pub addressMode: [hipTextureAddressMode; 3usize],
    pub channelDesc: hipChannelFormatDesc,
    pub sRGB: ::core::ffi::c_int,
    pub maxAnisotropy: ::core::ffi::c_uint,
    pub mipmapFilterMode: hipTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
    pub textureObject: hipTextureObject_t,
    pub numChannels: ::core::ffi::c_int,
    pub format: hipArray_Format,
}
impl hipDeviceArch_t {
    #[inline]
    pub fn hasGlobalInt32Atomics(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasGlobalInt32Atomics(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasGlobalInt32Atomics_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 0usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasGlobalInt32Atomics_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasGlobalFloatAtomicExch(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasGlobalFloatAtomicExch(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasGlobalFloatAtomicExch_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 1usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasGlobalFloatAtomicExch_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasSharedInt32Atomics(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasSharedInt32Atomics(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasSharedInt32Atomics_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 2usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasSharedInt32Atomics_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasSharedFloatAtomicExch(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasSharedFloatAtomicExch(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasSharedFloatAtomicExch_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 3usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasSharedFloatAtomicExch_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasFloatAtomicAdd(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasFloatAtomicAdd(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasFloatAtomicAdd_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 4usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasFloatAtomicAdd_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasGlobalInt64Atomics(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasGlobalInt64Atomics(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasGlobalInt64Atomics_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 5usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasGlobalInt64Atomics_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasSharedInt64Atomics(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasSharedInt64Atomics(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasSharedInt64Atomics_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 6usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasSharedInt64Atomics_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasDoubles(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasDoubles(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasDoubles_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 7usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasDoubles_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasWarpVote(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasWarpVote(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasWarpVote_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 8usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasWarpVote_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasWarpBallot(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasWarpBallot(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasWarpBallot_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 9usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasWarpBallot_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasWarpShuffle(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasWarpShuffle(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasWarpShuffle_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 10usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasWarpShuffle_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasFunnelShift(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasFunnelShift(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasFunnelShift_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 11usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasFunnelShift_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasThreadFenceSystem(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasThreadFenceSystem(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasThreadFenceSystem_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 12usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasThreadFenceSystem_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasSyncThreadsExt(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasSyncThreadsExt(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasSyncThreadsExt_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 13usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasSyncThreadsExt_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasSurfaceFuncs(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasSurfaceFuncs(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasSurfaceFuncs_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 14usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasSurfaceFuncs_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn has3dGrid(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_has3dGrid(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn has3dGrid_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 15usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_has3dGrid_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasDynamicParallelism(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasDynamicParallelism(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn hasDynamicParallelism_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 16usize, 1u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_hasDynamicParallelism_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 16usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(hasGlobalInt32Atomics: ::core::ffi::c_uint, hasGlobalFloatAtomicExch: ::core::ffi::c_uint, hasSharedInt32Atomics: ::core::ffi::c_uint, hasSharedFloatAtomicExch: ::core::ffi::c_uint, hasFloatAtomicAdd: ::core::ffi::c_uint, hasGlobalInt64Atomics: ::core::ffi::c_uint, hasSharedInt64Atomics: ::core::ffi::c_uint, hasDoubles: ::core::ffi::c_uint, hasWarpVote: ::core::ffi::c_uint, hasWarpBallot: ::core::ffi::c_uint, hasWarpShuffle: ::core::ffi::c_uint, hasFunnelShift: ::core::ffi::c_uint, hasThreadFenceSystem: ::core::ffi::c_uint, hasSyncThreadsExt: ::core::ffi::c_uint, hasSurfaceFuncs: ::core::ffi::c_uint, has3dGrid: ::core::ffi::c_uint, hasDynamicParallelism: ::core::ffi::c_uint) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let hasGlobalInt32Atomics: u32 = unsafe { ::core::mem::transmute(hasGlobalInt32Atomics) };
            hasGlobalInt32Atomics as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let hasGlobalFloatAtomicExch: u32 = unsafe { ::core::mem::transmute(hasGlobalFloatAtomicExch) };
            hasGlobalFloatAtomicExch as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let hasSharedInt32Atomics: u32 = unsafe { ::core::mem::transmute(hasSharedInt32Atomics) };
            hasSharedInt32Atomics as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let hasSharedFloatAtomicExch: u32 = unsafe { ::core::mem::transmute(hasSharedFloatAtomicExch) };
            hasSharedFloatAtomicExch as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let hasFloatAtomicAdd: u32 = unsafe { ::core::mem::transmute(hasFloatAtomicAdd) };
            hasFloatAtomicAdd as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let hasGlobalInt64Atomics: u32 = unsafe { ::core::mem::transmute(hasGlobalInt64Atomics) };
            hasGlobalInt64Atomics as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let hasSharedInt64Atomics: u32 = unsafe { ::core::mem::transmute(hasSharedInt64Atomics) };
            hasSharedInt64Atomics as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let hasDoubles: u32 = unsafe { ::core::mem::transmute(hasDoubles) };
            hasDoubles as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let hasWarpVote: u32 = unsafe { ::core::mem::transmute(hasWarpVote) };
            hasWarpVote as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let hasWarpBallot: u32 = unsafe { ::core::mem::transmute(hasWarpBallot) };
            hasWarpBallot as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let hasWarpShuffle: u32 = unsafe { ::core::mem::transmute(hasWarpShuffle) };
            hasWarpShuffle as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let hasFunnelShift: u32 = unsafe { ::core::mem::transmute(hasFunnelShift) };
            hasFunnelShift as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let hasThreadFenceSystem: u32 = unsafe { ::core::mem::transmute(hasThreadFenceSystem) };
            hasThreadFenceSystem as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let hasSyncThreadsExt: u32 = unsafe { ::core::mem::transmute(hasSyncThreadsExt) };
            hasSyncThreadsExt as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let hasSurfaceFuncs: u32 = unsafe { ::core::mem::transmute(hasSurfaceFuncs) };
            hasSurfaceFuncs as u64
        });
        __bindgen_bitfield_unit.set(15usize, 1u8, {
            let has3dGrid: u32 = unsafe { ::core::mem::transmute(has3dGrid) };
            has3dGrid as u64
        });
        __bindgen_bitfield_unit.set(16usize, 1u8, {
            let hasDynamicParallelism: u32 = unsafe { ::core::mem::transmute(hasDynamicParallelism) };
            hasDynamicParallelism as u64
        });
        __bindgen_bitfield_unit
    }
}
impl hipDeviceAttribute_t {
    pub const hipDeviceAttributeClockInstructionRate: hipDeviceAttribute_t = hipDeviceAttribute_t::hipDeviceAttributeAmdSpecificBegin;
}
impl hipDeviceAttribute_t {
    pub const hipDeviceAttributeEccEnabled: hipDeviceAttribute_t = hipDeviceAttribute_t::hipDeviceAttributeCudaCompatibleBegin;
}
impl hipDeviceAttribute_t {
    pub const hipDeviceAttributePciDomainID: hipDeviceAttribute_t = hipDeviceAttribute_t::hipDeviceAttributePciDomainId;
}
impl hipError_t {
    pub const hipErrorInitializationError: hipError_t = hipError_t::hipErrorNotInitialized;
}
impl hipError_t {
    pub const hipErrorInvalidResourceHandle: hipError_t = hipError_t::hipErrorInvalidHandle;
}
impl hipError_t {
    pub const hipErrorMapBufferObjectFailed: hipError_t = hipError_t::hipErrorMapFailed;
}
impl hipError_t {
    pub const hipErrorMemoryAllocation: hipError_t = hipError_t::hipErrorOutOfMemory;
}
impl hipMemLocationType {
    pub const hipMemLocationTypeNone: hipMemLocationType = hipMemLocationType::hipMemLocationTypeInvalid;
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") { 7 - (index % 8) } else { index % 8 };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe { *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize) };
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") { 7 - (index % 8) } else { index % 8 };
        let mask = 1 << bit_index;
        if val { byte | mask } else { byte & !mask }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe { (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize) };
        unsafe { *byte = Self::change_bit(*byte, index, val) };
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") { bit_width as usize - 1 - i } else { i };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if unsafe { Self::raw_get_bit(this, i + bit_offset) } {
                let index = if cfg!(target_endian = "big") { bit_width as usize - 1 - i } else { i };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") { bit_width as usize - 1 - i } else { i };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") { bit_width as usize - 1 - i } else { i };
            unsafe { Self::raw_set_bit(this, index + bit_offset, val_bit_is_set) };
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union HIP_RESOURCE_DESC_st__bindgen_ty_1 {
    pub array: HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1,
    pub mipmap: HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_2,
    pub linear: HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_3,
    pub pitch2D: HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_4,
    pub reserved: HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_5,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipArrayMapInfo__bindgen_ty_1 {
    pub mipmap: hipMipmappedArray,
    pub array: hipArray_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipArrayMapInfo__bindgen_ty_2 {
    pub sparseLevel: hipArrayMapInfo__bindgen_ty_2__bindgen_ty_1,
    pub miptail: hipArrayMapInfo__bindgen_ty_2__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipArrayMapInfo__bindgen_ty_3 {
    pub memHandle: hipMemGenericAllocationHandle_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipExternalMemoryHandleDesc_st__bindgen_ty_1 {
    pub fd: ::core::ffi::c_int,
    pub win32: hipExternalMemoryHandleDesc_st__bindgen_ty_1__bindgen_ty_1,
    pub nvSciBufObject: *const ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipExternalSemaphoreHandleDesc_st__bindgen_ty_1 {
    pub fd: ::core::ffi::c_int,
    pub win32: hipExternalSemaphoreHandleDesc_st__bindgen_ty_1__bindgen_ty_1,
    pub NvSciSyncObj: *const ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipExternalSemaphoreSignalParams_st__bindgen_ty_1__bindgen_ty_2 {
    pub fence: *mut ::core::ffi::c_void,
    pub reserved: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipExternalSemaphoreWaitParams_st__bindgen_ty_1__bindgen_ty_2 {
    pub fence: *mut ::core::ffi::c_void,
    pub reserved: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipGraphNodeParams__bindgen_ty_1 {
    pub reserved1: [::core::ffi::c_longlong; 29usize],
    pub kernel: hipKernelNodeParams,
    pub memcpy: hipMemcpyNodeParams,
    pub memset: hipMemsetParams,
    pub host: hipHostNodeParams,
    pub graph: hipChildGraphNodeParams,
    pub eventWait: hipEventWaitNodeParams,
    pub eventRecord: hipEventRecordNodeParams,
    pub extSemSignal: hipExternalSemaphoreSignalNodeParams,
    pub extSemWait: hipExternalSemaphoreWaitNodeParams,
    pub alloc: hipMemAllocNodeParams,
    pub free: hipMemFreeNodeParams,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipLaunchAttributeValue {
    pub pad: [::core::ffi::c_char; 64usize],
    pub accessPolicyWindow: hipAccessPolicyWindow,
    pub cooperative: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub syncPolicy: hipSynchronizationPolicy,
    pub memSyncDomainMap: hipLaunchMemSyncDomainMap,
    pub memSyncDomain: hipLaunchMemSyncDomain,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipLaunchAttribute_st__bindgen_ty_1 {
    pub val: hipLaunchAttributeValue,
    pub value: hipLaunchAttributeValue,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipMemAllocationProp__bindgen_ty_1 {
    pub requestedHandleType: hipMemAllocationHandleType,
    pub requestedHandleTypes: hipMemAllocationHandleType,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipMemcpy3DOperand__bindgen_ty_1 {
    pub ptr: hipMemcpy3DOperand__bindgen_ty_1__bindgen_ty_1,
    pub array: hipMemcpy3DOperand__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipResourceDesc__bindgen_ty_1 {
    pub array: hipResourceDesc__bindgen_ty_1__bindgen_ty_1,
    pub mipmap: hipResourceDesc__bindgen_ty_1__bindgen_ty_2,
    pub linear: hipResourceDesc__bindgen_ty_1__bindgen_ty_3,
    pub pitch2D: hipResourceDesc__bindgen_ty_1__bindgen_ty_4,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipStreamBatchMemOpParams_union {
    pub operation: hipStreamBatchMemOpType,
    pub waitValue: hipStreamBatchMemOpParams_union_hipStreamMemOpWaitValueParams_t,
    pub writeValue: hipStreamBatchMemOpParams_union_hipStreamMemOpWriteValueParams_t,
    pub flushRemoteWrites: hipStreamBatchMemOpParams_union_hipStreamMemOpFlushRemoteWritesParams_t,
    pub memoryBarrier: hipStreamBatchMemOpParams_union_hipStreamMemOpMemoryBarrierParams_t,
    pub pad: [u64; 6usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipStreamBatchMemOpParams_union_hipStreamMemOpWaitValueParams_t__bindgen_ty_1 {
    pub value: u32,
    pub value64: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipStreamBatchMemOpParams_union_hipStreamMemOpWriteValueParams_t__bindgen_ty_1 {
    pub value: u32,
    pub value64: u64,
}
pub unsafe fn hipApiName(id: u32) -> *const ::core::ffi::c_char {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(u32) -> *const ::core::ffi::c_char;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipApiName") });
        unsafe { _f(id) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipApiName(id: u32) -> *const ::core::ffi::c_char;
        }
        unsafe { hipApiName(id) }
    }
}
pub unsafe fn hipArray3DCreate(array: *mut hipArray_t, pAllocateArray: *const HIP_ARRAY3D_DESCRIPTOR) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipArray_t, *const HIP_ARRAY3D_DESCRIPTOR) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipArray3DCreate") });
        unsafe { _f(array, pAllocateArray) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipArray3DCreate(array: *mut hipArray_t, pAllocateArray: *const HIP_ARRAY3D_DESCRIPTOR) -> hipError_t;
        }
        unsafe { hipArray3DCreate(array, pAllocateArray) }
    }
}
pub unsafe fn hipArray3DGetDescriptor(pArrayDescriptor: *mut HIP_ARRAY3D_DESCRIPTOR, array: hipArray_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut HIP_ARRAY3D_DESCRIPTOR, hipArray_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipArray3DGetDescriptor") });
        unsafe { _f(pArrayDescriptor, array) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipArray3DGetDescriptor(pArrayDescriptor: *mut HIP_ARRAY3D_DESCRIPTOR, array: hipArray_t) -> hipError_t;
        }
        unsafe { hipArray3DGetDescriptor(pArrayDescriptor, array) }
    }
}
pub unsafe fn hipArrayCreate(pHandle: *mut hipArray_t, pAllocateArray: *const HIP_ARRAY_DESCRIPTOR) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipArray_t, *const HIP_ARRAY_DESCRIPTOR) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipArrayCreate") });
        unsafe { _f(pHandle, pAllocateArray) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipArrayCreate(pHandle: *mut hipArray_t, pAllocateArray: *const HIP_ARRAY_DESCRIPTOR) -> hipError_t;
        }
        unsafe { hipArrayCreate(pHandle, pAllocateArray) }
    }
}
pub unsafe fn hipArrayDestroy(array: hipArray_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipArray_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipArrayDestroy") });
        unsafe { _f(array) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipArrayDestroy(array: hipArray_t) -> hipError_t;
        }
        unsafe { hipArrayDestroy(array) }
    }
}
pub unsafe fn hipArrayGetDescriptor(pArrayDescriptor: *mut HIP_ARRAY_DESCRIPTOR, array: hipArray_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut HIP_ARRAY_DESCRIPTOR, hipArray_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipArrayGetDescriptor") });
        unsafe { _f(pArrayDescriptor, array) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipArrayGetDescriptor(pArrayDescriptor: *mut HIP_ARRAY_DESCRIPTOR, array: hipArray_t) -> hipError_t;
        }
        unsafe { hipArrayGetDescriptor(pArrayDescriptor, array) }
    }
}
pub unsafe fn hipArrayGetInfo(desc: *mut hipChannelFormatDesc, extent: *mut hipExtent, flags: *mut ::core::ffi::c_uint, array: hipArray_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipChannelFormatDesc, *mut hipExtent, *mut ::core::ffi::c_uint, hipArray_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipArrayGetInfo") });
        unsafe { _f(desc, extent, flags, array) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipArrayGetInfo(desc: *mut hipChannelFormatDesc, extent: *mut hipExtent, flags: *mut ::core::ffi::c_uint, array: hipArray_t) -> hipError_t;
        }
        unsafe { hipArrayGetInfo(desc, extent, flags, array) }
    }
}
pub unsafe fn hipBindTexture(offset: *mut usize, tex: *const textureReference, devPtr: *const ::core::ffi::c_void, desc: *const hipChannelFormatDesc, size: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut usize, *const textureReference, *const ::core::ffi::c_void, *const hipChannelFormatDesc, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipBindTexture") });
        unsafe { _f(offset, tex, devPtr, desc, size) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipBindTexture(offset: *mut usize, tex: *const textureReference, devPtr: *const ::core::ffi::c_void, desc: *const hipChannelFormatDesc, size: usize) -> hipError_t;
        }
        unsafe { hipBindTexture(offset, tex, devPtr, desc, size) }
    }
}
pub unsafe fn hipBindTexture2D(offset: *mut usize, tex: *const textureReference, devPtr: *const ::core::ffi::c_void, desc: *const hipChannelFormatDesc, width: usize, height: usize, pitch: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut usize, *const textureReference, *const ::core::ffi::c_void, *const hipChannelFormatDesc, usize, usize, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipBindTexture2D") });
        unsafe { _f(offset, tex, devPtr, desc, width, height, pitch) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipBindTexture2D(offset: *mut usize, tex: *const textureReference, devPtr: *const ::core::ffi::c_void, desc: *const hipChannelFormatDesc, width: usize, height: usize, pitch: usize) -> hipError_t;
        }
        unsafe { hipBindTexture2D(offset, tex, devPtr, desc, width, height, pitch) }
    }
}
pub unsafe fn hipBindTextureToArray(tex: *const textureReference, array: hipArray_const_t, desc: *const hipChannelFormatDesc) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const textureReference, hipArray_const_t, *const hipChannelFormatDesc) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipBindTextureToArray") });
        unsafe { _f(tex, array, desc) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipBindTextureToArray(tex: *const textureReference, array: hipArray_const_t, desc: *const hipChannelFormatDesc) -> hipError_t;
        }
        unsafe { hipBindTextureToArray(tex, array, desc) }
    }
}
pub unsafe fn hipBindTextureToMipmappedArray(tex: *const textureReference, mipmappedArray: hipMipmappedArray_const_t, desc: *const hipChannelFormatDesc) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const textureReference, hipMipmappedArray_const_t, *const hipChannelFormatDesc) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipBindTextureToMipmappedArray") });
        unsafe { _f(tex, mipmappedArray, desc) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipBindTextureToMipmappedArray(tex: *const textureReference, mipmappedArray: hipMipmappedArray_const_t, desc: *const hipChannelFormatDesc) -> hipError_t;
        }
        unsafe { hipBindTextureToMipmappedArray(tex, mipmappedArray, desc) }
    }
}
pub unsafe fn hipChooseDeviceR0600(device: *mut ::core::ffi::c_int, prop: *const hipDeviceProp_tR0600) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, *const hipDeviceProp_tR0600) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipChooseDeviceR0600") });
        unsafe { _f(device, prop) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipChooseDeviceR0600(device: *mut ::core::ffi::c_int, prop: *const hipDeviceProp_tR0600) -> hipError_t;
        }
        unsafe { hipChooseDeviceR0600(device, prop) }
    }
}
pub unsafe fn hipConfigureCall(gridDim: dim3, blockDim: dim3, sharedMem: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(dim3, dim3, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipConfigureCall") });
        unsafe { _f(gridDim, blockDim, sharedMem, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipConfigureCall(gridDim: dim3, blockDim: dim3, sharedMem: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipConfigureCall(gridDim, blockDim, sharedMem, stream) }
    }
}
pub unsafe fn hipCreateChannelDesc(x: ::core::ffi::c_int, y: ::core::ffi::c_int, z: ::core::ffi::c_int, w: ::core::ffi::c_int, f: hipChannelFormatKind) -> hipChannelFormatDesc {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, hipChannelFormatKind) -> hipChannelFormatDesc;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCreateChannelDesc") });
        unsafe { _f(x, y, z, w, f) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCreateChannelDesc(x: ::core::ffi::c_int, y: ::core::ffi::c_int, z: ::core::ffi::c_int, w: ::core::ffi::c_int, f: hipChannelFormatKind) -> hipChannelFormatDesc;
        }
        unsafe { hipCreateChannelDesc(x, y, z, w, f) }
    }
}
pub unsafe fn hipCreateSurfaceObject(pSurfObject: *mut hipSurfaceObject_t, pResDesc: *const hipResourceDesc) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipSurfaceObject_t, *const hipResourceDesc) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCreateSurfaceObject") });
        unsafe { _f(pSurfObject, pResDesc) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCreateSurfaceObject(pSurfObject: *mut hipSurfaceObject_t, pResDesc: *const hipResourceDesc) -> hipError_t;
        }
        unsafe { hipCreateSurfaceObject(pSurfObject, pResDesc) }
    }
}
pub unsafe fn hipCreateTextureObject(pTexObject: *mut hipTextureObject_t, pResDesc: *const hipResourceDesc, pTexDesc: *const hipTextureDesc, pResViewDesc: *const hipResourceViewDesc) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipTextureObject_t, *const hipResourceDesc, *const hipTextureDesc, *const hipResourceViewDesc) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCreateTextureObject") });
        unsafe { _f(pTexObject, pResDesc, pTexDesc, pResViewDesc) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCreateTextureObject(pTexObject: *mut hipTextureObject_t, pResDesc: *const hipResourceDesc, pTexDesc: *const hipTextureDesc, pResViewDesc: *const hipResourceViewDesc) -> hipError_t;
        }
        unsafe { hipCreateTextureObject(pTexObject, pResDesc, pTexDesc, pResViewDesc) }
    }
}
pub unsafe fn hipCtxCreate(ctx: *mut hipCtx_t, flags: ::core::ffi::c_uint, device: hipDevice_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipCtx_t, ::core::ffi::c_uint, hipDevice_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxCreate") });
        unsafe { _f(ctx, flags, device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxCreate(ctx: *mut hipCtx_t, flags: ::core::ffi::c_uint, device: hipDevice_t) -> hipError_t;
        }
        unsafe { hipCtxCreate(ctx, flags, device) }
    }
}
pub unsafe fn hipCtxDestroy(ctx: hipCtx_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipCtx_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxDestroy") });
        unsafe { _f(ctx) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxDestroy(ctx: hipCtx_t) -> hipError_t;
        }
        unsafe { hipCtxDestroy(ctx) }
    }
}
pub unsafe fn hipCtxDisablePeerAccess(peerCtx: hipCtx_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipCtx_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxDisablePeerAccess") });
        unsafe { _f(peerCtx) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxDisablePeerAccess(peerCtx: hipCtx_t) -> hipError_t;
        }
        unsafe { hipCtxDisablePeerAccess(peerCtx) }
    }
}
pub unsafe fn hipCtxEnablePeerAccess(peerCtx: hipCtx_t, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipCtx_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxEnablePeerAccess") });
        unsafe { _f(peerCtx, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxEnablePeerAccess(peerCtx: hipCtx_t, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipCtxEnablePeerAccess(peerCtx, flags) }
    }
}
pub unsafe fn hipCtxGetApiVersion(ctx: hipCtx_t, apiVersion: *mut ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipCtx_t, *mut ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxGetApiVersion") });
        unsafe { _f(ctx, apiVersion) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxGetApiVersion(ctx: hipCtx_t, apiVersion: *mut ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipCtxGetApiVersion(ctx, apiVersion) }
    }
}
pub unsafe fn hipCtxGetCacheConfig(cacheConfig: *mut hipFuncCache_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipFuncCache_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxGetCacheConfig") });
        unsafe { _f(cacheConfig) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxGetCacheConfig(cacheConfig: *mut hipFuncCache_t) -> hipError_t;
        }
        unsafe { hipCtxGetCacheConfig(cacheConfig) }
    }
}
pub unsafe fn hipCtxGetCurrent(ctx: *mut hipCtx_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipCtx_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxGetCurrent") });
        unsafe { _f(ctx) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxGetCurrent(ctx: *mut hipCtx_t) -> hipError_t;
        }
        unsafe { hipCtxGetCurrent(ctx) }
    }
}
pub unsafe fn hipCtxGetDevice(device: *mut hipDevice_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipDevice_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxGetDevice") });
        unsafe { _f(device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxGetDevice(device: *mut hipDevice_t) -> hipError_t;
        }
        unsafe { hipCtxGetDevice(device) }
    }
}
pub unsafe fn hipCtxGetFlags(flags: *mut ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxGetFlags") });
        unsafe { _f(flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxGetFlags(flags: *mut ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipCtxGetFlags(flags) }
    }
}
pub unsafe fn hipCtxGetSharedMemConfig(pConfig: *mut hipSharedMemConfig) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipSharedMemConfig) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxGetSharedMemConfig") });
        unsafe { _f(pConfig) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxGetSharedMemConfig(pConfig: *mut hipSharedMemConfig) -> hipError_t;
        }
        unsafe { hipCtxGetSharedMemConfig(pConfig) }
    }
}
pub unsafe fn hipCtxPopCurrent(ctx: *mut hipCtx_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipCtx_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxPopCurrent") });
        unsafe { _f(ctx) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxPopCurrent(ctx: *mut hipCtx_t) -> hipError_t;
        }
        unsafe { hipCtxPopCurrent(ctx) }
    }
}
pub unsafe fn hipCtxPushCurrent(ctx: hipCtx_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipCtx_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxPushCurrent") });
        unsafe { _f(ctx) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxPushCurrent(ctx: hipCtx_t) -> hipError_t;
        }
        unsafe { hipCtxPushCurrent(ctx) }
    }
}
pub unsafe fn hipCtxSetCacheConfig(cacheConfig: hipFuncCache_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipFuncCache_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxSetCacheConfig") });
        unsafe { _f(cacheConfig) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxSetCacheConfig(cacheConfig: hipFuncCache_t) -> hipError_t;
        }
        unsafe { hipCtxSetCacheConfig(cacheConfig) }
    }
}
pub unsafe fn hipCtxSetCurrent(ctx: hipCtx_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipCtx_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxSetCurrent") });
        unsafe { _f(ctx) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxSetCurrent(ctx: hipCtx_t) -> hipError_t;
        }
        unsafe { hipCtxSetCurrent(ctx) }
    }
}
pub unsafe fn hipCtxSetSharedMemConfig(config: hipSharedMemConfig) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipSharedMemConfig) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxSetSharedMemConfig") });
        unsafe { _f(config) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxSetSharedMemConfig(config: hipSharedMemConfig) -> hipError_t;
        }
        unsafe { hipCtxSetSharedMemConfig(config) }
    }
}
pub unsafe fn hipCtxSynchronize() -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn() -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipCtxSynchronize") });
        unsafe { _f() }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipCtxSynchronize() -> hipError_t;
        }
        unsafe { hipCtxSynchronize() }
    }
}
pub unsafe fn hipDestroyExternalMemory(extMem: hipExternalMemory_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipExternalMemory_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDestroyExternalMemory") });
        unsafe { _f(extMem) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDestroyExternalMemory(extMem: hipExternalMemory_t) -> hipError_t;
        }
        unsafe { hipDestroyExternalMemory(extMem) }
    }
}
pub unsafe fn hipDestroyExternalSemaphore(extSem: hipExternalSemaphore_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipExternalSemaphore_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDestroyExternalSemaphore") });
        unsafe { _f(extSem) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDestroyExternalSemaphore(extSem: hipExternalSemaphore_t) -> hipError_t;
        }
        unsafe { hipDestroyExternalSemaphore(extSem) }
    }
}
pub unsafe fn hipDestroySurfaceObject(surfaceObject: hipSurfaceObject_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipSurfaceObject_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDestroySurfaceObject") });
        unsafe { _f(surfaceObject) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDestroySurfaceObject(surfaceObject: hipSurfaceObject_t) -> hipError_t;
        }
        unsafe { hipDestroySurfaceObject(surfaceObject) }
    }
}
pub unsafe fn hipDestroyTextureObject(textureObject: hipTextureObject_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipTextureObject_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDestroyTextureObject") });
        unsafe { _f(textureObject) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDestroyTextureObject(textureObject: hipTextureObject_t) -> hipError_t;
        }
        unsafe { hipDestroyTextureObject(textureObject) }
    }
}
pub unsafe fn hipDeviceCanAccessPeer(canAccessPeer: *mut ::core::ffi::c_int, deviceId: ::core::ffi::c_int, peerDeviceId: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceCanAccessPeer") });
        unsafe { _f(canAccessPeer, deviceId, peerDeviceId) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceCanAccessPeer(canAccessPeer: *mut ::core::ffi::c_int, deviceId: ::core::ffi::c_int, peerDeviceId: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDeviceCanAccessPeer(canAccessPeer, deviceId, peerDeviceId) }
    }
}
pub unsafe fn hipDeviceComputeCapability(major: *mut ::core::ffi::c_int, minor: *mut ::core::ffi::c_int, device: hipDevice_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, *mut ::core::ffi::c_int, hipDevice_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceComputeCapability") });
        unsafe { _f(major, minor, device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceComputeCapability(major: *mut ::core::ffi::c_int, minor: *mut ::core::ffi::c_int, device: hipDevice_t) -> hipError_t;
        }
        unsafe { hipDeviceComputeCapability(major, minor, device) }
    }
}
pub unsafe fn hipDeviceDisablePeerAccess(peerDeviceId: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceDisablePeerAccess") });
        unsafe { _f(peerDeviceId) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceDisablePeerAccess(peerDeviceId: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDeviceDisablePeerAccess(peerDeviceId) }
    }
}
pub unsafe fn hipDeviceEnablePeerAccess(peerDeviceId: ::core::ffi::c_int, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceEnablePeerAccess") });
        unsafe { _f(peerDeviceId, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceEnablePeerAccess(peerDeviceId: ::core::ffi::c_int, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipDeviceEnablePeerAccess(peerDeviceId, flags) }
    }
}
pub unsafe fn hipDeviceGet(device: *mut hipDevice_t, ordinal: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipDevice_t, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGet") });
        unsafe { _f(device, ordinal) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGet(device: *mut hipDevice_t, ordinal: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDeviceGet(device, ordinal) }
    }
}
pub unsafe fn hipDeviceGetAttribute(pi: *mut ::core::ffi::c_int, attr: hipDeviceAttribute_t, deviceId: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, hipDeviceAttribute_t, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetAttribute") });
        unsafe { _f(pi, attr, deviceId) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetAttribute(pi: *mut ::core::ffi::c_int, attr: hipDeviceAttribute_t, deviceId: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDeviceGetAttribute(pi, attr, deviceId) }
    }
}
pub unsafe fn hipDeviceGetByPCIBusId(device: *mut ::core::ffi::c_int, pciBusId: *const ::core::ffi::c_char) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, *const ::core::ffi::c_char) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetByPCIBusId") });
        unsafe { _f(device, pciBusId) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetByPCIBusId(device: *mut ::core::ffi::c_int, pciBusId: *const ::core::ffi::c_char) -> hipError_t;
        }
        unsafe { hipDeviceGetByPCIBusId(device, pciBusId) }
    }
}
pub unsafe fn hipDeviceGetCacheConfig(cacheConfig: *mut hipFuncCache_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipFuncCache_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetCacheConfig") });
        unsafe { _f(cacheConfig) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetCacheConfig(cacheConfig: *mut hipFuncCache_t) -> hipError_t;
        }
        unsafe { hipDeviceGetCacheConfig(cacheConfig) }
    }
}
pub unsafe fn hipDeviceGetDefaultMemPool(mem_pool: *mut hipMemPool_t, device: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMemPool_t, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetDefaultMemPool") });
        unsafe { _f(mem_pool, device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetDefaultMemPool(mem_pool: *mut hipMemPool_t, device: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDeviceGetDefaultMemPool(mem_pool, device) }
    }
}
pub unsafe fn hipDeviceGetGraphMemAttribute(device: ::core::ffi::c_int, attr: hipGraphMemAttributeType, value: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_int, hipGraphMemAttributeType, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetGraphMemAttribute") });
        unsafe { _f(device, attr, value) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetGraphMemAttribute(device: ::core::ffi::c_int, attr: hipGraphMemAttributeType, value: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipDeviceGetGraphMemAttribute(device, attr, value) }
    }
}
pub unsafe fn hipDeviceGetLimit(pValue: *mut usize, limit: hipLimit_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut usize, hipLimit_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetLimit") });
        unsafe { _f(pValue, limit) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetLimit(pValue: *mut usize, limit: hipLimit_t) -> hipError_t;
        }
        unsafe { hipDeviceGetLimit(pValue, limit) }
    }
}
pub unsafe fn hipDeviceGetMemPool(mem_pool: *mut hipMemPool_t, device: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMemPool_t, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetMemPool") });
        unsafe { _f(mem_pool, device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetMemPool(mem_pool: *mut hipMemPool_t, device: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDeviceGetMemPool(mem_pool, device) }
    }
}
pub unsafe fn hipDeviceGetName(name: *mut ::core::ffi::c_char, len: ::core::ffi::c_int, device: hipDevice_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_char, ::core::ffi::c_int, hipDevice_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetName") });
        unsafe { _f(name, len, device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetName(name: *mut ::core::ffi::c_char, len: ::core::ffi::c_int, device: hipDevice_t) -> hipError_t;
        }
        unsafe { hipDeviceGetName(name, len, device) }
    }
}
pub unsafe fn hipDeviceGetP2PAttribute(value: *mut ::core::ffi::c_int, attr: hipDeviceP2PAttr, srcDevice: ::core::ffi::c_int, dstDevice: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, hipDeviceP2PAttr, ::core::ffi::c_int, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetP2PAttribute") });
        unsafe { _f(value, attr, srcDevice, dstDevice) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetP2PAttribute(value: *mut ::core::ffi::c_int, attr: hipDeviceP2PAttr, srcDevice: ::core::ffi::c_int, dstDevice: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDeviceGetP2PAttribute(value, attr, srcDevice, dstDevice) }
    }
}
pub unsafe fn hipDeviceGetPCIBusId(pciBusId: *mut ::core::ffi::c_char, len: ::core::ffi::c_int, device: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_char, ::core::ffi::c_int, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetPCIBusId") });
        unsafe { _f(pciBusId, len, device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetPCIBusId(pciBusId: *mut ::core::ffi::c_char, len: ::core::ffi::c_int, device: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDeviceGetPCIBusId(pciBusId, len, device) }
    }
}
pub unsafe fn hipDeviceGetSharedMemConfig(pConfig: *mut hipSharedMemConfig) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipSharedMemConfig) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetSharedMemConfig") });
        unsafe { _f(pConfig) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetSharedMemConfig(pConfig: *mut hipSharedMemConfig) -> hipError_t;
        }
        unsafe { hipDeviceGetSharedMemConfig(pConfig) }
    }
}
pub unsafe fn hipDeviceGetStreamPriorityRange(leastPriority: *mut ::core::ffi::c_int, greatestPriority: *mut ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetStreamPriorityRange") });
        unsafe { _f(leastPriority, greatestPriority) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetStreamPriorityRange(leastPriority: *mut ::core::ffi::c_int, greatestPriority: *mut ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDeviceGetStreamPriorityRange(leastPriority, greatestPriority) }
    }
}
pub unsafe fn hipDeviceGetTexture1DLinearMaxWidth(max_width: *mut usize, desc: *const hipChannelFormatDesc, device: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut usize, *const hipChannelFormatDesc, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetTexture1DLinearMaxWidth") });
        unsafe { _f(max_width, desc, device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetTexture1DLinearMaxWidth(max_width: *mut usize, desc: *const hipChannelFormatDesc, device: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDeviceGetTexture1DLinearMaxWidth(max_width, desc, device) }
    }
}
pub unsafe fn hipDeviceGetUuid(uuid: *mut hipUUID, device: hipDevice_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipUUID, hipDevice_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGetUuid") });
        unsafe { _f(uuid, device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGetUuid(uuid: *mut hipUUID, device: hipDevice_t) -> hipError_t;
        }
        unsafe { hipDeviceGetUuid(uuid, device) }
    }
}
pub unsafe fn hipDeviceGraphMemTrim(device: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceGraphMemTrim") });
        unsafe { _f(device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceGraphMemTrim(device: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDeviceGraphMemTrim(device) }
    }
}
pub unsafe fn hipDevicePrimaryCtxGetState(dev: hipDevice_t, flags: *mut ::core::ffi::c_uint, active: *mut ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDevice_t, *mut ::core::ffi::c_uint, *mut ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDevicePrimaryCtxGetState") });
        unsafe { _f(dev, flags, active) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDevicePrimaryCtxGetState(dev: hipDevice_t, flags: *mut ::core::ffi::c_uint, active: *mut ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDevicePrimaryCtxGetState(dev, flags, active) }
    }
}
pub unsafe fn hipDevicePrimaryCtxRelease(dev: hipDevice_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDevice_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDevicePrimaryCtxRelease") });
        unsafe { _f(dev) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDevicePrimaryCtxRelease(dev: hipDevice_t) -> hipError_t;
        }
        unsafe { hipDevicePrimaryCtxRelease(dev) }
    }
}
pub unsafe fn hipDevicePrimaryCtxReset(dev: hipDevice_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDevice_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDevicePrimaryCtxReset") });
        unsafe { _f(dev) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDevicePrimaryCtxReset(dev: hipDevice_t) -> hipError_t;
        }
        unsafe { hipDevicePrimaryCtxReset(dev) }
    }
}
pub unsafe fn hipDevicePrimaryCtxRetain(pctx: *mut hipCtx_t, dev: hipDevice_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipCtx_t, hipDevice_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDevicePrimaryCtxRetain") });
        unsafe { _f(pctx, dev) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDevicePrimaryCtxRetain(pctx: *mut hipCtx_t, dev: hipDevice_t) -> hipError_t;
        }
        unsafe { hipDevicePrimaryCtxRetain(pctx, dev) }
    }
}
pub unsafe fn hipDevicePrimaryCtxSetFlags(dev: hipDevice_t, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDevice_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDevicePrimaryCtxSetFlags") });
        unsafe { _f(dev, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDevicePrimaryCtxSetFlags(dev: hipDevice_t, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipDevicePrimaryCtxSetFlags(dev, flags) }
    }
}
pub unsafe fn hipDeviceReset() -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn() -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceReset") });
        unsafe { _f() }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceReset() -> hipError_t;
        }
        unsafe { hipDeviceReset() }
    }
}
pub unsafe fn hipDeviceSetCacheConfig(cacheConfig: hipFuncCache_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipFuncCache_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceSetCacheConfig") });
        unsafe { _f(cacheConfig) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceSetCacheConfig(cacheConfig: hipFuncCache_t) -> hipError_t;
        }
        unsafe { hipDeviceSetCacheConfig(cacheConfig) }
    }
}
pub unsafe fn hipDeviceSetGraphMemAttribute(device: ::core::ffi::c_int, attr: hipGraphMemAttributeType, value: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_int, hipGraphMemAttributeType, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceSetGraphMemAttribute") });
        unsafe { _f(device, attr, value) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceSetGraphMemAttribute(device: ::core::ffi::c_int, attr: hipGraphMemAttributeType, value: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipDeviceSetGraphMemAttribute(device, attr, value) }
    }
}
pub unsafe fn hipDeviceSetLimit(limit: hipLimit_t, value: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipLimit_t, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceSetLimit") });
        unsafe { _f(limit, value) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceSetLimit(limit: hipLimit_t, value: usize) -> hipError_t;
        }
        unsafe { hipDeviceSetLimit(limit, value) }
    }
}
pub unsafe fn hipDeviceSetMemPool(device: ::core::ffi::c_int, mem_pool: hipMemPool_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_int, hipMemPool_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceSetMemPool") });
        unsafe { _f(device, mem_pool) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceSetMemPool(device: ::core::ffi::c_int, mem_pool: hipMemPool_t) -> hipError_t;
        }
        unsafe { hipDeviceSetMemPool(device, mem_pool) }
    }
}
pub unsafe fn hipDeviceSetSharedMemConfig(config: hipSharedMemConfig) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipSharedMemConfig) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceSetSharedMemConfig") });
        unsafe { _f(config) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceSetSharedMemConfig(config: hipSharedMemConfig) -> hipError_t;
        }
        unsafe { hipDeviceSetSharedMemConfig(config) }
    }
}
pub unsafe fn hipDeviceSynchronize() -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn() -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceSynchronize") });
        unsafe { _f() }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceSynchronize() -> hipError_t;
        }
        unsafe { hipDeviceSynchronize() }
    }
}
pub unsafe fn hipDeviceTotalMem(bytes: *mut usize, device: hipDevice_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut usize, hipDevice_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDeviceTotalMem") });
        unsafe { _f(bytes, device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDeviceTotalMem(bytes: *mut usize, device: hipDevice_t) -> hipError_t;
        }
        unsafe { hipDeviceTotalMem(bytes, device) }
    }
}
pub unsafe fn hipDriverGetVersion(driverVersion: *mut ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDriverGetVersion") });
        unsafe { _f(driverVersion) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDriverGetVersion(driverVersion: *mut ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipDriverGetVersion(driverVersion) }
    }
}
pub unsafe fn hipDrvGetErrorName(hipError: hipError_t, errorString: *mut *const ::core::ffi::c_char) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipError_t, *mut *const ::core::ffi::c_char) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvGetErrorName") });
        unsafe { _f(hipError, errorString) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvGetErrorName(hipError: hipError_t, errorString: *mut *const ::core::ffi::c_char) -> hipError_t;
        }
        unsafe { hipDrvGetErrorName(hipError, errorString) }
    }
}
pub unsafe fn hipDrvGetErrorString(hipError: hipError_t, errorString: *mut *const ::core::ffi::c_char) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipError_t, *mut *const ::core::ffi::c_char) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvGetErrorString") });
        unsafe { _f(hipError, errorString) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvGetErrorString(hipError: hipError_t, errorString: *mut *const ::core::ffi::c_char) -> hipError_t;
        }
        unsafe { hipDrvGetErrorString(hipError, errorString) }
    }
}
pub unsafe fn hipDrvGraphAddMemFreeNode(phGraphNode: *mut hipGraphNode_t, hGraph: hipGraph_t, dependencies: *const hipGraphNode_t, numDependencies: usize, dptr: hipDeviceptr_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, hipDeviceptr_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvGraphAddMemFreeNode") });
        unsafe { _f(phGraphNode, hGraph, dependencies, numDependencies, dptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvGraphAddMemFreeNode(phGraphNode: *mut hipGraphNode_t, hGraph: hipGraph_t, dependencies: *const hipGraphNode_t, numDependencies: usize, dptr: hipDeviceptr_t) -> hipError_t;
        }
        unsafe { hipDrvGraphAddMemFreeNode(phGraphNode, hGraph, dependencies, numDependencies, dptr) }
    }
}
pub unsafe fn hipDrvGraphAddMemcpyNode(phGraphNode: *mut hipGraphNode_t, hGraph: hipGraph_t, dependencies: *const hipGraphNode_t, numDependencies: usize, copyParams: *const HIP_MEMCPY3D, ctx: hipCtx_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *const HIP_MEMCPY3D, hipCtx_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvGraphAddMemcpyNode") });
        unsafe { _f(phGraphNode, hGraph, dependencies, numDependencies, copyParams, ctx) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvGraphAddMemcpyNode(phGraphNode: *mut hipGraphNode_t, hGraph: hipGraph_t, dependencies: *const hipGraphNode_t, numDependencies: usize, copyParams: *const HIP_MEMCPY3D, ctx: hipCtx_t) -> hipError_t;
        }
        unsafe { hipDrvGraphAddMemcpyNode(phGraphNode, hGraph, dependencies, numDependencies, copyParams, ctx) }
    }
}
pub unsafe fn hipDrvGraphAddMemsetNode(phGraphNode: *mut hipGraphNode_t, hGraph: hipGraph_t, dependencies: *const hipGraphNode_t, numDependencies: usize, memsetParams: *const hipMemsetParams, ctx: hipCtx_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *const hipMemsetParams, hipCtx_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvGraphAddMemsetNode") });
        unsafe { _f(phGraphNode, hGraph, dependencies, numDependencies, memsetParams, ctx) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvGraphAddMemsetNode(phGraphNode: *mut hipGraphNode_t, hGraph: hipGraph_t, dependencies: *const hipGraphNode_t, numDependencies: usize, memsetParams: *const hipMemsetParams, ctx: hipCtx_t) -> hipError_t;
        }
        unsafe { hipDrvGraphAddMemsetNode(phGraphNode, hGraph, dependencies, numDependencies, memsetParams, ctx) }
    }
}
pub unsafe fn hipDrvGraphExecMemcpyNodeSetParams(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, copyParams: *const HIP_MEMCPY3D, ctx: hipCtx_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *const HIP_MEMCPY3D, hipCtx_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvGraphExecMemcpyNodeSetParams") });
        unsafe { _f(hGraphExec, hNode, copyParams, ctx) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvGraphExecMemcpyNodeSetParams(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, copyParams: *const HIP_MEMCPY3D, ctx: hipCtx_t) -> hipError_t;
        }
        unsafe { hipDrvGraphExecMemcpyNodeSetParams(hGraphExec, hNode, copyParams, ctx) }
    }
}
pub unsafe fn hipDrvGraphExecMemsetNodeSetParams(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, memsetParams: *const hipMemsetParams, ctx: hipCtx_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *const hipMemsetParams, hipCtx_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvGraphExecMemsetNodeSetParams") });
        unsafe { _f(hGraphExec, hNode, memsetParams, ctx) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvGraphExecMemsetNodeSetParams(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, memsetParams: *const hipMemsetParams, ctx: hipCtx_t) -> hipError_t;
        }
        unsafe { hipDrvGraphExecMemsetNodeSetParams(hGraphExec, hNode, memsetParams, ctx) }
    }
}
pub unsafe fn hipDrvGraphMemcpyNodeGetParams(hNode: hipGraphNode_t, nodeParams: *mut HIP_MEMCPY3D) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut HIP_MEMCPY3D) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvGraphMemcpyNodeGetParams") });
        unsafe { _f(hNode, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvGraphMemcpyNodeGetParams(hNode: hipGraphNode_t, nodeParams: *mut HIP_MEMCPY3D) -> hipError_t;
        }
        unsafe { hipDrvGraphMemcpyNodeGetParams(hNode, nodeParams) }
    }
}
pub unsafe fn hipDrvGraphMemcpyNodeSetParams(hNode: hipGraphNode_t, nodeParams: *const HIP_MEMCPY3D) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *const HIP_MEMCPY3D) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvGraphMemcpyNodeSetParams") });
        unsafe { _f(hNode, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvGraphMemcpyNodeSetParams(hNode: hipGraphNode_t, nodeParams: *const HIP_MEMCPY3D) -> hipError_t;
        }
        unsafe { hipDrvGraphMemcpyNodeSetParams(hNode, nodeParams) }
    }
}
pub unsafe fn hipDrvLaunchKernelEx(config: *const HIP_LAUNCH_CONFIG, f: hipFunction_t, params: *mut *mut ::core::ffi::c_void, extra: *mut *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const HIP_LAUNCH_CONFIG, hipFunction_t, *mut *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvLaunchKernelEx") });
        unsafe { _f(config, f, params, extra) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvLaunchKernelEx(config: *const HIP_LAUNCH_CONFIG, f: hipFunction_t, params: *mut *mut ::core::ffi::c_void, extra: *mut *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipDrvLaunchKernelEx(config, f, params, extra) }
    }
}
pub unsafe fn hipDrvMemcpy2DUnaligned(pCopy: *const hip_Memcpy2D) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const hip_Memcpy2D) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvMemcpy2DUnaligned") });
        unsafe { _f(pCopy) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvMemcpy2DUnaligned(pCopy: *const hip_Memcpy2D) -> hipError_t;
        }
        unsafe { hipDrvMemcpy2DUnaligned(pCopy) }
    }
}
pub unsafe fn hipDrvMemcpy3D(pCopy: *const HIP_MEMCPY3D) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const HIP_MEMCPY3D) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvMemcpy3D") });
        unsafe { _f(pCopy) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvMemcpy3D(pCopy: *const HIP_MEMCPY3D) -> hipError_t;
        }
        unsafe { hipDrvMemcpy3D(pCopy) }
    }
}
pub unsafe fn hipDrvMemcpy3DAsync(pCopy: *const HIP_MEMCPY3D, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const HIP_MEMCPY3D, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvMemcpy3DAsync") });
        unsafe { _f(pCopy, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvMemcpy3DAsync(pCopy: *const HIP_MEMCPY3D, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipDrvMemcpy3DAsync(pCopy, stream) }
    }
}
pub unsafe fn hipDrvPointerGetAttributes(numAttributes: ::core::ffi::c_uint, attributes: *mut hipPointer_attribute, data: *mut *mut ::core::ffi::c_void, ptr: hipDeviceptr_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_uint, *mut hipPointer_attribute, *mut *mut ::core::ffi::c_void, hipDeviceptr_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipDrvPointerGetAttributes") });
        unsafe { _f(numAttributes, attributes, data, ptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipDrvPointerGetAttributes(numAttributes: ::core::ffi::c_uint, attributes: *mut hipPointer_attribute, data: *mut *mut ::core::ffi::c_void, ptr: hipDeviceptr_t) -> hipError_t;
        }
        unsafe { hipDrvPointerGetAttributes(numAttributes, attributes, data, ptr) }
    }
}
pub unsafe fn hipEventCreate(event: *mut hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipEventCreate") });
        unsafe { _f(event) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipEventCreate(event: *mut hipEvent_t) -> hipError_t;
        }
        unsafe { hipEventCreate(event) }
    }
}
pub unsafe fn hipEventCreateWithFlags(event: *mut hipEvent_t, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipEvent_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipEventCreateWithFlags") });
        unsafe { _f(event, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipEventCreateWithFlags(event: *mut hipEvent_t, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipEventCreateWithFlags(event, flags) }
    }
}
pub unsafe fn hipEventDestroy(event: hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipEventDestroy") });
        unsafe { _f(event) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipEventDestroy(event: hipEvent_t) -> hipError_t;
        }
        unsafe { hipEventDestroy(event) }
    }
}
pub unsafe fn hipEventElapsedTime(ms: *mut f32, start: hipEvent_t, stop: hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut f32, hipEvent_t, hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipEventElapsedTime") });
        unsafe { _f(ms, start, stop) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipEventElapsedTime(ms: *mut f32, start: hipEvent_t, stop: hipEvent_t) -> hipError_t;
        }
        unsafe { hipEventElapsedTime(ms, start, stop) }
    }
}
pub unsafe fn hipEventQuery(event: hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipEventQuery") });
        unsafe { _f(event) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipEventQuery(event: hipEvent_t) -> hipError_t;
        }
        unsafe { hipEventQuery(event) }
    }
}
pub unsafe fn hipEventRecord(event: hipEvent_t, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipEvent_t, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipEventRecord") });
        unsafe { _f(event, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipEventRecord(event: hipEvent_t, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipEventRecord(event, stream) }
    }
}
pub unsafe fn hipEventRecordWithFlags(event: hipEvent_t, stream: hipStream_t, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipEvent_t, hipStream_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipEventRecordWithFlags") });
        unsafe { _f(event, stream, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipEventRecordWithFlags(event: hipEvent_t, stream: hipStream_t, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipEventRecordWithFlags(event, stream, flags) }
    }
}
pub unsafe fn hipEventRecord_spt(event: hipEvent_t, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipEvent_t, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipEventRecord_spt") });
        unsafe { _f(event, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipEventRecord_spt(event: hipEvent_t, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipEventRecord_spt(event, stream) }
    }
}
pub unsafe fn hipEventSynchronize(event: hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipEventSynchronize") });
        unsafe { _f(event) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipEventSynchronize(event: hipEvent_t) -> hipError_t;
        }
        unsafe { hipEventSynchronize(event) }
    }
}
pub unsafe fn hipExtGetLastError() -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn() -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipExtGetLastError") });
        unsafe { _f() }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipExtGetLastError() -> hipError_t;
        }
        unsafe { hipExtGetLastError() }
    }
}
pub unsafe fn hipExtGetLinkTypeAndHopCount(device1: ::core::ffi::c_int, device2: ::core::ffi::c_int, linktype: *mut u32, hopcount: *mut u32) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int, *mut u32, *mut u32) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipExtGetLinkTypeAndHopCount") });
        unsafe { _f(device1, device2, linktype, hopcount) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipExtGetLinkTypeAndHopCount(device1: ::core::ffi::c_int, device2: ::core::ffi::c_int, linktype: *mut u32, hopcount: *mut u32) -> hipError_t;
        }
        unsafe { hipExtGetLinkTypeAndHopCount(device1, device2, linktype, hopcount) }
    }
}
pub unsafe fn hipExtLaunchKernel(function_address: *const ::core::ffi::c_void, numBlocks: dim3, dimBlocks: dim3, args: *mut *mut ::core::ffi::c_void, sharedMemBytes: usize, stream: hipStream_t, startEvent: hipEvent_t, stopEvent: hipEvent_t, flags: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, dim3, dim3, *mut *mut ::core::ffi::c_void, usize, hipStream_t, hipEvent_t, hipEvent_t, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipExtLaunchKernel") });
        unsafe { _f(function_address, numBlocks, dimBlocks, args, sharedMemBytes, stream, startEvent, stopEvent, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipExtLaunchKernel(function_address: *const ::core::ffi::c_void, numBlocks: dim3, dimBlocks: dim3, args: *mut *mut ::core::ffi::c_void, sharedMemBytes: usize, stream: hipStream_t, startEvent: hipEvent_t, stopEvent: hipEvent_t, flags: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipExtLaunchKernel(function_address, numBlocks, dimBlocks, args, sharedMemBytes, stream, startEvent, stopEvent, flags) }
    }
}
pub unsafe fn hipExtLaunchMultiKernelMultiDevice(launchParamsList: *mut hipLaunchParams, numDevices: ::core::ffi::c_int, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipLaunchParams, ::core::ffi::c_int, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipExtLaunchMultiKernelMultiDevice") });
        unsafe { _f(launchParamsList, numDevices, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipExtLaunchMultiKernelMultiDevice(launchParamsList: *mut hipLaunchParams, numDevices: ::core::ffi::c_int, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipExtLaunchMultiKernelMultiDevice(launchParamsList, numDevices, flags) }
    }
}
pub unsafe fn hipExtMallocWithFlags(ptr: *mut *mut ::core::ffi::c_void, sizeBytes: usize, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, usize, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipExtMallocWithFlags") });
        unsafe { _f(ptr, sizeBytes, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipExtMallocWithFlags(ptr: *mut *mut ::core::ffi::c_void, sizeBytes: usize, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipExtMallocWithFlags(ptr, sizeBytes, flags) }
    }
}
pub unsafe fn hipExtStreamCreateWithCUMask(stream: *mut hipStream_t, cuMaskSize: u32, cuMask: *const u32) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipStream_t, u32, *const u32) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipExtStreamCreateWithCUMask") });
        unsafe { _f(stream, cuMaskSize, cuMask) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipExtStreamCreateWithCUMask(stream: *mut hipStream_t, cuMaskSize: u32, cuMask: *const u32) -> hipError_t;
        }
        unsafe { hipExtStreamCreateWithCUMask(stream, cuMaskSize, cuMask) }
    }
}
pub unsafe fn hipExtStreamGetCUMask(stream: hipStream_t, cuMaskSize: u32, cuMask: *mut u32) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, u32, *mut u32) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipExtStreamGetCUMask") });
        unsafe { _f(stream, cuMaskSize, cuMask) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipExtStreamGetCUMask(stream: hipStream_t, cuMaskSize: u32, cuMask: *mut u32) -> hipError_t;
        }
        unsafe { hipExtStreamGetCUMask(stream, cuMaskSize, cuMask) }
    }
}
pub unsafe fn hipExternalMemoryGetMappedBuffer(devPtr: *mut *mut ::core::ffi::c_void, extMem: hipExternalMemory_t, bufferDesc: *const hipExternalMemoryBufferDesc) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, hipExternalMemory_t, *const hipExternalMemoryBufferDesc) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipExternalMemoryGetMappedBuffer") });
        unsafe { _f(devPtr, extMem, bufferDesc) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipExternalMemoryGetMappedBuffer(devPtr: *mut *mut ::core::ffi::c_void, extMem: hipExternalMemory_t, bufferDesc: *const hipExternalMemoryBufferDesc) -> hipError_t;
        }
        unsafe { hipExternalMemoryGetMappedBuffer(devPtr, extMem, bufferDesc) }
    }
}
pub unsafe fn hipExternalMemoryGetMappedMipmappedArray(mipmap: *mut hipMipmappedArray_t, extMem: hipExternalMemory_t, mipmapDesc: *const hipExternalMemoryMipmappedArrayDesc) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMipmappedArray_t, hipExternalMemory_t, *const hipExternalMemoryMipmappedArrayDesc) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipExternalMemoryGetMappedMipmappedArray") });
        unsafe { _f(mipmap, extMem, mipmapDesc) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipExternalMemoryGetMappedMipmappedArray(mipmap: *mut hipMipmappedArray_t, extMem: hipExternalMemory_t, mipmapDesc: *const hipExternalMemoryMipmappedArrayDesc) -> hipError_t;
        }
        unsafe { hipExternalMemoryGetMappedMipmappedArray(mipmap, extMem, mipmapDesc) }
    }
}
pub unsafe fn hipFree(ptr: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipFree") });
        unsafe { _f(ptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipFree(ptr: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipFree(ptr) }
    }
}
pub unsafe fn hipFreeArray(array: hipArray_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipArray_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipFreeArray") });
        unsafe { _f(array) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipFreeArray(array: hipArray_t) -> hipError_t;
        }
        unsafe { hipFreeArray(array) }
    }
}
pub unsafe fn hipFreeAsync(dev_ptr: *mut ::core::ffi::c_void, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipFreeAsync") });
        unsafe { _f(dev_ptr, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipFreeAsync(dev_ptr: *mut ::core::ffi::c_void, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipFreeAsync(dev_ptr, stream) }
    }
}
pub unsafe fn hipFreeHost(ptr: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipFreeHost") });
        unsafe { _f(ptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipFreeHost(ptr: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipFreeHost(ptr) }
    }
}
pub unsafe fn hipFreeMipmappedArray(mipmappedArray: hipMipmappedArray_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipMipmappedArray_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipFreeMipmappedArray") });
        unsafe { _f(mipmappedArray) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipFreeMipmappedArray(mipmappedArray: hipMipmappedArray_t) -> hipError_t;
        }
        unsafe { hipFreeMipmappedArray(mipmappedArray) }
    }
}
pub unsafe fn hipFuncGetAttribute(value: *mut ::core::ffi::c_int, attrib: hipFunction_attribute, hfunc: hipFunction_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, hipFunction_attribute, hipFunction_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipFuncGetAttribute") });
        unsafe { _f(value, attrib, hfunc) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipFuncGetAttribute(value: *mut ::core::ffi::c_int, attrib: hipFunction_attribute, hfunc: hipFunction_t) -> hipError_t;
        }
        unsafe { hipFuncGetAttribute(value, attrib, hfunc) }
    }
}
pub unsafe fn hipFuncGetAttributes(attr: *mut hipFuncAttributes, func: *const ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipFuncAttributes, *const ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipFuncGetAttributes") });
        unsafe { _f(attr, func) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipFuncGetAttributes(attr: *mut hipFuncAttributes, func: *const ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipFuncGetAttributes(attr, func) }
    }
}
pub unsafe fn hipFuncSetAttribute(func: *const ::core::ffi::c_void, attr: hipFuncAttribute, value: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, hipFuncAttribute, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipFuncSetAttribute") });
        unsafe { _f(func, attr, value) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipFuncSetAttribute(func: *const ::core::ffi::c_void, attr: hipFuncAttribute, value: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipFuncSetAttribute(func, attr, value) }
    }
}
pub unsafe fn hipFuncSetCacheConfig(func: *const ::core::ffi::c_void, config: hipFuncCache_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, hipFuncCache_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipFuncSetCacheConfig") });
        unsafe { _f(func, config) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipFuncSetCacheConfig(func: *const ::core::ffi::c_void, config: hipFuncCache_t) -> hipError_t;
        }
        unsafe { hipFuncSetCacheConfig(func, config) }
    }
}
pub unsafe fn hipFuncSetSharedMemConfig(func: *const ::core::ffi::c_void, config: hipSharedMemConfig) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, hipSharedMemConfig) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipFuncSetSharedMemConfig") });
        unsafe { _f(func, config) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipFuncSetSharedMemConfig(func: *const ::core::ffi::c_void, config: hipSharedMemConfig) -> hipError_t;
        }
        unsafe { hipFuncSetSharedMemConfig(func, config) }
    }
}
pub unsafe fn hipGetChannelDesc(desc: *mut hipChannelFormatDesc, array: hipArray_const_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipChannelFormatDesc, hipArray_const_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetChannelDesc") });
        unsafe { _f(desc, array) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetChannelDesc(desc: *mut hipChannelFormatDesc, array: hipArray_const_t) -> hipError_t;
        }
        unsafe { hipGetChannelDesc(desc, array) }
    }
}
pub unsafe fn hipGetDevice(deviceId: *mut ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetDevice") });
        unsafe { _f(deviceId) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetDevice(deviceId: *mut ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipGetDevice(deviceId) }
    }
}
pub unsafe fn hipGetDeviceCount(count: *mut ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetDeviceCount") });
        unsafe { _f(count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetDeviceCount(count: *mut ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipGetDeviceCount(count) }
    }
}
pub unsafe fn hipGetDeviceFlags(flags: *mut ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetDeviceFlags") });
        unsafe { _f(flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetDeviceFlags(flags: *mut ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipGetDeviceFlags(flags) }
    }
}
pub unsafe fn hipGetDevicePropertiesR0600(prop: *mut hipDeviceProp_tR0600, deviceId: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipDeviceProp_tR0600, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetDevicePropertiesR0600") });
        unsafe { _f(prop, deviceId) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetDevicePropertiesR0600(prop: *mut hipDeviceProp_tR0600, deviceId: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipGetDevicePropertiesR0600(prop, deviceId) }
    }
}
pub unsafe fn hipGetDriverEntryPoint(symbol: *const ::core::ffi::c_char, funcPtr: *mut *mut ::core::ffi::c_void, flags: ::core::ffi::c_ulonglong, driverStatus: *mut hipDriverEntryPointQueryResult) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_char, *mut *mut ::core::ffi::c_void, ::core::ffi::c_ulonglong, *mut hipDriverEntryPointQueryResult) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetDriverEntryPoint") });
        unsafe { _f(symbol, funcPtr, flags, driverStatus) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetDriverEntryPoint(symbol: *const ::core::ffi::c_char, funcPtr: *mut *mut ::core::ffi::c_void, flags: ::core::ffi::c_ulonglong, driverStatus: *mut hipDriverEntryPointQueryResult) -> hipError_t;
        }
        unsafe { hipGetDriverEntryPoint(symbol, funcPtr, flags, driverStatus) }
    }
}
pub unsafe fn hipGetDriverEntryPoint_spt(symbol: *const ::core::ffi::c_char, funcPtr: *mut *mut ::core::ffi::c_void, flags: ::core::ffi::c_ulonglong, status: *mut hipDriverEntryPointQueryResult) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_char, *mut *mut ::core::ffi::c_void, ::core::ffi::c_ulonglong, *mut hipDriverEntryPointQueryResult) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetDriverEntryPoint_spt") });
        unsafe { _f(symbol, funcPtr, flags, status) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetDriverEntryPoint_spt(symbol: *const ::core::ffi::c_char, funcPtr: *mut *mut ::core::ffi::c_void, flags: ::core::ffi::c_ulonglong, status: *mut hipDriverEntryPointQueryResult) -> hipError_t;
        }
        unsafe { hipGetDriverEntryPoint_spt(symbol, funcPtr, flags, status) }
    }
}
pub unsafe fn hipGetErrorName(hip_error: hipError_t) -> *const ::core::ffi::c_char {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipError_t) -> *const ::core::ffi::c_char;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetErrorName") });
        unsafe { _f(hip_error) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetErrorName(hip_error: hipError_t) -> *const ::core::ffi::c_char;
        }
        unsafe { hipGetErrorName(hip_error) }
    }
}
pub unsafe fn hipGetErrorString(hipError: hipError_t) -> *const ::core::ffi::c_char {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipError_t) -> *const ::core::ffi::c_char;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetErrorString") });
        unsafe { _f(hipError) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetErrorString(hipError: hipError_t) -> *const ::core::ffi::c_char;
        }
        unsafe { hipGetErrorString(hipError) }
    }
}
pub unsafe fn hipGetFuncBySymbol(functionPtr: *mut hipFunction_t, symbolPtr: *const ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipFunction_t, *const ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetFuncBySymbol") });
        unsafe { _f(functionPtr, symbolPtr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetFuncBySymbol(functionPtr: *mut hipFunction_t, symbolPtr: *const ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipGetFuncBySymbol(functionPtr, symbolPtr) }
    }
}
pub unsafe fn hipGetLastError() -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn() -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetLastError") });
        unsafe { _f() }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetLastError() -> hipError_t;
        }
        unsafe { hipGetLastError() }
    }
}
pub unsafe fn hipGetMipmappedArrayLevel(levelArray: *mut hipArray_t, mipmappedArray: hipMipmappedArray_const_t, level: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipArray_t, hipMipmappedArray_const_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetMipmappedArrayLevel") });
        unsafe { _f(levelArray, mipmappedArray, level) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetMipmappedArrayLevel(levelArray: *mut hipArray_t, mipmappedArray: hipMipmappedArray_const_t, level: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipGetMipmappedArrayLevel(levelArray, mipmappedArray, level) }
    }
}
pub unsafe fn hipGetProcAddress(symbol: *const ::core::ffi::c_char, pfn: *mut *mut ::core::ffi::c_void, hipVersion: ::core::ffi::c_int, flags: u64, symbolStatus: *mut hipDriverProcAddressQueryResult) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_char, *mut *mut ::core::ffi::c_void, ::core::ffi::c_int, u64, *mut hipDriverProcAddressQueryResult) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetProcAddress") });
        unsafe { _f(symbol, pfn, hipVersion, flags, symbolStatus) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetProcAddress(symbol: *const ::core::ffi::c_char, pfn: *mut *mut ::core::ffi::c_void, hipVersion: ::core::ffi::c_int, flags: u64, symbolStatus: *mut hipDriverProcAddressQueryResult) -> hipError_t;
        }
        unsafe { hipGetProcAddress(symbol, pfn, hipVersion, flags, symbolStatus) }
    }
}
pub unsafe fn hipGetStreamDeviceId(stream: hipStream_t) -> ::core::ffi::c_int {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t) -> ::core::ffi::c_int;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetStreamDeviceId") });
        unsafe { _f(stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetStreamDeviceId(stream: hipStream_t) -> ::core::ffi::c_int;
        }
        unsafe { hipGetStreamDeviceId(stream) }
    }
}
pub unsafe fn hipGetSymbolAddress(devPtr: *mut *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetSymbolAddress") });
        unsafe { _f(devPtr, symbol) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetSymbolAddress(devPtr: *mut *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipGetSymbolAddress(devPtr, symbol) }
    }
}
pub unsafe fn hipGetSymbolSize(size: *mut usize, symbol: *const ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut usize, *const ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetSymbolSize") });
        unsafe { _f(size, symbol) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetSymbolSize(size: *mut usize, symbol: *const ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipGetSymbolSize(size, symbol) }
    }
}
pub unsafe fn hipGetTextureAlignmentOffset(offset: *mut usize, texref: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut usize, *const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetTextureAlignmentOffset") });
        unsafe { _f(offset, texref) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetTextureAlignmentOffset(offset: *mut usize, texref: *const textureReference) -> hipError_t;
        }
        unsafe { hipGetTextureAlignmentOffset(offset, texref) }
    }
}
pub unsafe fn hipGetTextureObjectResourceDesc(pResDesc: *mut hipResourceDesc, textureObject: hipTextureObject_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipResourceDesc, hipTextureObject_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetTextureObjectResourceDesc") });
        unsafe { _f(pResDesc, textureObject) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetTextureObjectResourceDesc(pResDesc: *mut hipResourceDesc, textureObject: hipTextureObject_t) -> hipError_t;
        }
        unsafe { hipGetTextureObjectResourceDesc(pResDesc, textureObject) }
    }
}
pub unsafe fn hipGetTextureObjectResourceViewDesc(pResViewDesc: *mut hipResourceViewDesc, textureObject: hipTextureObject_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipResourceViewDesc, hipTextureObject_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetTextureObjectResourceViewDesc") });
        unsafe { _f(pResViewDesc, textureObject) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetTextureObjectResourceViewDesc(pResViewDesc: *mut hipResourceViewDesc, textureObject: hipTextureObject_t) -> hipError_t;
        }
        unsafe { hipGetTextureObjectResourceViewDesc(pResViewDesc, textureObject) }
    }
}
pub unsafe fn hipGetTextureObjectTextureDesc(pTexDesc: *mut hipTextureDesc, textureObject: hipTextureObject_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipTextureDesc, hipTextureObject_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetTextureObjectTextureDesc") });
        unsafe { _f(pTexDesc, textureObject) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetTextureObjectTextureDesc(pTexDesc: *mut hipTextureDesc, textureObject: hipTextureObject_t) -> hipError_t;
        }
        unsafe { hipGetTextureObjectTextureDesc(pTexDesc, textureObject) }
    }
}
pub unsafe fn hipGetTextureReference(texref: *mut *const textureReference, symbol: *const ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *const textureReference, *const ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGetTextureReference") });
        unsafe { _f(texref, symbol) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGetTextureReference(texref: *mut *const textureReference, symbol: *const ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipGetTextureReference(texref, symbol) }
    }
}
pub unsafe fn hipGraphAddBatchMemOpNode(phGraphNode: *mut hipGraphNode_t, hGraph: hipGraph_t, dependencies: *const hipGraphNode_t, numDependencies: usize, nodeParams: *const hipBatchMemOpNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *const hipBatchMemOpNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddBatchMemOpNode") });
        unsafe { _f(phGraphNode, hGraph, dependencies, numDependencies, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddBatchMemOpNode(phGraphNode: *mut hipGraphNode_t, hGraph: hipGraph_t, dependencies: *const hipGraphNode_t, numDependencies: usize, nodeParams: *const hipBatchMemOpNodeParams) -> hipError_t;
        }
        unsafe { hipGraphAddBatchMemOpNode(phGraphNode, hGraph, dependencies, numDependencies, nodeParams) }
    }
}
pub unsafe fn hipGraphAddChildGraphNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, childGraph: hipGraph_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, hipGraph_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddChildGraphNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, childGraph) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddChildGraphNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, childGraph: hipGraph_t) -> hipError_t;
        }
        unsafe { hipGraphAddChildGraphNode(pGraphNode, graph, pDependencies, numDependencies, childGraph) }
    }
}
pub unsafe fn hipGraphAddDependencies(graph: hipGraph_t, from: *const hipGraphNode_t, to: *const hipGraphNode_t, numDependencies: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraph_t, *const hipGraphNode_t, *const hipGraphNode_t, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddDependencies") });
        unsafe { _f(graph, from, to, numDependencies) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddDependencies(graph: hipGraph_t, from: *const hipGraphNode_t, to: *const hipGraphNode_t, numDependencies: usize) -> hipError_t;
        }
        unsafe { hipGraphAddDependencies(graph, from, to, numDependencies) }
    }
}
pub unsafe fn hipGraphAddEmptyNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddEmptyNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddEmptyNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize) -> hipError_t;
        }
        unsafe { hipGraphAddEmptyNode(pGraphNode, graph, pDependencies, numDependencies) }
    }
}
pub unsafe fn hipGraphAddEventRecordNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, event: hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddEventRecordNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, event) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddEventRecordNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, event: hipEvent_t) -> hipError_t;
        }
        unsafe { hipGraphAddEventRecordNode(pGraphNode, graph, pDependencies, numDependencies, event) }
    }
}
pub unsafe fn hipGraphAddEventWaitNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, event: hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddEventWaitNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, event) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddEventWaitNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, event: hipEvent_t) -> hipError_t;
        }
        unsafe { hipGraphAddEventWaitNode(pGraphNode, graph, pDependencies, numDependencies, event) }
    }
}
pub unsafe fn hipGraphAddExternalSemaphoresSignalNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, nodeParams: *const hipExternalSemaphoreSignalNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *const hipExternalSemaphoreSignalNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddExternalSemaphoresSignalNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddExternalSemaphoresSignalNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, nodeParams: *const hipExternalSemaphoreSignalNodeParams) -> hipError_t;
        }
        unsafe { hipGraphAddExternalSemaphoresSignalNode(pGraphNode, graph, pDependencies, numDependencies, nodeParams) }
    }
}
pub unsafe fn hipGraphAddExternalSemaphoresWaitNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, nodeParams: *const hipExternalSemaphoreWaitNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *const hipExternalSemaphoreWaitNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddExternalSemaphoresWaitNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddExternalSemaphoresWaitNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, nodeParams: *const hipExternalSemaphoreWaitNodeParams) -> hipError_t;
        }
        unsafe { hipGraphAddExternalSemaphoresWaitNode(pGraphNode, graph, pDependencies, numDependencies, nodeParams) }
    }
}
pub unsafe fn hipGraphAddHostNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, pNodeParams: *const hipHostNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *const hipHostNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddHostNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddHostNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, pNodeParams: *const hipHostNodeParams) -> hipError_t;
        }
        unsafe { hipGraphAddHostNode(pGraphNode, graph, pDependencies, numDependencies, pNodeParams) }
    }
}
pub unsafe fn hipGraphAddKernelNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, pNodeParams: *const hipKernelNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *const hipKernelNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddKernelNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddKernelNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, pNodeParams: *const hipKernelNodeParams) -> hipError_t;
        }
        unsafe { hipGraphAddKernelNode(pGraphNode, graph, pDependencies, numDependencies, pNodeParams) }
    }
}
pub unsafe fn hipGraphAddMemAllocNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, pNodeParams: *mut hipMemAllocNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *mut hipMemAllocNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddMemAllocNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddMemAllocNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, pNodeParams: *mut hipMemAllocNodeParams) -> hipError_t;
        }
        unsafe { hipGraphAddMemAllocNode(pGraphNode, graph, pDependencies, numDependencies, pNodeParams) }
    }
}
pub unsafe fn hipGraphAddMemFreeNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, dev_ptr: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddMemFreeNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, dev_ptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddMemFreeNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, dev_ptr: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipGraphAddMemFreeNode(pGraphNode, graph, pDependencies, numDependencies, dev_ptr) }
    }
}
pub unsafe fn hipGraphAddMemcpyNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, pCopyParams: *const hipMemcpy3DParms) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *const hipMemcpy3DParms) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddMemcpyNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, pCopyParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddMemcpyNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, pCopyParams: *const hipMemcpy3DParms) -> hipError_t;
        }
        unsafe { hipGraphAddMemcpyNode(pGraphNode, graph, pDependencies, numDependencies, pCopyParams) }
    }
}
pub unsafe fn hipGraphAddMemcpyNode1D(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddMemcpyNode1D") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, dst, src, count, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddMemcpyNode1D(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipGraphAddMemcpyNode1D(pGraphNode, graph, pDependencies, numDependencies, dst, src, count, kind) }
    }
}
pub unsafe fn hipGraphAddMemcpyNodeFromSymbol(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, count: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddMemcpyNodeFromSymbol") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, dst, symbol, count, offset, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddMemcpyNodeFromSymbol(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, count: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipGraphAddMemcpyNodeFromSymbol(pGraphNode, graph, pDependencies, numDependencies, dst, symbol, count, offset, kind) }
    }
}
pub unsafe fn hipGraphAddMemcpyNodeToSymbol(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *const ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddMemcpyNodeToSymbol") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, symbol, src, count, offset, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddMemcpyNodeToSymbol(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipGraphAddMemcpyNodeToSymbol(pGraphNode, graph, pDependencies, numDependencies, symbol, src, count, offset, kind) }
    }
}
pub unsafe fn hipGraphAddMemsetNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, pMemsetParams: *const hipMemsetParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *const hipMemsetParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddMemsetNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, pMemsetParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddMemsetNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, pMemsetParams: *const hipMemsetParams) -> hipError_t;
        }
        unsafe { hipGraphAddMemsetNode(pGraphNode, graph, pDependencies, numDependencies, pMemsetParams) }
    }
}
pub unsafe fn hipGraphAddNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, nodeParams: *mut hipGraphNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraph_t, *const hipGraphNode_t, usize, *mut hipGraphNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphAddNode") });
        unsafe { _f(pGraphNode, graph, pDependencies, numDependencies, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphAddNode(pGraphNode: *mut hipGraphNode_t, graph: hipGraph_t, pDependencies: *const hipGraphNode_t, numDependencies: usize, nodeParams: *mut hipGraphNodeParams) -> hipError_t;
        }
        unsafe { hipGraphAddNode(pGraphNode, graph, pDependencies, numDependencies, nodeParams) }
    }
}
pub unsafe fn hipGraphBatchMemOpNodeGetParams(hNode: hipGraphNode_t, nodeParams_out: *mut hipBatchMemOpNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipBatchMemOpNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphBatchMemOpNodeGetParams") });
        unsafe { _f(hNode, nodeParams_out) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphBatchMemOpNodeGetParams(hNode: hipGraphNode_t, nodeParams_out: *mut hipBatchMemOpNodeParams) -> hipError_t;
        }
        unsafe { hipGraphBatchMemOpNodeGetParams(hNode, nodeParams_out) }
    }
}
pub unsafe fn hipGraphBatchMemOpNodeSetParams(hNode: hipGraphNode_t, nodeParams: *mut hipBatchMemOpNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipBatchMemOpNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphBatchMemOpNodeSetParams") });
        unsafe { _f(hNode, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphBatchMemOpNodeSetParams(hNode: hipGraphNode_t, nodeParams: *mut hipBatchMemOpNodeParams) -> hipError_t;
        }
        unsafe { hipGraphBatchMemOpNodeSetParams(hNode, nodeParams) }
    }
}
pub unsafe fn hipGraphChildGraphNodeGetGraph(node: hipGraphNode_t, pGraph: *mut hipGraph_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipGraph_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphChildGraphNodeGetGraph") });
        unsafe { _f(node, pGraph) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphChildGraphNodeGetGraph(node: hipGraphNode_t, pGraph: *mut hipGraph_t) -> hipError_t;
        }
        unsafe { hipGraphChildGraphNodeGetGraph(node, pGraph) }
    }
}
pub unsafe fn hipGraphClone(pGraphClone: *mut hipGraph_t, originalGraph: hipGraph_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraph_t, hipGraph_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphClone") });
        unsafe { _f(pGraphClone, originalGraph) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphClone(pGraphClone: *mut hipGraph_t, originalGraph: hipGraph_t) -> hipError_t;
        }
        unsafe { hipGraphClone(pGraphClone, originalGraph) }
    }
}
pub unsafe fn hipGraphCreate(pGraph: *mut hipGraph_t, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraph_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphCreate") });
        unsafe { _f(pGraph, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphCreate(pGraph: *mut hipGraph_t, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipGraphCreate(pGraph, flags) }
    }
}
pub unsafe fn hipGraphDebugDotPrint(graph: hipGraph_t, path: *const ::core::ffi::c_char, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraph_t, *const ::core::ffi::c_char, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphDebugDotPrint") });
        unsafe { _f(graph, path, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphDebugDotPrint(graph: hipGraph_t, path: *const ::core::ffi::c_char, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipGraphDebugDotPrint(graph, path, flags) }
    }
}
pub unsafe fn hipGraphDestroy(graph: hipGraph_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraph_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphDestroy") });
        unsafe { _f(graph) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphDestroy(graph: hipGraph_t) -> hipError_t;
        }
        unsafe { hipGraphDestroy(graph) }
    }
}
pub unsafe fn hipGraphDestroyNode(node: hipGraphNode_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphDestroyNode") });
        unsafe { _f(node) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphDestroyNode(node: hipGraphNode_t) -> hipError_t;
        }
        unsafe { hipGraphDestroyNode(node) }
    }
}
pub unsafe fn hipGraphEventRecordNodeGetEvent(node: hipGraphNode_t, event_out: *mut hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphEventRecordNodeGetEvent") });
        unsafe { _f(node, event_out) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphEventRecordNodeGetEvent(node: hipGraphNode_t, event_out: *mut hipEvent_t) -> hipError_t;
        }
        unsafe { hipGraphEventRecordNodeGetEvent(node, event_out) }
    }
}
pub unsafe fn hipGraphEventRecordNodeSetEvent(node: hipGraphNode_t, event: hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphEventRecordNodeSetEvent") });
        unsafe { _f(node, event) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphEventRecordNodeSetEvent(node: hipGraphNode_t, event: hipEvent_t) -> hipError_t;
        }
        unsafe { hipGraphEventRecordNodeSetEvent(node, event) }
    }
}
pub unsafe fn hipGraphEventWaitNodeGetEvent(node: hipGraphNode_t, event_out: *mut hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphEventWaitNodeGetEvent") });
        unsafe { _f(node, event_out) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphEventWaitNodeGetEvent(node: hipGraphNode_t, event_out: *mut hipEvent_t) -> hipError_t;
        }
        unsafe { hipGraphEventWaitNodeGetEvent(node, event_out) }
    }
}
pub unsafe fn hipGraphEventWaitNodeSetEvent(node: hipGraphNode_t, event: hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphEventWaitNodeSetEvent") });
        unsafe { _f(node, event) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphEventWaitNodeSetEvent(node: hipGraphNode_t, event: hipEvent_t) -> hipError_t;
        }
        unsafe { hipGraphEventWaitNodeSetEvent(node, event) }
    }
}
pub unsafe fn hipGraphExecBatchMemOpNodeSetParams(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, nodeParams: *const hipBatchMemOpNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *const hipBatchMemOpNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecBatchMemOpNodeSetParams") });
        unsafe { _f(hGraphExec, hNode, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecBatchMemOpNodeSetParams(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, nodeParams: *const hipBatchMemOpNodeParams) -> hipError_t;
        }
        unsafe { hipGraphExecBatchMemOpNodeSetParams(hGraphExec, hNode, nodeParams) }
    }
}
pub unsafe fn hipGraphExecChildGraphNodeSetParams(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, childGraph: hipGraph_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, hipGraph_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecChildGraphNodeSetParams") });
        unsafe { _f(hGraphExec, node, childGraph) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecChildGraphNodeSetParams(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, childGraph: hipGraph_t) -> hipError_t;
        }
        unsafe { hipGraphExecChildGraphNodeSetParams(hGraphExec, node, childGraph) }
    }
}
pub unsafe fn hipGraphExecDestroy(graphExec: hipGraphExec_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecDestroy") });
        unsafe { _f(graphExec) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecDestroy(graphExec: hipGraphExec_t) -> hipError_t;
        }
        unsafe { hipGraphExecDestroy(graphExec) }
    }
}
pub unsafe fn hipGraphExecEventRecordNodeSetEvent(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, event: hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecEventRecordNodeSetEvent") });
        unsafe { _f(hGraphExec, hNode, event) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecEventRecordNodeSetEvent(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, event: hipEvent_t) -> hipError_t;
        }
        unsafe { hipGraphExecEventRecordNodeSetEvent(hGraphExec, hNode, event) }
    }
}
pub unsafe fn hipGraphExecEventWaitNodeSetEvent(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, event: hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecEventWaitNodeSetEvent") });
        unsafe { _f(hGraphExec, hNode, event) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecEventWaitNodeSetEvent(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, event: hipEvent_t) -> hipError_t;
        }
        unsafe { hipGraphExecEventWaitNodeSetEvent(hGraphExec, hNode, event) }
    }
}
pub unsafe fn hipGraphExecExternalSemaphoresSignalNodeSetParams(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, nodeParams: *const hipExternalSemaphoreSignalNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *const hipExternalSemaphoreSignalNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecExternalSemaphoresSignalNodeSetParams") });
        unsafe { _f(hGraphExec, hNode, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecExternalSemaphoresSignalNodeSetParams(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, nodeParams: *const hipExternalSemaphoreSignalNodeParams) -> hipError_t;
        }
        unsafe { hipGraphExecExternalSemaphoresSignalNodeSetParams(hGraphExec, hNode, nodeParams) }
    }
}
pub unsafe fn hipGraphExecExternalSemaphoresWaitNodeSetParams(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, nodeParams: *const hipExternalSemaphoreWaitNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *const hipExternalSemaphoreWaitNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecExternalSemaphoresWaitNodeSetParams") });
        unsafe { _f(hGraphExec, hNode, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecExternalSemaphoresWaitNodeSetParams(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, nodeParams: *const hipExternalSemaphoreWaitNodeParams) -> hipError_t;
        }
        unsafe { hipGraphExecExternalSemaphoresWaitNodeSetParams(hGraphExec, hNode, nodeParams) }
    }
}
pub unsafe fn hipGraphExecGetFlags(graphExec: hipGraphExec_t, flags: *mut ::core::ffi::c_ulonglong) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, *mut ::core::ffi::c_ulonglong) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecGetFlags") });
        unsafe { _f(graphExec, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecGetFlags(graphExec: hipGraphExec_t, flags: *mut ::core::ffi::c_ulonglong) -> hipError_t;
        }
        unsafe { hipGraphExecGetFlags(graphExec, flags) }
    }
}
pub unsafe fn hipGraphExecHostNodeSetParams(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, pNodeParams: *const hipHostNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *const hipHostNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecHostNodeSetParams") });
        unsafe { _f(hGraphExec, node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecHostNodeSetParams(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, pNodeParams: *const hipHostNodeParams) -> hipError_t;
        }
        unsafe { hipGraphExecHostNodeSetParams(hGraphExec, node, pNodeParams) }
    }
}
pub unsafe fn hipGraphExecKernelNodeSetParams(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, pNodeParams: *const hipKernelNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *const hipKernelNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecKernelNodeSetParams") });
        unsafe { _f(hGraphExec, node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecKernelNodeSetParams(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, pNodeParams: *const hipKernelNodeParams) -> hipError_t;
        }
        unsafe { hipGraphExecKernelNodeSetParams(hGraphExec, node, pNodeParams) }
    }
}
pub unsafe fn hipGraphExecMemcpyNodeSetParams(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, pNodeParams: *mut hipMemcpy3DParms) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *mut hipMemcpy3DParms) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecMemcpyNodeSetParams") });
        unsafe { _f(hGraphExec, node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecMemcpyNodeSetParams(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, pNodeParams: *mut hipMemcpy3DParms) -> hipError_t;
        }
        unsafe { hipGraphExecMemcpyNodeSetParams(hGraphExec, node, pNodeParams) }
    }
}
pub unsafe fn hipGraphExecMemcpyNodeSetParams1D(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecMemcpyNodeSetParams1D") });
        unsafe { _f(hGraphExec, node, dst, src, count, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecMemcpyNodeSetParams1D(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipGraphExecMemcpyNodeSetParams1D(hGraphExec, node, dst, src, count, kind) }
    }
}
pub unsafe fn hipGraphExecMemcpyNodeSetParamsFromSymbol(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, count: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecMemcpyNodeSetParamsFromSymbol") });
        unsafe { _f(hGraphExec, node, dst, symbol, count, offset, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecMemcpyNodeSetParamsFromSymbol(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, count: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipGraphExecMemcpyNodeSetParamsFromSymbol(hGraphExec, node, dst, symbol, count, offset, kind) }
    }
}
pub unsafe fn hipGraphExecMemcpyNodeSetParamsToSymbol(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecMemcpyNodeSetParamsToSymbol") });
        unsafe { _f(hGraphExec, node, symbol, src, count, offset, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecMemcpyNodeSetParamsToSymbol(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipGraphExecMemcpyNodeSetParamsToSymbol(hGraphExec, node, symbol, src, count, offset, kind) }
    }
}
pub unsafe fn hipGraphExecMemsetNodeSetParams(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, pNodeParams: *const hipMemsetParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *const hipMemsetParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecMemsetNodeSetParams") });
        unsafe { _f(hGraphExec, node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecMemsetNodeSetParams(hGraphExec: hipGraphExec_t, node: hipGraphNode_t, pNodeParams: *const hipMemsetParams) -> hipError_t;
        }
        unsafe { hipGraphExecMemsetNodeSetParams(hGraphExec, node, pNodeParams) }
    }
}
pub unsafe fn hipGraphExecNodeSetParams(graphExec: hipGraphExec_t, node: hipGraphNode_t, nodeParams: *mut hipGraphNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *mut hipGraphNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecNodeSetParams") });
        unsafe { _f(graphExec, node, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecNodeSetParams(graphExec: hipGraphExec_t, node: hipGraphNode_t, nodeParams: *mut hipGraphNodeParams) -> hipError_t;
        }
        unsafe { hipGraphExecNodeSetParams(graphExec, node, nodeParams) }
    }
}
pub unsafe fn hipGraphExecUpdate(hGraphExec: hipGraphExec_t, hGraph: hipGraph_t, hErrorNode_out: *mut hipGraphNode_t, updateResult_out: *mut hipGraphExecUpdateResult) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraph_t, *mut hipGraphNode_t, *mut hipGraphExecUpdateResult) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExecUpdate") });
        unsafe { _f(hGraphExec, hGraph, hErrorNode_out, updateResult_out) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExecUpdate(hGraphExec: hipGraphExec_t, hGraph: hipGraph_t, hErrorNode_out: *mut hipGraphNode_t, updateResult_out: *mut hipGraphExecUpdateResult) -> hipError_t;
        }
        unsafe { hipGraphExecUpdate(hGraphExec, hGraph, hErrorNode_out, updateResult_out) }
    }
}
pub unsafe fn hipGraphExternalSemaphoresSignalNodeGetParams(hNode: hipGraphNode_t, params_out: *mut hipExternalSemaphoreSignalNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipExternalSemaphoreSignalNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExternalSemaphoresSignalNodeGetParams") });
        unsafe { _f(hNode, params_out) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExternalSemaphoresSignalNodeGetParams(hNode: hipGraphNode_t, params_out: *mut hipExternalSemaphoreSignalNodeParams) -> hipError_t;
        }
        unsafe { hipGraphExternalSemaphoresSignalNodeGetParams(hNode, params_out) }
    }
}
pub unsafe fn hipGraphExternalSemaphoresSignalNodeSetParams(hNode: hipGraphNode_t, nodeParams: *const hipExternalSemaphoreSignalNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *const hipExternalSemaphoreSignalNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExternalSemaphoresSignalNodeSetParams") });
        unsafe { _f(hNode, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExternalSemaphoresSignalNodeSetParams(hNode: hipGraphNode_t, nodeParams: *const hipExternalSemaphoreSignalNodeParams) -> hipError_t;
        }
        unsafe { hipGraphExternalSemaphoresSignalNodeSetParams(hNode, nodeParams) }
    }
}
pub unsafe fn hipGraphExternalSemaphoresWaitNodeGetParams(hNode: hipGraphNode_t, params_out: *mut hipExternalSemaphoreWaitNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipExternalSemaphoreWaitNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExternalSemaphoresWaitNodeGetParams") });
        unsafe { _f(hNode, params_out) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExternalSemaphoresWaitNodeGetParams(hNode: hipGraphNode_t, params_out: *mut hipExternalSemaphoreWaitNodeParams) -> hipError_t;
        }
        unsafe { hipGraphExternalSemaphoresWaitNodeGetParams(hNode, params_out) }
    }
}
pub unsafe fn hipGraphExternalSemaphoresWaitNodeSetParams(hNode: hipGraphNode_t, nodeParams: *const hipExternalSemaphoreWaitNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *const hipExternalSemaphoreWaitNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphExternalSemaphoresWaitNodeSetParams") });
        unsafe { _f(hNode, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphExternalSemaphoresWaitNodeSetParams(hNode: hipGraphNode_t, nodeParams: *const hipExternalSemaphoreWaitNodeParams) -> hipError_t;
        }
        unsafe { hipGraphExternalSemaphoresWaitNodeSetParams(hNode, nodeParams) }
    }
}
pub unsafe fn hipGraphGetEdges(graph: hipGraph_t, from: *mut hipGraphNode_t, to: *mut hipGraphNode_t, numEdges: *mut usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraph_t, *mut hipGraphNode_t, *mut hipGraphNode_t, *mut usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphGetEdges") });
        unsafe { _f(graph, from, to, numEdges) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphGetEdges(graph: hipGraph_t, from: *mut hipGraphNode_t, to: *mut hipGraphNode_t, numEdges: *mut usize) -> hipError_t;
        }
        unsafe { hipGraphGetEdges(graph, from, to, numEdges) }
    }
}
pub unsafe fn hipGraphGetNodes(graph: hipGraph_t, nodes: *mut hipGraphNode_t, numNodes: *mut usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraph_t, *mut hipGraphNode_t, *mut usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphGetNodes") });
        unsafe { _f(graph, nodes, numNodes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphGetNodes(graph: hipGraph_t, nodes: *mut hipGraphNode_t, numNodes: *mut usize) -> hipError_t;
        }
        unsafe { hipGraphGetNodes(graph, nodes, numNodes) }
    }
}
pub unsafe fn hipGraphGetRootNodes(graph: hipGraph_t, pRootNodes: *mut hipGraphNode_t, pNumRootNodes: *mut usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraph_t, *mut hipGraphNode_t, *mut usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphGetRootNodes") });
        unsafe { _f(graph, pRootNodes, pNumRootNodes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphGetRootNodes(graph: hipGraph_t, pRootNodes: *mut hipGraphNode_t, pNumRootNodes: *mut usize) -> hipError_t;
        }
        unsafe { hipGraphGetRootNodes(graph, pRootNodes, pNumRootNodes) }
    }
}
pub unsafe fn hipGraphHostNodeGetParams(node: hipGraphNode_t, pNodeParams: *mut hipHostNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipHostNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphHostNodeGetParams") });
        unsafe { _f(node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphHostNodeGetParams(node: hipGraphNode_t, pNodeParams: *mut hipHostNodeParams) -> hipError_t;
        }
        unsafe { hipGraphHostNodeGetParams(node, pNodeParams) }
    }
}
pub unsafe fn hipGraphHostNodeSetParams(node: hipGraphNode_t, pNodeParams: *const hipHostNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *const hipHostNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphHostNodeSetParams") });
        unsafe { _f(node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphHostNodeSetParams(node: hipGraphNode_t, pNodeParams: *const hipHostNodeParams) -> hipError_t;
        }
        unsafe { hipGraphHostNodeSetParams(node, pNodeParams) }
    }
}
pub unsafe fn hipGraphInstantiate(pGraphExec: *mut hipGraphExec_t, graph: hipGraph_t, pErrorNode: *mut hipGraphNode_t, pLogBuffer: *mut ::core::ffi::c_char, bufferSize: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphExec_t, hipGraph_t, *mut hipGraphNode_t, *mut ::core::ffi::c_char, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphInstantiate") });
        unsafe { _f(pGraphExec, graph, pErrorNode, pLogBuffer, bufferSize) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphInstantiate(pGraphExec: *mut hipGraphExec_t, graph: hipGraph_t, pErrorNode: *mut hipGraphNode_t, pLogBuffer: *mut ::core::ffi::c_char, bufferSize: usize) -> hipError_t;
        }
        unsafe { hipGraphInstantiate(pGraphExec, graph, pErrorNode, pLogBuffer, bufferSize) }
    }
}
pub unsafe fn hipGraphInstantiateWithFlags(pGraphExec: *mut hipGraphExec_t, graph: hipGraph_t, flags: ::core::ffi::c_ulonglong) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphExec_t, hipGraph_t, ::core::ffi::c_ulonglong) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphInstantiateWithFlags") });
        unsafe { _f(pGraphExec, graph, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphInstantiateWithFlags(pGraphExec: *mut hipGraphExec_t, graph: hipGraph_t, flags: ::core::ffi::c_ulonglong) -> hipError_t;
        }
        unsafe { hipGraphInstantiateWithFlags(pGraphExec, graph, flags) }
    }
}
pub unsafe fn hipGraphInstantiateWithParams(pGraphExec: *mut hipGraphExec_t, graph: hipGraph_t, instantiateParams: *mut hipGraphInstantiateParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphExec_t, hipGraph_t, *mut hipGraphInstantiateParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphInstantiateWithParams") });
        unsafe { _f(pGraphExec, graph, instantiateParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphInstantiateWithParams(pGraphExec: *mut hipGraphExec_t, graph: hipGraph_t, instantiateParams: *mut hipGraphInstantiateParams) -> hipError_t;
        }
        unsafe { hipGraphInstantiateWithParams(pGraphExec, graph, instantiateParams) }
    }
}
pub unsafe fn hipGraphKernelNodeCopyAttributes(hSrc: hipGraphNode_t, hDst: hipGraphNode_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, hipGraphNode_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphKernelNodeCopyAttributes") });
        unsafe { _f(hSrc, hDst) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphKernelNodeCopyAttributes(hSrc: hipGraphNode_t, hDst: hipGraphNode_t) -> hipError_t;
        }
        unsafe { hipGraphKernelNodeCopyAttributes(hSrc, hDst) }
    }
}
pub unsafe fn hipGraphKernelNodeGetAttribute(hNode: hipGraphNode_t, attr: hipLaunchAttributeID, value: *mut hipLaunchAttributeValue) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, hipLaunchAttributeID, *mut hipLaunchAttributeValue) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphKernelNodeGetAttribute") });
        unsafe { _f(hNode, attr, value) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphKernelNodeGetAttribute(hNode: hipGraphNode_t, attr: hipLaunchAttributeID, value: *mut hipLaunchAttributeValue) -> hipError_t;
        }
        unsafe { hipGraphKernelNodeGetAttribute(hNode, attr, value) }
    }
}
pub unsafe fn hipGraphKernelNodeGetParams(node: hipGraphNode_t, pNodeParams: *mut hipKernelNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipKernelNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphKernelNodeGetParams") });
        unsafe { _f(node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphKernelNodeGetParams(node: hipGraphNode_t, pNodeParams: *mut hipKernelNodeParams) -> hipError_t;
        }
        unsafe { hipGraphKernelNodeGetParams(node, pNodeParams) }
    }
}
pub unsafe fn hipGraphKernelNodeSetAttribute(hNode: hipGraphNode_t, attr: hipLaunchAttributeID, value: *const hipLaunchAttributeValue) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, hipLaunchAttributeID, *const hipLaunchAttributeValue) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphKernelNodeSetAttribute") });
        unsafe { _f(hNode, attr, value) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphKernelNodeSetAttribute(hNode: hipGraphNode_t, attr: hipLaunchAttributeID, value: *const hipLaunchAttributeValue) -> hipError_t;
        }
        unsafe { hipGraphKernelNodeSetAttribute(hNode, attr, value) }
    }
}
pub unsafe fn hipGraphKernelNodeSetParams(node: hipGraphNode_t, pNodeParams: *const hipKernelNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *const hipKernelNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphKernelNodeSetParams") });
        unsafe { _f(node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphKernelNodeSetParams(node: hipGraphNode_t, pNodeParams: *const hipKernelNodeParams) -> hipError_t;
        }
        unsafe { hipGraphKernelNodeSetParams(node, pNodeParams) }
    }
}
pub unsafe fn hipGraphLaunch(graphExec: hipGraphExec_t, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphLaunch") });
        unsafe { _f(graphExec, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphLaunch(graphExec: hipGraphExec_t, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipGraphLaunch(graphExec, stream) }
    }
}
pub unsafe fn hipGraphLaunch_spt(graphExec: hipGraphExec_t, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphLaunch_spt") });
        unsafe { _f(graphExec, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphLaunch_spt(graphExec: hipGraphExec_t, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipGraphLaunch_spt(graphExec, stream) }
    }
}
pub unsafe fn hipGraphMemAllocNodeGetParams(node: hipGraphNode_t, pNodeParams: *mut hipMemAllocNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipMemAllocNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphMemAllocNodeGetParams") });
        unsafe { _f(node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphMemAllocNodeGetParams(node: hipGraphNode_t, pNodeParams: *mut hipMemAllocNodeParams) -> hipError_t;
        }
        unsafe { hipGraphMemAllocNodeGetParams(node, pNodeParams) }
    }
}
pub unsafe fn hipGraphMemFreeNodeGetParams(node: hipGraphNode_t, dev_ptr: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphMemFreeNodeGetParams") });
        unsafe { _f(node, dev_ptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphMemFreeNodeGetParams(node: hipGraphNode_t, dev_ptr: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipGraphMemFreeNodeGetParams(node, dev_ptr) }
    }
}
pub unsafe fn hipGraphMemcpyNodeGetParams(node: hipGraphNode_t, pNodeParams: *mut hipMemcpy3DParms) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipMemcpy3DParms) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphMemcpyNodeGetParams") });
        unsafe { _f(node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphMemcpyNodeGetParams(node: hipGraphNode_t, pNodeParams: *mut hipMemcpy3DParms) -> hipError_t;
        }
        unsafe { hipGraphMemcpyNodeGetParams(node, pNodeParams) }
    }
}
pub unsafe fn hipGraphMemcpyNodeSetParams(node: hipGraphNode_t, pNodeParams: *const hipMemcpy3DParms) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *const hipMemcpy3DParms) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphMemcpyNodeSetParams") });
        unsafe { _f(node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphMemcpyNodeSetParams(node: hipGraphNode_t, pNodeParams: *const hipMemcpy3DParms) -> hipError_t;
        }
        unsafe { hipGraphMemcpyNodeSetParams(node, pNodeParams) }
    }
}
pub unsafe fn hipGraphMemcpyNodeSetParams1D(node: hipGraphNode_t, dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphMemcpyNodeSetParams1D") });
        unsafe { _f(node, dst, src, count, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphMemcpyNodeSetParams1D(node: hipGraphNode_t, dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipGraphMemcpyNodeSetParams1D(node, dst, src, count, kind) }
    }
}
pub unsafe fn hipGraphMemcpyNodeSetParamsFromSymbol(node: hipGraphNode_t, dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, count: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphMemcpyNodeSetParamsFromSymbol") });
        unsafe { _f(node, dst, symbol, count, offset, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphMemcpyNodeSetParamsFromSymbol(node: hipGraphNode_t, dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, count: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipGraphMemcpyNodeSetParamsFromSymbol(node, dst, symbol, count, offset, kind) }
    }
}
pub unsafe fn hipGraphMemcpyNodeSetParamsToSymbol(node: hipGraphNode_t, symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphMemcpyNodeSetParamsToSymbol") });
        unsafe { _f(node, symbol, src, count, offset, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphMemcpyNodeSetParamsToSymbol(node: hipGraphNode_t, symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipGraphMemcpyNodeSetParamsToSymbol(node, symbol, src, count, offset, kind) }
    }
}
pub unsafe fn hipGraphMemsetNodeGetParams(node: hipGraphNode_t, pNodeParams: *mut hipMemsetParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipMemsetParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphMemsetNodeGetParams") });
        unsafe { _f(node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphMemsetNodeGetParams(node: hipGraphNode_t, pNodeParams: *mut hipMemsetParams) -> hipError_t;
        }
        unsafe { hipGraphMemsetNodeGetParams(node, pNodeParams) }
    }
}
pub unsafe fn hipGraphMemsetNodeSetParams(node: hipGraphNode_t, pNodeParams: *const hipMemsetParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *const hipMemsetParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphMemsetNodeSetParams") });
        unsafe { _f(node, pNodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphMemsetNodeSetParams(node: hipGraphNode_t, pNodeParams: *const hipMemsetParams) -> hipError_t;
        }
        unsafe { hipGraphMemsetNodeSetParams(node, pNodeParams) }
    }
}
pub unsafe fn hipGraphNodeFindInClone(pNode: *mut hipGraphNode_t, originalNode: hipGraphNode_t, clonedGraph: hipGraph_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipGraphNode_t, hipGraphNode_t, hipGraph_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphNodeFindInClone") });
        unsafe { _f(pNode, originalNode, clonedGraph) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphNodeFindInClone(pNode: *mut hipGraphNode_t, originalNode: hipGraphNode_t, clonedGraph: hipGraph_t) -> hipError_t;
        }
        unsafe { hipGraphNodeFindInClone(pNode, originalNode, clonedGraph) }
    }
}
pub unsafe fn hipGraphNodeGetDependencies(node: hipGraphNode_t, pDependencies: *mut hipGraphNode_t, pNumDependencies: *mut usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipGraphNode_t, *mut usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphNodeGetDependencies") });
        unsafe { _f(node, pDependencies, pNumDependencies) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphNodeGetDependencies(node: hipGraphNode_t, pDependencies: *mut hipGraphNode_t, pNumDependencies: *mut usize) -> hipError_t;
        }
        unsafe { hipGraphNodeGetDependencies(node, pDependencies, pNumDependencies) }
    }
}
pub unsafe fn hipGraphNodeGetDependentNodes(node: hipGraphNode_t, pDependentNodes: *mut hipGraphNode_t, pNumDependentNodes: *mut usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipGraphNode_t, *mut usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphNodeGetDependentNodes") });
        unsafe { _f(node, pDependentNodes, pNumDependentNodes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphNodeGetDependentNodes(node: hipGraphNode_t, pDependentNodes: *mut hipGraphNode_t, pNumDependentNodes: *mut usize) -> hipError_t;
        }
        unsafe { hipGraphNodeGetDependentNodes(node, pDependentNodes, pNumDependentNodes) }
    }
}
pub unsafe fn hipGraphNodeGetEnabled(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, isEnabled: *mut ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, *mut ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphNodeGetEnabled") });
        unsafe { _f(hGraphExec, hNode, isEnabled) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphNodeGetEnabled(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, isEnabled: *mut ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipGraphNodeGetEnabled(hGraphExec, hNode, isEnabled) }
    }
}
pub unsafe fn hipGraphNodeGetType(node: hipGraphNode_t, pType: *mut hipGraphNodeType) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipGraphNodeType) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphNodeGetType") });
        unsafe { _f(node, pType) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphNodeGetType(node: hipGraphNode_t, pType: *mut hipGraphNodeType) -> hipError_t;
        }
        unsafe { hipGraphNodeGetType(node, pType) }
    }
}
pub unsafe fn hipGraphNodeSetEnabled(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, isEnabled: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipGraphNode_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphNodeSetEnabled") });
        unsafe { _f(hGraphExec, hNode, isEnabled) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphNodeSetEnabled(hGraphExec: hipGraphExec_t, hNode: hipGraphNode_t, isEnabled: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipGraphNodeSetEnabled(hGraphExec, hNode, isEnabled) }
    }
}
pub unsafe fn hipGraphNodeSetParams(node: hipGraphNode_t, nodeParams: *mut hipGraphNodeParams) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphNode_t, *mut hipGraphNodeParams) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphNodeSetParams") });
        unsafe { _f(node, nodeParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphNodeSetParams(node: hipGraphNode_t, nodeParams: *mut hipGraphNodeParams) -> hipError_t;
        }
        unsafe { hipGraphNodeSetParams(node, nodeParams) }
    }
}
pub unsafe fn hipGraphReleaseUserObject(graph: hipGraph_t, object: hipUserObject_t, count: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraph_t, hipUserObject_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphReleaseUserObject") });
        unsafe { _f(graph, object, count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphReleaseUserObject(graph: hipGraph_t, object: hipUserObject_t, count: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipGraphReleaseUserObject(graph, object, count) }
    }
}
pub unsafe fn hipGraphRemoveDependencies(graph: hipGraph_t, from: *const hipGraphNode_t, to: *const hipGraphNode_t, numDependencies: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraph_t, *const hipGraphNode_t, *const hipGraphNode_t, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphRemoveDependencies") });
        unsafe { _f(graph, from, to, numDependencies) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphRemoveDependencies(graph: hipGraph_t, from: *const hipGraphNode_t, to: *const hipGraphNode_t, numDependencies: usize) -> hipError_t;
        }
        unsafe { hipGraphRemoveDependencies(graph, from, to, numDependencies) }
    }
}
pub unsafe fn hipGraphRetainUserObject(graph: hipGraph_t, object: hipUserObject_t, count: ::core::ffi::c_uint, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraph_t, hipUserObject_t, ::core::ffi::c_uint, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphRetainUserObject") });
        unsafe { _f(graph, object, count, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphRetainUserObject(graph: hipGraph_t, object: hipUserObject_t, count: ::core::ffi::c_uint, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipGraphRetainUserObject(graph, object, count, flags) }
    }
}
pub unsafe fn hipGraphUpload(graphExec: hipGraphExec_t, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphExec_t, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphUpload") });
        unsafe { _f(graphExec, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphUpload(graphExec: hipGraphExec_t, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipGraphUpload(graphExec, stream) }
    }
}
pub unsafe fn hipGraphicsMapResources(count: ::core::ffi::c_int, resources: *mut hipGraphicsResource_t, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_int, *mut hipGraphicsResource_t, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphicsMapResources") });
        unsafe { _f(count, resources, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphicsMapResources(count: ::core::ffi::c_int, resources: *mut hipGraphicsResource_t, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipGraphicsMapResources(count, resources, stream) }
    }
}
pub unsafe fn hipGraphicsResourceGetMappedPointer(devPtr: *mut *mut ::core::ffi::c_void, size: *mut usize, resource: hipGraphicsResource_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, *mut usize, hipGraphicsResource_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphicsResourceGetMappedPointer") });
        unsafe { _f(devPtr, size, resource) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphicsResourceGetMappedPointer(devPtr: *mut *mut ::core::ffi::c_void, size: *mut usize, resource: hipGraphicsResource_t) -> hipError_t;
        }
        unsafe { hipGraphicsResourceGetMappedPointer(devPtr, size, resource) }
    }
}
pub unsafe fn hipGraphicsSubResourceGetMappedArray(array: *mut hipArray_t, resource: hipGraphicsResource_t, arrayIndex: ::core::ffi::c_uint, mipLevel: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipArray_t, hipGraphicsResource_t, ::core::ffi::c_uint, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphicsSubResourceGetMappedArray") });
        unsafe { _f(array, resource, arrayIndex, mipLevel) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphicsSubResourceGetMappedArray(array: *mut hipArray_t, resource: hipGraphicsResource_t, arrayIndex: ::core::ffi::c_uint, mipLevel: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipGraphicsSubResourceGetMappedArray(array, resource, arrayIndex, mipLevel) }
    }
}
pub unsafe fn hipGraphicsUnmapResources(count: ::core::ffi::c_int, resources: *mut hipGraphicsResource_t, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_int, *mut hipGraphicsResource_t, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphicsUnmapResources") });
        unsafe { _f(count, resources, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphicsUnmapResources(count: ::core::ffi::c_int, resources: *mut hipGraphicsResource_t, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipGraphicsUnmapResources(count, resources, stream) }
    }
}
pub unsafe fn hipGraphicsUnregisterResource(resource: hipGraphicsResource_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipGraphicsResource_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipGraphicsUnregisterResource") });
        unsafe { _f(resource) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipGraphicsUnregisterResource(resource: hipGraphicsResource_t) -> hipError_t;
        }
        unsafe { hipGraphicsUnregisterResource(resource) }
    }
}
pub unsafe fn hipHostAlloc(ptr: *mut *mut ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, usize, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipHostAlloc") });
        unsafe { _f(ptr, size, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipHostAlloc(ptr: *mut *mut ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipHostAlloc(ptr, size, flags) }
    }
}
pub unsafe fn hipHostFree(ptr: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipHostFree") });
        unsafe { _f(ptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipHostFree(ptr: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipHostFree(ptr) }
    }
}
pub unsafe fn hipHostGetDevicePointer(devPtr: *mut *mut ::core::ffi::c_void, hstPtr: *mut ::core::ffi::c_void, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipHostGetDevicePointer") });
        unsafe { _f(devPtr, hstPtr, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipHostGetDevicePointer(devPtr: *mut *mut ::core::ffi::c_void, hstPtr: *mut ::core::ffi::c_void, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipHostGetDevicePointer(devPtr, hstPtr, flags) }
    }
}
pub unsafe fn hipHostGetFlags(flagsPtr: *mut ::core::ffi::c_uint, hostPtr: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_uint, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipHostGetFlags") });
        unsafe { _f(flagsPtr, hostPtr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipHostGetFlags(flagsPtr: *mut ::core::ffi::c_uint, hostPtr: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipHostGetFlags(flagsPtr, hostPtr) }
    }
}
pub unsafe fn hipHostMalloc(ptr: *mut *mut ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, usize, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipHostMalloc") });
        unsafe { _f(ptr, size, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipHostMalloc(ptr: *mut *mut ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipHostMalloc(ptr, size, flags) }
    }
}
pub unsafe fn hipHostRegister(hostPtr: *mut ::core::ffi::c_void, sizeBytes: usize, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipHostRegister") });
        unsafe { _f(hostPtr, sizeBytes, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipHostRegister(hostPtr: *mut ::core::ffi::c_void, sizeBytes: usize, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipHostRegister(hostPtr, sizeBytes, flags) }
    }
}
pub unsafe fn hipHostUnregister(hostPtr: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipHostUnregister") });
        unsafe { _f(hostPtr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipHostUnregister(hostPtr: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipHostUnregister(hostPtr) }
    }
}
pub unsafe fn hipImportExternalMemory(extMem_out: *mut hipExternalMemory_t, memHandleDesc: *const hipExternalMemoryHandleDesc) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipExternalMemory_t, *const hipExternalMemoryHandleDesc) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipImportExternalMemory") });
        unsafe { _f(extMem_out, memHandleDesc) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipImportExternalMemory(extMem_out: *mut hipExternalMemory_t, memHandleDesc: *const hipExternalMemoryHandleDesc) -> hipError_t;
        }
        unsafe { hipImportExternalMemory(extMem_out, memHandleDesc) }
    }
}
pub unsafe fn hipImportExternalSemaphore(extSem_out: *mut hipExternalSemaphore_t, semHandleDesc: *const hipExternalSemaphoreHandleDesc) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipExternalSemaphore_t, *const hipExternalSemaphoreHandleDesc) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipImportExternalSemaphore") });
        unsafe { _f(extSem_out, semHandleDesc) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipImportExternalSemaphore(extSem_out: *mut hipExternalSemaphore_t, semHandleDesc: *const hipExternalSemaphoreHandleDesc) -> hipError_t;
        }
        unsafe { hipImportExternalSemaphore(extSem_out, semHandleDesc) }
    }
}
pub unsafe fn hipInit(flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipInit") });
        unsafe { _f(flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipInit(flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipInit(flags) }
    }
}
pub unsafe fn hipIpcCloseMemHandle(devPtr: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipIpcCloseMemHandle") });
        unsafe { _f(devPtr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipIpcCloseMemHandle(devPtr: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipIpcCloseMemHandle(devPtr) }
    }
}
pub unsafe fn hipIpcGetEventHandle(handle: *mut hipIpcEventHandle_t, event: hipEvent_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipIpcEventHandle_t, hipEvent_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipIpcGetEventHandle") });
        unsafe { _f(handle, event) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipIpcGetEventHandle(handle: *mut hipIpcEventHandle_t, event: hipEvent_t) -> hipError_t;
        }
        unsafe { hipIpcGetEventHandle(handle, event) }
    }
}
pub unsafe fn hipIpcGetMemHandle(handle: *mut hipIpcMemHandle_t, devPtr: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipIpcMemHandle_t, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipIpcGetMemHandle") });
        unsafe { _f(handle, devPtr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipIpcGetMemHandle(handle: *mut hipIpcMemHandle_t, devPtr: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipIpcGetMemHandle(handle, devPtr) }
    }
}
pub unsafe fn hipIpcOpenEventHandle(event: *mut hipEvent_t, handle: hipIpcEventHandle_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipEvent_t, hipIpcEventHandle_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipIpcOpenEventHandle") });
        unsafe { _f(event, handle) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipIpcOpenEventHandle(event: *mut hipEvent_t, handle: hipIpcEventHandle_t) -> hipError_t;
        }
        unsafe { hipIpcOpenEventHandle(event, handle) }
    }
}
pub unsafe fn hipIpcOpenMemHandle(devPtr: *mut *mut ::core::ffi::c_void, handle: hipIpcMemHandle_t, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, hipIpcMemHandle_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipIpcOpenMemHandle") });
        unsafe { _f(devPtr, handle, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipIpcOpenMemHandle(devPtr: *mut *mut ::core::ffi::c_void, handle: hipIpcMemHandle_t, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipIpcOpenMemHandle(devPtr, handle, flags) }
    }
}
pub unsafe fn hipKernelGetLibrary(library: *mut hipLibrary_t, kernel: hipKernel_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipLibrary_t, hipKernel_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipKernelGetLibrary") });
        unsafe { _f(library, kernel) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipKernelGetLibrary(library: *mut hipLibrary_t, kernel: hipKernel_t) -> hipError_t;
        }
        unsafe { hipKernelGetLibrary(library, kernel) }
    }
}
pub unsafe fn hipKernelGetName(name: *mut *const ::core::ffi::c_char, kernel: hipKernel_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *const ::core::ffi::c_char, hipKernel_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipKernelGetName") });
        unsafe { _f(name, kernel) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipKernelGetName(name: *mut *const ::core::ffi::c_char, kernel: hipKernel_t) -> hipError_t;
        }
        unsafe { hipKernelGetName(name, kernel) }
    }
}
pub unsafe fn hipKernelNameRef(f: hipFunction_t) -> *const ::core::ffi::c_char {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipFunction_t) -> *const ::core::ffi::c_char;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipKernelNameRef") });
        unsafe { _f(f) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipKernelNameRef(f: hipFunction_t) -> *const ::core::ffi::c_char;
        }
        unsafe { hipKernelNameRef(f) }
    }
}
pub unsafe fn hipKernelNameRefByPtr(hostFunction: *const ::core::ffi::c_void, stream: hipStream_t) -> *const ::core::ffi::c_char {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, hipStream_t) -> *const ::core::ffi::c_char;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipKernelNameRefByPtr") });
        unsafe { _f(hostFunction, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipKernelNameRefByPtr(hostFunction: *const ::core::ffi::c_void, stream: hipStream_t) -> *const ::core::ffi::c_char;
        }
        unsafe { hipKernelNameRefByPtr(hostFunction, stream) }
    }
}
pub unsafe fn hipLaunchByPtr(func: *const ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLaunchByPtr") });
        unsafe { _f(func) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLaunchByPtr(func: *const ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipLaunchByPtr(func) }
    }
}
pub unsafe fn hipLaunchCooperativeKernel(f: *const ::core::ffi::c_void, gridDim: dim3, blockDimX: dim3, kernelParams: *mut *mut ::core::ffi::c_void, sharedMemBytes: ::core::ffi::c_uint, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, dim3, dim3, *mut *mut ::core::ffi::c_void, ::core::ffi::c_uint, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLaunchCooperativeKernel") });
        unsafe { _f(f, gridDim, blockDimX, kernelParams, sharedMemBytes, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLaunchCooperativeKernel(f: *const ::core::ffi::c_void, gridDim: dim3, blockDimX: dim3, kernelParams: *mut *mut ::core::ffi::c_void, sharedMemBytes: ::core::ffi::c_uint, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipLaunchCooperativeKernel(f, gridDim, blockDimX, kernelParams, sharedMemBytes, stream) }
    }
}
pub unsafe fn hipLaunchCooperativeKernelMultiDevice(launchParamsList: *mut hipLaunchParams, numDevices: ::core::ffi::c_int, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipLaunchParams, ::core::ffi::c_int, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLaunchCooperativeKernelMultiDevice") });
        unsafe { _f(launchParamsList, numDevices, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLaunchCooperativeKernelMultiDevice(launchParamsList: *mut hipLaunchParams, numDevices: ::core::ffi::c_int, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipLaunchCooperativeKernelMultiDevice(launchParamsList, numDevices, flags) }
    }
}
pub unsafe fn hipLaunchCooperativeKernel_spt(f: *const ::core::ffi::c_void, gridDim: dim3, blockDim: dim3, kernelParams: *mut *mut ::core::ffi::c_void, sharedMemBytes: u32, hStream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, dim3, dim3, *mut *mut ::core::ffi::c_void, u32, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLaunchCooperativeKernel_spt") });
        unsafe { _f(f, gridDim, blockDim, kernelParams, sharedMemBytes, hStream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLaunchCooperativeKernel_spt(f: *const ::core::ffi::c_void, gridDim: dim3, blockDim: dim3, kernelParams: *mut *mut ::core::ffi::c_void, sharedMemBytes: u32, hStream: hipStream_t) -> hipError_t;
        }
        unsafe { hipLaunchCooperativeKernel_spt(f, gridDim, blockDim, kernelParams, sharedMemBytes, hStream) }
    }
}
pub unsafe fn hipLaunchHostFunc(stream: hipStream_t, fn_: hipHostFn_t, userData: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, hipHostFn_t, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLaunchHostFunc") });
        unsafe { _f(stream, fn_, userData) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLaunchHostFunc(stream: hipStream_t, fn_: hipHostFn_t, userData: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipLaunchHostFunc(stream, fn_, userData) }
    }
}
pub unsafe fn hipLaunchHostFunc_spt(stream: hipStream_t, fn_: hipHostFn_t, userData: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, hipHostFn_t, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLaunchHostFunc_spt") });
        unsafe { _f(stream, fn_, userData) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLaunchHostFunc_spt(stream: hipStream_t, fn_: hipHostFn_t, userData: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipLaunchHostFunc_spt(stream, fn_, userData) }
    }
}
pub unsafe fn hipLaunchKernel(function_address: *const ::core::ffi::c_void, numBlocks: dim3, dimBlocks: dim3, args: *mut *mut ::core::ffi::c_void, sharedMemBytes: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, dim3, dim3, *mut *mut ::core::ffi::c_void, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLaunchKernel") });
        unsafe { _f(function_address, numBlocks, dimBlocks, args, sharedMemBytes, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLaunchKernel(function_address: *const ::core::ffi::c_void, numBlocks: dim3, dimBlocks: dim3, args: *mut *mut ::core::ffi::c_void, sharedMemBytes: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipLaunchKernel(function_address, numBlocks, dimBlocks, args, sharedMemBytes, stream) }
    }
}
pub unsafe fn hipLaunchKernelExC(config: *const hipLaunchConfig_t, fPtr: *const ::core::ffi::c_void, args: *mut *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const hipLaunchConfig_t, *const ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLaunchKernelExC") });
        unsafe { _f(config, fPtr, args) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLaunchKernelExC(config: *const hipLaunchConfig_t, fPtr: *const ::core::ffi::c_void, args: *mut *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipLaunchKernelExC(config, fPtr, args) }
    }
}
pub unsafe fn hipLaunchKernel_spt(function_address: *const ::core::ffi::c_void, numBlocks: dim3, dimBlocks: dim3, args: *mut *mut ::core::ffi::c_void, sharedMemBytes: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, dim3, dim3, *mut *mut ::core::ffi::c_void, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLaunchKernel_spt") });
        unsafe { _f(function_address, numBlocks, dimBlocks, args, sharedMemBytes, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLaunchKernel_spt(function_address: *const ::core::ffi::c_void, numBlocks: dim3, dimBlocks: dim3, args: *mut *mut ::core::ffi::c_void, sharedMemBytes: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipLaunchKernel_spt(function_address, numBlocks, dimBlocks, args, sharedMemBytes, stream) }
    }
}
pub unsafe fn hipLibraryEnumerateKernels(kernels: *mut hipKernel_t, numKernels: ::core::ffi::c_uint, library: hipLibrary_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipKernel_t, ::core::ffi::c_uint, hipLibrary_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLibraryEnumerateKernels") });
        unsafe { _f(kernels, numKernels, library) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLibraryEnumerateKernels(kernels: *mut hipKernel_t, numKernels: ::core::ffi::c_uint, library: hipLibrary_t) -> hipError_t;
        }
        unsafe { hipLibraryEnumerateKernels(kernels, numKernels, library) }
    }
}
pub unsafe fn hipLibraryGetKernel(pKernel: *mut hipKernel_t, library: hipLibrary_t, name: *const ::core::ffi::c_char) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipKernel_t, hipLibrary_t, *const ::core::ffi::c_char) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLibraryGetKernel") });
        unsafe { _f(pKernel, library, name) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLibraryGetKernel(pKernel: *mut hipKernel_t, library: hipLibrary_t, name: *const ::core::ffi::c_char) -> hipError_t;
        }
        unsafe { hipLibraryGetKernel(pKernel, library, name) }
    }
}
pub unsafe fn hipLibraryGetKernelCount(count: *mut ::core::ffi::c_uint, library: hipLibrary_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_uint, hipLibrary_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLibraryGetKernelCount") });
        unsafe { _f(count, library) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLibraryGetKernelCount(count: *mut ::core::ffi::c_uint, library: hipLibrary_t) -> hipError_t;
        }
        unsafe { hipLibraryGetKernelCount(count, library) }
    }
}
pub unsafe fn hipLibraryLoadData(library: *mut hipLibrary_t, code: *const ::core::ffi::c_void, jitOptions: *mut hipJitOption, jitOptionsValues: *mut *mut ::core::ffi::c_void, numJitOptions: ::core::ffi::c_uint, libraryOptions: *mut hipLibraryOption, libraryOptionValues: *mut *mut ::core::ffi::c_void, numLibraryOptions: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipLibrary_t, *const ::core::ffi::c_void, *mut hipJitOption, *mut *mut ::core::ffi::c_void, ::core::ffi::c_uint, *mut hipLibraryOption, *mut *mut ::core::ffi::c_void, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLibraryLoadData") });
        unsafe { _f(library, code, jitOptions, jitOptionsValues, numJitOptions, libraryOptions, libraryOptionValues, numLibraryOptions) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLibraryLoadData(library: *mut hipLibrary_t, code: *const ::core::ffi::c_void, jitOptions: *mut hipJitOption, jitOptionsValues: *mut *mut ::core::ffi::c_void, numJitOptions: ::core::ffi::c_uint, libraryOptions: *mut hipLibraryOption, libraryOptionValues: *mut *mut ::core::ffi::c_void, numLibraryOptions: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipLibraryLoadData(library, code, jitOptions, jitOptionsValues, numJitOptions, libraryOptions, libraryOptionValues, numLibraryOptions) }
    }
}
pub unsafe fn hipLibraryLoadFromFile(library: *mut hipLibrary_t, fileName: *const ::core::ffi::c_char, jitOptions: *mut hipJitOption, jitOptionsValues: *mut *mut ::core::ffi::c_void, numJitOptions: ::core::ffi::c_uint, libraryOptions: *mut hipLibraryOption, libraryOptionValues: *mut *mut ::core::ffi::c_void, numLibraryOptions: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipLibrary_t, *const ::core::ffi::c_char, *mut hipJitOption, *mut *mut ::core::ffi::c_void, ::core::ffi::c_uint, *mut hipLibraryOption, *mut *mut ::core::ffi::c_void, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLibraryLoadFromFile") });
        unsafe { _f(library, fileName, jitOptions, jitOptionsValues, numJitOptions, libraryOptions, libraryOptionValues, numLibraryOptions) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLibraryLoadFromFile(library: *mut hipLibrary_t, fileName: *const ::core::ffi::c_char, jitOptions: *mut hipJitOption, jitOptionsValues: *mut *mut ::core::ffi::c_void, numJitOptions: ::core::ffi::c_uint, libraryOptions: *mut hipLibraryOption, libraryOptionValues: *mut *mut ::core::ffi::c_void, numLibraryOptions: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipLibraryLoadFromFile(library, fileName, jitOptions, jitOptionsValues, numJitOptions, libraryOptions, libraryOptionValues, numLibraryOptions) }
    }
}
pub unsafe fn hipLibraryUnload(library: hipLibrary_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipLibrary_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLibraryUnload") });
        unsafe { _f(library) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLibraryUnload(library: hipLibrary_t) -> hipError_t;
        }
        unsafe { hipLibraryUnload(library) }
    }
}
pub unsafe fn hipLinkAddData(state: hipLinkState_t, type_: hipJitInputType, data: *mut ::core::ffi::c_void, size: usize, name: *const ::core::ffi::c_char, numOptions: ::core::ffi::c_uint, options: *mut hipJitOption, optionValues: *mut *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipLinkState_t, hipJitInputType, *mut ::core::ffi::c_void, usize, *const ::core::ffi::c_char, ::core::ffi::c_uint, *mut hipJitOption, *mut *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLinkAddData") });
        unsafe { _f(state, type_, data, size, name, numOptions, options, optionValues) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLinkAddData(state: hipLinkState_t, type_: hipJitInputType, data: *mut ::core::ffi::c_void, size: usize, name: *const ::core::ffi::c_char, numOptions: ::core::ffi::c_uint, options: *mut hipJitOption, optionValues: *mut *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipLinkAddData(state, type_, data, size, name, numOptions, options, optionValues) }
    }
}
pub unsafe fn hipLinkAddFile(state: hipLinkState_t, type_: hipJitInputType, path: *const ::core::ffi::c_char, numOptions: ::core::ffi::c_uint, options: *mut hipJitOption, optionValues: *mut *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipLinkState_t, hipJitInputType, *const ::core::ffi::c_char, ::core::ffi::c_uint, *mut hipJitOption, *mut *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLinkAddFile") });
        unsafe { _f(state, type_, path, numOptions, options, optionValues) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLinkAddFile(state: hipLinkState_t, type_: hipJitInputType, path: *const ::core::ffi::c_char, numOptions: ::core::ffi::c_uint, options: *mut hipJitOption, optionValues: *mut *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipLinkAddFile(state, type_, path, numOptions, options, optionValues) }
    }
}
pub unsafe fn hipLinkComplete(state: hipLinkState_t, hipBinOut: *mut *mut ::core::ffi::c_void, sizeOut: *mut usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipLinkState_t, *mut *mut ::core::ffi::c_void, *mut usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLinkComplete") });
        unsafe { _f(state, hipBinOut, sizeOut) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLinkComplete(state: hipLinkState_t, hipBinOut: *mut *mut ::core::ffi::c_void, sizeOut: *mut usize) -> hipError_t;
        }
        unsafe { hipLinkComplete(state, hipBinOut, sizeOut) }
    }
}
pub unsafe fn hipLinkCreate(numOptions: ::core::ffi::c_uint, options: *mut hipJitOption, optionValues: *mut *mut ::core::ffi::c_void, stateOut: *mut hipLinkState_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_uint, *mut hipJitOption, *mut *mut ::core::ffi::c_void, *mut hipLinkState_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLinkCreate") });
        unsafe { _f(numOptions, options, optionValues, stateOut) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLinkCreate(numOptions: ::core::ffi::c_uint, options: *mut hipJitOption, optionValues: *mut *mut ::core::ffi::c_void, stateOut: *mut hipLinkState_t) -> hipError_t;
        }
        unsafe { hipLinkCreate(numOptions, options, optionValues, stateOut) }
    }
}
pub unsafe fn hipLinkDestroy(state: hipLinkState_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipLinkState_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipLinkDestroy") });
        unsafe { _f(state) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipLinkDestroy(state: hipLinkState_t) -> hipError_t;
        }
        unsafe { hipLinkDestroy(state) }
    }
}
pub unsafe fn hipMalloc(ptr: *mut *mut ::core::ffi::c_void, size: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMalloc") });
        unsafe { _f(ptr, size) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMalloc(ptr: *mut *mut ::core::ffi::c_void, size: usize) -> hipError_t;
        }
        unsafe { hipMalloc(ptr, size) }
    }
}
pub unsafe fn hipMalloc3D(pitchedDevPtr: *mut hipPitchedPtr, extent: hipExtent) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipPitchedPtr, hipExtent) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMalloc3D") });
        unsafe { _f(pitchedDevPtr, extent) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMalloc3D(pitchedDevPtr: *mut hipPitchedPtr, extent: hipExtent) -> hipError_t;
        }
        unsafe { hipMalloc3D(pitchedDevPtr, extent) }
    }
}
pub unsafe fn hipMalloc3DArray(array: *mut hipArray_t, desc: *const hipChannelFormatDesc, extent: hipExtent, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipArray_t, *const hipChannelFormatDesc, hipExtent, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMalloc3DArray") });
        unsafe { _f(array, desc, extent, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMalloc3DArray(array: *mut hipArray_t, desc: *const hipChannelFormatDesc, extent: hipExtent, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipMalloc3DArray(array, desc, extent, flags) }
    }
}
pub unsafe fn hipMallocArray(array: *mut hipArray_t, desc: *const hipChannelFormatDesc, width: usize, height: usize, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipArray_t, *const hipChannelFormatDesc, usize, usize, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMallocArray") });
        unsafe { _f(array, desc, width, height, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMallocArray(array: *mut hipArray_t, desc: *const hipChannelFormatDesc, width: usize, height: usize, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipMallocArray(array, desc, width, height, flags) }
    }
}
pub unsafe fn hipMallocAsync(dev_ptr: *mut *mut ::core::ffi::c_void, size: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMallocAsync") });
        unsafe { _f(dev_ptr, size, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMallocAsync(dev_ptr: *mut *mut ::core::ffi::c_void, size: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMallocAsync(dev_ptr, size, stream) }
    }
}
pub unsafe fn hipMallocFromPoolAsync(dev_ptr: *mut *mut ::core::ffi::c_void, size: usize, mem_pool: hipMemPool_t, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, usize, hipMemPool_t, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMallocFromPoolAsync") });
        unsafe { _f(dev_ptr, size, mem_pool, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMallocFromPoolAsync(dev_ptr: *mut *mut ::core::ffi::c_void, size: usize, mem_pool: hipMemPool_t, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMallocFromPoolAsync(dev_ptr, size, mem_pool, stream) }
    }
}
pub unsafe fn hipMallocHost(ptr: *mut *mut ::core::ffi::c_void, size: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMallocHost") });
        unsafe { _f(ptr, size) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMallocHost(ptr: *mut *mut ::core::ffi::c_void, size: usize) -> hipError_t;
        }
        unsafe { hipMallocHost(ptr, size) }
    }
}
pub unsafe fn hipMallocManaged(dev_ptr: *mut *mut ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, usize, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMallocManaged") });
        unsafe { _f(dev_ptr, size, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMallocManaged(dev_ptr: *mut *mut ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipMallocManaged(dev_ptr, size, flags) }
    }
}
pub unsafe fn hipMallocMipmappedArray(mipmappedArray: *mut hipMipmappedArray_t, desc: *const hipChannelFormatDesc, extent: hipExtent, numLevels: ::core::ffi::c_uint, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMipmappedArray_t, *const hipChannelFormatDesc, hipExtent, ::core::ffi::c_uint, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMallocMipmappedArray") });
        unsafe { _f(mipmappedArray, desc, extent, numLevels, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMallocMipmappedArray(mipmappedArray: *mut hipMipmappedArray_t, desc: *const hipChannelFormatDesc, extent: hipExtent, numLevels: ::core::ffi::c_uint, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipMallocMipmappedArray(mipmappedArray, desc, extent, numLevels, flags) }
    }
}
pub unsafe fn hipMallocPitch(ptr: *mut *mut ::core::ffi::c_void, pitch: *mut usize, width: usize, height: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, *mut usize, usize, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMallocPitch") });
        unsafe { _f(ptr, pitch, width, height) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMallocPitch(ptr: *mut *mut ::core::ffi::c_void, pitch: *mut usize, width: usize, height: usize) -> hipError_t;
        }
        unsafe { hipMallocPitch(ptr, pitch, width, height) }
    }
}
pub unsafe fn hipMemAddressFree(devPtr: *mut ::core::ffi::c_void, size: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemAddressFree") });
        unsafe { _f(devPtr, size) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemAddressFree(devPtr: *mut ::core::ffi::c_void, size: usize) -> hipError_t;
        }
        unsafe { hipMemAddressFree(devPtr, size) }
    }
}
pub unsafe fn hipMemAddressReserve(ptr: *mut *mut ::core::ffi::c_void, size: usize, alignment: usize, addr: *mut ::core::ffi::c_void, flags: ::core::ffi::c_ulonglong) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, usize, usize, *mut ::core::ffi::c_void, ::core::ffi::c_ulonglong) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemAddressReserve") });
        unsafe { _f(ptr, size, alignment, addr, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemAddressReserve(ptr: *mut *mut ::core::ffi::c_void, size: usize, alignment: usize, addr: *mut ::core::ffi::c_void, flags: ::core::ffi::c_ulonglong) -> hipError_t;
        }
        unsafe { hipMemAddressReserve(ptr, size, alignment, addr, flags) }
    }
}
pub unsafe fn hipMemAdvise(dev_ptr: *const ::core::ffi::c_void, count: usize, advice: hipMemoryAdvise, device: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, usize, hipMemoryAdvise, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemAdvise") });
        unsafe { _f(dev_ptr, count, advice, device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemAdvise(dev_ptr: *const ::core::ffi::c_void, count: usize, advice: hipMemoryAdvise, device: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipMemAdvise(dev_ptr, count, advice, device) }
    }
}
pub unsafe fn hipMemAdvise_v2(dev_ptr: *const ::core::ffi::c_void, count: usize, advice: hipMemoryAdvise, location: hipMemLocation) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, usize, hipMemoryAdvise, hipMemLocation) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemAdvise_v2") });
        unsafe { _f(dev_ptr, count, advice, location) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemAdvise_v2(dev_ptr: *const ::core::ffi::c_void, count: usize, advice: hipMemoryAdvise, location: hipMemLocation) -> hipError_t;
        }
        unsafe { hipMemAdvise_v2(dev_ptr, count, advice, location) }
    }
}
pub unsafe fn hipMemAllocHost(ptr: *mut *mut ::core::ffi::c_void, size: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemAllocHost") });
        unsafe { _f(ptr, size) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemAllocHost(ptr: *mut *mut ::core::ffi::c_void, size: usize) -> hipError_t;
        }
        unsafe { hipMemAllocHost(ptr, size) }
    }
}
pub unsafe fn hipMemAllocPitch(dptr: *mut hipDeviceptr_t, pitch: *mut usize, widthInBytes: usize, height: usize, elementSizeBytes: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipDeviceptr_t, *mut usize, usize, usize, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemAllocPitch") });
        unsafe { _f(dptr, pitch, widthInBytes, height, elementSizeBytes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemAllocPitch(dptr: *mut hipDeviceptr_t, pitch: *mut usize, widthInBytes: usize, height: usize, elementSizeBytes: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipMemAllocPitch(dptr, pitch, widthInBytes, height, elementSizeBytes) }
    }
}
pub unsafe fn hipMemCreate(handle: *mut hipMemGenericAllocationHandle_t, size: usize, prop: *const hipMemAllocationProp, flags: ::core::ffi::c_ulonglong) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMemGenericAllocationHandle_t, usize, *const hipMemAllocationProp, ::core::ffi::c_ulonglong) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemCreate") });
        unsafe { _f(handle, size, prop, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemCreate(handle: *mut hipMemGenericAllocationHandle_t, size: usize, prop: *const hipMemAllocationProp, flags: ::core::ffi::c_ulonglong) -> hipError_t;
        }
        unsafe { hipMemCreate(handle, size, prop, flags) }
    }
}
pub unsafe fn hipMemExportToShareableHandle(shareableHandle: *mut ::core::ffi::c_void, handle: hipMemGenericAllocationHandle_t, handleType: hipMemAllocationHandleType, flags: ::core::ffi::c_ulonglong) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, hipMemGenericAllocationHandle_t, hipMemAllocationHandleType, ::core::ffi::c_ulonglong) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemExportToShareableHandle") });
        unsafe { _f(shareableHandle, handle, handleType, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemExportToShareableHandle(shareableHandle: *mut ::core::ffi::c_void, handle: hipMemGenericAllocationHandle_t, handleType: hipMemAllocationHandleType, flags: ::core::ffi::c_ulonglong) -> hipError_t;
        }
        unsafe { hipMemExportToShareableHandle(shareableHandle, handle, handleType, flags) }
    }
}
pub unsafe fn hipMemGetAccess(flags: *mut ::core::ffi::c_ulonglong, location: *const hipMemLocation, ptr: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_ulonglong, *const hipMemLocation, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemGetAccess") });
        unsafe { _f(flags, location, ptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemGetAccess(flags: *mut ::core::ffi::c_ulonglong, location: *const hipMemLocation, ptr: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipMemGetAccess(flags, location, ptr) }
    }
}
pub unsafe fn hipMemGetAddressRange(pbase: *mut hipDeviceptr_t, psize: *mut usize, dptr: hipDeviceptr_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipDeviceptr_t, *mut usize, hipDeviceptr_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemGetAddressRange") });
        unsafe { _f(pbase, psize, dptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemGetAddressRange(pbase: *mut hipDeviceptr_t, psize: *mut usize, dptr: hipDeviceptr_t) -> hipError_t;
        }
        unsafe { hipMemGetAddressRange(pbase, psize, dptr) }
    }
}
pub unsafe fn hipMemGetAllocationGranularity(granularity: *mut usize, prop: *const hipMemAllocationProp, option: hipMemAllocationGranularity_flags) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut usize, *const hipMemAllocationProp, hipMemAllocationGranularity_flags) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemGetAllocationGranularity") });
        unsafe { _f(granularity, prop, option) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemGetAllocationGranularity(granularity: *mut usize, prop: *const hipMemAllocationProp, option: hipMemAllocationGranularity_flags) -> hipError_t;
        }
        unsafe { hipMemGetAllocationGranularity(granularity, prop, option) }
    }
}
pub unsafe fn hipMemGetAllocationPropertiesFromHandle(prop: *mut hipMemAllocationProp, handle: hipMemGenericAllocationHandle_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMemAllocationProp, hipMemGenericAllocationHandle_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemGetAllocationPropertiesFromHandle") });
        unsafe { _f(prop, handle) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemGetAllocationPropertiesFromHandle(prop: *mut hipMemAllocationProp, handle: hipMemGenericAllocationHandle_t) -> hipError_t;
        }
        unsafe { hipMemGetAllocationPropertiesFromHandle(prop, handle) }
    }
}
pub unsafe fn hipMemGetHandleForAddressRange(handle: *mut ::core::ffi::c_void, dptr: hipDeviceptr_t, size: usize, handleType: hipMemRangeHandleType, flags: ::core::ffi::c_ulonglong) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, hipDeviceptr_t, usize, hipMemRangeHandleType, ::core::ffi::c_ulonglong) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemGetHandleForAddressRange") });
        unsafe { _f(handle, dptr, size, handleType, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemGetHandleForAddressRange(handle: *mut ::core::ffi::c_void, dptr: hipDeviceptr_t, size: usize, handleType: hipMemRangeHandleType, flags: ::core::ffi::c_ulonglong) -> hipError_t;
        }
        unsafe { hipMemGetHandleForAddressRange(handle, dptr, size, handleType, flags) }
    }
}
pub unsafe fn hipMemGetInfo(free: *mut usize, total: *mut usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut usize, *mut usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemGetInfo") });
        unsafe { _f(free, total) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemGetInfo(free: *mut usize, total: *mut usize) -> hipError_t;
        }
        unsafe { hipMemGetInfo(free, total) }
    }
}
pub unsafe fn hipMemImportFromShareableHandle(handle: *mut hipMemGenericAllocationHandle_t, osHandle: *mut ::core::ffi::c_void, shHandleType: hipMemAllocationHandleType) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMemGenericAllocationHandle_t, *mut ::core::ffi::c_void, hipMemAllocationHandleType) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemImportFromShareableHandle") });
        unsafe { _f(handle, osHandle, shHandleType) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemImportFromShareableHandle(handle: *mut hipMemGenericAllocationHandle_t, osHandle: *mut ::core::ffi::c_void, shHandleType: hipMemAllocationHandleType) -> hipError_t;
        }
        unsafe { hipMemImportFromShareableHandle(handle, osHandle, shHandleType) }
    }
}
pub unsafe fn hipMemMap(ptr: *mut ::core::ffi::c_void, size: usize, offset: usize, handle: hipMemGenericAllocationHandle_t, flags: ::core::ffi::c_ulonglong) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, usize, hipMemGenericAllocationHandle_t, ::core::ffi::c_ulonglong) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemMap") });
        unsafe { _f(ptr, size, offset, handle, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemMap(ptr: *mut ::core::ffi::c_void, size: usize, offset: usize, handle: hipMemGenericAllocationHandle_t, flags: ::core::ffi::c_ulonglong) -> hipError_t;
        }
        unsafe { hipMemMap(ptr, size, offset, handle, flags) }
    }
}
pub unsafe fn hipMemMapArrayAsync(mapInfoList: *mut hipArrayMapInfo, count: ::core::ffi::c_uint, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipArrayMapInfo, ::core::ffi::c_uint, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemMapArrayAsync") });
        unsafe { _f(mapInfoList, count, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemMapArrayAsync(mapInfoList: *mut hipArrayMapInfo, count: ::core::ffi::c_uint, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemMapArrayAsync(mapInfoList, count, stream) }
    }
}
pub unsafe fn hipMemPoolCreate(mem_pool: *mut hipMemPool_t, pool_props: *const hipMemPoolProps) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMemPool_t, *const hipMemPoolProps) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPoolCreate") });
        unsafe { _f(mem_pool, pool_props) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPoolCreate(mem_pool: *mut hipMemPool_t, pool_props: *const hipMemPoolProps) -> hipError_t;
        }
        unsafe { hipMemPoolCreate(mem_pool, pool_props) }
    }
}
pub unsafe fn hipMemPoolDestroy(mem_pool: hipMemPool_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipMemPool_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPoolDestroy") });
        unsafe { _f(mem_pool) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPoolDestroy(mem_pool: hipMemPool_t) -> hipError_t;
        }
        unsafe { hipMemPoolDestroy(mem_pool) }
    }
}
pub unsafe fn hipMemPoolExportPointer(export_data: *mut hipMemPoolPtrExportData, dev_ptr: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMemPoolPtrExportData, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPoolExportPointer") });
        unsafe { _f(export_data, dev_ptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPoolExportPointer(export_data: *mut hipMemPoolPtrExportData, dev_ptr: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipMemPoolExportPointer(export_data, dev_ptr) }
    }
}
pub unsafe fn hipMemPoolExportToShareableHandle(shared_handle: *mut ::core::ffi::c_void, mem_pool: hipMemPool_t, handle_type: hipMemAllocationHandleType, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, hipMemPool_t, hipMemAllocationHandleType, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPoolExportToShareableHandle") });
        unsafe { _f(shared_handle, mem_pool, handle_type, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPoolExportToShareableHandle(shared_handle: *mut ::core::ffi::c_void, mem_pool: hipMemPool_t, handle_type: hipMemAllocationHandleType, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipMemPoolExportToShareableHandle(shared_handle, mem_pool, handle_type, flags) }
    }
}
pub unsafe fn hipMemPoolGetAccess(flags: *mut hipMemAccessFlags, mem_pool: hipMemPool_t, location: *mut hipMemLocation) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMemAccessFlags, hipMemPool_t, *mut hipMemLocation) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPoolGetAccess") });
        unsafe { _f(flags, mem_pool, location) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPoolGetAccess(flags: *mut hipMemAccessFlags, mem_pool: hipMemPool_t, location: *mut hipMemLocation) -> hipError_t;
        }
        unsafe { hipMemPoolGetAccess(flags, mem_pool, location) }
    }
}
pub unsafe fn hipMemPoolGetAttribute(mem_pool: hipMemPool_t, attr: hipMemPoolAttr, value: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipMemPool_t, hipMemPoolAttr, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPoolGetAttribute") });
        unsafe { _f(mem_pool, attr, value) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPoolGetAttribute(mem_pool: hipMemPool_t, attr: hipMemPoolAttr, value: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipMemPoolGetAttribute(mem_pool, attr, value) }
    }
}
pub unsafe fn hipMemPoolImportFromShareableHandle(mem_pool: *mut hipMemPool_t, shared_handle: *mut ::core::ffi::c_void, handle_type: hipMemAllocationHandleType, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMemPool_t, *mut ::core::ffi::c_void, hipMemAllocationHandleType, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPoolImportFromShareableHandle") });
        unsafe { _f(mem_pool, shared_handle, handle_type, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPoolImportFromShareableHandle(mem_pool: *mut hipMemPool_t, shared_handle: *mut ::core::ffi::c_void, handle_type: hipMemAllocationHandleType, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipMemPoolImportFromShareableHandle(mem_pool, shared_handle, handle_type, flags) }
    }
}
pub unsafe fn hipMemPoolImportPointer(dev_ptr: *mut *mut ::core::ffi::c_void, mem_pool: hipMemPool_t, export_data: *mut hipMemPoolPtrExportData) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, hipMemPool_t, *mut hipMemPoolPtrExportData) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPoolImportPointer") });
        unsafe { _f(dev_ptr, mem_pool, export_data) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPoolImportPointer(dev_ptr: *mut *mut ::core::ffi::c_void, mem_pool: hipMemPool_t, export_data: *mut hipMemPoolPtrExportData) -> hipError_t;
        }
        unsafe { hipMemPoolImportPointer(dev_ptr, mem_pool, export_data) }
    }
}
pub unsafe fn hipMemPoolSetAccess(mem_pool: hipMemPool_t, desc_list: *const hipMemAccessDesc, count: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipMemPool_t, *const hipMemAccessDesc, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPoolSetAccess") });
        unsafe { _f(mem_pool, desc_list, count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPoolSetAccess(mem_pool: hipMemPool_t, desc_list: *const hipMemAccessDesc, count: usize) -> hipError_t;
        }
        unsafe { hipMemPoolSetAccess(mem_pool, desc_list, count) }
    }
}
pub unsafe fn hipMemPoolSetAttribute(mem_pool: hipMemPool_t, attr: hipMemPoolAttr, value: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipMemPool_t, hipMemPoolAttr, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPoolSetAttribute") });
        unsafe { _f(mem_pool, attr, value) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPoolSetAttribute(mem_pool: hipMemPool_t, attr: hipMemPoolAttr, value: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipMemPoolSetAttribute(mem_pool, attr, value) }
    }
}
pub unsafe fn hipMemPoolTrimTo(mem_pool: hipMemPool_t, min_bytes_to_hold: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipMemPool_t, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPoolTrimTo") });
        unsafe { _f(mem_pool, min_bytes_to_hold) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPoolTrimTo(mem_pool: hipMemPool_t, min_bytes_to_hold: usize) -> hipError_t;
        }
        unsafe { hipMemPoolTrimTo(mem_pool, min_bytes_to_hold) }
    }
}
pub unsafe fn hipMemPrefetchAsync(dev_ptr: *const ::core::ffi::c_void, count: usize, device: ::core::ffi::c_int, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, usize, ::core::ffi::c_int, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPrefetchAsync") });
        unsafe { _f(dev_ptr, count, device, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPrefetchAsync(dev_ptr: *const ::core::ffi::c_void, count: usize, device: ::core::ffi::c_int, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemPrefetchAsync(dev_ptr, count, device, stream) }
    }
}
pub unsafe fn hipMemPrefetchAsync_v2(dev_ptr: *const ::core::ffi::c_void, count: usize, location: hipMemLocation, flags: ::core::ffi::c_uint, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, usize, hipMemLocation, ::core::ffi::c_uint, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPrefetchAsync_v2") });
        unsafe { _f(dev_ptr, count, location, flags, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPrefetchAsync_v2(dev_ptr: *const ::core::ffi::c_void, count: usize, location: hipMemLocation, flags: ::core::ffi::c_uint, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemPrefetchAsync_v2(dev_ptr, count, location, flags, stream) }
    }
}
pub unsafe fn hipMemPtrGetInfo(ptr: *mut ::core::ffi::c_void, size: *mut usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemPtrGetInfo") });
        unsafe { _f(ptr, size) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemPtrGetInfo(ptr: *mut ::core::ffi::c_void, size: *mut usize) -> hipError_t;
        }
        unsafe { hipMemPtrGetInfo(ptr, size) }
    }
}
pub unsafe fn hipMemRangeGetAttribute(data: *mut ::core::ffi::c_void, data_size: usize, attribute: hipMemRangeAttribute, dev_ptr: *const ::core::ffi::c_void, count: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, hipMemRangeAttribute, *const ::core::ffi::c_void, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemRangeGetAttribute") });
        unsafe { _f(data, data_size, attribute, dev_ptr, count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemRangeGetAttribute(data: *mut ::core::ffi::c_void, data_size: usize, attribute: hipMemRangeAttribute, dev_ptr: *const ::core::ffi::c_void, count: usize) -> hipError_t;
        }
        unsafe { hipMemRangeGetAttribute(data, data_size, attribute, dev_ptr, count) }
    }
}
pub unsafe fn hipMemRangeGetAttributes(data: *mut *mut ::core::ffi::c_void, data_sizes: *mut usize, attributes: *mut hipMemRangeAttribute, num_attributes: usize, dev_ptr: *const ::core::ffi::c_void, count: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, *mut usize, *mut hipMemRangeAttribute, usize, *const ::core::ffi::c_void, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemRangeGetAttributes") });
        unsafe { _f(data, data_sizes, attributes, num_attributes, dev_ptr, count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemRangeGetAttributes(data: *mut *mut ::core::ffi::c_void, data_sizes: *mut usize, attributes: *mut hipMemRangeAttribute, num_attributes: usize, dev_ptr: *const ::core::ffi::c_void, count: usize) -> hipError_t;
        }
        unsafe { hipMemRangeGetAttributes(data, data_sizes, attributes, num_attributes, dev_ptr, count) }
    }
}
pub unsafe fn hipMemRelease(handle: hipMemGenericAllocationHandle_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipMemGenericAllocationHandle_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemRelease") });
        unsafe { _f(handle) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemRelease(handle: hipMemGenericAllocationHandle_t) -> hipError_t;
        }
        unsafe { hipMemRelease(handle) }
    }
}
pub unsafe fn hipMemRetainAllocationHandle(handle: *mut hipMemGenericAllocationHandle_t, addr: *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMemGenericAllocationHandle_t, *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemRetainAllocationHandle") });
        unsafe { _f(handle, addr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemRetainAllocationHandle(handle: *mut hipMemGenericAllocationHandle_t, addr: *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipMemRetainAllocationHandle(handle, addr) }
    }
}
pub unsafe fn hipMemSetAccess(ptr: *mut ::core::ffi::c_void, size: usize, desc: *const hipMemAccessDesc, count: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, *const hipMemAccessDesc, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemSetAccess") });
        unsafe { _f(ptr, size, desc, count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemSetAccess(ptr: *mut ::core::ffi::c_void, size: usize, desc: *const hipMemAccessDesc, count: usize) -> hipError_t;
        }
        unsafe { hipMemSetAccess(ptr, size, desc, count) }
    }
}
pub unsafe fn hipMemUnmap(ptr: *mut ::core::ffi::c_void, size: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemUnmap") });
        unsafe { _f(ptr, size) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemUnmap(ptr: *mut ::core::ffi::c_void, size: usize) -> hipError_t;
        }
        unsafe { hipMemUnmap(ptr, size) }
    }
}
pub unsafe fn hipMemcpy(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy") });
        unsafe { _f(dst, src, sizeBytes, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpy(dst, src, sizeBytes, kind) }
    }
}
pub unsafe fn hipMemcpy2D(dst: *mut ::core::ffi::c_void, dpitch: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, *const ::core::ffi::c_void, usize, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2D") });
        unsafe { _f(dst, dpitch, src, spitch, width, height, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2D(dst: *mut ::core::ffi::c_void, dpitch: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpy2D(dst, dpitch, src, spitch, width, height, kind) }
    }
}
pub unsafe fn hipMemcpy2DArrayToArray(dst: hipArray_t, wOffsetDst: usize, hOffsetDst: usize, src: hipArray_const_t, wOffsetSrc: usize, hOffsetSrc: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipArray_t, usize, usize, hipArray_const_t, usize, usize, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2DArrayToArray") });
        unsafe { _f(dst, wOffsetDst, hOffsetDst, src, wOffsetSrc, hOffsetSrc, width, height, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2DArrayToArray(dst: hipArray_t, wOffsetDst: usize, hOffsetDst: usize, src: hipArray_const_t, wOffsetSrc: usize, hOffsetSrc: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpy2DArrayToArray(dst, wOffsetDst, hOffsetDst, src, wOffsetSrc, hOffsetSrc, width, height, kind) }
    }
}
pub unsafe fn hipMemcpy2DAsync(dst: *mut ::core::ffi::c_void, dpitch: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, *const ::core::ffi::c_void, usize, usize, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2DAsync") });
        unsafe { _f(dst, dpitch, src, spitch, width, height, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2DAsync(dst: *mut ::core::ffi::c_void, dpitch: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpy2DAsync(dst, dpitch, src, spitch, width, height, kind, stream) }
    }
}
pub unsafe fn hipMemcpy2DAsync_spt(dst: *mut ::core::ffi::c_void, dpitch: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, *const ::core::ffi::c_void, usize, usize, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2DAsync_spt") });
        unsafe { _f(dst, dpitch, src, spitch, width, height, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2DAsync_spt(dst: *mut ::core::ffi::c_void, dpitch: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpy2DAsync_spt(dst, dpitch, src, spitch, width, height, kind, stream) }
    }
}
pub unsafe fn hipMemcpy2DFromArray(dst: *mut ::core::ffi::c_void, dpitch: usize, src: hipArray_const_t, wOffset: usize, hOffset: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, hipArray_const_t, usize, usize, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2DFromArray") });
        unsafe { _f(dst, dpitch, src, wOffset, hOffset, width, height, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2DFromArray(dst: *mut ::core::ffi::c_void, dpitch: usize, src: hipArray_const_t, wOffset: usize, hOffset: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpy2DFromArray(dst, dpitch, src, wOffset, hOffset, width, height, kind) }
    }
}
pub unsafe fn hipMemcpy2DFromArrayAsync(dst: *mut ::core::ffi::c_void, dpitch: usize, src: hipArray_const_t, wOffset: usize, hOffset: usize, width: usize, height: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, hipArray_const_t, usize, usize, usize, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2DFromArrayAsync") });
        unsafe { _f(dst, dpitch, src, wOffset, hOffset, width, height, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2DFromArrayAsync(dst: *mut ::core::ffi::c_void, dpitch: usize, src: hipArray_const_t, wOffset: usize, hOffset: usize, width: usize, height: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpy2DFromArrayAsync(dst, dpitch, src, wOffset, hOffset, width, height, kind, stream) }
    }
}
pub unsafe fn hipMemcpy2DFromArrayAsync_spt(dst: *mut ::core::ffi::c_void, dpitch: usize, src: hipArray_const_t, wOffsetSrc: usize, hOffsetSrc: usize, width: usize, height: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, hipArray_const_t, usize, usize, usize, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2DFromArrayAsync_spt") });
        unsafe { _f(dst, dpitch, src, wOffsetSrc, hOffsetSrc, width, height, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2DFromArrayAsync_spt(dst: *mut ::core::ffi::c_void, dpitch: usize, src: hipArray_const_t, wOffsetSrc: usize, hOffsetSrc: usize, width: usize, height: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpy2DFromArrayAsync_spt(dst, dpitch, src, wOffsetSrc, hOffsetSrc, width, height, kind, stream) }
    }
}
pub unsafe fn hipMemcpy2DFromArray_spt(dst: *mut ::core::ffi::c_void, dpitch: usize, src: hipArray_const_t, wOffset: usize, hOffset: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, hipArray_const_t, usize, usize, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2DFromArray_spt") });
        unsafe { _f(dst, dpitch, src, wOffset, hOffset, width, height, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2DFromArray_spt(dst: *mut ::core::ffi::c_void, dpitch: usize, src: hipArray_const_t, wOffset: usize, hOffset: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpy2DFromArray_spt(dst, dpitch, src, wOffset, hOffset, width, height, kind) }
    }
}
pub unsafe fn hipMemcpy2DToArray(dst: hipArray_t, wOffset: usize, hOffset: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipArray_t, usize, usize, *const ::core::ffi::c_void, usize, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2DToArray") });
        unsafe { _f(dst, wOffset, hOffset, src, spitch, width, height, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2DToArray(dst: hipArray_t, wOffset: usize, hOffset: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpy2DToArray(dst, wOffset, hOffset, src, spitch, width, height, kind) }
    }
}
pub unsafe fn hipMemcpy2DToArrayAsync(dst: hipArray_t, wOffset: usize, hOffset: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipArray_t, usize, usize, *const ::core::ffi::c_void, usize, usize, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2DToArrayAsync") });
        unsafe { _f(dst, wOffset, hOffset, src, spitch, width, height, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2DToArrayAsync(dst: hipArray_t, wOffset: usize, hOffset: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpy2DToArrayAsync(dst, wOffset, hOffset, src, spitch, width, height, kind, stream) }
    }
}
pub unsafe fn hipMemcpy2DToArrayAsync_spt(dst: hipArray_t, wOffset: usize, hOffset: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipArray_t, usize, usize, *const ::core::ffi::c_void, usize, usize, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2DToArrayAsync_spt") });
        unsafe { _f(dst, wOffset, hOffset, src, spitch, width, height, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2DToArrayAsync_spt(dst: hipArray_t, wOffset: usize, hOffset: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpy2DToArrayAsync_spt(dst, wOffset, hOffset, src, spitch, width, height, kind, stream) }
    }
}
pub unsafe fn hipMemcpy2DToArray_spt(dst: hipArray_t, wOffset: usize, hOffset: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipArray_t, usize, usize, *const ::core::ffi::c_void, usize, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2DToArray_spt") });
        unsafe { _f(dst, wOffset, hOffset, src, spitch, width, height, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2DToArray_spt(dst: hipArray_t, wOffset: usize, hOffset: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpy2DToArray_spt(dst, wOffset, hOffset, src, spitch, width, height, kind) }
    }
}
pub unsafe fn hipMemcpy2D_spt(dst: *mut ::core::ffi::c_void, dpitch: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, *const ::core::ffi::c_void, usize, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy2D_spt") });
        unsafe { _f(dst, dpitch, src, spitch, width, height, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy2D_spt(dst: *mut ::core::ffi::c_void, dpitch: usize, src: *const ::core::ffi::c_void, spitch: usize, width: usize, height: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpy2D_spt(dst, dpitch, src, spitch, width, height, kind) }
    }
}
pub unsafe fn hipMemcpy3D(p: *const hipMemcpy3DParms) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const hipMemcpy3DParms) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy3D") });
        unsafe { _f(p) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy3D(p: *const hipMemcpy3DParms) -> hipError_t;
        }
        unsafe { hipMemcpy3D(p) }
    }
}
pub unsafe fn hipMemcpy3DAsync(p: *const hipMemcpy3DParms, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const hipMemcpy3DParms, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy3DAsync") });
        unsafe { _f(p, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy3DAsync(p: *const hipMemcpy3DParms, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpy3DAsync(p, stream) }
    }
}
pub unsafe fn hipMemcpy3DAsync_spt(p: *const hipMemcpy3DParms, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const hipMemcpy3DParms, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy3DAsync_spt") });
        unsafe { _f(p, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy3DAsync_spt(p: *const hipMemcpy3DParms, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpy3DAsync_spt(p, stream) }
    }
}
pub unsafe fn hipMemcpy3DBatchAsync(numOps: usize, opList: *mut hipMemcpy3DBatchOp, failIdx: *mut usize, flags: ::core::ffi::c_ulonglong, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(usize, *mut hipMemcpy3DBatchOp, *mut usize, ::core::ffi::c_ulonglong, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy3DBatchAsync") });
        unsafe { _f(numOps, opList, failIdx, flags, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy3DBatchAsync(numOps: usize, opList: *mut hipMemcpy3DBatchOp, failIdx: *mut usize, flags: ::core::ffi::c_ulonglong, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpy3DBatchAsync(numOps, opList, failIdx, flags, stream) }
    }
}
pub unsafe fn hipMemcpy3DPeer(p: *mut hipMemcpy3DPeerParms) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMemcpy3DPeerParms) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy3DPeer") });
        unsafe { _f(p) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy3DPeer(p: *mut hipMemcpy3DPeerParms) -> hipError_t;
        }
        unsafe { hipMemcpy3DPeer(p) }
    }
}
pub unsafe fn hipMemcpy3DPeerAsync(p: *mut hipMemcpy3DPeerParms, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMemcpy3DPeerParms, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy3DPeerAsync") });
        unsafe { _f(p, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy3DPeerAsync(p: *mut hipMemcpy3DPeerParms, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpy3DPeerAsync(p, stream) }
    }
}
pub unsafe fn hipMemcpy3D_spt(p: *const hipMemcpy3DParms) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const hipMemcpy3DParms) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy3D_spt") });
        unsafe { _f(p) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy3D_spt(p: *const hipMemcpy3DParms) -> hipError_t;
        }
        unsafe { hipMemcpy3D_spt(p) }
    }
}
pub unsafe fn hipMemcpyAsync(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyAsync") });
        unsafe { _f(dst, src, sizeBytes, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyAsync(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyAsync(dst, src, sizeBytes, kind, stream) }
    }
}
pub unsafe fn hipMemcpyAsync_spt(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyAsync_spt") });
        unsafe { _f(dst, src, sizeBytes, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyAsync_spt(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyAsync_spt(dst, src, sizeBytes, kind, stream) }
    }
}
pub unsafe fn hipMemcpyAtoA(dstArray: hipArray_t, dstOffset: usize, srcArray: hipArray_t, srcOffset: usize, ByteCount: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipArray_t, usize, hipArray_t, usize, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyAtoA") });
        unsafe { _f(dstArray, dstOffset, srcArray, srcOffset, ByteCount) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyAtoA(dstArray: hipArray_t, dstOffset: usize, srcArray: hipArray_t, srcOffset: usize, ByteCount: usize) -> hipError_t;
        }
        unsafe { hipMemcpyAtoA(dstArray, dstOffset, srcArray, srcOffset, ByteCount) }
    }
}
pub unsafe fn hipMemcpyAtoD(dstDevice: hipDeviceptr_t, srcArray: hipArray_t, srcOffset: usize, ByteCount: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, hipArray_t, usize, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyAtoD") });
        unsafe { _f(dstDevice, srcArray, srcOffset, ByteCount) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyAtoD(dstDevice: hipDeviceptr_t, srcArray: hipArray_t, srcOffset: usize, ByteCount: usize) -> hipError_t;
        }
        unsafe { hipMemcpyAtoD(dstDevice, srcArray, srcOffset, ByteCount) }
    }
}
pub unsafe fn hipMemcpyAtoH(dst: *mut ::core::ffi::c_void, srcArray: hipArray_t, srcOffset: usize, count: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, hipArray_t, usize, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyAtoH") });
        unsafe { _f(dst, srcArray, srcOffset, count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyAtoH(dst: *mut ::core::ffi::c_void, srcArray: hipArray_t, srcOffset: usize, count: usize) -> hipError_t;
        }
        unsafe { hipMemcpyAtoH(dst, srcArray, srcOffset, count) }
    }
}
pub unsafe fn hipMemcpyAtoHAsync(dstHost: *mut ::core::ffi::c_void, srcArray: hipArray_t, srcOffset: usize, ByteCount: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, hipArray_t, usize, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyAtoHAsync") });
        unsafe { _f(dstHost, srcArray, srcOffset, ByteCount, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyAtoHAsync(dstHost: *mut ::core::ffi::c_void, srcArray: hipArray_t, srcOffset: usize, ByteCount: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyAtoHAsync(dstHost, srcArray, srcOffset, ByteCount, stream) }
    }
}
pub unsafe fn hipMemcpyBatchAsync(dsts: *mut *mut ::core::ffi::c_void, srcs: *mut *mut ::core::ffi::c_void, sizes: *mut usize, count: usize, attrs: *mut hipMemcpyAttributes, attrsIdxs: *mut usize, numAttrs: usize, failIdx: *mut usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void, *mut usize, usize, *mut hipMemcpyAttributes, *mut usize, usize, *mut usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyBatchAsync") });
        unsafe { _f(dsts, srcs, sizes, count, attrs, attrsIdxs, numAttrs, failIdx, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyBatchAsync(dsts: *mut *mut ::core::ffi::c_void, srcs: *mut *mut ::core::ffi::c_void, sizes: *mut usize, count: usize, attrs: *mut hipMemcpyAttributes, attrsIdxs: *mut usize, numAttrs: usize, failIdx: *mut usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyBatchAsync(dsts, srcs, sizes, count, attrs, attrsIdxs, numAttrs, failIdx, stream) }
    }
}
pub unsafe fn hipMemcpyDtoA(dstArray: hipArray_t, dstOffset: usize, srcDevice: hipDeviceptr_t, ByteCount: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipArray_t, usize, hipDeviceptr_t, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyDtoA") });
        unsafe { _f(dstArray, dstOffset, srcDevice, ByteCount) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyDtoA(dstArray: hipArray_t, dstOffset: usize, srcDevice: hipDeviceptr_t, ByteCount: usize) -> hipError_t;
        }
        unsafe { hipMemcpyDtoA(dstArray, dstOffset, srcDevice, ByteCount) }
    }
}
pub unsafe fn hipMemcpyDtoD(dst: hipDeviceptr_t, src: hipDeviceptr_t, sizeBytes: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, hipDeviceptr_t, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyDtoD") });
        unsafe { _f(dst, src, sizeBytes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyDtoD(dst: hipDeviceptr_t, src: hipDeviceptr_t, sizeBytes: usize) -> hipError_t;
        }
        unsafe { hipMemcpyDtoD(dst, src, sizeBytes) }
    }
}
pub unsafe fn hipMemcpyDtoDAsync(dst: hipDeviceptr_t, src: hipDeviceptr_t, sizeBytes: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, hipDeviceptr_t, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyDtoDAsync") });
        unsafe { _f(dst, src, sizeBytes, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyDtoDAsync(dst: hipDeviceptr_t, src: hipDeviceptr_t, sizeBytes: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyDtoDAsync(dst, src, sizeBytes, stream) }
    }
}
pub unsafe fn hipMemcpyDtoH(dst: *mut ::core::ffi::c_void, src: hipDeviceptr_t, sizeBytes: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, hipDeviceptr_t, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyDtoH") });
        unsafe { _f(dst, src, sizeBytes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyDtoH(dst: *mut ::core::ffi::c_void, src: hipDeviceptr_t, sizeBytes: usize) -> hipError_t;
        }
        unsafe { hipMemcpyDtoH(dst, src, sizeBytes) }
    }
}
pub unsafe fn hipMemcpyDtoHAsync(dst: *mut ::core::ffi::c_void, src: hipDeviceptr_t, sizeBytes: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, hipDeviceptr_t, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyDtoHAsync") });
        unsafe { _f(dst, src, sizeBytes, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyDtoHAsync(dst: *mut ::core::ffi::c_void, src: hipDeviceptr_t, sizeBytes: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyDtoHAsync(dst, src, sizeBytes, stream) }
    }
}
pub unsafe fn hipMemcpyFromArray(dst: *mut ::core::ffi::c_void, srcArray: hipArray_const_t, wOffset: usize, hOffset: usize, count: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, hipArray_const_t, usize, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyFromArray") });
        unsafe { _f(dst, srcArray, wOffset, hOffset, count, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyFromArray(dst: *mut ::core::ffi::c_void, srcArray: hipArray_const_t, wOffset: usize, hOffset: usize, count: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpyFromArray(dst, srcArray, wOffset, hOffset, count, kind) }
    }
}
pub unsafe fn hipMemcpyFromArray_spt(dst: *mut ::core::ffi::c_void, src: hipArray_const_t, wOffsetSrc: usize, hOffset: usize, count: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, hipArray_const_t, usize, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyFromArray_spt") });
        unsafe { _f(dst, src, wOffsetSrc, hOffset, count, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyFromArray_spt(dst: *mut ::core::ffi::c_void, src: hipArray_const_t, wOffsetSrc: usize, hOffset: usize, count: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpyFromArray_spt(dst, src, wOffsetSrc, hOffset, count, kind) }
    }
}
pub unsafe fn hipMemcpyFromSymbol(dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyFromSymbol") });
        unsafe { _f(dst, symbol, sizeBytes, offset, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyFromSymbol(dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpyFromSymbol(dst, symbol, sizeBytes, offset, kind) }
    }
}
pub unsafe fn hipMemcpyFromSymbolAsync(dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyFromSymbolAsync") });
        unsafe { _f(dst, symbol, sizeBytes, offset, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyFromSymbolAsync(dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyFromSymbolAsync(dst, symbol, sizeBytes, offset, kind, stream) }
    }
}
pub unsafe fn hipMemcpyFromSymbolAsync_spt(dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyFromSymbolAsync_spt") });
        unsafe { _f(dst, symbol, sizeBytes, offset, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyFromSymbolAsync_spt(dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyFromSymbolAsync_spt(dst, symbol, sizeBytes, offset, kind, stream) }
    }
}
pub unsafe fn hipMemcpyFromSymbol_spt(dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyFromSymbol_spt") });
        unsafe { _f(dst, symbol, sizeBytes, offset, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyFromSymbol_spt(dst: *mut ::core::ffi::c_void, symbol: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpyFromSymbol_spt(dst, symbol, sizeBytes, offset, kind) }
    }
}
pub unsafe fn hipMemcpyHtoA(dstArray: hipArray_t, dstOffset: usize, srcHost: *const ::core::ffi::c_void, count: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipArray_t, usize, *const ::core::ffi::c_void, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyHtoA") });
        unsafe { _f(dstArray, dstOffset, srcHost, count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyHtoA(dstArray: hipArray_t, dstOffset: usize, srcHost: *const ::core::ffi::c_void, count: usize) -> hipError_t;
        }
        unsafe { hipMemcpyHtoA(dstArray, dstOffset, srcHost, count) }
    }
}
pub unsafe fn hipMemcpyHtoAAsync(dstArray: hipArray_t, dstOffset: usize, srcHost: *const ::core::ffi::c_void, ByteCount: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipArray_t, usize, *const ::core::ffi::c_void, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyHtoAAsync") });
        unsafe { _f(dstArray, dstOffset, srcHost, ByteCount, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyHtoAAsync(dstArray: hipArray_t, dstOffset: usize, srcHost: *const ::core::ffi::c_void, ByteCount: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyHtoAAsync(dstArray, dstOffset, srcHost, ByteCount, stream) }
    }
}
pub unsafe fn hipMemcpyHtoD(dst: hipDeviceptr_t, src: *const ::core::ffi::c_void, sizeBytes: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, *const ::core::ffi::c_void, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyHtoD") });
        unsafe { _f(dst, src, sizeBytes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyHtoD(dst: hipDeviceptr_t, src: *const ::core::ffi::c_void, sizeBytes: usize) -> hipError_t;
        }
        unsafe { hipMemcpyHtoD(dst, src, sizeBytes) }
    }
}
pub unsafe fn hipMemcpyHtoDAsync(dst: hipDeviceptr_t, src: *const ::core::ffi::c_void, sizeBytes: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, *const ::core::ffi::c_void, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyHtoDAsync") });
        unsafe { _f(dst, src, sizeBytes, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyHtoDAsync(dst: hipDeviceptr_t, src: *const ::core::ffi::c_void, sizeBytes: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyHtoDAsync(dst, src, sizeBytes, stream) }
    }
}
pub unsafe fn hipMemcpyParam2D(pCopy: *const hip_Memcpy2D) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const hip_Memcpy2D) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyParam2D") });
        unsafe { _f(pCopy) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyParam2D(pCopy: *const hip_Memcpy2D) -> hipError_t;
        }
        unsafe { hipMemcpyParam2D(pCopy) }
    }
}
pub unsafe fn hipMemcpyParam2DAsync(pCopy: *const hip_Memcpy2D, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const hip_Memcpy2D, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyParam2DAsync") });
        unsafe { _f(pCopy, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyParam2DAsync(pCopy: *const hip_Memcpy2D, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyParam2DAsync(pCopy, stream) }
    }
}
pub unsafe fn hipMemcpyPeer(dst: *mut ::core::ffi::c_void, dstDeviceId: ::core::ffi::c_int, src: *const ::core::ffi::c_void, srcDeviceId: ::core::ffi::c_int, sizeBytes: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int, *const ::core::ffi::c_void, ::core::ffi::c_int, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyPeer") });
        unsafe { _f(dst, dstDeviceId, src, srcDeviceId, sizeBytes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyPeer(dst: *mut ::core::ffi::c_void, dstDeviceId: ::core::ffi::c_int, src: *const ::core::ffi::c_void, srcDeviceId: ::core::ffi::c_int, sizeBytes: usize) -> hipError_t;
        }
        unsafe { hipMemcpyPeer(dst, dstDeviceId, src, srcDeviceId, sizeBytes) }
    }
}
pub unsafe fn hipMemcpyPeerAsync(dst: *mut ::core::ffi::c_void, dstDeviceId: ::core::ffi::c_int, src: *const ::core::ffi::c_void, srcDevice: ::core::ffi::c_int, sizeBytes: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int, *const ::core::ffi::c_void, ::core::ffi::c_int, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyPeerAsync") });
        unsafe { _f(dst, dstDeviceId, src, srcDevice, sizeBytes, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyPeerAsync(dst: *mut ::core::ffi::c_void, dstDeviceId: ::core::ffi::c_int, src: *const ::core::ffi::c_void, srcDevice: ::core::ffi::c_int, sizeBytes: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyPeerAsync(dst, dstDeviceId, src, srcDevice, sizeBytes, stream) }
    }
}
pub unsafe fn hipMemcpyToArray(dst: hipArray_t, wOffset: usize, hOffset: usize, src: *const ::core::ffi::c_void, count: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipArray_t, usize, usize, *const ::core::ffi::c_void, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyToArray") });
        unsafe { _f(dst, wOffset, hOffset, src, count, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyToArray(dst: hipArray_t, wOffset: usize, hOffset: usize, src: *const ::core::ffi::c_void, count: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpyToArray(dst, wOffset, hOffset, src, count, kind) }
    }
}
pub unsafe fn hipMemcpyToSymbol(symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyToSymbol") });
        unsafe { _f(symbol, src, sizeBytes, offset, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyToSymbol(symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpyToSymbol(symbol, src, sizeBytes, offset, kind) }
    }
}
pub unsafe fn hipMemcpyToSymbolAsync(symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyToSymbolAsync") });
        unsafe { _f(symbol, src, sizeBytes, offset, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyToSymbolAsync(symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyToSymbolAsync(symbol, src, sizeBytes, offset, kind, stream) }
    }
}
pub unsafe fn hipMemcpyToSymbolAsync_spt(symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyToSymbolAsync_spt") });
        unsafe { _f(symbol, src, sizeBytes, offset, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyToSymbolAsync_spt(symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyToSymbolAsync_spt(symbol, src, sizeBytes, offset, kind, stream) }
    }
}
pub unsafe fn hipMemcpyToSymbol_spt(symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void, usize, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyToSymbol_spt") });
        unsafe { _f(symbol, src, sizeBytes, offset, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyToSymbol_spt(symbol: *const ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, offset: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpyToSymbol_spt(symbol, src, sizeBytes, offset, kind) }
    }
}
pub unsafe fn hipMemcpyWithStream(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, hipMemcpyKind, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpyWithStream") });
        unsafe { _f(dst, src, sizeBytes, kind, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpyWithStream(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, kind: hipMemcpyKind, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemcpyWithStream(dst, src, sizeBytes, kind, stream) }
    }
}
pub unsafe fn hipMemcpy_spt(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, kind: hipMemcpyKind) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void, usize, hipMemcpyKind) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemcpy_spt") });
        unsafe { _f(dst, src, sizeBytes, kind) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemcpy_spt(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, sizeBytes: usize, kind: hipMemcpyKind) -> hipError_t;
        }
        unsafe { hipMemcpy_spt(dst, src, sizeBytes, kind) }
    }
}
pub unsafe fn hipMemset(dst: *mut ::core::ffi::c_void, value: ::core::ffi::c_int, sizeBytes: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemset") });
        unsafe { _f(dst, value, sizeBytes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemset(dst: *mut ::core::ffi::c_void, value: ::core::ffi::c_int, sizeBytes: usize) -> hipError_t;
        }
        unsafe { hipMemset(dst, value, sizeBytes) }
    }
}
pub unsafe fn hipMemset2D(dst: *mut ::core::ffi::c_void, pitch: usize, value: ::core::ffi::c_int, width: usize, height: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, ::core::ffi::c_int, usize, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemset2D") });
        unsafe { _f(dst, pitch, value, width, height) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemset2D(dst: *mut ::core::ffi::c_void, pitch: usize, value: ::core::ffi::c_int, width: usize, height: usize) -> hipError_t;
        }
        unsafe { hipMemset2D(dst, pitch, value, width, height) }
    }
}
pub unsafe fn hipMemset2DAsync(dst: *mut ::core::ffi::c_void, pitch: usize, value: ::core::ffi::c_int, width: usize, height: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, ::core::ffi::c_int, usize, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemset2DAsync") });
        unsafe { _f(dst, pitch, value, width, height, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemset2DAsync(dst: *mut ::core::ffi::c_void, pitch: usize, value: ::core::ffi::c_int, width: usize, height: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemset2DAsync(dst, pitch, value, width, height, stream) }
    }
}
pub unsafe fn hipMemset2DAsync_spt(dst: *mut ::core::ffi::c_void, pitch: usize, value: ::core::ffi::c_int, width: usize, height: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, ::core::ffi::c_int, usize, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemset2DAsync_spt") });
        unsafe { _f(dst, pitch, value, width, height, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemset2DAsync_spt(dst: *mut ::core::ffi::c_void, pitch: usize, value: ::core::ffi::c_int, width: usize, height: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemset2DAsync_spt(dst, pitch, value, width, height, stream) }
    }
}
pub unsafe fn hipMemset2D_spt(dst: *mut ::core::ffi::c_void, pitch: usize, value: ::core::ffi::c_int, width: usize, height: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, ::core::ffi::c_int, usize, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemset2D_spt") });
        unsafe { _f(dst, pitch, value, width, height) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemset2D_spt(dst: *mut ::core::ffi::c_void, pitch: usize, value: ::core::ffi::c_int, width: usize, height: usize) -> hipError_t;
        }
        unsafe { hipMemset2D_spt(dst, pitch, value, width, height) }
    }
}
pub unsafe fn hipMemset3D(pitchedDevPtr: hipPitchedPtr, value: ::core::ffi::c_int, extent: hipExtent) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipPitchedPtr, ::core::ffi::c_int, hipExtent) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemset3D") });
        unsafe { _f(pitchedDevPtr, value, extent) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemset3D(pitchedDevPtr: hipPitchedPtr, value: ::core::ffi::c_int, extent: hipExtent) -> hipError_t;
        }
        unsafe { hipMemset3D(pitchedDevPtr, value, extent) }
    }
}
pub unsafe fn hipMemset3DAsync(pitchedDevPtr: hipPitchedPtr, value: ::core::ffi::c_int, extent: hipExtent, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipPitchedPtr, ::core::ffi::c_int, hipExtent, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemset3DAsync") });
        unsafe { _f(pitchedDevPtr, value, extent, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemset3DAsync(pitchedDevPtr: hipPitchedPtr, value: ::core::ffi::c_int, extent: hipExtent, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemset3DAsync(pitchedDevPtr, value, extent, stream) }
    }
}
pub unsafe fn hipMemset3DAsync_spt(pitchedDevPtr: hipPitchedPtr, value: ::core::ffi::c_int, extent: hipExtent, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipPitchedPtr, ::core::ffi::c_int, hipExtent, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemset3DAsync_spt") });
        unsafe { _f(pitchedDevPtr, value, extent, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemset3DAsync_spt(pitchedDevPtr: hipPitchedPtr, value: ::core::ffi::c_int, extent: hipExtent, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemset3DAsync_spt(pitchedDevPtr, value, extent, stream) }
    }
}
pub unsafe fn hipMemset3D_spt(pitchedDevPtr: hipPitchedPtr, value: ::core::ffi::c_int, extent: hipExtent) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipPitchedPtr, ::core::ffi::c_int, hipExtent) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemset3D_spt") });
        unsafe { _f(pitchedDevPtr, value, extent) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemset3D_spt(pitchedDevPtr: hipPitchedPtr, value: ::core::ffi::c_int, extent: hipExtent) -> hipError_t;
        }
        unsafe { hipMemset3D_spt(pitchedDevPtr, value, extent) }
    }
}
pub unsafe fn hipMemsetAsync(dst: *mut ::core::ffi::c_void, value: ::core::ffi::c_int, sizeBytes: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetAsync") });
        unsafe { _f(dst, value, sizeBytes, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetAsync(dst: *mut ::core::ffi::c_void, value: ::core::ffi::c_int, sizeBytes: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemsetAsync(dst, value, sizeBytes, stream) }
    }
}
pub unsafe fn hipMemsetAsync_spt(dst: *mut ::core::ffi::c_void, value: ::core::ffi::c_int, sizeBytes: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetAsync_spt") });
        unsafe { _f(dst, value, sizeBytes, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetAsync_spt(dst: *mut ::core::ffi::c_void, value: ::core::ffi::c_int, sizeBytes: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemsetAsync_spt(dst, value, sizeBytes, stream) }
    }
}
pub unsafe fn hipMemsetD16(dest: hipDeviceptr_t, value: ::core::ffi::c_ushort, count: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, ::core::ffi::c_ushort, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetD16") });
        unsafe { _f(dest, value, count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetD16(dest: hipDeviceptr_t, value: ::core::ffi::c_ushort, count: usize) -> hipError_t;
        }
        unsafe { hipMemsetD16(dest, value, count) }
    }
}
pub unsafe fn hipMemsetD16Async(dest: hipDeviceptr_t, value: ::core::ffi::c_ushort, count: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, ::core::ffi::c_ushort, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetD16Async") });
        unsafe { _f(dest, value, count, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetD16Async(dest: hipDeviceptr_t, value: ::core::ffi::c_ushort, count: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemsetD16Async(dest, value, count, stream) }
    }
}
pub unsafe fn hipMemsetD2D16(dst: hipDeviceptr_t, dstPitch: usize, value: ::core::ffi::c_ushort, width: usize, height: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, usize, ::core::ffi::c_ushort, usize, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetD2D16") });
        unsafe { _f(dst, dstPitch, value, width, height) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetD2D16(dst: hipDeviceptr_t, dstPitch: usize, value: ::core::ffi::c_ushort, width: usize, height: usize) -> hipError_t;
        }
        unsafe { hipMemsetD2D16(dst, dstPitch, value, width, height) }
    }
}
pub unsafe fn hipMemsetD2D16Async(dst: hipDeviceptr_t, dstPitch: usize, value: ::core::ffi::c_ushort, width: usize, height: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, usize, ::core::ffi::c_ushort, usize, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetD2D16Async") });
        unsafe { _f(dst, dstPitch, value, width, height, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetD2D16Async(dst: hipDeviceptr_t, dstPitch: usize, value: ::core::ffi::c_ushort, width: usize, height: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemsetD2D16Async(dst, dstPitch, value, width, height, stream) }
    }
}
pub unsafe fn hipMemsetD2D32(dst: hipDeviceptr_t, dstPitch: usize, value: ::core::ffi::c_uint, width: usize, height: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, usize, ::core::ffi::c_uint, usize, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetD2D32") });
        unsafe { _f(dst, dstPitch, value, width, height) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetD2D32(dst: hipDeviceptr_t, dstPitch: usize, value: ::core::ffi::c_uint, width: usize, height: usize) -> hipError_t;
        }
        unsafe { hipMemsetD2D32(dst, dstPitch, value, width, height) }
    }
}
pub unsafe fn hipMemsetD2D32Async(dst: hipDeviceptr_t, dstPitch: usize, value: ::core::ffi::c_uint, width: usize, height: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, usize, ::core::ffi::c_uint, usize, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetD2D32Async") });
        unsafe { _f(dst, dstPitch, value, width, height, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetD2D32Async(dst: hipDeviceptr_t, dstPitch: usize, value: ::core::ffi::c_uint, width: usize, height: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemsetD2D32Async(dst, dstPitch, value, width, height, stream) }
    }
}
pub unsafe fn hipMemsetD2D8(dst: hipDeviceptr_t, dstPitch: usize, value: ::core::ffi::c_uchar, width: usize, height: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, usize, ::core::ffi::c_uchar, usize, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetD2D8") });
        unsafe { _f(dst, dstPitch, value, width, height) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetD2D8(dst: hipDeviceptr_t, dstPitch: usize, value: ::core::ffi::c_uchar, width: usize, height: usize) -> hipError_t;
        }
        unsafe { hipMemsetD2D8(dst, dstPitch, value, width, height) }
    }
}
pub unsafe fn hipMemsetD2D8Async(dst: hipDeviceptr_t, dstPitch: usize, value: ::core::ffi::c_uchar, width: usize, height: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, usize, ::core::ffi::c_uchar, usize, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetD2D8Async") });
        unsafe { _f(dst, dstPitch, value, width, height, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetD2D8Async(dst: hipDeviceptr_t, dstPitch: usize, value: ::core::ffi::c_uchar, width: usize, height: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemsetD2D8Async(dst, dstPitch, value, width, height, stream) }
    }
}
pub unsafe fn hipMemsetD32(dest: hipDeviceptr_t, value: ::core::ffi::c_int, count: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, ::core::ffi::c_int, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetD32") });
        unsafe { _f(dest, value, count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetD32(dest: hipDeviceptr_t, value: ::core::ffi::c_int, count: usize) -> hipError_t;
        }
        unsafe { hipMemsetD32(dest, value, count) }
    }
}
pub unsafe fn hipMemsetD32Async(dst: hipDeviceptr_t, value: ::core::ffi::c_int, count: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, ::core::ffi::c_int, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetD32Async") });
        unsafe { _f(dst, value, count, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetD32Async(dst: hipDeviceptr_t, value: ::core::ffi::c_int, count: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemsetD32Async(dst, value, count, stream) }
    }
}
pub unsafe fn hipMemsetD8(dest: hipDeviceptr_t, value: ::core::ffi::c_uchar, count: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, ::core::ffi::c_uchar, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetD8") });
        unsafe { _f(dest, value, count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetD8(dest: hipDeviceptr_t, value: ::core::ffi::c_uchar, count: usize) -> hipError_t;
        }
        unsafe { hipMemsetD8(dest, value, count) }
    }
}
pub unsafe fn hipMemsetD8Async(dest: hipDeviceptr_t, value: ::core::ffi::c_uchar, count: usize, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipDeviceptr_t, ::core::ffi::c_uchar, usize, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemsetD8Async") });
        unsafe { _f(dest, value, count, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemsetD8Async(dest: hipDeviceptr_t, value: ::core::ffi::c_uchar, count: usize, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipMemsetD8Async(dest, value, count, stream) }
    }
}
pub unsafe fn hipMemset_spt(dst: *mut ::core::ffi::c_void, value: ::core::ffi::c_int, sizeBytes: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMemset_spt") });
        unsafe { _f(dst, value, sizeBytes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMemset_spt(dst: *mut ::core::ffi::c_void, value: ::core::ffi::c_int, sizeBytes: usize) -> hipError_t;
        }
        unsafe { hipMemset_spt(dst, value, sizeBytes) }
    }
}
pub unsafe fn hipMipmappedArrayCreate(pHandle: *mut hipMipmappedArray_t, pMipmappedArrayDesc: *mut HIP_ARRAY3D_DESCRIPTOR, numMipmapLevels: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMipmappedArray_t, *mut HIP_ARRAY3D_DESCRIPTOR, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMipmappedArrayCreate") });
        unsafe { _f(pHandle, pMipmappedArrayDesc, numMipmapLevels) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMipmappedArrayCreate(pHandle: *mut hipMipmappedArray_t, pMipmappedArrayDesc: *mut HIP_ARRAY3D_DESCRIPTOR, numMipmapLevels: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipMipmappedArrayCreate(pHandle, pMipmappedArrayDesc, numMipmapLevels) }
    }
}
pub unsafe fn hipMipmappedArrayDestroy(hMipmappedArray: hipMipmappedArray_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipMipmappedArray_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMipmappedArrayDestroy") });
        unsafe { _f(hMipmappedArray) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMipmappedArrayDestroy(hMipmappedArray: hipMipmappedArray_t) -> hipError_t;
        }
        unsafe { hipMipmappedArrayDestroy(hMipmappedArray) }
    }
}
pub unsafe fn hipMipmappedArrayGetLevel(pLevelArray: *mut hipArray_t, hMipMappedArray: hipMipmappedArray_t, level: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipArray_t, hipMipmappedArray_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipMipmappedArrayGetLevel") });
        unsafe { _f(pLevelArray, hMipMappedArray, level) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipMipmappedArrayGetLevel(pLevelArray: *mut hipArray_t, hMipMappedArray: hipMipmappedArray_t, level: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipMipmappedArrayGetLevel(pLevelArray, hMipMappedArray, level) }
    }
}
pub unsafe fn hipModuleGetFunction(function: *mut hipFunction_t, module: hipModule_t, kname: *const ::core::ffi::c_char) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipFunction_t, hipModule_t, *const ::core::ffi::c_char) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleGetFunction") });
        unsafe { _f(function, module, kname) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleGetFunction(function: *mut hipFunction_t, module: hipModule_t, kname: *const ::core::ffi::c_char) -> hipError_t;
        }
        unsafe { hipModuleGetFunction(function, module, kname) }
    }
}
pub unsafe fn hipModuleGetFunctionCount(count: *mut ::core::ffi::c_uint, mod_: hipModule_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_uint, hipModule_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleGetFunctionCount") });
        unsafe { _f(count, mod_) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleGetFunctionCount(count: *mut ::core::ffi::c_uint, mod_: hipModule_t) -> hipError_t;
        }
        unsafe { hipModuleGetFunctionCount(count, mod_) }
    }
}
pub unsafe fn hipModuleGetGlobal(dptr: *mut hipDeviceptr_t, bytes: *mut usize, hmod: hipModule_t, name: *const ::core::ffi::c_char) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipDeviceptr_t, *mut usize, hipModule_t, *const ::core::ffi::c_char) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleGetGlobal") });
        unsafe { _f(dptr, bytes, hmod, name) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleGetGlobal(dptr: *mut hipDeviceptr_t, bytes: *mut usize, hmod: hipModule_t, name: *const ::core::ffi::c_char) -> hipError_t;
        }
        unsafe { hipModuleGetGlobal(dptr, bytes, hmod, name) }
    }
}
pub unsafe fn hipModuleGetTexRef(texRef: *mut *mut textureReference, hmod: hipModule_t, name: *const ::core::ffi::c_char) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut *mut textureReference, hipModule_t, *const ::core::ffi::c_char) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleGetTexRef") });
        unsafe { _f(texRef, hmod, name) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleGetTexRef(texRef: *mut *mut textureReference, hmod: hipModule_t, name: *const ::core::ffi::c_char) -> hipError_t;
        }
        unsafe { hipModuleGetTexRef(texRef, hmod, name) }
    }
}
pub unsafe fn hipModuleLaunchCooperativeKernel(f: hipFunction_t, gridDimX: ::core::ffi::c_uint, gridDimY: ::core::ffi::c_uint, gridDimZ: ::core::ffi::c_uint, blockDimX: ::core::ffi::c_uint, blockDimY: ::core::ffi::c_uint, blockDimZ: ::core::ffi::c_uint, sharedMemBytes: ::core::ffi::c_uint, stream: hipStream_t, kernelParams: *mut *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipFunction_t, ::core::ffi::c_uint, ::core::ffi::c_uint, ::core::ffi::c_uint, ::core::ffi::c_uint, ::core::ffi::c_uint, ::core::ffi::c_uint, ::core::ffi::c_uint, hipStream_t, *mut *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleLaunchCooperativeKernel") });
        unsafe { _f(f, gridDimX, gridDimY, gridDimZ, blockDimX, blockDimY, blockDimZ, sharedMemBytes, stream, kernelParams) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleLaunchCooperativeKernel(f: hipFunction_t, gridDimX: ::core::ffi::c_uint, gridDimY: ::core::ffi::c_uint, gridDimZ: ::core::ffi::c_uint, blockDimX: ::core::ffi::c_uint, blockDimY: ::core::ffi::c_uint, blockDimZ: ::core::ffi::c_uint, sharedMemBytes: ::core::ffi::c_uint, stream: hipStream_t, kernelParams: *mut *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipModuleLaunchCooperativeKernel(f, gridDimX, gridDimY, gridDimZ, blockDimX, blockDimY, blockDimZ, sharedMemBytes, stream, kernelParams) }
    }
}
pub unsafe fn hipModuleLaunchCooperativeKernelMultiDevice(launchParamsList: *mut hipFunctionLaunchParams, numDevices: ::core::ffi::c_uint, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipFunctionLaunchParams, ::core::ffi::c_uint, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleLaunchCooperativeKernelMultiDevice") });
        unsafe { _f(launchParamsList, numDevices, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleLaunchCooperativeKernelMultiDevice(launchParamsList: *mut hipFunctionLaunchParams, numDevices: ::core::ffi::c_uint, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipModuleLaunchCooperativeKernelMultiDevice(launchParamsList, numDevices, flags) }
    }
}
pub unsafe fn hipModuleLaunchKernel(f: hipFunction_t, gridDimX: ::core::ffi::c_uint, gridDimY: ::core::ffi::c_uint, gridDimZ: ::core::ffi::c_uint, blockDimX: ::core::ffi::c_uint, blockDimY: ::core::ffi::c_uint, blockDimZ: ::core::ffi::c_uint, sharedMemBytes: ::core::ffi::c_uint, stream: hipStream_t, kernelParams: *mut *mut ::core::ffi::c_void, extra: *mut *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipFunction_t, ::core::ffi::c_uint, ::core::ffi::c_uint, ::core::ffi::c_uint, ::core::ffi::c_uint, ::core::ffi::c_uint, ::core::ffi::c_uint, ::core::ffi::c_uint, hipStream_t, *mut *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleLaunchKernel") });
        unsafe { _f(f, gridDimX, gridDimY, gridDimZ, blockDimX, blockDimY, blockDimZ, sharedMemBytes, stream, kernelParams, extra) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleLaunchKernel(f: hipFunction_t, gridDimX: ::core::ffi::c_uint, gridDimY: ::core::ffi::c_uint, gridDimZ: ::core::ffi::c_uint, blockDimX: ::core::ffi::c_uint, blockDimY: ::core::ffi::c_uint, blockDimZ: ::core::ffi::c_uint, sharedMemBytes: ::core::ffi::c_uint, stream: hipStream_t, kernelParams: *mut *mut ::core::ffi::c_void, extra: *mut *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipModuleLaunchKernel(f, gridDimX, gridDimY, gridDimZ, blockDimX, blockDimY, blockDimZ, sharedMemBytes, stream, kernelParams, extra) }
    }
}
pub unsafe fn hipModuleLoad(module: *mut hipModule_t, fname: *const ::core::ffi::c_char) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipModule_t, *const ::core::ffi::c_char) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleLoad") });
        unsafe { _f(module, fname) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleLoad(module: *mut hipModule_t, fname: *const ::core::ffi::c_char) -> hipError_t;
        }
        unsafe { hipModuleLoad(module, fname) }
    }
}
pub unsafe fn hipModuleLoadData(module: *mut hipModule_t, image: *const ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipModule_t, *const ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleLoadData") });
        unsafe { _f(module, image) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleLoadData(module: *mut hipModule_t, image: *const ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipModuleLoadData(module, image) }
    }
}
pub unsafe fn hipModuleLoadDataEx(module: *mut hipModule_t, image: *const ::core::ffi::c_void, numOptions: ::core::ffi::c_uint, options: *mut hipJitOption, optionValues: *mut *mut ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipModule_t, *const ::core::ffi::c_void, ::core::ffi::c_uint, *mut hipJitOption, *mut *mut ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleLoadDataEx") });
        unsafe { _f(module, image, numOptions, options, optionValues) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleLoadDataEx(module: *mut hipModule_t, image: *const ::core::ffi::c_void, numOptions: ::core::ffi::c_uint, options: *mut hipJitOption, optionValues: *mut *mut ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipModuleLoadDataEx(module, image, numOptions, options, optionValues) }
    }
}
pub unsafe fn hipModuleLoadFatBinary(module: *mut hipModule_t, fatbin: *const ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipModule_t, *const ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleLoadFatBinary") });
        unsafe { _f(module, fatbin) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleLoadFatBinary(module: *mut hipModule_t, fatbin: *const ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipModuleLoadFatBinary(module, fatbin) }
    }
}
pub unsafe fn hipModuleOccupancyMaxActiveBlocksPerMultiprocessor(numBlocks: *mut ::core::ffi::c_int, f: hipFunction_t, blockSize: ::core::ffi::c_int, dynSharedMemPerBlk: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, hipFunction_t, ::core::ffi::c_int, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleOccupancyMaxActiveBlocksPerMultiprocessor") });
        unsafe { _f(numBlocks, f, blockSize, dynSharedMemPerBlk) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleOccupancyMaxActiveBlocksPerMultiprocessor(numBlocks: *mut ::core::ffi::c_int, f: hipFunction_t, blockSize: ::core::ffi::c_int, dynSharedMemPerBlk: usize) -> hipError_t;
        }
        unsafe { hipModuleOccupancyMaxActiveBlocksPerMultiprocessor(numBlocks, f, blockSize, dynSharedMemPerBlk) }
    }
}
pub unsafe fn hipModuleOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(numBlocks: *mut ::core::ffi::c_int, f: hipFunction_t, blockSize: ::core::ffi::c_int, dynSharedMemPerBlk: usize, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, hipFunction_t, ::core::ffi::c_int, usize, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleOccupancyMaxActiveBlocksPerMultiprocessorWithFlags") });
        unsafe { _f(numBlocks, f, blockSize, dynSharedMemPerBlk, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(numBlocks: *mut ::core::ffi::c_int, f: hipFunction_t, blockSize: ::core::ffi::c_int, dynSharedMemPerBlk: usize, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipModuleOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(numBlocks, f, blockSize, dynSharedMemPerBlk, flags) }
    }
}
pub unsafe fn hipModuleOccupancyMaxPotentialBlockSize(gridSize: *mut ::core::ffi::c_int, blockSize: *mut ::core::ffi::c_int, f: hipFunction_t, dynSharedMemPerBlk: usize, blockSizeLimit: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, *mut ::core::ffi::c_int, hipFunction_t, usize, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleOccupancyMaxPotentialBlockSize") });
        unsafe { _f(gridSize, blockSize, f, dynSharedMemPerBlk, blockSizeLimit) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleOccupancyMaxPotentialBlockSize(gridSize: *mut ::core::ffi::c_int, blockSize: *mut ::core::ffi::c_int, f: hipFunction_t, dynSharedMemPerBlk: usize, blockSizeLimit: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipModuleOccupancyMaxPotentialBlockSize(gridSize, blockSize, f, dynSharedMemPerBlk, blockSizeLimit) }
    }
}
pub unsafe fn hipModuleOccupancyMaxPotentialBlockSizeWithFlags(gridSize: *mut ::core::ffi::c_int, blockSize: *mut ::core::ffi::c_int, f: hipFunction_t, dynSharedMemPerBlk: usize, blockSizeLimit: ::core::ffi::c_int, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, *mut ::core::ffi::c_int, hipFunction_t, usize, ::core::ffi::c_int, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleOccupancyMaxPotentialBlockSizeWithFlags") });
        unsafe { _f(gridSize, blockSize, f, dynSharedMemPerBlk, blockSizeLimit, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleOccupancyMaxPotentialBlockSizeWithFlags(gridSize: *mut ::core::ffi::c_int, blockSize: *mut ::core::ffi::c_int, f: hipFunction_t, dynSharedMemPerBlk: usize, blockSizeLimit: ::core::ffi::c_int, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipModuleOccupancyMaxPotentialBlockSizeWithFlags(gridSize, blockSize, f, dynSharedMemPerBlk, blockSizeLimit, flags) }
    }
}
pub unsafe fn hipModuleUnload(module: hipModule_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipModule_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipModuleUnload") });
        unsafe { _f(module) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipModuleUnload(module: hipModule_t) -> hipError_t;
        }
        unsafe { hipModuleUnload(module) }
    }
}
pub unsafe fn hipOccupancyAvailableDynamicSMemPerBlock(dynamicSmemSize: *mut usize, f: *const ::core::ffi::c_void, numBlocks: ::core::ffi::c_int, blockSize: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut usize, *const ::core::ffi::c_void, ::core::ffi::c_int, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipOccupancyAvailableDynamicSMemPerBlock") });
        unsafe { _f(dynamicSmemSize, f, numBlocks, blockSize) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipOccupancyAvailableDynamicSMemPerBlock(dynamicSmemSize: *mut usize, f: *const ::core::ffi::c_void, numBlocks: ::core::ffi::c_int, blockSize: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipOccupancyAvailableDynamicSMemPerBlock(dynamicSmemSize, f, numBlocks, blockSize) }
    }
}
pub unsafe fn hipOccupancyMaxActiveBlocksPerMultiprocessor(numBlocks: *mut ::core::ffi::c_int, f: *const ::core::ffi::c_void, blockSize: ::core::ffi::c_int, dynSharedMemPerBlk: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, *const ::core::ffi::c_void, ::core::ffi::c_int, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipOccupancyMaxActiveBlocksPerMultiprocessor") });
        unsafe { _f(numBlocks, f, blockSize, dynSharedMemPerBlk) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipOccupancyMaxActiveBlocksPerMultiprocessor(numBlocks: *mut ::core::ffi::c_int, f: *const ::core::ffi::c_void, blockSize: ::core::ffi::c_int, dynSharedMemPerBlk: usize) -> hipError_t;
        }
        unsafe { hipOccupancyMaxActiveBlocksPerMultiprocessor(numBlocks, f, blockSize, dynSharedMemPerBlk) }
    }
}
pub unsafe fn hipOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(numBlocks: *mut ::core::ffi::c_int, f: *const ::core::ffi::c_void, blockSize: ::core::ffi::c_int, dynSharedMemPerBlk: usize, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, *const ::core::ffi::c_void, ::core::ffi::c_int, usize, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipOccupancyMaxActiveBlocksPerMultiprocessorWithFlags") });
        unsafe { _f(numBlocks, f, blockSize, dynSharedMemPerBlk, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(numBlocks: *mut ::core::ffi::c_int, f: *const ::core::ffi::c_void, blockSize: ::core::ffi::c_int, dynSharedMemPerBlk: usize, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(numBlocks, f, blockSize, dynSharedMemPerBlk, flags) }
    }
}
pub unsafe fn hipOccupancyMaxPotentialBlockSize(gridSize: *mut ::core::ffi::c_int, blockSize: *mut ::core::ffi::c_int, f: *const ::core::ffi::c_void, dynSharedMemPerBlk: usize, blockSizeLimit: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *const ::core::ffi::c_void, usize, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipOccupancyMaxPotentialBlockSize") });
        unsafe { _f(gridSize, blockSize, f, dynSharedMemPerBlk, blockSizeLimit) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipOccupancyMaxPotentialBlockSize(gridSize: *mut ::core::ffi::c_int, blockSize: *mut ::core::ffi::c_int, f: *const ::core::ffi::c_void, dynSharedMemPerBlk: usize, blockSizeLimit: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipOccupancyMaxPotentialBlockSize(gridSize, blockSize, f, dynSharedMemPerBlk, blockSizeLimit) }
    }
}
pub unsafe fn hipPeekAtLastError() -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn() -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipPeekAtLastError") });
        unsafe { _f() }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipPeekAtLastError() -> hipError_t;
        }
        unsafe { hipPeekAtLastError() }
    }
}
pub unsafe fn hipPointerGetAttribute(data: *mut ::core::ffi::c_void, attribute: hipPointer_attribute, ptr: hipDeviceptr_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_void, hipPointer_attribute, hipDeviceptr_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipPointerGetAttribute") });
        unsafe { _f(data, attribute, ptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipPointerGetAttribute(data: *mut ::core::ffi::c_void, attribute: hipPointer_attribute, ptr: hipDeviceptr_t) -> hipError_t;
        }
        unsafe { hipPointerGetAttribute(data, attribute, ptr) }
    }
}
pub unsafe fn hipPointerGetAttributes(attributes: *mut hipPointerAttribute_t, ptr: *const ::core::ffi::c_void) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipPointerAttribute_t, *const ::core::ffi::c_void) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipPointerGetAttributes") });
        unsafe { _f(attributes, ptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipPointerGetAttributes(attributes: *mut hipPointerAttribute_t, ptr: *const ::core::ffi::c_void) -> hipError_t;
        }
        unsafe { hipPointerGetAttributes(attributes, ptr) }
    }
}
pub unsafe fn hipPointerSetAttribute(value: *const ::core::ffi::c_void, attribute: hipPointer_attribute, ptr: hipDeviceptr_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, hipPointer_attribute, hipDeviceptr_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipPointerSetAttribute") });
        unsafe { _f(value, attribute, ptr) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipPointerSetAttribute(value: *const ::core::ffi::c_void, attribute: hipPointer_attribute, ptr: hipDeviceptr_t) -> hipError_t;
        }
        unsafe { hipPointerSetAttribute(value, attribute, ptr) }
    }
}
pub unsafe fn hipProfilerStart() -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn() -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipProfilerStart") });
        unsafe { _f() }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipProfilerStart() -> hipError_t;
        }
        unsafe { hipProfilerStart() }
    }
}
pub unsafe fn hipProfilerStop() -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn() -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipProfilerStop") });
        unsafe { _f() }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipProfilerStop() -> hipError_t;
        }
        unsafe { hipProfilerStop() }
    }
}
pub unsafe fn hipRuntimeGetVersion(runtimeVersion: *mut ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipRuntimeGetVersion") });
        unsafe { _f(runtimeVersion) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipRuntimeGetVersion(runtimeVersion: *mut ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipRuntimeGetVersion(runtimeVersion) }
    }
}
pub unsafe fn hipSetDevice(deviceId: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipSetDevice") });
        unsafe { _f(deviceId) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipSetDevice(deviceId: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipSetDevice(deviceId) }
    }
}
pub unsafe fn hipSetDeviceFlags(flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipSetDeviceFlags") });
        unsafe { _f(flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipSetDeviceFlags(flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipSetDeviceFlags(flags) }
    }
}
pub unsafe fn hipSetValidDevices(device_arr: *mut ::core::ffi::c_int, len: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipSetValidDevices") });
        unsafe { _f(device_arr, len) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipSetValidDevices(device_arr: *mut ::core::ffi::c_int, len: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipSetValidDevices(device_arr, len) }
    }
}
pub unsafe fn hipSetupArgument(arg: *const ::core::ffi::c_void, size: usize, offset: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const ::core::ffi::c_void, usize, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipSetupArgument") });
        unsafe { _f(arg, size, offset) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipSetupArgument(arg: *const ::core::ffi::c_void, size: usize, offset: usize) -> hipError_t;
        }
        unsafe { hipSetupArgument(arg, size, offset) }
    }
}
pub unsafe fn hipSignalExternalSemaphoresAsync(extSemArray: *const hipExternalSemaphore_t, paramsArray: *const hipExternalSemaphoreSignalParams, numExtSems: ::core::ffi::c_uint, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const hipExternalSemaphore_t, *const hipExternalSemaphoreSignalParams, ::core::ffi::c_uint, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipSignalExternalSemaphoresAsync") });
        unsafe { _f(extSemArray, paramsArray, numExtSems, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipSignalExternalSemaphoresAsync(extSemArray: *const hipExternalSemaphore_t, paramsArray: *const hipExternalSemaphoreSignalParams, numExtSems: ::core::ffi::c_uint, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipSignalExternalSemaphoresAsync(extSemArray, paramsArray, numExtSems, stream) }
    }
}
pub unsafe fn hipStreamAddCallback(stream: hipStream_t, callback: hipStreamCallback_t, userData: *mut ::core::ffi::c_void, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, hipStreamCallback_t, *mut ::core::ffi::c_void, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamAddCallback") });
        unsafe { _f(stream, callback, userData, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamAddCallback(stream: hipStream_t, callback: hipStreamCallback_t, userData: *mut ::core::ffi::c_void, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipStreamAddCallback(stream, callback, userData, flags) }
    }
}
pub unsafe fn hipStreamAddCallback_spt(stream: hipStream_t, callback: hipStreamCallback_t, userData: *mut ::core::ffi::c_void, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, hipStreamCallback_t, *mut ::core::ffi::c_void, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamAddCallback_spt") });
        unsafe { _f(stream, callback, userData, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamAddCallback_spt(stream: hipStream_t, callback: hipStreamCallback_t, userData: *mut ::core::ffi::c_void, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipStreamAddCallback_spt(stream, callback, userData, flags) }
    }
}
pub unsafe fn hipStreamAttachMemAsync(stream: hipStream_t, dev_ptr: *mut ::core::ffi::c_void, length: usize, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut ::core::ffi::c_void, usize, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamAttachMemAsync") });
        unsafe { _f(stream, dev_ptr, length, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamAttachMemAsync(stream: hipStream_t, dev_ptr: *mut ::core::ffi::c_void, length: usize, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipStreamAttachMemAsync(stream, dev_ptr, length, flags) }
    }
}
pub unsafe fn hipStreamBatchMemOp(stream: hipStream_t, count: ::core::ffi::c_uint, paramArray: *mut hipStreamBatchMemOpParams, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, ::core::ffi::c_uint, *mut hipStreamBatchMemOpParams, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamBatchMemOp") });
        unsafe { _f(stream, count, paramArray, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamBatchMemOp(stream: hipStream_t, count: ::core::ffi::c_uint, paramArray: *mut hipStreamBatchMemOpParams, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipStreamBatchMemOp(stream, count, paramArray, flags) }
    }
}
pub unsafe fn hipStreamBeginCapture(stream: hipStream_t, mode: hipStreamCaptureMode) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, hipStreamCaptureMode) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamBeginCapture") });
        unsafe { _f(stream, mode) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamBeginCapture(stream: hipStream_t, mode: hipStreamCaptureMode) -> hipError_t;
        }
        unsafe { hipStreamBeginCapture(stream, mode) }
    }
}
pub unsafe fn hipStreamBeginCaptureToGraph(stream: hipStream_t, graph: hipGraph_t, dependencies: *const hipGraphNode_t, dependencyData: *const hipGraphEdgeData, numDependencies: usize, mode: hipStreamCaptureMode) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, hipGraph_t, *const hipGraphNode_t, *const hipGraphEdgeData, usize, hipStreamCaptureMode) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamBeginCaptureToGraph") });
        unsafe { _f(stream, graph, dependencies, dependencyData, numDependencies, mode) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamBeginCaptureToGraph(stream: hipStream_t, graph: hipGraph_t, dependencies: *const hipGraphNode_t, dependencyData: *const hipGraphEdgeData, numDependencies: usize, mode: hipStreamCaptureMode) -> hipError_t;
        }
        unsafe { hipStreamBeginCaptureToGraph(stream, graph, dependencies, dependencyData, numDependencies, mode) }
    }
}
pub unsafe fn hipStreamBeginCapture_spt(stream: hipStream_t, mode: hipStreamCaptureMode) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, hipStreamCaptureMode) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamBeginCapture_spt") });
        unsafe { _f(stream, mode) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamBeginCapture_spt(stream: hipStream_t, mode: hipStreamCaptureMode) -> hipError_t;
        }
        unsafe { hipStreamBeginCapture_spt(stream, mode) }
    }
}
pub unsafe fn hipStreamCopyAttributes(dst: hipStream_t, src: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamCopyAttributes") });
        unsafe { _f(dst, src) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamCopyAttributes(dst: hipStream_t, src: hipStream_t) -> hipError_t;
        }
        unsafe { hipStreamCopyAttributes(dst, src) }
    }
}
pub unsafe fn hipStreamCreate(stream: *mut hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamCreate") });
        unsafe { _f(stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamCreate(stream: *mut hipStream_t) -> hipError_t;
        }
        unsafe { hipStreamCreate(stream) }
    }
}
pub unsafe fn hipStreamCreateWithFlags(stream: *mut hipStream_t, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipStream_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamCreateWithFlags") });
        unsafe { _f(stream, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamCreateWithFlags(stream: *mut hipStream_t, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipStreamCreateWithFlags(stream, flags) }
    }
}
pub unsafe fn hipStreamCreateWithPriority(stream: *mut hipStream_t, flags: ::core::ffi::c_uint, priority: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipStream_t, ::core::ffi::c_uint, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamCreateWithPriority") });
        unsafe { _f(stream, flags, priority) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamCreateWithPriority(stream: *mut hipStream_t, flags: ::core::ffi::c_uint, priority: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipStreamCreateWithPriority(stream, flags, priority) }
    }
}
pub unsafe fn hipStreamDestroy(stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamDestroy") });
        unsafe { _f(stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamDestroy(stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipStreamDestroy(stream) }
    }
}
pub unsafe fn hipStreamEndCapture(stream: hipStream_t, pGraph: *mut hipGraph_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut hipGraph_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamEndCapture") });
        unsafe { _f(stream, pGraph) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamEndCapture(stream: hipStream_t, pGraph: *mut hipGraph_t) -> hipError_t;
        }
        unsafe { hipStreamEndCapture(stream, pGraph) }
    }
}
pub unsafe fn hipStreamEndCapture_spt(stream: hipStream_t, pGraph: *mut hipGraph_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut hipGraph_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamEndCapture_spt") });
        unsafe { _f(stream, pGraph) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamEndCapture_spt(stream: hipStream_t, pGraph: *mut hipGraph_t) -> hipError_t;
        }
        unsafe { hipStreamEndCapture_spt(stream, pGraph) }
    }
}
pub unsafe fn hipStreamGetAttribute(stream: hipStream_t, attr: hipLaunchAttributeID, value_out: *mut hipLaunchAttributeValue) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, hipLaunchAttributeID, *mut hipLaunchAttributeValue) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamGetAttribute") });
        unsafe { _f(stream, attr, value_out) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamGetAttribute(stream: hipStream_t, attr: hipLaunchAttributeID, value_out: *mut hipLaunchAttributeValue) -> hipError_t;
        }
        unsafe { hipStreamGetAttribute(stream, attr, value_out) }
    }
}
pub unsafe fn hipStreamGetCaptureInfo(stream: hipStream_t, pCaptureStatus: *mut hipStreamCaptureStatus, pId: *mut ::core::ffi::c_ulonglong) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut hipStreamCaptureStatus, *mut ::core::ffi::c_ulonglong) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamGetCaptureInfo") });
        unsafe { _f(stream, pCaptureStatus, pId) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamGetCaptureInfo(stream: hipStream_t, pCaptureStatus: *mut hipStreamCaptureStatus, pId: *mut ::core::ffi::c_ulonglong) -> hipError_t;
        }
        unsafe { hipStreamGetCaptureInfo(stream, pCaptureStatus, pId) }
    }
}
pub unsafe fn hipStreamGetCaptureInfo_spt(stream: hipStream_t, pCaptureStatus: *mut hipStreamCaptureStatus, pId: *mut ::core::ffi::c_ulonglong) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut hipStreamCaptureStatus, *mut ::core::ffi::c_ulonglong) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamGetCaptureInfo_spt") });
        unsafe { _f(stream, pCaptureStatus, pId) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamGetCaptureInfo_spt(stream: hipStream_t, pCaptureStatus: *mut hipStreamCaptureStatus, pId: *mut ::core::ffi::c_ulonglong) -> hipError_t;
        }
        unsafe { hipStreamGetCaptureInfo_spt(stream, pCaptureStatus, pId) }
    }
}
pub unsafe fn hipStreamGetCaptureInfo_v2(stream: hipStream_t, captureStatus_out: *mut hipStreamCaptureStatus, id_out: *mut ::core::ffi::c_ulonglong, graph_out: *mut hipGraph_t, dependencies_out: *mut *const hipGraphNode_t, numDependencies_out: *mut usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut hipStreamCaptureStatus, *mut ::core::ffi::c_ulonglong, *mut hipGraph_t, *mut *const hipGraphNode_t, *mut usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamGetCaptureInfo_v2") });
        unsafe { _f(stream, captureStatus_out, id_out, graph_out, dependencies_out, numDependencies_out) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamGetCaptureInfo_v2(stream: hipStream_t, captureStatus_out: *mut hipStreamCaptureStatus, id_out: *mut ::core::ffi::c_ulonglong, graph_out: *mut hipGraph_t, dependencies_out: *mut *const hipGraphNode_t, numDependencies_out: *mut usize) -> hipError_t;
        }
        unsafe { hipStreamGetCaptureInfo_v2(stream, captureStatus_out, id_out, graph_out, dependencies_out, numDependencies_out) }
    }
}
pub unsafe fn hipStreamGetCaptureInfo_v2_spt(stream: hipStream_t, captureStatus_out: *mut hipStreamCaptureStatus, id_out: *mut ::core::ffi::c_ulonglong, graph_out: *mut hipGraph_t, dependencies_out: *mut *const hipGraphNode_t, numDependencies_out: *mut usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut hipStreamCaptureStatus, *mut ::core::ffi::c_ulonglong, *mut hipGraph_t, *mut *const hipGraphNode_t, *mut usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamGetCaptureInfo_v2_spt") });
        unsafe { _f(stream, captureStatus_out, id_out, graph_out, dependencies_out, numDependencies_out) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamGetCaptureInfo_v2_spt(stream: hipStream_t, captureStatus_out: *mut hipStreamCaptureStatus, id_out: *mut ::core::ffi::c_ulonglong, graph_out: *mut hipGraph_t, dependencies_out: *mut *const hipGraphNode_t, numDependencies_out: *mut usize) -> hipError_t;
        }
        unsafe { hipStreamGetCaptureInfo_v2_spt(stream, captureStatus_out, id_out, graph_out, dependencies_out, numDependencies_out) }
    }
}
pub unsafe fn hipStreamGetDevice(stream: hipStream_t, device: *mut hipDevice_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut hipDevice_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamGetDevice") });
        unsafe { _f(stream, device) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamGetDevice(stream: hipStream_t, device: *mut hipDevice_t) -> hipError_t;
        }
        unsafe { hipStreamGetDevice(stream, device) }
    }
}
pub unsafe fn hipStreamGetFlags(stream: hipStream_t, flags: *mut ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamGetFlags") });
        unsafe { _f(stream, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamGetFlags(stream: hipStream_t, flags: *mut ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipStreamGetFlags(stream, flags) }
    }
}
pub unsafe fn hipStreamGetFlags_spt(stream: hipStream_t, flags: *mut ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamGetFlags_spt") });
        unsafe { _f(stream, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamGetFlags_spt(stream: hipStream_t, flags: *mut ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipStreamGetFlags_spt(stream, flags) }
    }
}
pub unsafe fn hipStreamGetId(stream: hipStream_t, streamId: *mut ::core::ffi::c_ulonglong) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut ::core::ffi::c_ulonglong) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamGetId") });
        unsafe { _f(stream, streamId) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamGetId(stream: hipStream_t, streamId: *mut ::core::ffi::c_ulonglong) -> hipError_t;
        }
        unsafe { hipStreamGetId(stream, streamId) }
    }
}
pub unsafe fn hipStreamGetPriority(stream: hipStream_t, priority: *mut ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamGetPriority") });
        unsafe { _f(stream, priority) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamGetPriority(stream: hipStream_t, priority: *mut ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipStreamGetPriority(stream, priority) }
    }
}
pub unsafe fn hipStreamGetPriority_spt(stream: hipStream_t, priority: *mut ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamGetPriority_spt") });
        unsafe { _f(stream, priority) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamGetPriority_spt(stream: hipStream_t, priority: *mut ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipStreamGetPriority_spt(stream, priority) }
    }
}
pub unsafe fn hipStreamIsCapturing(stream: hipStream_t, pCaptureStatus: *mut hipStreamCaptureStatus) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut hipStreamCaptureStatus) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamIsCapturing") });
        unsafe { _f(stream, pCaptureStatus) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamIsCapturing(stream: hipStream_t, pCaptureStatus: *mut hipStreamCaptureStatus) -> hipError_t;
        }
        unsafe { hipStreamIsCapturing(stream, pCaptureStatus) }
    }
}
pub unsafe fn hipStreamIsCapturing_spt(stream: hipStream_t, pCaptureStatus: *mut hipStreamCaptureStatus) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut hipStreamCaptureStatus) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamIsCapturing_spt") });
        unsafe { _f(stream, pCaptureStatus) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamIsCapturing_spt(stream: hipStream_t, pCaptureStatus: *mut hipStreamCaptureStatus) -> hipError_t;
        }
        unsafe { hipStreamIsCapturing_spt(stream, pCaptureStatus) }
    }
}
pub unsafe fn hipStreamQuery(stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamQuery") });
        unsafe { _f(stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamQuery(stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipStreamQuery(stream) }
    }
}
pub unsafe fn hipStreamQuery_spt(stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamQuery_spt") });
        unsafe { _f(stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamQuery_spt(stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipStreamQuery_spt(stream) }
    }
}
pub unsafe fn hipStreamSetAttribute(stream: hipStream_t, attr: hipLaunchAttributeID, value: *const hipLaunchAttributeValue) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, hipLaunchAttributeID, *const hipLaunchAttributeValue) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamSetAttribute") });
        unsafe { _f(stream, attr, value) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamSetAttribute(stream: hipStream_t, attr: hipLaunchAttributeID, value: *const hipLaunchAttributeValue) -> hipError_t;
        }
        unsafe { hipStreamSetAttribute(stream, attr, value) }
    }
}
pub unsafe fn hipStreamSynchronize(stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamSynchronize") });
        unsafe { _f(stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamSynchronize(stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipStreamSynchronize(stream) }
    }
}
pub unsafe fn hipStreamSynchronize_spt(stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamSynchronize_spt") });
        unsafe { _f(stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamSynchronize_spt(stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipStreamSynchronize_spt(stream) }
    }
}
pub unsafe fn hipStreamUpdateCaptureDependencies(stream: hipStream_t, dependencies: *mut hipGraphNode_t, numDependencies: usize, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut hipGraphNode_t, usize, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamUpdateCaptureDependencies") });
        unsafe { _f(stream, dependencies, numDependencies, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamUpdateCaptureDependencies(stream: hipStream_t, dependencies: *mut hipGraphNode_t, numDependencies: usize, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipStreamUpdateCaptureDependencies(stream, dependencies, numDependencies, flags) }
    }
}
pub unsafe fn hipStreamWaitEvent(stream: hipStream_t, event: hipEvent_t, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, hipEvent_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamWaitEvent") });
        unsafe { _f(stream, event, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamWaitEvent(stream: hipStream_t, event: hipEvent_t, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipStreamWaitEvent(stream, event, flags) }
    }
}
pub unsafe fn hipStreamWaitEvent_spt(stream: hipStream_t, event: hipEvent_t, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, hipEvent_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamWaitEvent_spt") });
        unsafe { _f(stream, event, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamWaitEvent_spt(stream: hipStream_t, event: hipEvent_t, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipStreamWaitEvent_spt(stream, event, flags) }
    }
}
pub unsafe fn hipStreamWaitValue32(stream: hipStream_t, ptr: *mut ::core::ffi::c_void, value: u32, flags: ::core::ffi::c_uint, mask: u32) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut ::core::ffi::c_void, u32, ::core::ffi::c_uint, u32) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamWaitValue32") });
        unsafe { _f(stream, ptr, value, flags, mask) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamWaitValue32(stream: hipStream_t, ptr: *mut ::core::ffi::c_void, value: u32, flags: ::core::ffi::c_uint, mask: u32) -> hipError_t;
        }
        unsafe { hipStreamWaitValue32(stream, ptr, value, flags, mask) }
    }
}
pub unsafe fn hipStreamWaitValue64(stream: hipStream_t, ptr: *mut ::core::ffi::c_void, value: u64, flags: ::core::ffi::c_uint, mask: u64) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut ::core::ffi::c_void, u64, ::core::ffi::c_uint, u64) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamWaitValue64") });
        unsafe { _f(stream, ptr, value, flags, mask) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamWaitValue64(stream: hipStream_t, ptr: *mut ::core::ffi::c_void, value: u64, flags: ::core::ffi::c_uint, mask: u64) -> hipError_t;
        }
        unsafe { hipStreamWaitValue64(stream, ptr, value, flags, mask) }
    }
}
pub unsafe fn hipStreamWriteValue32(stream: hipStream_t, ptr: *mut ::core::ffi::c_void, value: u32, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut ::core::ffi::c_void, u32, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamWriteValue32") });
        unsafe { _f(stream, ptr, value, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamWriteValue32(stream: hipStream_t, ptr: *mut ::core::ffi::c_void, value: u32, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipStreamWriteValue32(stream, ptr, value, flags) }
    }
}
pub unsafe fn hipStreamWriteValue64(stream: hipStream_t, ptr: *mut ::core::ffi::c_void, value: u64, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipStream_t, *mut ::core::ffi::c_void, u64, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipStreamWriteValue64") });
        unsafe { _f(stream, ptr, value, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipStreamWriteValue64(stream: hipStream_t, ptr: *mut ::core::ffi::c_void, value: u64, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipStreamWriteValue64(stream, ptr, value, flags) }
    }
}
pub unsafe fn hipTexObjectCreate(pTexObject: *mut hipTextureObject_t, pResDesc: *const HIP_RESOURCE_DESC, pTexDesc: *const HIP_TEXTURE_DESC, pResViewDesc: *const HIP_RESOURCE_VIEW_DESC) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipTextureObject_t, *const HIP_RESOURCE_DESC, *const HIP_TEXTURE_DESC, *const HIP_RESOURCE_VIEW_DESC) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexObjectCreate") });
        unsafe { _f(pTexObject, pResDesc, pTexDesc, pResViewDesc) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexObjectCreate(pTexObject: *mut hipTextureObject_t, pResDesc: *const HIP_RESOURCE_DESC, pTexDesc: *const HIP_TEXTURE_DESC, pResViewDesc: *const HIP_RESOURCE_VIEW_DESC) -> hipError_t;
        }
        unsafe { hipTexObjectCreate(pTexObject, pResDesc, pTexDesc, pResViewDesc) }
    }
}
pub unsafe fn hipTexObjectDestroy(texObject: hipTextureObject_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipTextureObject_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexObjectDestroy") });
        unsafe { _f(texObject) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexObjectDestroy(texObject: hipTextureObject_t) -> hipError_t;
        }
        unsafe { hipTexObjectDestroy(texObject) }
    }
}
pub unsafe fn hipTexObjectGetResourceDesc(pResDesc: *mut HIP_RESOURCE_DESC, texObject: hipTextureObject_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut HIP_RESOURCE_DESC, hipTextureObject_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexObjectGetResourceDesc") });
        unsafe { _f(pResDesc, texObject) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexObjectGetResourceDesc(pResDesc: *mut HIP_RESOURCE_DESC, texObject: hipTextureObject_t) -> hipError_t;
        }
        unsafe { hipTexObjectGetResourceDesc(pResDesc, texObject) }
    }
}
pub unsafe fn hipTexObjectGetResourceViewDesc(pResViewDesc: *mut HIP_RESOURCE_VIEW_DESC, texObject: hipTextureObject_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut HIP_RESOURCE_VIEW_DESC, hipTextureObject_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexObjectGetResourceViewDesc") });
        unsafe { _f(pResViewDesc, texObject) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexObjectGetResourceViewDesc(pResViewDesc: *mut HIP_RESOURCE_VIEW_DESC, texObject: hipTextureObject_t) -> hipError_t;
        }
        unsafe { hipTexObjectGetResourceViewDesc(pResViewDesc, texObject) }
    }
}
pub unsafe fn hipTexObjectGetTextureDesc(pTexDesc: *mut HIP_TEXTURE_DESC, texObject: hipTextureObject_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut HIP_TEXTURE_DESC, hipTextureObject_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexObjectGetTextureDesc") });
        unsafe { _f(pTexDesc, texObject) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexObjectGetTextureDesc(pTexDesc: *mut HIP_TEXTURE_DESC, texObject: hipTextureObject_t) -> hipError_t;
        }
        unsafe { hipTexObjectGetTextureDesc(pTexDesc, texObject) }
    }
}
pub unsafe fn hipTexRefGetAddress(dev_ptr: *mut hipDeviceptr_t, texRef: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipDeviceptr_t, *const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefGetAddress") });
        unsafe { _f(dev_ptr, texRef) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefGetAddress(dev_ptr: *mut hipDeviceptr_t, texRef: *const textureReference) -> hipError_t;
        }
        unsafe { hipTexRefGetAddress(dev_ptr, texRef) }
    }
}
pub unsafe fn hipTexRefGetAddressMode(pam: *mut hipTextureAddressMode, texRef: *const textureReference, dim: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipTextureAddressMode, *const textureReference, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefGetAddressMode") });
        unsafe { _f(pam, texRef, dim) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefGetAddressMode(pam: *mut hipTextureAddressMode, texRef: *const textureReference, dim: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipTexRefGetAddressMode(pam, texRef, dim) }
    }
}
pub unsafe fn hipTexRefGetArray(pArray: *mut hipArray_t, texRef: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipArray_t, *const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefGetArray") });
        unsafe { _f(pArray, texRef) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefGetArray(pArray: *mut hipArray_t, texRef: *const textureReference) -> hipError_t;
        }
        unsafe { hipTexRefGetArray(pArray, texRef) }
    }
}
pub unsafe fn hipTexRefGetBorderColor(pBorderColor: *mut f32, texRef: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut f32, *const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefGetBorderColor") });
        unsafe { _f(pBorderColor, texRef) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefGetBorderColor(pBorderColor: *mut f32, texRef: *const textureReference) -> hipError_t;
        }
        unsafe { hipTexRefGetBorderColor(pBorderColor, texRef) }
    }
}
pub unsafe fn hipTexRefGetFilterMode(pfm: *mut hipTextureFilterMode, texRef: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipTextureFilterMode, *const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefGetFilterMode") });
        unsafe { _f(pfm, texRef) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefGetFilterMode(pfm: *mut hipTextureFilterMode, texRef: *const textureReference) -> hipError_t;
        }
        unsafe { hipTexRefGetFilterMode(pfm, texRef) }
    }
}
pub unsafe fn hipTexRefGetFlags(pFlags: *mut ::core::ffi::c_uint, texRef: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_uint, *const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefGetFlags") });
        unsafe { _f(pFlags, texRef) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefGetFlags(pFlags: *mut ::core::ffi::c_uint, texRef: *const textureReference) -> hipError_t;
        }
        unsafe { hipTexRefGetFlags(pFlags, texRef) }
    }
}
pub unsafe fn hipTexRefGetFormat(pFormat: *mut hipArray_Format, pNumChannels: *mut ::core::ffi::c_int, texRef: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipArray_Format, *mut ::core::ffi::c_int, *const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefGetFormat") });
        unsafe { _f(pFormat, pNumChannels, texRef) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefGetFormat(pFormat: *mut hipArray_Format, pNumChannels: *mut ::core::ffi::c_int, texRef: *const textureReference) -> hipError_t;
        }
        unsafe { hipTexRefGetFormat(pFormat, pNumChannels, texRef) }
    }
}
pub unsafe fn hipTexRefGetMaxAnisotropy(pmaxAnsio: *mut ::core::ffi::c_int, texRef: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut ::core::ffi::c_int, *const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefGetMaxAnisotropy") });
        unsafe { _f(pmaxAnsio, texRef) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefGetMaxAnisotropy(pmaxAnsio: *mut ::core::ffi::c_int, texRef: *const textureReference) -> hipError_t;
        }
        unsafe { hipTexRefGetMaxAnisotropy(pmaxAnsio, texRef) }
    }
}
pub unsafe fn hipTexRefGetMipMappedArray(pArray: *mut hipMipmappedArray_t, texRef: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipMipmappedArray_t, *const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefGetMipMappedArray") });
        unsafe { _f(pArray, texRef) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefGetMipMappedArray(pArray: *mut hipMipmappedArray_t, texRef: *const textureReference) -> hipError_t;
        }
        unsafe { hipTexRefGetMipMappedArray(pArray, texRef) }
    }
}
pub unsafe fn hipTexRefGetMipmapFilterMode(pfm: *mut hipTextureFilterMode, texRef: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipTextureFilterMode, *const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefGetMipmapFilterMode") });
        unsafe { _f(pfm, texRef) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefGetMipmapFilterMode(pfm: *mut hipTextureFilterMode, texRef: *const textureReference) -> hipError_t;
        }
        unsafe { hipTexRefGetMipmapFilterMode(pfm, texRef) }
    }
}
pub unsafe fn hipTexRefGetMipmapLevelBias(pbias: *mut f32, texRef: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut f32, *const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefGetMipmapLevelBias") });
        unsafe { _f(pbias, texRef) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefGetMipmapLevelBias(pbias: *mut f32, texRef: *const textureReference) -> hipError_t;
        }
        unsafe { hipTexRefGetMipmapLevelBias(pbias, texRef) }
    }
}
pub unsafe fn hipTexRefGetMipmapLevelClamp(pminMipmapLevelClamp: *mut f32, pmaxMipmapLevelClamp: *mut f32, texRef: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut f32, *mut f32, *const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefGetMipmapLevelClamp") });
        unsafe { _f(pminMipmapLevelClamp, pmaxMipmapLevelClamp, texRef) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefGetMipmapLevelClamp(pminMipmapLevelClamp: *mut f32, pmaxMipmapLevelClamp: *mut f32, texRef: *const textureReference) -> hipError_t;
        }
        unsafe { hipTexRefGetMipmapLevelClamp(pminMipmapLevelClamp, pmaxMipmapLevelClamp, texRef) }
    }
}
pub unsafe fn hipTexRefSetAddress(ByteOffset: *mut usize, texRef: *mut textureReference, dptr: hipDeviceptr_t, bytes: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut usize, *mut textureReference, hipDeviceptr_t, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetAddress") });
        unsafe { _f(ByteOffset, texRef, dptr, bytes) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetAddress(ByteOffset: *mut usize, texRef: *mut textureReference, dptr: hipDeviceptr_t, bytes: usize) -> hipError_t;
        }
        unsafe { hipTexRefSetAddress(ByteOffset, texRef, dptr, bytes) }
    }
}
pub unsafe fn hipTexRefSetAddress2D(texRef: *mut textureReference, desc: *const HIP_ARRAY_DESCRIPTOR, dptr: hipDeviceptr_t, Pitch: usize) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut textureReference, *const HIP_ARRAY_DESCRIPTOR, hipDeviceptr_t, usize) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetAddress2D") });
        unsafe { _f(texRef, desc, dptr, Pitch) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetAddress2D(texRef: *mut textureReference, desc: *const HIP_ARRAY_DESCRIPTOR, dptr: hipDeviceptr_t, Pitch: usize) -> hipError_t;
        }
        unsafe { hipTexRefSetAddress2D(texRef, desc, dptr, Pitch) }
    }
}
pub unsafe fn hipTexRefSetAddressMode(texRef: *mut textureReference, dim: ::core::ffi::c_int, am: hipTextureAddressMode) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut textureReference, ::core::ffi::c_int, hipTextureAddressMode) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetAddressMode") });
        unsafe { _f(texRef, dim, am) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetAddressMode(texRef: *mut textureReference, dim: ::core::ffi::c_int, am: hipTextureAddressMode) -> hipError_t;
        }
        unsafe { hipTexRefSetAddressMode(texRef, dim, am) }
    }
}
pub unsafe fn hipTexRefSetArray(tex: *mut textureReference, array: hipArray_const_t, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut textureReference, hipArray_const_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetArray") });
        unsafe { _f(tex, array, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetArray(tex: *mut textureReference, array: hipArray_const_t, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipTexRefSetArray(tex, array, flags) }
    }
}
pub unsafe fn hipTexRefSetBorderColor(texRef: *mut textureReference, pBorderColor: *mut f32) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut textureReference, *mut f32) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetBorderColor") });
        unsafe { _f(texRef, pBorderColor) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetBorderColor(texRef: *mut textureReference, pBorderColor: *mut f32) -> hipError_t;
        }
        unsafe { hipTexRefSetBorderColor(texRef, pBorderColor) }
    }
}
pub unsafe fn hipTexRefSetFilterMode(texRef: *mut textureReference, fm: hipTextureFilterMode) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut textureReference, hipTextureFilterMode) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetFilterMode") });
        unsafe { _f(texRef, fm) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetFilterMode(texRef: *mut textureReference, fm: hipTextureFilterMode) -> hipError_t;
        }
        unsafe { hipTexRefSetFilterMode(texRef, fm) }
    }
}
pub unsafe fn hipTexRefSetFlags(texRef: *mut textureReference, Flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut textureReference, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetFlags") });
        unsafe { _f(texRef, Flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetFlags(texRef: *mut textureReference, Flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipTexRefSetFlags(texRef, Flags) }
    }
}
pub unsafe fn hipTexRefSetFormat(texRef: *mut textureReference, fmt: hipArray_Format, NumPackedComponents: ::core::ffi::c_int) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut textureReference, hipArray_Format, ::core::ffi::c_int) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetFormat") });
        unsafe { _f(texRef, fmt, NumPackedComponents) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetFormat(texRef: *mut textureReference, fmt: hipArray_Format, NumPackedComponents: ::core::ffi::c_int) -> hipError_t;
        }
        unsafe { hipTexRefSetFormat(texRef, fmt, NumPackedComponents) }
    }
}
pub unsafe fn hipTexRefSetMaxAnisotropy(texRef: *mut textureReference, maxAniso: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut textureReference, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetMaxAnisotropy") });
        unsafe { _f(texRef, maxAniso) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetMaxAnisotropy(texRef: *mut textureReference, maxAniso: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipTexRefSetMaxAnisotropy(texRef, maxAniso) }
    }
}
pub unsafe fn hipTexRefSetMipmapFilterMode(texRef: *mut textureReference, fm: hipTextureFilterMode) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut textureReference, hipTextureFilterMode) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetMipmapFilterMode") });
        unsafe { _f(texRef, fm) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetMipmapFilterMode(texRef: *mut textureReference, fm: hipTextureFilterMode) -> hipError_t;
        }
        unsafe { hipTexRefSetMipmapFilterMode(texRef, fm) }
    }
}
pub unsafe fn hipTexRefSetMipmapLevelBias(texRef: *mut textureReference, bias: f32) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut textureReference, f32) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetMipmapLevelBias") });
        unsafe { _f(texRef, bias) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetMipmapLevelBias(texRef: *mut textureReference, bias: f32) -> hipError_t;
        }
        unsafe { hipTexRefSetMipmapLevelBias(texRef, bias) }
    }
}
pub unsafe fn hipTexRefSetMipmapLevelClamp(texRef: *mut textureReference, minMipMapLevelClamp: f32, maxMipMapLevelClamp: f32) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut textureReference, f32, f32) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetMipmapLevelClamp") });
        unsafe { _f(texRef, minMipMapLevelClamp, maxMipMapLevelClamp) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetMipmapLevelClamp(texRef: *mut textureReference, minMipMapLevelClamp: f32, maxMipMapLevelClamp: f32) -> hipError_t;
        }
        unsafe { hipTexRefSetMipmapLevelClamp(texRef, minMipMapLevelClamp, maxMipMapLevelClamp) }
    }
}
pub unsafe fn hipTexRefSetMipmappedArray(texRef: *mut textureReference, mipmappedArray: *mut hipMipmappedArray, Flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut textureReference, *mut hipMipmappedArray, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipTexRefSetMipmappedArray") });
        unsafe { _f(texRef, mipmappedArray, Flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipTexRefSetMipmappedArray(texRef: *mut textureReference, mipmappedArray: *mut hipMipmappedArray, Flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipTexRefSetMipmappedArray(texRef, mipmappedArray, Flags) }
    }
}
pub unsafe fn hipThreadExchangeStreamCaptureMode(mode: *mut hipStreamCaptureMode) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipStreamCaptureMode) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipThreadExchangeStreamCaptureMode") });
        unsafe { _f(mode) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipThreadExchangeStreamCaptureMode(mode: *mut hipStreamCaptureMode) -> hipError_t;
        }
        unsafe { hipThreadExchangeStreamCaptureMode(mode) }
    }
}
pub unsafe fn hipUnbindTexture(tex: *const textureReference) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const textureReference) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipUnbindTexture") });
        unsafe { _f(tex) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipUnbindTexture(tex: *const textureReference) -> hipError_t;
        }
        unsafe { hipUnbindTexture(tex) }
    }
}
pub unsafe fn hipUserObjectCreate(object_out: *mut hipUserObject_t, ptr: *mut ::core::ffi::c_void, destroy: hipHostFn_t, initialRefcount: ::core::ffi::c_uint, flags: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*mut hipUserObject_t, *mut ::core::ffi::c_void, hipHostFn_t, ::core::ffi::c_uint, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipUserObjectCreate") });
        unsafe { _f(object_out, ptr, destroy, initialRefcount, flags) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipUserObjectCreate(object_out: *mut hipUserObject_t, ptr: *mut ::core::ffi::c_void, destroy: hipHostFn_t, initialRefcount: ::core::ffi::c_uint, flags: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipUserObjectCreate(object_out, ptr, destroy, initialRefcount, flags) }
    }
}
pub unsafe fn hipUserObjectRelease(object: hipUserObject_t, count: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipUserObject_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipUserObjectRelease") });
        unsafe { _f(object, count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipUserObjectRelease(object: hipUserObject_t, count: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipUserObjectRelease(object, count) }
    }
}
pub unsafe fn hipUserObjectRetain(object: hipUserObject_t, count: ::core::ffi::c_uint) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(hipUserObject_t, ::core::ffi::c_uint) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipUserObjectRetain") });
        unsafe { _f(object, count) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipUserObjectRetain(object: hipUserObject_t, count: ::core::ffi::c_uint) -> hipError_t;
        }
        unsafe { hipUserObjectRetain(object, count) }
    }
}
pub unsafe fn hipWaitExternalSemaphoresAsync(extSemArray: *const hipExternalSemaphore_t, paramsArray: *const hipExternalSemaphoreWaitParams, numExtSems: ::core::ffi::c_uint, stream: hipStream_t) -> hipError_t {
    #[cfg(feature = "dynamic-loading")]
    {
        type _F = unsafe extern "C" fn(*const hipExternalSemaphore_t, *const hipExternalSemaphoreWaitParams, ::core::ffi::c_uint, hipStream_t) -> hipError_t;
        static _S: OnceLock<_F> = OnceLock::new();
        let _f = _S.get_or_init(|| unsafe { load::<_F>("hipWaitExternalSemaphoresAsync") });
        unsafe { _f(extSemArray, paramsArray, numExtSems, stream) }
    }
    #[cfg(not(feature = "dynamic-loading"))]
    {
        unsafe extern "C" {
            fn hipWaitExternalSemaphoresAsync(extSemArray: *const hipExternalSemaphore_t, paramsArray: *const hipExternalSemaphoreWaitParams, numExtSems: ::core::ffi::c_uint, stream: hipStream_t) -> hipError_t;
        }
        unsafe { hipWaitExternalSemaphoresAsync(extSemArray, paramsArray, numExtSems, stream) }
    }
}
#[cfg(feature = "dynamic-loading")]
pub unsafe fn is_rocmlib_present() -> bool {
    let lib_names = ["amdhip64"];
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
        let lib_names = std::vec!["amdhip64"];
        let choices: std::vec::Vec<_> = lib_names.iter().map(|l| crate::get_lib_name_candidates(l)).flatten().collect();
        for choice in choices.iter() {
            if let Ok(lib) = ::libloading::Library::new(choice) {
                return lib;
            }
        }
        crate::panic_no_lib_found(lib_names[0], &choices);
    })
}
