//! Loading of the tray icon image.

use image::RgbaImage;
use image::imageops::{self, FilterType};
use tray_icon::Icon;

use crate::ui::load_window_icon;

/// Size of the tray icon image, in pixels.
///
/// The embedded icon is 256×256; downscaling it here with a good filter looks
/// much better than letting Windows shrink it to the notification area size.
const TRAY_ICON_SIZE: u32 = 32;

/// Returns the application icon sized for the notification area.
///
/// Returns `None` if the embedded image cannot be decoded, in which case the
/// tray simply shows an empty icon.
#[must_use]
pub fn load_tray_icon() -> Option<Icon> {
    let data = load_window_icon()?;
    let image = RgbaImage::from_raw(data.width, data.height, data.rgba)?;
    let resized = imageops::resize(&image, TRAY_ICON_SIZE, TRAY_ICON_SIZE, FilterType::Lanczos3);
    Icon::from_rgba(resized.into_raw(), TRAY_ICON_SIZE, TRAY_ICON_SIZE).ok()
}
