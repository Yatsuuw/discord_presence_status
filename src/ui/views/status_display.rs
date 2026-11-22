//! Definition of [`StatusDisplay`].

use eframe::egui::Color32;

/// How a presence status is presented in the header.
#[derive(Debug, Clone, PartialEq)]
pub struct StatusDisplay {
    /// Color of the status dot and text.
    pub color: Color32,
    /// Short status text.
    pub text: String,
    /// Longer explanation, shown on hover.
    pub detail: Option<String>,
}
