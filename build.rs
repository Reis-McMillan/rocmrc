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

    let bindings = bindgen::Builder::default()
        .header("src/driver/sys/wrapper.h")
        .clang_arg(format!("-I{rocm_path}/include"))
        .allowlist_function("hip.*")
        .allowlist_type("hip.*")
        .allowlist_var("(hip|HIP)_.*")
        .default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: false, is_global: false })
        .generate()
        .expect("bindgen");

    bindings
        .write_to_file("src/driver/sys/sys.rs")
        .expect("write bindings");
}
