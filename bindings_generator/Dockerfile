# syntax=docker/dockerfile:1.7
#
# Image that holds the rocBLAS + hipBLASLt headers (plus matching HIP & LLVM
# slivers) for every ROCm version listed in `rocmrc`'s `rocm-*` cargo features.
# Used as the engine for `bindings_generator`: bind-mount the workspace, the
# entrypoint loops the generator across versions, the merged sys.rs files land
# back on the host via the mount.
#
# Build:
#   docker buildx build -f bindings_generator/Dockerfile \
#       -t rocmrc-gen:latest bindings_generator/
#
# Run:
#   docker run --rm \
#       --platform=linux/amd64 \
#       -e CARGO_HOME=/workspace/.cargo-container \
#       -e CARGO_TARGET_DIR=/tmp/target \
#       -v $(pwd):/workspace \
#       rocmrc-gen:latest
#
# Container runs as root; the entrypoint script chowns its outputs to the
# workspace owner before exiting so host-side git/edit operations Just Work.

FROM --platform=linux/amd64 ubuntu:22.04 AS final

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
        curl ca-certificates gnupg lsb-release \
        build-essential pkg-config g++ \
        dpkg-dev \
 && rm -rf /var/lib/apt/lists/*

# AMD signing key. Same key signs every per-version repo so we install it once.
RUN mkdir -p /etc/apt/keyrings \
 && curl -fsSL https://repo.radeon.com/rocm/rocm.gpg.key \
    | gpg --dearmor -o /etc/apt/keyrings/rocm.gpg

# Canonical install location for the shared rocm-llvm (clang/libclang).
# This is the version whose llvm tree everyone else symlinks to.
ARG SHARED_LLVM_VERSION="7.2.3"

# Install rocm-llvm ONCE from the canonical version. Saves ~15 GB of duplicate
# clang installs across the per-version loop. bindgen only cares about clang
# semantics (which are stable across recent LLVM versions), not whether the
# clang on disk came from the exact ROCm release whose headers we're parsing.
RUN set -eux; \
    echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/rocm.gpg] \
          https://repo.radeon.com/rocm/apt/$SHARED_LLVM_VERSION jammy main" \
        > /etc/apt/sources.list.d/rocm.list; \
    apt-get update; \
    mkdir -p /tmp/llvm-debs && cd /tmp/llvm-debs; \
    apt-get download rocm-llvm; \
    for deb in *.deb; do \
        dpkg-deb -x "$deb" /; \
    done; \
    cd / && rm -rf /tmp/llvm-debs; \
    if [ -d "/opt/rocm-$SHARED_LLVM_VERSION/lib/llvm" ] \
        && [ ! -e "/opt/rocm-$SHARED_LLVM_VERSION/llvm" ]; then \
        ln -sfn "lib/llvm" "/opt/rocm-$SHARED_LLVM_VERSION/llvm"; \
    fi; \
    rm /etc/apt/sources.list.d/rocm.list; \
    rm -rf /var/lib/apt/lists/*

# Per-version slice install. AMD's package names collide across version repos
# (every repo provides a `rocblas` named the same), so `apt install` of a
# second version refuses or upgrades. We sidestep apt's package db entirely:
# `apt-get download` fetches the .debs, then `dpkg-deb -x` unpacks them into
# per-version /opt/rocm-X.Y.Z trees (which is the layout the .debs ship with).
ARG ROCM_VERSIONS="6.0.2 6.1.5 6.2.4 6.3.3 6.4.3 7.0.2 7.1.1 7.2.1 7.2.2 7.2.3"
# hipblas-dev and hipblas-common-dev are complementary across the 6.x → 7.x
# split: older versions ship `hipblas/hipblas.h` from hipblas-dev; 6.4+ moved
# the public type defs into hipblas-common-dev. hipblaslt.h transitively
# includes whichever path its version expects, so we list both and let the
# WARN-skip take care of the version where one's absent.
ARG ROCBLAS_PKGS="rocblas-dev hipblas-dev hipblas-common-dev hipblaslt-dev \
                  hip-runtime-amd hip-dev rocm-core"
RUN set -eux; \
    for ver in $ROCM_VERSIONS; do \
        echo "=== ROCm $ver ==="; \
        echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/rocm.gpg] \
              https://repo.radeon.com/rocm/apt/$ver jammy main" \
            > /etc/apt/sources.list.d/rocm.list; \
        apt-get update; \
        mkdir -p /tmp/debs-$ver; \
        cd /tmp/debs-$ver; \
        for pkg in $ROCBLAS_PKGS; do \
            apt-get download "$pkg" 2>/dev/null \
                || echo "WARN: $pkg not in $ver repo (skipping)"; \
        done; \
        for deb in *.deb; do \
            dpkg-deb -x "$deb" /; \
        done; \
        cd / && rm -rf /tmp/debs-$ver; \
    done; \
    for ver in $ROCM_VERSIONS; do \
        if [ "$ver" != "$SHARED_LLVM_VERSION" ]; then \
            mkdir -p "/opt/rocm-$ver"; \
            ln -sfn "/opt/rocm-$SHARED_LLVM_VERSION/llvm" "/opt/rocm-$ver/llvm"; \
        fi; \
    done; \
    rm /etc/apt/sources.list.d/rocm.list; \
    rm -rf /var/lib/apt/lists/*

# Rust toolchain. `--profile default` includes rustfmt, which merge.rs invokes
# to format the unified bindings file. Don't drop to --profile minimal here.
#
# The container runs as root (the default). Files written into the bind-mounted
# workspace would normally land root-owned on the host; the entrypoint script
# chowns them back to the workspace owner before exiting so the host user
# can git add / edit them without sudo.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --profile default --default-toolchain stable
ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /workspace

COPY regen-bindings.sh /usr/local/bin/regen-bindings.sh
RUN chmod +x /usr/local/bin/regen-bindings.sh

ENTRYPOINT ["/usr/local/bin/regen-bindings.sh"]
