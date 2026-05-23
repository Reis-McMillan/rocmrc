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
        .arg("--print-resource_dir")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string());

    let mut builder = bindgen::Builder::default()
        .header("src/driver/sys/wrapper.h")
        .clang_arg(format!("-I{rocm_path}/include"));
    
    if let Some(rd) = resource_dir {
        builder = builder.clang_arg(format!("-resource-dir={rd}"));
    }

    let bindings = builder
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
