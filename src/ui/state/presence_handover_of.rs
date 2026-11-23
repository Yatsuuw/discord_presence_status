//! Description of the displayed activity for the next process.

use crate::launch::PresenceHandover;
use crate::ui::state::PresenceApp;

/// Returns the activity to hand over to a background process, if one is
/// displayed.
#[must_use]
pub fn presence_handover_of(app: &PresenceApp) -> Option<PresenceHandover> {
    app.active.as_ref().map(|active| PresenceHandover {
        folder: active.folder.clone(),
        started_at_millis: active.started_at_millis,
    })
}
