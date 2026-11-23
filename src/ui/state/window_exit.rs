//! Definition of [`WindowExit`], why the main window is closing.

/// The reason the main window was asked to close.
///
/// Closing the window destroys it together with its graphics context, which
/// frees most of the memory; what happens next depends on this reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowExit {
    /// Keep running behind the tray icon; the window is recreated on demand.
    HideToTray,
    /// Stop the application.
    Quit,
}
