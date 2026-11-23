//! Definition of [`LaunchMode`], how the application starts.

use crate::launch::PresenceHandover;

/// How the application starts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LaunchMode {
    /// Normal start: open the main window.
    #[default]
    Window,
    /// Start behind the tray icon without any window, optionally resuming
    /// the activity displayed by the previous process.
    Background(Option<PresenceHandover>),
}
