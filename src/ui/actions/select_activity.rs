//! Opening of an activity in the editor.

use std::path::Path;

use crate::ui::state::{PresenceApp, draft_from_stored};

/// Opens the activity stored in `folder` in the editor.
///
/// Unsaved changes of the previously edited activity are discarded.
pub fn select_activity(app: &mut PresenceApp, folder: &Path) {
    app.delete_requested = false;
    app.draft = app
        .activities
        .iter()
        .find(|stored| stored.folder == folder)
        .map(draft_from_stored);
}
