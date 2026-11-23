#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${1:?Usage: scripts/build-rust-android.sh <jniLibs-output-dir>}"

if ! cargo ndk --version >/dev/null 2>&1; then
    echo "cargo-ndk is required. Install it with: cargo install --locked cargo-ndk" >&2
    exit 127
fi

mkdir -p "$OUTPUT_DIR"
cd "$ROOT_DIR/rust"
cargo ndk \
    -t armeabi-v7a \
    -t arm64-v8a \
    -t x86 \
    -t x86_64 \
    -o "$OUTPUT_DIR" \
    build --package corrode-native --release

for abi in armeabi-v7a arm64-v8a x86 x86_64; do
    library="$OUTPUT_DIR/$abi/libcorrode_native.so"
    if [[ ! -s "$library" ]]; then
        echo "Rust build did not produce $library" >&2
        exit 1
    fi
done
