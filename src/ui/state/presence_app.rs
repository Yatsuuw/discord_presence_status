//! Definition of [`PresenceApp`], the whole state of the user interface.

use std::path::PathBuf;

use crate::i18n::Language;
use crate::presence::{PresenceHandle, PresenceStatus};
use crate::storage::StoredActivity;
use crate::ui::keyboard::SharedAltCodeInput;
use crate::ui::state::{ActivePresence, ActivityDraft, Dialog, Notice, RepaintTarget, WindowExit};
use crate::ui::tray::SystemTray;

/// Everything the user interface knows.
#[derive(Debug)]
pub struct PresenceApp {
    /// The `user_activity/` folder in use.
    pub activities_directory: PathBuf,
    /// The `settings.toml` file holding the user preferences.
    pub settings_path: PathBuf,
    /// Language of the interface.
    pub language: Language,
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
    /// The modal dialog currently open, if any.
    pub dialog: Option<Dialog>,
    /// The tray icon, or `None` when it could not be created (and in tests).
    pub tray: Option<SystemTray>,
    /// Why the main window is closing, once the user asked for it.
    pub window_exit: Option<WindowExit>,
    /// The window to wake up when the worker or the tray icon has news.
    pub repaint_target: RepaintTarget,
    /// Alt code corrections waiting for the next frame.
    pub alt_code_input: SharedAltCodeInput,
}
