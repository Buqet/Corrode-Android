# Corrode Rust migration

This is an unofficial fork of Telegram for Android. The original Java/Kotlin UI, Telegram protocol client, C/C++ native code, and session storage remain upstream code; this branch starts the migration incrementally rather than claiming a Rust rewrite.

## First slice

Phone-number digit normalization at the `auth.sendCode` login entry point is implemented in the `corrode-native` Rust `cdylib` and called through JNI. Only the transient phone-number text crosses this boundary. Login codes, API hashes, 2FA passwords, session material, and message contents are not passed to this module. If the native normalizer rejects input, the existing Java `PhoneFormat.stripExceptNumbers` behavior remains the fallback.

The Rust function has unit tests. The Android build task compiles `libcorrode_native.so` for the four ABIs the Telegram app currently packages and adds them from Gradle's generated JNI directory. `TMessagesProj` still needs its normal Android SDK/NDK and submodules for a complete app build.

## Build prerequisites

Install the Android SDK/NDK required by this source tree (SDK 36 and NDK `27.2.12479018`), Rust stable with the Android targets listed in `rust-toolchain.toml`, and `cargo-ndk`:

```sh
cargo install --locked cargo-ndk
cargo test --manifest-path rust/Cargo.toml
./gradlew :TMessagesProj_App:assembleAfatDebug
```

The Gradle `preBuild` task invokes `scripts/build-rust-android.sh` and fails clearly if `cargo-ndk` or the configured NDK is missing.

## License and identity

The upstream Android client is GPL-2.0. Keep the upstream license and copyright notices, mark modified upstream files with the Corrode change date, and publish the complete corresponding source when distributing builds. Corrode is unofficial; do not present it as the official Telegram app or use Telegram's standard logo as Corrode branding. Obtain and use Corrode's own Telegram API credentials before producing a distributable build.
