//! Deletion of the activity open in the editor.

use crate::storage::delete_activity;
use crate::ui::actions::{reload_activities, show_error, show_success, stop_presence};
use crate::ui::state::PresenceApp;

/// Permanently deletes the activity open in the editor (after confirmation).
///
/// A new, never saved activity is simply discarded. If the activity is
/// displayed on Discord, it is removed from the profile first.
pub fn delete_selected_activity(app: &mut PresenceApp) {
    app.delete_requested = false;
    let Some(draft) = app.draft.take() else {
        return;
    };
    let Some(folder) = draft.folder else {
        return;
    };

    if app
        .active
        .as_ref()
        .is_some_and(|active| active.folder == folder)
    {
        stop_presence(app);
    }

    match delete_activity(&app.activities_directory, &folder) {
        Ok(()) => show_success(app, format!("\"{}\" deleted.", draft.name)),
        Err(error) => show_error(app, error),
    }
    reload_activities(app);
}
