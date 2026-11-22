//! Definition of [`ActivityDraft`], the content of the editor form.

use std::path::PathBuf;

use crate::activity::ActivityKind;

/// The activity being edited, as typed by the user.
///
/// Text fields are kept as plain strings, exactly as typed; they are trimmed
/// and converted into an [`Activity`](crate::activity::Activity) only when
/// saving (see [`draft_to_activity`](crate::ui::state::draft_to_activity)).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ActivityDraft {
    /// Folder of the activity on disk; `None` for an activity never saved.
    pub folder: Option<PathBuf>,
    /// Activity name.
    pub name: String,
    /// Discord application ID.
    pub application_id: String,
    /// Activity kind ("Playing", "Watching"…).
    pub kind: ActivityKind,
    /// Description (first line).
    pub details: String,
    /// Second line.
    pub state: String,
    /// Discord image: asset key or `https://` URL.
    pub large_image: String,
    /// Tooltip of the Discord image.
    pub large_text: String,
    /// File name of the local image already saved with the activity.
    pub local_image: Option<String>,
    /// Image chosen by the user and not saved yet.
    pub new_image: Option<PathBuf>,
    /// Image currently shown in the editor preview.
    pub preview_image: Option<PathBuf>,
}
