//! Window configuration.

use eframe::NativeOptions;
use eframe::egui::ViewportBuilder;

use crate::constants::APP_DISPLAY_NAME;
use crate::ui::load_window_icon;

/// Initial size of the main window, in logical pixels.
const INITIAL_SIZE: [f32; 2] = [900.0, 620.0];

/// Smallest size the main window can be resized to, in logical pixels.
const MINIMUM_SIZE: [f32; 2] = [720.0, 480.0];

/// Returns the options used to create the main window.
#[must_use]
pub fn build_native_options() -> NativeOptions {
    let mut viewport = ViewportBuilder::default()
        .with_title(APP_DISPLAY_NAME)
        .with_app_id("discord_presence_status")
        .with_inner_size(INITIAL_SIZE)
        .with_min_inner_size(MINIMUM_SIZE);

    if let Some(icon) = load_window_icon() {
        viewport = viewport.with_icon(icon);
    }

    NativeOptions {
        viewport,
        ..NativeOptions::default()
    }
}
