const KNOWN_ROCM_VERSIONS: &[&str] = &[
    "06002", "06015", "06024", "06033", "06043", "07002", "07011", "07020", "07021", "07022",
    "07023", "07024",
];

fn main() {
    let rocm_path = std::env::var("ROCM_PATH").unwrap_or_else(|_| "/opt/rocm".to_string());

    println!("cargo:rustc-link-search=native={rocm_path}/lib");

    // When `dynamic-loading` is on, symbols are resolved at runtime via
    // libloading. Skip the `-l` directives so we don't *also* statically pull
    // them in at link time (that would defeat the point of dynamic loading
    // and risk double-init / version mismatch).
    let dynamic = std::env::var_os("CARGO_FEATURE_DYNAMIC_LOADING").is_some();
    if !dynamic {
        println!("cargo:rustc-link-lib=dylib=amdhip64");
        println!("cargo:rustc-link-lib=dylib=hiprtc");

        if std::env::var_os("CARGO_FEATURE_ROCBLAS").is_some() {
            println!("cargo:rustc-link-lib=dylib=rocblas");
        }
        if std::env::var_os("CARGO_FEATURE_HIPBLASLT").is_some() {
            println!("cargo:rustc-link-lib=dylib=hipblaslt");
        }
    }

    let values = KNOWN_ROCM_VERSIONS
        .iter()
        .map(|c| format!("\"{c}\""))
        .collect::<Vec<_>>()
        .join(", ");
    println!("cargo:rustc-check-cfg=cfg(rocm_ver, values({values}))");

    let probe_build_system =
        std::env::var_os("CARGO_FEATURE_ROCM_VERSION_FROM_BUILD_SYSTEM").is_some();

    if let Some(code) = rocm_version_from_features().or_else(|| {
        if probe_build_system {
            rocm_version_from_build_system()
                .and_then(|(major, minor, patch)| nearest_binding_within_minor(major, minor, patch))
        } else {
            None
        }
    }) {
        let (major, minor, _patch) = parse_version_code(&code);
        println!("cargo:rustc-cfg=rocm_ver=\"{code}\"");
        println!("cargo:rustc-env=ROCM_MAJOR_VERSION={major}");
        println!("cargo:rustc-env=ROCM_MINOR_VERSION={minor}");
    } else if dynamic {
        // dynamic-loading enabled without a pinned/detected version: env!()
        // would fail at expand time, so emit zeros and rely on the unversioned
        // soname fallbacks. (rocblas/hipblaslt still won't compile without a
        // version — their `sys` bindings are gated on `rocm_ver`.)
        println!("cargo:rustc-env=ROCM_MAJOR_VERSION=0");
        println!("cargo:rustc-env=ROCM_MINOR_VERSION=0");
    }

    println!("cargo:rerun-if-env-changed=ROCM_PATH");
}

fn parse_version_code(code: &str) -> (u32, u32, u32) {
    let major = code[0..2].parse().expect("major digits");
    let minor = code[2..4].parse().expect("minor digits");
    let patch = code[4..].parse().expect("patch digits");
    (major, minor, patch)
}

fn rocm_version_from_features() -> Option<String> {
    for code in KNOWN_ROCM_VERSIONS.iter().rev() {
        if std::env::var_os(format!("CARGO_FEATURE_ROCM_{code}")).is_some() {
            return Some((*code).to_string());
        }
    }
    None
}

fn rocm_version_from_build_system() -> Option<(u32, u32, u32)> {
    let output = std::process::Command::new("amd-smi")
        .arg("version")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    // The version line may land on stdout or stderr depending on the build;
    // scan both.
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let version = text
        .split("ROCm version:")
        .nth(1)?
        .split_whitespace()
        .next()?;
    let mut parts = version.split('.');
    let major = parts.next()?.trim().parse::<u32>().ok()?;
    let minor = parts.next()?.trim().parse::<u32>().ok()?;
    let patch = parts.next()?.trim().parse::<u32>().ok()?;
    Some((major, minor, patch))
}

/// Map a detected `(major, minor, patch)` onto a committed binding code:
/// exact patch if we have it, otherwise the newest committed patch sharing the
/// same `major.minor` (the C API is patch-stable within a minor). `None` if no
/// binding exists for that `major.minor` at all.
fn nearest_binding_within_minor(major: u32, minor: u32, patch: u32) -> Option<String> {
    let mut best: Option<(u32, &str)> = None;
    let mut exact: Option<&str> = None;
    for code in KNOWN_ROCM_VERSIONS {
        let (cmaj, cmin, cpat) = parse_version_code(code);
        if cmaj == major && cmin == minor {
            if cpat == patch {
                exact = Some(code);
            }
            if best.is_none_or(|(bp, _)| cpat > bp) {
                best = Some((cpat, code));
            }
        }
    }
    exact.or(best.map(|(_, c)| c)).map(str::to_string)
}
