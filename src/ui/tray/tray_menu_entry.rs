//! Definition of [`TrayMenuEntry`], one activity listed in the tray menu.

use std::path::PathBuf;

/// An activity the user can start from the tray menu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrayMenuEntry {
    /// The activity name, as shown in the menu.
    pub name: String,
    /// The folder of the activity, used to start it.
    pub folder: PathBuf,
    /// `true` when this activity is the one displayed on Discord.
    pub is_active: bool,
}
