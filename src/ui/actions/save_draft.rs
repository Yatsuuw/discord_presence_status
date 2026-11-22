//! Saving of the activity open in the editor.

use eframe::egui;

use crate::presence::{PresenceCommand, PresenceRequest};
use crate::storage::{StoredActivity, save_activity};
use crate::ui::actions::{reload_activities, show_error, show_success};
use crate::ui::state::{PresenceApp, draft_from_stored, draft_to_activity};

/// Saves the activity open in the editor and returns it on success.
///
/// If that activity is currently displayed on Discord, the presence is
/// updated right away while keeping its timer running.
pub fn save_draft(app: &mut PresenceApp, context: &egui::Context) -> Option<StoredActivity> {
    let draft = app.draft.as_ref()?;
    let previous_folder = draft.folder.clone();

    let result = save_activity(
        &app.activities_directory,
        draft_to_activity(draft),
        previous_folder.as_deref(),
        draft.new_image.as_deref(),
    );

    let stored = match result {
        Ok(stored) => stored,
        Err(error) => {
            show_error(app, error);
            return None;
        }
    };

    // The image file may have been replaced under the same name.
    context.forget_all_images();

    if let Some(active) = app
        .active
        .as_mut()
        .filter(|active| Some(&active.folder) == previous_folder.as_ref())
    {
        active.folder.clone_from(&stored.folder);
        let request = PresenceRequest {
            activity: stored.activity.clone(),
            started_at_millis: active.started_at_millis,
        };
        let _ = app.presence.commands.send(PresenceCommand::Show(request));
    }

    reload_activities(app);
    app.draft = Some(draft_from_stored(&stored));
    show_success(app, format!("\"{}\" saved.", stored.activity.name));
    Some(stored)
}
