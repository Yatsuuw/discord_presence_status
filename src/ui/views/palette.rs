//! Colors used to convey meaning in the interface.

use eframe::egui::Color32;

/// Something works: activity displayed, save succeeded.
pub const SUCCESS_COLOR: Color32 = Color32::from_rgb(34, 197, 94);

/// Something is pending: waiting for Discord.
pub const WARNING_COLOR: Color32 = Color32::from_rgb(245, 158, 11);

/// Something failed.
pub const ERROR_COLOR: Color32 = Color32::from_rgb(239, 68, 68);

/// Nothing is happening.
pub const IDLE_COLOR: Color32 = Color32::from_rgb(148, 163, 184);
