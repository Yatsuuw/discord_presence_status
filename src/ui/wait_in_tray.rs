//! Life of the application behind its tray icon, without any window.

use crate::ui::actions::{
    handle_tray_action, poll_presence_status, poll_tray_commands, sync_tray_menu,
};
use crate::ui::state::{PresenceApp, TrayWakeUp};
use crate::ui::tray::wait_for_tray_message;

/// Keeps the presence running with no window open, until the user asks for
/// the window again or quits from the tray menu.
///
/// Without a window there is no graphics context: only the tray icon and the
/// presence worker remain, which keeps the memory use minimal. Tray choices
/// (start or stop an activity) are applied directly.
pub fn wait_in_tray(app: &mut PresenceApp) -> TrayWakeUp {
    loop {
        for action in poll_tray_commands(app) {
            if let Some(wake_up) = handle_tray_action(app, action) {
                return wake_up;
            }
        }
        poll_presence_status(app);
        sync_tray_menu(app);

        if !wait_for_tray_message() {
            return TrayWakeUp::Quit;
        }
    }
}
