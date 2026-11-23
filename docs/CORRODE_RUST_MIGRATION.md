# Corrode Rust migration

This is an unofficial fork of Telegram for Android. The original Java/Kotlin UI, Telegram protocol client, C/C++ native code, and session storage remain upstream code; this branch starts the migration incrementally rather than claiming a Rust rewrite.

## First slice

Phone-number digit normalization at the `auth.sendCode` login entry point is implemented in the `corrode-native` Rust `cdylib` and called through JNI. Only the transient phone-number text crosses this boundary. Login codes, API hashes, 2FA passwords, session material, and message contents are not passed to this module. If the native normalizer rejects input, the existing Java `PhoneFormat.stripExceptNumbers` behavior remains the fallback.

The Rust function has unit tests. The Android build task compiles `libcorrode_native.so` for the four ABIs the Telegram app currently packages and adds them from Gradle's generated JNI directory. `TMessagesProj` still needs its normal Android SDK/NDK and submodules for a complete app build.

## Build and credentials

The official source currently declares SDK 36 and NDK `27.2.12479018`. Install Rust stable with the Android targets listed in `rust-toolchain.toml`, and `cargo-ndk`:

```sh
cargo install --locked cargo-ndk
cargo test --manifest-path rust/Cargo.toml
./gradlew :TMessagesProj_App:assembleAfatDebug
```

The Gradle `preBuild` task invokes `scripts/build-rust-android.sh` and fails clearly if `cargo-ndk` or the configured NDK is missing. Corrode does not reuse Telegram's API ID/hash: builds default to ID `0` and an empty hash. For a private local login test, set `CORRODE_API_ID` and `CORRODE_API_HASH` in the build environment to credentials obtained for Corrode. Do not commit or upload those credentials; the public CI APK is intentionally built without them and cannot sign in.

The GitHub Actions Android SDK package list is space-separated, as required by `android-actions/setup-android`; multiline YAML blocks and comma-separated package lists are passed as a single package and fail.

The APK uses a Corrode application ID and an explicitly unofficial app label, so it can be installed alongside the official stable app. It uses placeholder Firebase configuration and disables upstream Google auth, SafetyNet, billing, passkeys, map credentials, and SMS Retriever hashes. Consequently, sign-in, push notifications, Google sign-in, maps, and automatic SMS retrieval require Corrode-specific configuration. The temporary launcher icon is the generic Android icon; replace it with original Corrode branding before any public distribution.

## License and identity

The upstream Android client is GPL-2.0. Keep the upstream license and copyright notices, mark modified upstream files with the Corrode change date, and publish the complete corresponding source when distributing builds. Corrode is unofficial; do not present it as the official Telegram app or use Telegram's standard logo as Corrode branding. Obtain and use Corrode's own Telegram API credentials before producing a distributable build.

## Sources

- Official Android source and build prerequisites: [TelegramMessenger/Telegram-Android README](https://github.com/TelegramMessenger/Telegram-Android).
- Upstream license text: [Telegram Android LICENSE](https://github.com/TelegramMessenger/Telegram-Android/blob/master/LICENSE).
- Telegram API and branding requirements are stated in the official source README above.
- Android JNI output layout and supported cargo-ndk build flags: [cargo-ndk](https://github.com/bbqsrc/cargo-ndk).
- Telegram's reproducible-build notes: [core.telegram.org/reproducible-builds](https://core.telegram.org/reproducible-builds).
- GitHub Actions SDK setup and package-list input: [android-actions/setup-android](https://github.com/android-actions/setup-android).
