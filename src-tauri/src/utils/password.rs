pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".into());
    }
    if password.len() > 128 {
        return Err("Password must be at most 128 characters".into());
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err("Password must contain a lowercase letter".into());
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err("Password must contain an uppercase letter".into());
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("Password must contain a digit".into());
    }
    if !password.chars().any(|c| c.is_ascii_punctuation()) {
        return Err("Password must contain a special character (!@#$%^&* etc.)".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        assert!(validate_password("Abcdef1!").is_ok());
    }

    #[test]
    fn test_too_short() {
        assert!(validate_password("Ab1!").is_err());
    }

    #[test]
    fn test_no_uppercase() {
        assert!(validate_password("abcdef1!").is_err());
    }

    #[test]
    fn test_no_digit() {
        assert!(validate_password("Abcdefg!").is_err());
    }

    #[test]
    fn test_no_special_char() {
        assert!(validate_password("Abcdefg1").is_err());
    }
}
