//! Text shown when hovering the tray icon.

use crate::constants::APP_DISPLAY_NAME;

/// Longest tooltip kept, in characters.
///
/// Windows truncates notification area tooltips to 128 UTF-16 units; cutting
/// a little earlier keeps the text readable.
const MAX_TOOLTIP_LENGTH: usize = 120;

/// Returns the tray tooltip: the application name, followed by the
/// displayed activity when there is one.
#[must_use]
pub fn tray_tooltip(active_name: Option<&str>) -> String {
    let text = match active_name {
        Some(name) => format!("{APP_DISPLAY_NAME}\nDisplaying \"{name}\""),
        None => APP_DISPLAY_NAME.to_owned(),
    };
    text.chars().take(MAX_TOOLTIP_LENGTH).collect()
}
