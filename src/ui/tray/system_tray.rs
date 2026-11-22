//! Definition of [`SystemTray`], the live tray icon.

use std::fmt;
use std::sync::mpsc::Receiver;

use tray_icon::TrayIcon;

use crate::ui::tray::{TrayCommand, TrayMenuModel};

/// The tray icon shown in the notification area, removed when dropped.
pub struct SystemTray {
    /// The native tray icon.
    pub icon: TrayIcon,
    /// Clicks in the tray menu and on the icon, in order.
    pub commands: Receiver<TrayCommand>,
    /// The description of the menu currently installed, if any.
    ///
    /// Set to `None` to force the menu to be rebuilt on the next frame.
    pub shown_model: Option<TrayMenuModel>,
}

impl fmt::Debug for SystemTray {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SystemTray")
            .field("icon", &self.icon.id())
            .field("shown_model", &self.shown_model)
            .finish_non_exhaustive()
    }
}
