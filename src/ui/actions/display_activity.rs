//! Display of a saved activity on Discord.

use crate::i18n::{fill_placeholders, translations_for};
use crate::presence::{PresenceCommand, PresenceRequest, current_unix_millis};
use crate::storage::StoredActivity;
use crate::ui::actions::{show_error, show_success};
use crate::ui::state::{ActivePresence, PresenceApp};

/// Asks Discord to display `stored`, as saved on disk.
///
/// Discord's elapsed timer starts now. Any previously displayed activity is
/// replaced.
pub fn display_activity(app: &mut PresenceApp, stored: StoredActivity) {
    let texts = translations_for(app.language);
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
        show_error(app, texts.notice_worker_stopped);
        return;
    }

    app.active = Some(ActivePresence {
        folder: stored.folder,
        started_at_millis,
    });
    show_success(
        app,
        fill_placeholders(texts.notice_sent, &[("name", &stored.activity.name)]),
    );
}
