//! Definition of [`StoredActivity`], an activity together with its folder.

use std::path::PathBuf;

use crate::activity::Activity;

/// An activity loaded from (or saved to) the activities folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredActivity {
    /// The folder containing `activity.toml` and the optional local image.
    pub folder: PathBuf,
    /// The activity read from `activity.toml`.
    pub activity: Activity,
}
