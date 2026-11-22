//! Replacement of the placeholders of a translated text.

/// Replaces every `{key}` of `template` by its value.
///
/// Placeholders without a value are left untouched, which makes a mistake
/// visible instead of silently dropping text.
///
/// ```
/// use discord_presence_status::i18n::fill_placeholders;
///
/// let text = fill_placeholders("\"{name}\" saved.", &[("name", "Coding")]);
/// assert_eq!(text, "\"Coding\" saved.");
/// ```
#[must_use]
pub fn fill_placeholders(template: &str, values: &[(&str, &str)]) -> String {
    values
        .iter()
        .fold(template.to_owned(), |text, (key, value)| {
            text.replace(&format!("{{{key}}}"), value)
        })
}
