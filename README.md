# rocmrc

Minimal safe ROCm bindings for Rust — the AMD analog of [`cudarc`](https://github.com/coreylowman/cudarc).

> **Status:** pre-release (`0.1`). Surface mirrors `cudarc` closely so call sites stay near-identical when porting from NVIDIA targets.
> **License:** dual `MIT OR Apache-2.0`.

## Why

ROCm exposes HIP, rocBLAS, and hipBLASLt — the AMD equivalents of CUDA, cuBLAS, and cuBLASLt. `cudarc` covers the NVIDIA side of the ML stack with idiomatic Rust wrappers; `rocmrc` is the matching set of wrappers for AMD GPUs. The goal is that a downstream crate (e.g. a ML runtime) can keep separate `#[cfg(feature = "cuda")]` / `#[cfg(feature = "rocm")]` backends whose code reads almost identically.

Design choices that mirror cudarc:

- Stream-bound handles (`RocblasHandle` owns an `Arc<HipStream>`, etc.).
- RAII descriptors for hipBLASLt (`MatrixLayout`, `MatmulDesc`, `MatmulPref`).
- Trait-based dispatch over scalar `T` for the BLAS surface: `Gemm<T>`, `Gemv<T>`, `Axpy<T>`, `Scal<T>`, `Nrm2<T>`, `Dot<T>`, `Copy<T>`.
- Descriptor pattern for fused matmul with bias / ReLU / GELU epilogues.

## Modules

| Module      | What it wraps                                                                | Feature gate |
| ----------- | ---------------------------------------------------------------------------- | ------------ |
| `driver`    | HIP runtime API (streams, memory, modules, kernel launch)                    | always-on    |
| `hiprtc`    | hipRTC (runtime compilation of HIP C++ → hsaco code objects)                 | always-on    |
| `rocblas`   | rocBLAS L1/L2/L3 via `Gemm<T>` / `Gemv<T>` / `Axpy<T>` / `Scal<T>` / ...     | `rocblas`    |
| `hipblaslt` | hipBLASLt fused matmul (bias / ReLU / GELU epilogues) via RAII descriptors   | `hipblaslt`  |

## Quickstart

A minimal HIP kernel compiled at runtime via hipRTC, loaded as a module, and launched on the default stream — see [`examples/vec_add.rs`](examples/vec_add.rs) for the full code:

```rust
use rocmrc::{HipContext, HipModule};

const SRC: &str = r#"
extern "C" __global__
void vec_add(float* out, const float* a, const float* b, int n) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) out[i] = a[i] + b[i];
}
"#;

let ctx = HipContext::new(0)?;
let (hsaco, _log) = rocmrc::hiprtc::compile(SRC, ctx.gfx_arch())?;
let module = HipModule::from_hsaco(hsaco.as_bytes())?;
let func = module.get_function("vec_add")?;
// ...allocate slices, copy data, launch, copy back...
```

Run it:

```sh
ROCM_PATH=/opt/rocm cargo run --example vec_add
```

## Feature flags

| Feature            | Purpose                                                                                                                  |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------ |
| `rocblas`          | Enable the rocBLAS module + link `librocblas` (or load it at runtime with `dynamic-loading`).                            |
| `hipblaslt`        | Enable the hipBLASLt module + link `libhipblaslt` (or load it at runtime with `dynamic-loading`).                        |
| `dynamic-loading`  | Resolve ROCm symbols at runtime via `libloading` instead of link-time. Skips the `-l` directives in `build.rs`.          |
| `rocm-XYYYY`       | Selects which ROCm patch the committed bindings target. Pick **exactly one** matching your installed ROCm runtime.       |

The `rocm-XYYYY` features encode `major.minor.patch` as `XX YY Z` (e.g. `7.2.1` → `rocm-07021`).

## ROCm version axis

The version axis follows what AMD actually publishes on `repo.radeon.com/rocm/apt/<ver>/ubuntu jammy main`. AMD does not publish `.0` patches on jammy, so the axis is patch-suffixed:

| Feature        | ROCm version |
| -------------- | ------------ |
| `rocm-06002`   | 6.0.2        |
| `rocm-06015`   | 6.1.5        |
| `rocm-06024`   | 6.2.4        |
| `rocm-06033`   | 6.3.3        |
| `rocm-06043`   | 6.4.3        |
| `rocm-07002`   | 7.0.2        |
| `rocm-07011`   | 7.1.1        |
| `rocm-07021`   | 7.2.1        |
| `rocm-07022`   | 7.2.2        |
| `rocm-07023`   | 7.2.3        |

Versions are added on demand as new ROCm releases are processed by the [bindings generator](bindings_generator/).

## Linking modes

**Static (default).** `build.rs` emits `cargo:rustc-link-lib=dylib=…` for each enabled module (`amdhip64`, `hiprtc`, and `rocblas` / `hipblaslt` when their features are on). Requires the matching `.so` files to be on the linker search path at build time — typically `/opt/rocm/lib`, which `build.rs` adds via `cargo:rustc-link-search=native=$ROCM_PATH/lib`.

**Dynamic (`--features dynamic-loading`).** `build.rs` skips the `-l` directives entirely; instead, each generated `sys/mod.rs` ships `rocmlib()` and `is_rocmlib_present()` helpers (powered by `libloading`) that resolve the ROCm libraries on first call. The search-path logic lives in [`get_lib_name_candidates`](src/lib.rs) — it tries `lib<name>.so` first (resolved via `LD_LIBRARY_PATH` / `ldconfig`), then versioned sonames, then `/opt/rocm/lib/` prefixes. Set `ROCM_PATH=/path/to/rocm` at runtime to point at a non-default install.

## Examples

```sh
# driver + hiprtc only — no rocm-* feature needed
cargo run --example vec_add

# rocBLAS GEMM + L1 round trip
cargo run --features rocblas,rocm-07021 --example sgemm

# hipBLASLt matmul with bias + ReLU epilogue
cargo run --features hipblaslt,rocm-07021 --example matmul_lt
```

The examples live in [`examples/`](examples/) and are kept in lock-step with the safe-wrapper API.

## Regenerating bindings

Bindings are committed in-tree under `src/<mod>/sys/mod.rs` so contributors don't need ROCm installed locally to build `rocmrc` (modulo the `rocm-*` feature gating). When AMD ships a new ROCm patch and you want to add it to the version axis, regenerate via the dockerized [bindings_generator](bindings_generator/):

```sh
docker buildx build -f bindings_generator/Dockerfile \
    -t rocmrc-gen:latest bindings_generator
docker run --rm --platform=linux/amd64 \
    -e CARGO_HOME=/workspace/.cargo-container \
    -e CARGO_TARGET_DIR=/tmp/target \
    -v $(pwd):/workspace \
    rocmrc-gen:latest
```

The container holds slices of every supported ROCm patch (rocBLAS, hipBLASLt, HIP, and a shared `rocm-llvm`), runs bindgen per version for the BLAS modules, and merges per-version outputs into a unified `mod.rs` with `#[cfg(feature = "rocm-XYYYY")]` gates where symbols changed. `bindings_generator/regen-bindings.sh` is the entrypoint; the merge logic lives in [`bindings_generator/src/merge.rs`](bindings_generator/src/merge.rs).

## License

Dual-licensed under either:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

## Acknowledgments

API shape, layout, and the dual-license arrangement follow [`cudarc`](https://github.com/chelsea0x3b/cudarc) by Chelsea Lowman and contributors. `rocmrc` exists so the Rust ML/GPU stack can target AMD with the same ergonomics.
