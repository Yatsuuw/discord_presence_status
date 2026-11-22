//! Validation of a Discord application ID.

/// Shortest Discord snowflake identifier, in digits.
const MIN_DIGITS: usize = 17;

/// Longest Discord snowflake identifier, in digits.
const MAX_DIGITS: usize = 20;

/// Tells whether `value` looks like a Discord application ID.
///
/// Discord identifiers ("snowflakes") are unsigned 64-bit integers written in
/// decimal, which in practice means 17 to 20 digits.
///
/// ```
/// use discord_presence_status::activity::is_valid_application_id;
///
/// assert!(is_valid_application_id("123456789012345678"));
/// assert!(!is_valid_application_id("not-an-id"));
/// ```
#[must_use]
pub fn is_valid_application_id(value: &str) -> bool {
    (MIN_DIGITS..=MAX_DIGITS).contains(&value.len())
        && value.bytes().all(|byte| byte.is_ascii_digit())
}
