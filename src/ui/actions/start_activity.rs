//! Display of a listed activity on Discord, without opening it.

use std::path::Path;

use crate::i18n::translations_for;
use crate::presence::current_unix_millis;
use crate::ui::actions::{display_activity, reload_activities, show_error};
use crate::ui::state::PresenceApp;

/// Asks Discord to display the saved activity stored in `folder`.
///
/// Used by the tray menu: the editor and its unsaved changes are left
/// untouched.
pub fn start_activity(app: &mut PresenceApp, folder: &Path) {
    let stored = app
        .activities
        .iter()
        .find(|stored| stored.folder == folder)
        .cloned();
    if let Some(stored) = stored {
        display_activity(app, stored, current_unix_millis());
    } else {
        reload_activities(app);
        show_error(app, translations_for(app.language).notice_activity_missing);
    }
}
