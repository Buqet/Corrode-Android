use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use std::ptr;

/// Keep the same digit-stripping behavior as the current Java login path.
/// Non-ASCII numeric input is rejected so Java can apply its existing fallback.
pub fn normalize_phone(raw: &str) -> Option<String> {
    let mut digits = String::with_capacity(raw.len());
    for character in raw.chars() {
        if character.is_ascii_digit() {
            digits.push(character);
        } else if character.is_numeric() {
            return None;
        }
    }

    (!digits.is_empty()).then_some(digits)
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_telegram_messenger_CorrodeNative_nativeNormalizePhone(
    mut env: JNIEnv,
    _class: JClass,
    raw_phone: JString,
) -> jstring {
    let raw_phone: String = match env.get_string(&raw_phone) {
        Ok(value) => value.into(),
        Err(_) => return ptr::null_mut(),
    };

    let Some(normalized) = normalize_phone(&raw_phone) else {
        return ptr::null_mut();
    };

    match env.new_string(normalized) {
        Ok(value) => value.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_phone;

    #[test]
    fn strips_phone_formatting_and_keeps_ascii_digits() {
        assert_eq!(
            normalize_phone("+1 (555) 010-2040"),
            Some("15550102040".to_owned())
        );
    }

    #[test]
    fn rejects_empty_and_non_ascii_numeric_input_for_java_fallback() {
        assert_eq!(normalize_phone(" +()- "), None);
        assert_eq!(normalize_phone("١٢٣٤"), None);
    }
}
