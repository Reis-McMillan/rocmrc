fn main() {
    let rocm_path = std::env::var("ROCM_PATH").unwrap_or_else(|_| "/opt/rocm".to_string());
    if std::env::var_os("LIBCLANG_PATH").is_none() {
        unsafe { std::env::set_var("LIBCLANG_PATH", format!("{rocm_path}/llvm/lib")); }
    }
    
    println!("cargo:rustc-link-search=native={rocm_path}/lib");
    println!("cargo:rustc-link-lib=dylib=amdhip64");
    println!("cargo:rustc-link-lib=dylib=hiprtc");
    println!("cargo:rerun-if-env-changed=ROCM_PATH");
    println!("cargo:rerun-if-changed=src/driver/sys/wrapper.h");

    let resource_dir = std::process::Command::new(format!("{rocm_path}/llvm/bin/clang"))
        .arg("--print-resource-dir")
        .output()
        .expect("failed to invoke ROCm clang");

    let rd = String::from_utf8(resource_dir.stdout)
        .expect("clang resource-dir not utf8")
        .trim()
        .to_string();

    assert!(!rd.is_empty(), "clang --print-resource-dir returned empty");

    let bindings = bindgen::Builder::default()
        .header("src/driver/sys/wrapper.h")
        .clang_arg(format!("-I{rocm_path}/include"))
        .clang_arg(format!("-resource-dir={rd}"))
        .allowlist_function("hip.*")
        .allowlist_type("hip.*")
        .allowlist_var("(hip|HIP)_.*")
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        .generate()
        .expect("bindgen");
        bindings
            .write_to_file("src/driver/sys/sys.rs")
            .expect("write bindings");
    }
