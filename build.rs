fn main() {
    let rocm_path = std::env::var("ROCM_PATH").unwrap_or_else(|_| "/opt/rocm".to_string());

    println!("cargo:rustc-link-search=native={rocm_path}/lib");
    println!("cargo:rustc-link-lib=dylib=amdhip64");
    println!("cargo:rustc-link-lib=dylib=hiprtc");

    if std::env::var_os("CARGO_FEATURE_ROCBLAS").is_some() {
        println!("cargo:rustc-link-lib=dylib=rocblas");
    }
    if std::env::var_os("CARGO_FEATURE_HIPBLASLT").is_some() {
        println!("cargo:rustc-link-lib=dylib=hipblaslt");
    }

    println!("cargo:rerun-if-env-changed=ROCM_PATH");
}
