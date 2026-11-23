//! Handling of tray menu choices while no window is open.

use crate::ui::UiAction;
use crate::ui::actions::{start_activity, stop_presence};
use crate::ui::state::{PresenceApp, TrayWakeUp};

/// Applies a tray menu choice made while the application runs without a
/// window, and returns whether the wait behind the tray icon must end.
///
/// Only the actions the tray menu can produce are expected; any other is
/// ignored.
pub fn handle_tray_action(app: &mut PresenceApp, action: UiAction) -> Option<TrayWakeUp> {
    match action {
        UiAction::ShowWindow => Some(TrayWakeUp::ShowWindow),
        UiAction::Quit => Some(TrayWakeUp::Quit),
        UiAction::StartActivity(folder) => {
            start_activity(app, &folder);
            None
        }
        UiAction::StopPresence => {
            stop_presence(app);
            None
        }
        _ => None,
    }
}
