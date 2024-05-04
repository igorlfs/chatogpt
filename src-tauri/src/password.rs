const MIN_PASSWORD_LENGTH: usize = 12;

pub fn is_password_secure(string: &str) -> String {
    if let Some(reason) = get_password_unsafety_reason(string) {
        format!("Password {string} is unsafe. Reason: {reason}.")
    } else {
        "Password is secure.".to_string()
    }
}

fn get_password_unsafety_reason(string: &str) -> Option<String> {
    if string.len() < MIN_PASSWORD_LENGTH {
        Some("it's too short".to_string())
    } else if !string.chars().any(|c| c.is_numeric()) {
        Some("it doesn't contain any numbers".to_string())
    } else if !string.chars().any(|c| c.is_alphabetic()) {
        Some("it doesn't contain any letters".to_string())
    } else if !string.chars().any(|c| c.is_uppercase()) {
        Some("it doesn't contain any uppercase letters".to_string())
    } else if !string.chars().any(|c| c.is_ascii_punctuation()) {
        Some("it doesn't contain any recognized special characters".to_string())
    } else if string.to_lowercase().contains("password") {
        Some("it shouldn't contain the word 'password'".to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::get_password_unsafety_reason;

    #[test]
    fn test_password_is_correct() {
        let reason = get_password_unsafety_reason("abcDef01234@");
        assert!(reason.is_none());
    }

    #[test]
    fn test_password_is_short() {
        let reason = get_password_unsafety_reason("abc").unwrap();
        assert!(reason.contains("short"));
    }

    #[test]
    fn test_password_missing_numbers() {
        let reason = get_password_unsafety_reason("abcdefghijkl").unwrap();
        assert!(reason.contains("numbers"));
    }

    #[test]
    fn test_password_missing_letters() {
        let reason = get_password_unsafety_reason("012345678901").unwrap();
        assert!(reason.contains("letters"));
    }

    #[test]
    fn test_password_missing_uppercase_letters() {
        let reason = get_password_unsafety_reason("0123456789ab").unwrap();
        assert!(reason.contains("uppercase"));
    }

    #[test]
    fn test_password_missing_special_characters() {
        let reason = get_password_unsafety_reason("0123456Abcdef").unwrap();
        assert!(reason.contains("special"));
    }

    #[test]
    fn test_password_should_not_contain_string_password() {
        let reason = get_password_unsafety_reason("password123P@").unwrap();
        assert!(reason.contains("password"));
    }

    #[test]
    fn test_password_should_not_contain_string_password_any_case() {
        let reason = get_password_unsafety_reason("PassWord123P@").unwrap();
        assert!(reason.contains("password"));
    }
}
