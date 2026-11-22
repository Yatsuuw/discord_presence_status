//! Conversion of an activity name into a safe folder name.

use crate::constants::MAX_FOLDER_NAME_LENGTH;

/// Characters Windows forbids in file and folder names.
const FORBIDDEN_CHARACTERS: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

/// Device names Windows reserves, whatever the extension or letter case.
const RESERVED_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// Turns an activity name into a folder name valid on every supported system.
///
/// * forbidden and control characters become `_`;
/// * leading/trailing spaces and trailing dots are removed (Windows drops them);
/// * reserved device names such as `CON` are prefixed with `_`;
/// * the result is limited to [`MAX_FOLDER_NAME_LENGTH`] characters.
///
/// An empty string is returned when nothing usable remains.
///
/// ```
/// use discord_presence_status::activity::sanitize_folder_name;
///
/// assert_eq!(sanitize_folder_name("Rust: the game?"), "Rust_ the game_");
/// assert_eq!(sanitize_folder_name("con"), "_con");
/// ```
#[must_use]
pub fn sanitize_folder_name(name: &str) -> String {
    let replaced: String = name
        .chars()
        .map(|character| {
            if character.is_control() || FORBIDDEN_CHARACTERS.contains(&character) {
                '_'
            } else {
                character
            }
        })
        .take(MAX_FOLDER_NAME_LENGTH)
        .collect();

    let trimmed = replaced.trim().trim_end_matches(['.', ' ']);

    let stem = trimmed.split('.').next().unwrap_or_default();
    let is_reserved = RESERVED_NAMES
        .iter()
        .any(|reserved| reserved.eq_ignore_ascii_case(stem));

    if is_reserved {
        format!("_{trimmed}")
    } else {
        trimmed.to_owned()
    }
}
