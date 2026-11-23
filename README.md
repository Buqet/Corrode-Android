<div align="center">
  <img src="docs/assets/corrode-icon.png" alt="Иконка Corrode" width="144" />
  <h1>Corrode для Android</h1>
  <p><strong>Неофициальный Android-клиент Telegram с постепенным переносом логики на Rust.</strong></p>
  <p>
    <a href="https://github.com/Buqet/Corrode-Android/actions/workflows/corrode-rust-android.yml?query=branch%3Acorrode%2Frust-phone-normalizer"><img src="https://github.com/Buqet/Corrode-Android/actions/workflows/corrode-rust-android.yml/badge.svg?branch=corrode%2Frust-phone-normalizer" alt="Rust CI" /></a>
    <a href="https://github.com/Buqet/Corrode-Android/actions/workflows/corrode-android-apk.yml?query=branch%3Acorrode%2Frust-phone-normalizer"><img src="https://github.com/Buqet/Corrode-Android/actions/workflows/corrode-android-apk.yml/badge.svg?branch=corrode%2Frust-phone-normalizer" alt="Android APK CI" /></a>
  </p>
</div>

> Corrode не является официальным приложением Telegram и не связан с Telegram FZ-LLC. Это отдельный экспериментальный форк; название, package ID и тестовая иконка Corrode отличаются от официального приложения.

## О проекте

Corrode основан на [открытом исходном коде Telegram для Android](https://github.com/TelegramMessenger/Telegram-Android). Вместо рискованной одномоментной переписи проект переносит небольшие, изолированные части постепенно: существующие экраны, сетевой протокол и остальную Android-интеграцию пока сохраняет upstream-код.

### Уже работает

- Нормализация номера телефона перед `auth.sendCode` выполняется Rust-модулем через JNI.
- При неподдерживаемом вводе остаётся Java fallback.
- Для Rust-среза есть unit-тесты; Android workflow собирает native-библиотеки для `armeabi-v7a`, `arm64-v8a`, `x86` и `x86_64`.
- APK помечен как Corrode и использует отдельный application ID `com.buqet.corrode.beta`.

Это пока не переписанный на Rust мессенджер: UI, Telegram API/MTProto-клиент, обработка сообщений и основная логика остаются на исходных Java/Kotlin/C++ компонентах.

## Скачать тестовый APK

Откройте [последний запуск сборки APK в GitHub Actions](https://github.com/Buqet/Corrode-Android/actions/workflows/corrode-android-apk.yml?query=branch%3Acorrode%2Frust-phone-normalizer), выберите успешный run и скачайте artifact `corrode-android-afat-debug`. Артефакт хранится ограниченное время; это debug-сборка, а не релиз.

**Важно:** публичный CI собирает APK с `CORRODE_API_ID=0` и пустым API hash. Он подходит для проверки установки, интерфейса и Rust/JNI-интеграции, но **войти в Telegram через него нельзя**. Для входа нужна отдельная приватная сборка с собственными Telegram API credentials. Firebase/Google Sign-In, push, карты, SafetyNet, платежи, passkeys и автоматическое чтение SMS также требуют Corrode-конфигурации и в тестовом APK отключены или не настроены.

## Собрать локально

Нужны JDK 21, Android SDK 36, Android NDK `27.2.12479018`, Rust stable с Android targets из `rust-toolchain.toml` и `cargo-ndk`.

```bash
git clone --recurse-submodules https://github.com/Buqet/Corrode-Android.git
cd Corrode-Android
git checkout corrode/rust-phone-normalizer
cargo install --locked cargo-ndk
cargo test --manifest-path rust/Cargo.toml
./gradlew --no-daemon :TMessagesProj_App:assembleAfatDebug
```

Gradle запускает `scripts/build-rust-android.sh` во время `preBuild` и включает собранные Rust `.so` в APK. Полная инструкция и границы первого Rust-среза находятся в [`docs/CORRODE_RUST_MIGRATION.md`](docs/CORRODE_RUST_MIGRATION.md).

### Сборка с возможностью входа

Сначала получите собственные `api_id` и `api_hash` по [инструкции Telegram](https://core.telegram.org/api/obtaining_api_id), затем задайте их только в локальном окружении:

```bash
export CORRODE_API_ID="ваш_api_id"
export CORRODE_API_HASH="ваш_api_hash"
./gradlew --no-daemon :TMessagesProj_App:assembleAfatDebug
```

Не добавляйте credentials в Git, публичные CI logs, README или публичные артефакты. Для реального использования отдельно настройте собственные Firebase/Google-сервисы и внимательно выполните [рекомендации по безопасности MTProto](https://core.telegram.org/mtproto/security_guidelines).

## Что переносить дальше

Следующий разумный кандидат — чистая Rust-функция классификации ответов авторизации и серверных ошибок: она может сопоставлять типизированные события с переходами состояния, не принимая коды входа, пароли, ключи сессии или содержимое сообщений. Java пока оставит у себя сетевые запросы и показ экранов. Этот этап — предложение для следующей итерации, а не уже реализованная функция.

## Лицензия и upstream

Исходный Android-клиент распространяется по [GPL-2.0](https://github.com/TelegramMessenger/Telegram-Android/blob/master/LICENSE). Corrode сохраняет upstream copyright/license notices; при распространении производных сборок нужно соблюдать условия GPL и публиковать соответствующий исходный код.
