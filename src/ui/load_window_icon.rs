//! Loading of the window icon embedded in the executable.

use eframe::egui::IconData;

/// The application icon, embedded at compile time.
const ICON_PNG: &[u8] = include_bytes!("../../assets/icons/app.png");

/// Decodes the embedded application icon.
///
/// Returns `None` if decoding fails, in which case the window simply keeps
/// the default icon: a missing icon is not worth refusing to start.
#[must_use]
pub fn load_window_icon() -> Option<IconData> {
    eframe::icon_data::from_png_bytes(ICON_PNG).ok()
}
