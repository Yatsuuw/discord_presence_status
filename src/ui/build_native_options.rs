//! Window configuration.

use eframe::NativeOptions;
use eframe::egui::ViewportBuilder;

use crate::constants::APP_DISPLAY_NAME;
use crate::ui::keyboard::{SharedAltCodeInput, install_alt_code_hook};
use crate::ui::load_window_icon;
use crate::ui::state::RepaintTarget;

/// Initial size of the main window, in logical pixels.
const INITIAL_SIZE: [f32; 2] = [900.0, 620.0];

/// Smallest size the main window can be resized to, in logical pixels.
const MINIMUM_SIZE: [f32; 2] = [720.0, 480.0];

/// Returns the options used to create the main window.
///
/// The event loop watches the keyboard messages to support Alt codes (see
/// [`crate::ui::keyboard`]); `alt_code_input` and `repaint_target` are
/// shared with that hook.
#[must_use]
pub fn build_native_options(
    alt_code_input: &SharedAltCodeInput,
    repaint_target: &RepaintTarget,
) -> NativeOptions {
    let mut viewport = ViewportBuilder::default()
        .with_title(APP_DISPLAY_NAME)
        .with_app_id("discord_presence_status")
        .with_inner_size(INITIAL_SIZE)
        .with_min_inner_size(MINIMUM_SIZE);

    if let Some(icon) = load_window_icon() {
        viewport = viewport.with_icon(icon);
    }

    let alt_code_input = std::sync::Arc::clone(alt_code_input);
    let repaint_target = std::sync::Arc::clone(repaint_target);
    NativeOptions {
        viewport,
        event_loop_builder: Some(Box::new(move |builder| {
            install_alt_code_hook(builder, &alt_code_input, &repaint_target);
        })),
        ..NativeOptions::default()
    }
}
