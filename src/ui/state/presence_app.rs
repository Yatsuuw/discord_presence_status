//! Definition of [`PresenceApp`], the whole state of the user interface.

use std::path::PathBuf;

use crate::presence::{PresenceHandle, PresenceStatus};
use crate::storage::StoredActivity;
use crate::ui::state::{ActivePresence, ActivityDraft, Notice};
use crate::ui::tray::SystemTray;

/// Everything the user interface knows.
#[derive(Debug)]
pub struct PresenceApp {
    /// The `user_activity/` folder in use.
    pub activities_directory: PathBuf,
    /// Activities successfully loaded from disk, sorted by name.
    pub activities: Vec<StoredActivity>,
    /// Messages describing activity folders that could not be loaded.
    pub load_errors: Vec<String>,
    /// The activity open in the editor, if any.
    pub draft: Option<ActivityDraft>,
    /// The message shown in the footer, if any.
    pub notice: Option<Notice>,
    /// Channels to the presence worker.
    pub presence: PresenceHandle,
    /// The last status reported by the presence worker.
    pub status: PresenceStatus,
    /// The activity the user asked Discord to display, if any.
    pub active: Option<ActivePresence>,
    /// `true` while the delete confirmation dialog is open.
    pub delete_requested: bool,
    /// The tray icon, or `None` when it could not be created (and in tests).
    pub tray: Option<SystemTray>,
    /// `true` while the main window is hidden to the tray.
    pub window_hidden: bool,
    /// `true` once the user chose "Quit" in the tray: the next close request
    /// really closes the application.
    pub quit_requested: bool,
}
