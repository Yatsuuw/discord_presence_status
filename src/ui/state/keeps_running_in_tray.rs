//! Decision between hiding to the tray and quitting.

use crate::ui::state::PresenceApp;

/// Returns `true` when closing or minimising the window should hide it to
/// the tray instead.
///
/// That is the case while an activity is displayed on Discord (it would
/// disappear if the application quit), as long as a tray icon exists to
/// bring the window back and the user did not choose "Quit" in the tray.
#[must_use]
pub fn keeps_running_in_tray(app: &PresenceApp) -> bool {
    app.tray.is_some() && app.active.is_some() && !app.quit_requested
}
