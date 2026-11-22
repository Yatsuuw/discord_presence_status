//! Display of a listed activity on Discord, without opening it.

use std::path::Path;

use crate::ui::actions::{display_activity, show_error};
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
    match stored {
        Some(stored) => display_activity(app, stored),
        None => show_error(
            app,
            format!(
                "the activity in \"{}\" no longer exists; click Reload",
                folder.display()
            ),
        ),
    }
}
