//! Resumption of the activity handed over by the previous process.

use crate::launch::PresenceHandover;
use crate::ui::actions::display_activity;
use crate::ui::state::PresenceApp;

/// Displays again the activity described by `handover`, keeping its timer.
///
/// Does nothing if the activity no longer exists.
pub fn resume_presence(app: &mut PresenceApp, handover: &PresenceHandover) {
    let stored = app
        .activities
        .iter()
        .find(|stored| stored.folder == handover.folder)
        .cloned();
    if let Some(stored) = stored {
        display_activity(app, stored, handover.started_at_millis);
    }
}
