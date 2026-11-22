//! Opening of an empty editor.

use crate::ui::state::{ActivityDraft, PresenceApp};

/// Opens the editor on a new, empty activity.
pub fn create_new_activity(app: &mut PresenceApp) {
    app.dialog = None;
    app.draft = Some(ActivityDraft::default());
}
