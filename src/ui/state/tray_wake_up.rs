//! Definition of [`TrayWakeUp`], why the application leaves the tray.

/// What ends the wait behind the tray icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayWakeUp {
    /// Recreate the main window.
    ShowWindow,
    /// Stop the application.
    Quit,
}
