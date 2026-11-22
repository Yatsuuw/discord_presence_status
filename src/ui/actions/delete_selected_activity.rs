//! Deletion of the activity open in the editor.

use crate::i18n::{fill_placeholders, translations_for};
use crate::storage::delete_activity;
use crate::ui::actions::{reload_activities, report_error, show_success, stop_presence};
use crate::ui::state::PresenceApp;

/// Permanently deletes the activity open in the editor (after confirmation).
///
/// A new, never saved activity is simply discarded. If the activity is
/// displayed on Discord, it is removed from the profile first.
pub fn delete_selected_activity(app: &mut PresenceApp) {
    app.dialog = None;
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
        Ok(()) => {
            let texts = translations_for(app.language);
            show_success(
                app,
                fill_placeholders(texts.notice_deleted, &[("name", &draft.name)]),
            );
        }
        Err(error) => report_error(app, &error),
    }
    reload_activities(app);
}
