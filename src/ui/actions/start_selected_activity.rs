//! Display of the edited activity on Discord.

use eframe::egui;

use crate::presence::{PresenceCommand, PresenceRequest, current_unix_millis};
use crate::ui::actions::{save_draft, show_error, show_success};
use crate::ui::state::{ActivePresence, PresenceApp};

/// Saves the activity open in the editor, then asks Discord to display it.
///
/// Discord's elapsed timer starts now. Any previously displayed activity is
/// replaced.
pub fn start_selected_activity(app: &mut PresenceApp, context: &egui::Context) {
    let Some(stored) = save_draft(app, context) else {
        return;
    };

    let started_at_millis = current_unix_millis();
    let request = PresenceRequest {
        activity: stored.activity.clone(),
        started_at_millis,
    };

    if app
        .presence
        .commands
        .send(PresenceCommand::Show(request))
        .is_err()
    {
        show_error(
            app,
            "the presence worker stopped unexpectedly; please restart the application",
        );
        return;
    }

    app.active = Some(ActivePresence {
        folder: stored.folder,
        started_at_millis,
    });
    show_success(
        app,
        format!("\"{}\" sent to Discord.", stored.activity.name),
    );
}
