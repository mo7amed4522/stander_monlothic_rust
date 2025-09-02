//! Input validation utilities

use regex::Regex;
use std::sync::OnceLock;


static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
static USERNAME_REGEX: OnceLock<Regex> = OnceLock::new();
static PASSWORD_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn validate_email(email: &str) -> bool {
    let regex = EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    });
    regex.is_match(email)
}
pub fn validate_username(username: &str) -> bool {
    let regex = USERNAME_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9_]{3,30}$").unwrap()
    });
    regex.is_match(username)
}
pub fn validate_password(password: &str) -> bool {
    let regex = PASSWORD_REGEX.get_or_init(|| {
        Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d@$!%*?&]{8,}$").unwrap()
    });
    regex.is_match(password)
}
pub fn validate_uuid(uuid_str: &str) -> bool {
    uuid::Uuid::parse_str(uuid_str).is_ok()
}
pub fn validate_non_empty(value: &str) -> bool {
    !value.trim().is_empty()
}
pub fn validate_length(value: &str, min: usize, max: usize) -> bool {
    let len = value.len();
    len >= min && len <= max
}
pub fn validate_positive_number(value: i32) -> bool {
    value > 0
}
pub fn validate_non_negative_number(value: i32) -> bool {
    value >= 0
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_email_validation() {
        assert!(validate_email("test@example.com"));
        assert!(validate_email("user.name+tag@domain.co.uk"));
        assert!(!validate_email("invalid-email"));
        assert!(!validate_email("@domain.com"));
        assert!(!validate_email("user@"));
    }
    #[test]
    fn test_username_validation() {
        assert!(validate_username("user123"));
        assert!(validate_username("test_user"));
        assert!(!validate_username("us"));
        assert!(!validate_username("user-name"));
        assert!(!validate_username("user@name"));
    }
    #[test]
    fn test_password_validation() {
        assert!(validate_password("Password123"));
        assert!(validate_password("MySecure1"));
        assert!(!validate_password("password"));
        assert!(!validate_password("PASSWORD123"));
        assert!(!validate_password("Password"));
        assert!(!validate_password("12345678"));
    }
}
