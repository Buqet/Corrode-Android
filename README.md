<div align="center">
  <img src="docs/assets/corrode-icon.png" alt="Иконка Corrode" width="144" />
  <h1>Corrode для Android</h1>
  <p><strong>Неофициальный Android-клиент Telegram с постепенным переносом логики на Rust.</strong></p>
  <p>
    <a href="https://github.com/Buqet/Corrode-Android/actions/workflows/corrode-rust-android.yml?query=branch%3Acorrode%2Frust-phone-normalizer"><img src="https://github.com/Buqet/Corrode-Android/actions/workflows/corrode-rust-android.yml/badge.svg?branch=corrode%2Frust-phone-normalizer" alt="Rust CI" /></a>
    <a href="https://github.com/Buqet/Corrode-Android/actions/workflows/corrode-android-apk.yml?query=branch%3Acorrode%2Frust-phone-normalizer"><img src="https://github.com/Buqet/Corrode-Android/actions/workflows/corrode-android-apk.yml/badge.svg?branch=corrode%2Frust-phone-normalizer" alt="Android APK CI" /></a>
  </p>
</div>

> Corrode не является официальным приложением Telegram и не связан с Telegram FZ-LLC. Это отдельный экспериментальный форк; название, package ID и иконка Corrode отличаются от официального приложения.

## О проекте

Corrode основан на [открытом исходном коде Telegram для Android](https://github.com/TelegramMessenger/Telegram-Android). Вместо рискованной одномоментной переписи проект переносит небольшие, изолированные части постепенно: существующие экраны, сетевой протокол и остальную Android-интеграцию пока сохраняет upstream-код.

### Уже работает

- Нормализация номера телефона перед `auth.sendCode` выполняется Rust-модулем через JNI.
- При неподдерживаемом вводе остаётся Java fallback.
- Для Rust-среза есть unit-тесты; Android workflow собирает native-библиотеки для `armeabi-v7a`, `arm64-v8a`, `x86` и `x86_64`.
- APK помечен как Corrode и использует отдельный application ID `com.buqet.corrode.beta`.
- Сохранён полный upstream-поток авторизации: номер телефона, одноразовый код, регистрация и пароль 2FA. Для работы сети требуются собственные Telegram API credentials Corrode.

Это пока не переписанный на Rust мессенджер: UI, Telegram API/MTProto-клиент, обработка сообщений и основная логика остаются на исходных Java/Kotlin/C++ компонентах.

## Скачать тестовый APK

Откройте [запуски сборки APK в GitHub Actions](https://github.com/Buqet/Corrode-Android/actions/workflows/corrode-android-apk.yml?query=branch%3Acorrode%2Frust-phone-normalizer), выберите успешный run и скачайте artifact `corrode-android-afat-debug`. Обычные сборки без API credentials подходят для проверки установки, интерфейса и Rust/JNI-интеграции, но войти через них нельзя. Это debug-сборки, а не релизы.

## Вход в аккаунт Telegram

Экраны номера, кода, регистрации и 2FA уже есть в сохранённом Telegram login flow. Чтобы разрешить запросы `auth.sendCode`, нужны **собственные** `api_id` и `api_hash` Corrode с [my.telegram.org](https://my.telegram.org) (раздел API development tools); официальные credentials Telegram не используются.

Для CI добавьте GitHub repository secrets `CORRODE_API_ID` и `CORRODE_API_HASH` в **Settings → Secrets and variables → Actions**. После этого workflow использует их для сборки. На push login-enabled APK собирается для проверки, но не загружается в artifacts автоматически. Чтобы вручную загрузить его как Actions artifact, запустите `workflow_dispatch` и включите `publish_login_apk`.

**Важно о приватности:** API credentials встраиваются в клиентский APK как BuildConfig-значения и могут быть извлечены из файла. Не считайте публичный APK способом защитить `api_hash`; публикуйте login-enabled artifact только если готовы раскрыть эти app credentials. Без добавленных secrets workflow создаёт тестовую сборку без рабочего входа.

Для локальной сборки задайте credentials только в окружении:

```bash
export CORRODE_API_ID="ваш_api_id"
export CORRODE_API_HASH="ваш_api_hash"
./gradlew --no-daemon --build-cache :TMessagesProj_App:assembleAfatDebug
```

Не добавляйте credentials в Git или вывод сборки. Google Sign-In, push, карты, SafetyNet, платежи, passkeys и автоматическое чтение SMS требуют отдельных Corrode-конфигураций и в тестовой сборке отключены или не настроены.

## Собрать локально

Нужны JDK 21, Android SDK 36, Android NDK `27.2.12479018`, Rust stable с Android targets из `rust-toolchain.toml` и `cargo-ndk`.

```bash
git clone --recurse-submodules https://github.com/Buqet/Corrode-Android.git
cd Corrode-Android
git checkout corrode/rust-phone-normalizer
cargo install --locked cargo-ndk
cargo test --manifest-path rust/Cargo.toml
./gradlew --no-daemon --build-cache :TMessagesProj_App:assembleAfatDebug
```

Gradle запускает `scripts/build-rust-android.sh` во время `preBuild` и включает собранные Rust `.so` в APK. CI использует Gradle dependency/build cache, а также кэш Cargo/Rust и CMake/NDK intermediates, чтобы ускорить повторные сборки. Первый запуск и сборка после существенных изменений всё равно требуют полной native-компиляции. Полная инструкция и границы первого Rust-среза находятся в [`docs/CORRODE_RUST_MIGRATION.md`](docs/CORRODE_RUST_MIGRATION.md).

## Что переносить дальше

Следующий разумный кандидат — чистая Rust-функция классификации ответов авторизации и серверных ошибок: она может сопоставлять типизированные события с переходами состояния, не принимая коды входа, пароли, ключи сессии или содержимое сообщений. Java пока оставит у себя сетевые запросы и показ экранов. Этот этап — предложение для следующей итерации, а не уже реализованная функция.

## Лицензия и upstream

Исходный Android-клиент распространяется по [GPL-2.0](https://github.com/TelegramMessenger/Telegram-Android/blob/master/LICENSE). Corrode сохраняет upstream copyright/license notices; при распространении производных сборок нужно соблюдать условия GPL и публиковать соответствующий исходный код.
