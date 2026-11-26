#!/usr/bin/env bash
set -euo pipefail

export ANDROID_HOME="${ANDROID_HOME:-/Users/mac/Library/Android/sdk}"
export ANDROID_NDK_HOME="${ANDROID_NDK_HOME:-$ANDROID_HOME/ndk/28.2.13676358}"
TOOLCHAIN="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin"

export CC_aarch64_linux_android="$TOOLCHAIN/aarch64-linux-android26-clang"
export AR_aarch64_linux_android="$TOOLCHAIN/llvm-ar"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$TOOLCHAIN/aarch64-linux-android26-clang"

export CC_x86_64_linux_android="$TOOLCHAIN/x86_64-linux-android26-clang"
export AR_x86_64_linux_android="$TOOLCHAIN/llvm-ar"
export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="$TOOLCHAIN/x86_64-linux-android26-clang"

echo "==> 1. Building libbaca_core.so for arm64-v8a & x86_64..."
cargo build -p baca-core --release --target aarch64-linux-android
cargo build -p baca-core --release --target x86_64-linux-android

JNI_DIR="android-native/app/src/main/jniLibs"
mkdir -p "$JNI_DIR/arm64-v8a" "$JNI_DIR/x86_64"
cp target/aarch64-linux-android/release/libbaca_core.so "$JNI_DIR/arm64-v8a/"
cp target/x86_64-linux-android/release/libbaca_core.so "$JNI_DIR/x86_64/"

echo "==> 2. Generating Kotlin UniFFI bindings..."
cargo build -p baca-core
cargo run -p baca-core --bin uniffi-bindgen -- generate \
  --library target/debug/libbaca_core.dylib \
  --language kotlin \
  --out-dir android-native/app/src/main/kotlin

echo "==> 3. Building Native Android APK..."
GRADLE_BIN=$(ls -d ~/.gradle/wrapper/dists/gradle-8.14-all/*/gradle-8.14/bin/gradle | head -n 1)
(cd android-native && "$GRADLE_BIN" assembleDebug)

echo "==> Done! APK at android-native/app/build/outputs/apk/debug/app-debug.apk"
