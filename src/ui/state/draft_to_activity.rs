//! Conversion of an editor draft into an activity.

use crate::activity::Activity;
use crate::ui::state::{ActivityDraft, optional_text};

/// Builds the [`Activity`] described by the editor form.
///
/// Surrounding whitespace is removed and empty optional fields become
/// `None`, so they are omitted from `activity.toml`.
#[must_use]
pub fn draft_to_activity(draft: &ActivityDraft) -> Activity {
    Activity {
        name: draft.name.trim().to_owned(),
        application_id: draft.application_id.trim().to_owned(),
        kind: draft.kind,
        details: optional_text(&draft.details),
        state: optional_text(&draft.state),
        large_image: optional_text(&draft.large_image),
        large_text: optional_text(&draft.large_text),
        local_image: draft.local_image.clone(),
    }
}
