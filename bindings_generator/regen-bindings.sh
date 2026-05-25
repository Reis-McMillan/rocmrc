#!/bin/sh
# Run inside the rocmrc-gen image (see Dockerfile in this dir).
#
# Loops bindings_generator over every supported ROCm version for the BLAS
# modules (which is the whole point of the version axis), then regenerates
# driver + hiprtc once against the newest install (HIP runtime / hipRTC APIs
# are stable across ROCm releases so per-version is unnecessary).
#
# Outputs land via the bind-mounted workspace:
#   bindings_generator/out/<mod>/sys/linked/sys_<ver>.rs   (gitignored)
#   src/<mod>/sys/mod.rs                                    (committed)
#
# The container runs as root, which means everything we write under /workspace
# would land root-owned on the host. The trap at the bottom chowns the dirs we
# touched back to whatever UID/GID owns /workspace itself (i.e. the host user).

set -eu

VERSIONS="6.0.2 6.1.5 6.2.4 6.3.3 6.4.3 7.0.2 7.1.1 7.2.1 7.2.2 7.2.3"
LATEST="7.2.3"

# Capture the workspace owner once so we can hand outputs back at exit. EXIT
# trap (not just success) so even a failed bindgen leaves the partial outputs
# in the host user's hands instead of locked behind root ownership.
HOST_UID="$(stat -c '%u' /workspace)"
HOST_GID="$(stat -c '%g' /workspace)"
chown_back() {
    [ -d /workspace/bindings_generator/out ] && \
        chown -R "$HOST_UID:$HOST_GID" /workspace/bindings_generator/out
    [ -d /workspace/.cargo-container ] && \
        chown -R "$HOST_UID:$HOST_GID" /workspace/.cargo-container
    for mod in driver hiprtc rocblas hipblaslt; do
        [ -e "/workspace/src/$mod/sys/mod.rs" ] && \
            chown "$HOST_UID:$HOST_GID" "/workspace/src/$mod/sys/mod.rs"
    done
}
trap chown_back EXIT

# rocBLAS + hipBLASLt: per-version regeneration.
for ver in $VERSIONS; do
    rocm_path="/opt/rocm-$ver"
    if [ ! -d "$rocm_path" ]; then
        echo "SKIP: $rocm_path missing (was the version dropped from the Dockerfile?)" >&2
        continue
    fi
    echo "=== rocBLAS + hipBLASLt @ ROCm $ver ==="
    cargo run --quiet --release -p bindings_generator -- \
        --rocm-path  "$rocm_path" \
        --rocm-version "$ver" \
        --target rocblas
    cargo run --quiet --release -p bindings_generator -- \
        --rocm-path  "$rocm_path" \
        --rocm-version "$ver" \
        --target hipblaslt
done

# Driver + hiprtc: one-shot against the newest ROCm.
echo "=== driver + hiprtc @ ROCm $LATEST (one-shot) ==="
cargo run --quiet --release -p bindings_generator -- \
    --rocm-path  "/opt/rocm-$LATEST" \
    --rocm-version "$LATEST" \
    --target driver
cargo run --quiet --release -p bindings_generator -- \
    --rocm-path  "/opt/rocm-$LATEST" \
    --rocm-version "$LATEST" \
    --target hiprtc

echo "done."
