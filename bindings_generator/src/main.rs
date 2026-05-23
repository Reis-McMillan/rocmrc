use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use rayon::prelude::*;

mod merge;
mod module;
mod version;

use crate::{module::create_modules, version::Version};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// ROCm install root containing `include/`, `llvm/bin/clang`, etc.
    #[arg(long, env = "ROCM_PATH", default_value = "/opt/rocm")]
    rocm_path: PathBuf,

    /// ROCm release version this install corresponds to (e.g. "6.3.0").
    #[arg(long)]
    rocm_version: Version,

    /// Generate only the named target (e.g. "rocblas"). Substring match.
    #[arg(long)]
    target: Option<String>,

    /// Skip per-version bindgen; just re-run merge.
    #[arg(long)]
    skip_bindings: bool,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    let mut modules = create_modules();
    if let Some(t) = &args.target {
        modules.retain(|m| m.rocmrc_name.contains(t));
        anyhow::ensure!(!modules.is_empty(), "no modules match target {t:?}");
    }

    modules.retain(|m| m.supports_rocm_version(args.rocm_version));

    if !args.skip_bindings {
        modules
            .par_iter()
            .try_for_each(|m| m.run_bindgen(&args.rocm_path, args.rocm_version))?;
    }
    merge::merge_bindings(&modules)?;
    Ok(())
}
