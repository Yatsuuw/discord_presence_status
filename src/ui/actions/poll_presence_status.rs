//! Collection of the statuses reported by the presence worker.

use crate::ui::state::PresenceApp;

/// Applies every status the presence worker reported since the last frame.
pub fn poll_presence_status(app: &mut PresenceApp) {
    while let Ok(status) = app.presence.statuses.try_recv() {
        app.status = status;
    }
}
