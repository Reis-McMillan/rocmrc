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

    // Expose the selected ROCm version to the crate at compile time. Reads:
    //   env!("ROCM_MAJOR_VERSION") / env!("ROCM_MINOR_VERSION")
    // …from get_lib_name_candidates() and any other version-sensitive code.
    if let Some((major, minor)) = rocm_version_from_features() {
        println!("cargo:rustc-env=ROCM_MAJOR_VERSION={major}");
        println!("cargo:rustc-env=ROCM_MINOR_VERSION={minor}");
    } else if dynamic {
        // dynamic-loading enabled without a rocm-* feature: env!() would fail
        // at expand time, so emit zeros and rely on the soname fallbacks.
        println!("cargo:rustc-env=ROCM_MAJOR_VERSION=0");
        println!("cargo:rustc-env=ROCM_MINOR_VERSION=0");
    }

    println!("cargo:rerun-if-env-changed=ROCM_PATH");
}

/// Map the selected `rocm-XYYY` cargo feature to its (major, minor) version
/// pair. Cargo doesn't give us a "which feature in this set is on" query, so
/// we enumerate. Highest-version-first so that if multiple are accidentally
/// enabled we still pick the newest.
fn rocm_version_from_features() -> Option<(u32, u32)> {
    let probes: &[(&str, u32, u32)] = &[
        ("ROCM_07023", 7, 2),
        ("ROCM_07022", 7, 2),
        ("ROCM_07021", 7, 2),
        ("ROCM_07011", 7, 1),
        ("ROCM_07002", 7, 0),
        ("ROCM_06043", 6, 4),
        ("ROCM_06033", 6, 3),
        ("ROCM_06024", 6, 2),
        ("ROCM_06015", 6, 1),
        ("ROCM_06002", 6, 0),
    ];
    for (name, major, minor) in probes {
        if std::env::var_os(format!("CARGO_FEATURE_{name}")).is_some() {
            return Some((*major, *minor));
        }
    }
    None
}
