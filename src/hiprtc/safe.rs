//! Safe wrappers for hipRTC — runtime compilation of HIP C++ to HSACO.
//!
//! Mirrors cudarc's `nvrtc::safe` design, adapted for HIP's binary-only
//! output: HSACO is the AMD analog of CUDA's CUBIN (precompiled code
//! object), not PTX. HIP has no JIT-on-load path, so this module only
//! deals in compiled bytes — there is no "ship source / driver compiles"
//! variant.

use super::{result, sys};

use core::ffi::{CStr, c_char};
use std::borrow::Cow;
use std::ffi::CString;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

pub use result::HiprtcError;

/// Compiled HSA code object — the AMD analog of CUDA's CUBIN. Always
/// binary; the `gfx_version` it was compiled for is tracked alongside.
#[derive(Debug, Clone)]
pub struct Hsaco(pub(crate) HsacoKind, pub(crate) GfxVersion);

impl Hsaco {
    /// Load a pre-compiled `.hsaco` file. Bytes are read lazily — see
    /// [`Self::as_bytes`].
    pub fn from_file<P: Into<PathBuf>>(path: P, gfx_version: GfxVersion) -> Self {
        Self(HsacoKind::File(path.into()), gfx_version)
    }

    /// Wrap raw HSACO bytes in memory.
    pub fn from_binary(data: Vec<u8>, gfx_version: GfxVersion) -> Self {
        Self(HsacoKind::Binary(data), gfx_version)
    }

    /// Return the HSACO bytes. Reads the file on demand for
    /// [`HsacoKind::File`].
    pub fn as_bytes(&self) -> std::io::Result<Cow<'_, [u8]>> {
        match &self.0 {
            HsacoKind::Binary(b) => Ok(Cow::Borrowed(b)),
            HsacoKind::File(p) => Ok(Cow::Owned(std::fs::read(p)?)),
        }
    }

    /// The gfx arch this HSACO was compiled against.
    pub fn gfx_version(&self) -> GfxVersion {
        self.1
    }

    /// Direct access to the underlying source-form (file path vs in-memory
    /// bytes).
    pub fn kind(&self) -> &HsacoKind {
        &self.0
    }
}

/// Source-form of an [`Hsaco`]'s bytes.
#[derive(Debug, Clone)]
pub enum HsacoKind {
    /// In-memory bytes (e.g. from [`compile_hsaco`]).
    Binary(Vec<u8>),
    /// Path to a `.hsaco` file on disk; read lazily.
    File(PathBuf),
}

/// AMD GPU architecture targets. Tracked alongside [`Hsaco`] bytes so
/// runtime loaders can sanity-check before loading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GfxVersion {
    // Instinct (MI300 family).
    Gfx942,
    Gfx950,
    // Radeon / Radeon PRO (RDNA3, RDNA4).
    Gfx1100,
    Gfx1101,
    Gfx1102,
    Gfx1200,
    Gfx1201,
}

impl GfxVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Gfx942 => "gfx942",
            Self::Gfx950 => "gfx950",
            Self::Gfx1100 => "gfx1100",
            Self::Gfx1101 => "gfx1101",
            Self::Gfx1102 => "gfx1102",
            Self::Gfx1200 => "gfx1200",
            Self::Gfx1201 => "gfx1201",
        }
    }
}

impl fmt::Display for GfxVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for GfxVersion {
    type Err = String;

    /// Accepts inputs like `"gfx1100"`, `"GFX1100"`, `"gfx1100\n"`, or the
    /// extended form HIP's `hipDeviceGetName` sometimes returns
    /// (`"gfx1100:sramecc-:xnack-"`). Anything after the first `:` is
    /// stripped.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let head = s.trim().split(':').next().unwrap_or("");
        match head.to_lowercase().as_str() {
            "gfx942" => Ok(Self::Gfx942),
            "gfx950" => Ok(Self::Gfx950),
            "gfx1100" => Ok(Self::Gfx1100),
            "gfx1101" => Ok(Self::Gfx1101),
            "gfx1102" => Ok(Self::Gfx1102),
            "gfx1200" => Ok(Self::Gfx1200),
            "gfx1201" => Ok(Self::Gfx1201),
            _ => Err(format!(
                "Unknown or unsupported AMD GPU architecture: '{s}'"
            )),
        }
    }
}

/// Compile HIP C++ source to HSACO with default options.
///
/// `gfx_version` is forwarded to `--offload-arch`.
pub fn compile_hsaco<S: AsRef<str>>(
    src: S,
    gfx_version: GfxVersion,
) -> Result<Hsaco, CompileError> {
    compile_hsaco_with_opts(src, gfx_version, Default::default())
}

/// Compile HIP C++ source to HSACO with explicit options.
///
/// `gfx_version` populates `opts.arch` if it wasn't set explicitly.
///
/// # Example
/// ```no_run
/// use rocmrc::hiprtc::{compile_hsaco_with_opts, CompileOptions, GfxVersion};
/// let opts = CompileOptions {
///     opt_level: Some(3),
///     defines: vec!["NDEBUG".into()],
///     ..Default::default()
/// };
/// let hsaco = compile_hsaco_with_opts(
///     r#"extern "C" __global__ void kernel() {}"#,
///     GfxVersion::Gfx1100,
///     opts,
/// ).unwrap();
/// ```
pub fn compile_hsaco_with_opts<S: AsRef<str>>(
    src: S,
    gfx_version: GfxVersion,
    mut opts: CompileOptions,
) -> Result<Hsaco, CompileError> {
    if opts.arch.is_none() {
        opts.arch = Some(gfx_version.as_str().to_string());
    }
    // hipRTC's default header search path doesn't cover ROCm's own headers, so
    // kernels that `#include <hip/...>` (e.g. `hip_fp16.h` for `__half`) fail
    // with "file not found". Add `$ROCM_PATH/include` (default `/opt/rocm`)
    // unless the caller already supplied it.
    let rocm_include = {
        let root = std::env::var("ROCM_PATH").unwrap_or_else(|_| "/opt/rocm".to_string());
        format!("{root}/include")
    };
    if !opts.include_paths.contains(&rocm_include) {
        opts.include_paths.push(rocm_include);
    }
    let prog = Program::create(src, opts.name.as_deref())?;
    prog.compile(gfx_version, opts)
}

pub(crate) struct Program {
    prog: sys::hiprtcProgram,
    // Held to ensure storage outlives the hiprtcProgram.
    _src: CString,
    _name: Option<CString>,
}

impl Program {
    pub(crate) fn create<S: AsRef<str>>(src: S, name: Option<&str>) -> Result<Self, CompileError> {
        let src = CString::new(src.as_ref().as_bytes())
            .expect("program code cannot contain null terminators");
        let name =
            name.map(|s| CString::new(s).expect("program name cannot contain null terminators"));
        let prog =
            result::create_program(&src, name.as_deref()).map_err(CompileError::CreationError)?;
        Ok(Self {
            prog,
            _src: src,
            _name: name,
        })
    }

    pub(crate) fn compile(
        self,
        gfx_version: GfxVersion,
        opts: CompileOptions,
    ) -> Result<Hsaco, CompileError> {
        let options = opts.build();

        result::compile_program(self.prog, &options).map_err(|e| {
            let log_raw = result::get_program_log(self.prog).unwrap_or_default();
            let log_ptr = log_raw.as_ptr() as *const c_char;
            let log = unsafe { CStr::from_ptr(log_ptr) }.to_owned();
            CompileError::CompileError {
                hiprtc: e,
                options,
                log,
            }
        })?;

        let bytes_c = result::get_hsaco(self.prog).map_err(CompileError::GetHsacoError)?;
        // `result::get_hsaco` returns Vec<c_char>; HSACO is binary bytes.
        // c_char is i8 on most platforms — same bit pattern, just a
        // signedness flip. Zero-copy reinterpret.
        let bytes: Vec<u8> = unsafe {
            let mut v = std::mem::ManuallyDrop::new(bytes_c);
            Vec::from_raw_parts(v.as_mut_ptr() as *mut u8, v.len(), v.capacity())
        };
        Ok(Hsaco(HsacoKind::Binary(bytes), gfx_version))
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        let prog = std::mem::replace(&mut self.prog, std::ptr::null_mut());
        if !prog.is_null() {
            // Match cudarc — destroy errors during Drop should never
            // happen with a valid hiprtcProgram. Panic if they do.
            result::destroy_program(prog).unwrap();
        }
    }
}

/// Error from a hipRTC compilation pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileError {
    /// `hiprtcCreateProgram` failed.
    CreationError(HiprtcError),
    /// `hiprtcCompileProgram` failed. `log` is the diagnostic output
    /// retrieved via `hiprtcGetProgramLog`.
    CompileError {
        hiprtc: HiprtcError,
        options: Vec<String>,
        log: CString,
    },
    GetLogError(HiprtcError),
    GetHsacoError(HiprtcError),
    DestroyError(HiprtcError),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CompileError {}

/// Compile-time flags forwarded to `hiprtcCompileProgram`.
///
/// hipRTC takes clang/LLVM-style command-line arguments (unlike NVRTC's
/// PTX-specific flags). This struct surfaces the most common ones with
/// typed fields plus an escape hatch ([`Self::options`]) for raw flags.
///
/// # Example
/// ```
/// use rocmrc::hiprtc::CompileOptions;
/// let opts = CompileOptions {
///     opt_level: Some(2),
///     fast_math: Some(true),
///     defines: vec!["BATCH_SIZE=32".into()],
///     ..Default::default()
/// };
/// ```
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct CompileOptions {
    /// Optimization level (`0`–`3`). Maps to `-O{n}`.
    pub opt_level: Option<u8>,
    /// Enable / disable fast-math optimizations.
    /// Maps to `-ffast-math` / `-fno-fast-math`.
    pub fast_math: Option<bool>,
    /// Preprocessor macro definitions, each as `NAME` or `NAME=value`.
    /// Maps to `-D{def}`.
    pub defines: Vec<String>,
    /// Header search paths. Maps to `-I{path}`.
    pub include_paths: Vec<String>,
    /// Target GPU architecture (e.g. `"gfx1100"`). Maps to
    /// `--offload-arch={arch}`. Usually populated automatically by
    /// [`compile_hsaco_with_opts`] from the `gfx_version` argument; set
    /// here to override.
    pub arch: Option<String>,
    /// Source name used in diagnostics. Mirrors cudarc's `CompileOptions::name`.
    pub name: Option<String>,
    /// Raw additional flags appended verbatim. Use this for any hipRTC
    /// flag not surfaced as a typed field above.
    pub options: Vec<String>,
}

impl CompileOptions {
    pub(crate) fn build(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();

        if let Some(level) = self.opt_level {
            out.push(format!("-O{level}"));
        }

        match self.fast_math {
            Some(true) => out.push("-ffast-math".into()),
            Some(false) => out.push("-fno-fast-math".into()),
            None => {}
        }

        for def in &self.defines {
            out.push(format!("-D{def}"));
        }

        for path in &self.include_paths {
            out.push(format!("-I{path}"));
        }

        if let Some(arch) = &self.arch {
            out.push(format!("--offload-arch={arch}"));
        }

        for opt in &self.options {
            out.push(opt.clone());
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gfx_from_str_canonical() {
        assert_eq!(
            "gfx1100".parse::<GfxVersion>().unwrap(),
            GfxVersion::Gfx1100
        );
    }

    #[test]
    fn gfx_from_str_uppercase() {
        assert_eq!("GFX942".parse::<GfxVersion>().unwrap(), GfxVersion::Gfx942);
    }

    #[test]
    fn gfx_from_str_strips_feature_suffix() {
        // The form `hipDeviceGetName` sometimes returns.
        let parsed = "gfx1100:sramecc-:xnack-".parse::<GfxVersion>().unwrap();
        assert_eq!(parsed, GfxVersion::Gfx1100);
    }

    #[test]
    fn gfx_from_str_unknown_errs() {
        assert!("gfx9999".parse::<GfxVersion>().is_err());
    }

    #[test]
    fn compile_options_empty() {
        let opts = CompileOptions::default();
        assert!(opts.build().is_empty());
    }

    #[test]
    fn compile_options_opt_level() {
        let opts = CompileOptions {
            opt_level: Some(3),
            ..Default::default()
        };
        assert_eq!(opts.build(), vec!["-O3"]);
    }

    #[test]
    fn compile_options_fast_math() {
        let opts = CompileOptions {
            fast_math: Some(true),
            ..Default::default()
        };
        assert_eq!(opts.build(), vec!["-ffast-math"]);

        let opts = CompileOptions {
            fast_math: Some(false),
            ..Default::default()
        };
        assert_eq!(opts.build(), vec!["-fno-fast-math"]);
    }

    #[test]
    fn compile_options_multi() {
        let opts = CompileOptions {
            opt_level: Some(2),
            defines: vec!["NDEBUG".into(), "N=16".into()],
            include_paths: vec!["/opt/rocm/include".into()],
            arch: Some("gfx1100".into()),
            ..Default::default()
        };
        assert_eq!(
            opts.build(),
            vec![
                "-O2",
                "-DNDEBUG",
                "-DN=16",
                "-I/opt/rocm/include",
                "--offload-arch=gfx1100",
            ]
        );
    }
}
