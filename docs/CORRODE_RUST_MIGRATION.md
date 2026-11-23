# Corrode Rust migration

This is an unofficial fork of Telegram for Android. The original Java/Kotlin UI, Telegram protocol client, C/C++ native code, and session storage remain upstream code; this branch starts the migration incrementally rather than claiming a Rust rewrite.

## First slice

Phone-number digit normalization at the `auth.sendCode` login entry point is implemented in the `corrode-native` Rust `cdylib` and called through JNI. Only the transient phone-number text crosses this boundary. Login codes, API hashes, 2FA passwords, session material, and message contents are not passed to this module. If the native normalizer rejects input, the existing Java `PhoneFormat.stripExceptNumbers` behavior remains the fallback.

The upstream Java login flow still provides phone entry, code entry, sign-up, and 2FA password verification. It reads the Corrode Telegram API credentials from `BuildVars.APP_ID` and `BuildVars.APP_HASH`; it does not send OTPs, passwords, or session state to Rust. A build with API ID `0` and an empty hash cannot sign in.

The Rust function has unit tests. The Android build task compiles `libcorrode_native.so` for the four ABIs the Telegram app currently packages and adds them from Gradle's generated JNI directory. `TMessagesProj` still needs its normal Android SDK/NDK and submodules for a complete app build.

## Build, caching, and credentials

The official source currently declares SDK 36 and NDK `27.2.12479018`. Install JDK 21, Rust stable with the Android targets listed in `rust-toolchain.toml`, and `cargo-ndk`:

```sh
cargo install --locked cargo-ndk
cargo test --manifest-path rust/Cargo.toml
./gradlew --no-daemon --build-cache :TMessagesProj_App:assembleAfatDebug
```

Gradle's local task-output cache is enabled with `org.gradle.caching=true`; CI uses `gradle/actions/setup-gradle` for Gradle dependencies/build state, an Actions cache for Cargo registry/git/target outputs, and a separate cache for `TMessagesProj/.cxx` CMake/Ninja intermediates. A first build or a cache miss still has to compile the full native app.

The Gradle build reads `CORRODE_API_ID` and `CORRODE_API_HASH` from project properties or the environment; it never falls back to Telegram's upstream values. For local login testing, obtain your own credentials for Corrode from [Telegram API development tools](https://my.telegram.org) and set them only in the local shell:

```sh
export CORRODE_API_ID="your_api_id"
export CORRODE_API_HASH="your_api_hash"
./gradlew --no-daemon --build-cache :TMessagesProj_App:assembleAfatDebug
```

For CI, add repository Actions secrets named `CORRODE_API_ID` and `CORRODE_API_HASH`. The workflow validates that both are present and well-formed, consumes them without printing their values, and builds the existing login flow. Without them, it warns and builds an installable UI/JNI test APK with login disabled.

**Credential disclosure:** the Telegram API ID/hash are compiled into the client APK and are recoverable by anyone who obtains that APK. A login-enabled build is therefore not a way to keep these app credentials secret. On push, CI does not upload a login-enabled APK. Upload requires a manual `workflow_dispatch` with `publish_login_apk=true`, an explicit choice because the resulting Actions artifact contains those embedded credentials. Never commit the raw values or print them in logs.

The Android SDK package list is passed to `android-actions/setup-android` as one space-separated string. The APK workflow is configured to cache Gradle, Rust/Cargo, and CMake/NDK state for repeated builds.

## Corrode app configuration

The APK uses a Corrode application ID and label and includes the supplied Corrode launcher icon in all five density buckets. It uses placeholder Firebase configuration and has Google auth, SafetyNet, billing, passkeys, maps credentials, and SMS Retriever hashes disabled or unconfigured. These are separate integrations; manual Telegram phone/code/2FA login uses the Corrode API credentials described above.

## License and identity

The upstream Android client is GPL-2.0. Keep the upstream license and copyright notices, mark modified upstream files with the Corrode change date, and publish the complete corresponding source when distributing builds. Corrode is unofficial; do not present it as the official Telegram app or use Telegram's standard logo as Corrode branding. Obtain and use Corrode's own Telegram API credentials before producing a login-enabled build.

## Sources

- Official Android source and build prerequisites: [TelegramMessenger/Telegram-Android README](https://github.com/TelegramMessenger/Telegram-Android).
- Upstream license text: [Telegram Android LICENSE](https://github.com/TelegramMessenger/Telegram-Android/blob/master/LICENSE).
- Telegram API credentials: [my.telegram.org](https://my.telegram.org).
- Android JNI output layout and supported cargo-ndk build flags: [cargo-ndk](https://github.com/bbqsrc/cargo-ndk).
- Telegram's reproducible-build notes: [core.telegram.org/reproducible-builds](https://core.telegram.org/reproducible-builds).
- GitHub Actions SDK setup and package-list input: [android-actions/setup-android](https://github.com/android-actions/setup-android).
