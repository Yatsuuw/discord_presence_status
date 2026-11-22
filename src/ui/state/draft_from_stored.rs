//! Conversion of a stored activity into an editor draft.

use crate::storage::{StoredActivity, local_image_path};
use crate::ui::state::ActivityDraft;

/// Fills the editor form with the content of `stored`.
#[must_use]
pub fn draft_from_stored(stored: &StoredActivity) -> ActivityDraft {
    let activity = &stored.activity;
    ActivityDraft {
        folder: Some(stored.folder.clone()),
        name: activity.name.clone(),
        application_id: activity.application_id.clone(),
        kind: activity.kind,
        details: activity.details.clone().unwrap_or_default(),
        state: activity.state.clone().unwrap_or_default(),
        large_image: activity.large_image.clone().unwrap_or_default(),
        large_text: activity.large_text.clone().unwrap_or_default(),
        local_image: activity.local_image.clone(),
        new_image: None,
        preview_image: local_image_path(stored),
    }
}
