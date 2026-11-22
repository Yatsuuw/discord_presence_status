//! Conversion of a form field into an optional value.

/// Trims `text` and returns `None` when nothing is left.
///
/// ```
/// use discord_presence_status::ui::state::optional_text;
///
/// assert_eq!(optional_text("  hello "), Some("hello".to_owned()));
/// assert_eq!(optional_text("   "), None);
/// ```
#[must_use]
pub fn optional_text(text: &str) -> Option<String> {
    let trimmed = text.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}
