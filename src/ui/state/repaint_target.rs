//! Definition of [`RepaintTarget`], the window to wake up from other threads.

use std::sync::{Arc, Mutex};

use eframe::egui;

/// The egui context of the open window, shared with background threads.
///
/// The presence worker and the tray icon use it to wake the window up. It
/// holds `None` while the window is closed (application hidden to the tray),
/// and a new context each time the window is recreated.
pub type RepaintTarget = Arc<Mutex<Option<egui::Context>>>;
