use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result};
use bindgen::Builder;

use crate::version::Version;

/// Per-module knobs for bindgen + linking. Equivalent of cudarc's `ModuleConfig`,
/// refit for ROCm's "headers live in a local install" model.
#[derive(Debug)]
#[allow(dead_code)]
pub struct ModuleConfig {
    /// Module directory under `src/` (e.g. `"hip"`, `"hiprtc"`, `"rocblas"`, `"hipblaslt"`).
    pub rocmrc_name: &'static str,
    /// Subdirs of `<rocm_path>/include` that bindgen should add to `-I`.
    /// Most modules just need `""`. Empty string = the include root itself.
    pub include_subpaths: Vec<&'static str>,
    pub allowlist: Filters,
    pub blocklist: Filters,
    /// Dynamic libraries this module links against. Tracked here so build.rs
    /// can iterate; bindings_generator itself doesn't link anything.
    pub libs: Vec<&'static str>,
    pub clang_args: Vec<&'static str>,
    pub allowlist_recursively: bool,
    pub raw_lines: Vec<&'static str>,
    pub min_rocm_version: Option<Version>,
    /// Other modules whose include dirs must be on the clang path. For our
    /// current local-install pipeline this is informational, since every module
    /// already sees `<rocm_path>/include`. Kept for parity with cudarc and for
    /// future source variants.
    pub module_dependencies: Vec<&'static str>,
    pub feature_prefix: &'static str,
    pub bitflag_enums: Vec<&'static str>,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            rocmrc_name: "",
            include_subpaths: vec![""],
            allowlist: Filters::none(),
            blocklist: Filters::none(),
            libs: vec![],
            clang_args: vec![],
            allowlist_recursively: true,
            raw_lines: vec![],
            min_rocm_version: None,
            module_dependencies: vec![],
            feature_prefix: "rocm",
            bitflag_enums: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Filters {
    pub types: Vec<&'static str>,
    pub functions: Vec<&'static str>,
    pub vars: Vec<&'static str>,
}

impl Filters {
    pub fn none() -> Self {
        Self {
            types: vec![],
            functions: vec![],
            vars: vec![],
        }
    }
}

impl ModuleConfig {
    pub fn supports_rocm_version(&self, v: Version) -> bool {
        match self.min_rocm_version {
            None => true,
            Some(min) => v >= min,
        }
    }

    /// Run bindgen against headers in `<rocm_path>/include` for this module.
    ///
    /// Reads the hand-written wrapper at
    ///   `<workspace_root>/src/<rocmrc_name>/sys/wrapper.h`
    /// and writes the generated bindings to
    ///   `<generator_root>/out/<rocmrc_name>/sys/linked/sys_<version>.rs`
    /// (gitignored). `merge.rs` then consumes that file.
    pub fn run_bindgen(&self, rocm_path: &Path, version: Version) -> Result<()> {
        let wrapper_h = workspace_root()
            .join("src")
            .join(self.rocmrc_name)
            .join("sys")
            .join("wrapper.h");
        if !wrapper_h.exists() {
            anyhow::bail!(
                "missing wrapper.h for {}: expected at {}",
                self.rocmrc_name,
                wrapper_h.display()
            );
        }

        let linked_dir = generator_root()
            .join("out")
            .join(self.rocmrc_name)
            .join("sys")
            .join("linked");
        fs::create_dir_all(&linked_dir)
            .with_context(|| format!("creating {}", linked_dir.display()))?;

        let resource_dir = clang_resource_dir(rocm_path)?;
        let include_root = rocm_path.join("include");

        let mut builder = Builder::default()
            .default_enum_style(bindgen::EnumVariation::Rust {
                non_exhaustive: false,
            })
            .derive_default(false)
            .derive_eq(true)
            .derive_hash(true)
            .derive_ord(true)
            .generate_comments(false)
            .layout_tests(false)
            .use_core()
            .allowlist_recursively(self.allowlist_recursively)
            .header(wrapper_h.to_string_lossy())
            .clang_arg("-D__HIP_PLATFORM_AMD__")
            .clang_arg(format!("-resource-dir={}", resource_dir.display()));

        for sub in &self.include_subpaths {
            let p = if sub.is_empty() {
                include_root.clone()
            } else {
                include_root.join(sub)
            };
            builder = builder.clang_arg(format!("-I{}", p.display()));
        }

        for arg in &self.clang_args {
            builder = builder.clang_arg(*arg);
        }

        for pat in &self.allowlist.types {
            builder = builder.allowlist_type(*pat);
        }
        for pat in &self.allowlist.functions {
            builder = builder.allowlist_function(*pat);
        }
        for pat in &self.allowlist.vars {
            builder = builder.allowlist_var(*pat);
        }
        for pat in &self.blocklist.types {
            builder = builder.blocklist_type(*pat);
        }
        for pat in &self.blocklist.functions {
            builder = builder.blocklist_function(*pat);
        }
        for pat in &self.blocklist.vars {
            builder = builder.blocklist_var(*pat);
        }
        for line in &self.raw_lines {
            builder = builder.raw_line(*line);
        }
        for n in &self.bitflag_enums {
            builder = builder.bitfield_enum(*n);
        }

        let bindings = builder
            .generate()
            .with_context(|| format!("bindgen failed for {}", wrapper_h.display()))?;

        let out = linked_dir.join(format!("sys_{version}.rs"));
        bindings
            .write_to_file(&out)
            .with_context(|| format!("writing {}", out.display()))?;
        log::info!("wrote {}", out.display());
        Ok(())
    }
}

/// The rocmrc crate root (parent of this tool's manifest dir). Where the
/// hand-written `wrapper.h` files live, and where the merged `mod.rs` files
/// land after `merge.rs` runs.
fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("bindings_generator manifest must have a parent")
        .to_path_buf()
}

/// This tool's own manifest dir. Holds the gitignored `out/` tree where
/// bindgen drops per-version files for `merge.rs` to consume.
fn generator_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// `<rocm_path>/llvm/bin/clang --print-resource-dir`
fn clang_resource_dir(rocm_path: &Path) -> Result<PathBuf> {
    let clang = rocm_path.join("llvm").join("bin").join("clang");
    let out = Command::new(&clang)
        .arg("--print-resource-dir")
        .output()
        .with_context(|| format!("invoking {}", clang.display()))?;
    if !out.status.success() {
        anyhow::bail!(
            "{} --print-resource-dir failed: {}",
            clang.display(),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let dir = String::from_utf8(out.stdout)
        .context("clang resource-dir output not utf8")?
        .trim()
        .to_string();
    anyhow::ensure!(!dir.is_empty(), "clang --print-resource-dir returned empty");
    Ok(PathBuf::from(dir))
}

/// Initial four modules. Filters are first-cut; tune as bindgen output is reviewed.
pub fn create_modules() -> Vec<ModuleConfig> {
    vec![
        ModuleConfig {
            rocmrc_name: "hip",
            allowlist: Filters {
                functions: vec!["^hip.*"],
                types: vec!["^hip.*", "^HIP.*"],
                vars: vec!["^(hip|HIP)_.*"],
            },
            libs: vec!["amdhip64"],
            ..Default::default()
        },
        ModuleConfig {
            rocmrc_name: "hiprtc",
            allowlist: Filters {
                functions: vec!["^hiprtc.*"],
                types: vec!["^hiprtc.*"],
                vars: vec!["^HIPRTC.*"],
            },
            libs: vec!["hiprtc"],
            module_dependencies: vec!["hip"],
            ..Default::default()
        },
        ModuleConfig {
            rocmrc_name: "rocblas",
            allowlist: Filters {
                functions: vec!["^rocblas_.*"],
                types: vec!["^rocblas_.*"],
                vars: vec!["^(rocblas|ROCBLAS)_.*"],
            },
            libs: vec!["rocblas"],
            module_dependencies: vec!["hip"],
            ..Default::default()
        },
        ModuleConfig {
            rocmrc_name: "hipblaslt",
            allowlist: Filters {
                functions: vec!["^hipblasLt.*"],
                types: vec!["^hipblas(Lt)?.*"],
                vars: vec!["^HIPBLASLT?_.*"],
            },
            // hipblaslt.h transitively #include's C++ headers (e.g. <memory>),
            // so the wrapper must be parsed as C++.
            clang_args: vec!["-x", "c++"],
            libs: vec!["hipblaslt"],
            module_dependencies: vec!["hip"],
            ..Default::default()
        },
    ]
}
