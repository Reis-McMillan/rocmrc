//! hipRTC: runtime compilation of HIP C++ source into hsaco code objects.
//!
//! Analogue of `cudarc::nvrtc`. `Hsaco` corresponds to `cudarc::nvrtc::Ptx`.

pub mod result;
pub mod sys;

#[derive(Debug, thiserror::Error)]
pub enum HiprtcError {
    #[error("hipRTC error code {0:?}")]
    Hiprtc(sys::hiprtcResult),
    #[error("source contained an interior null byte")]
    NullInSource,
    #[error("compile option contained an interior null byte")]
    NullInOption,
}

/// Compiled code-object blob (hsaco). Loadable via `HipModule::from_hsaco`.
/// Analogue of `cudarc::nvrtc::Ptx`.
pub struct Hsaco(pub Vec<u8>);

impl Hsaco {
    pub fn from_bytes(b: Vec<u8>) -> Self {
        Self(b)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Compile HIP C++ `src` for `gfx_arch` (e.g. `"gfx1100"`).
///
/// Returns the code object and any compiler log that was produced.
/// On compile failure, the log is included in the returned error message via
/// `Debug`; surface it for the user.
pub fn compile(src: &str, gfx_arch: &str) -> Result<(Hsaco, String), HiprtcError> {
    let prog = result::create_program(src, "kernel.hip")?;
    let arch_opt = format!("--offload-arch={gfx_arch}");
    let opts = [arch_opt.as_str()];
    let compile_res = result::compile_program(prog, &opts);
    let log = result::get_program_log(prog).unwrap_or_default();
    if let Err(e) = compile_res {
        let _ = result::destroy_program(prog);
        eprintln!("hipRTC compile failed; log:\n{log}");
        return Err(e);
    }
    let code = result::get_code(prog)?;
    let _ = result::destroy_program(prog);
    Ok((Hsaco(code), log))
}
