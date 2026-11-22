//! Opening of the activities folder in the file explorer.

use std::process::Command;

use crate::storage::ensure_directory;
use crate::ui::actions::show_error;
use crate::ui::state::PresenceApp;

/// Program used to open a folder on the current operating system.
const FILE_EXPLORER: &str = if cfg!(windows) {
    "explorer"
} else if cfg!(target_os = "macos") {
    "open"
} else {
    "xdg-open"
};

/// Opens the activities folder in the system file explorer.
pub fn open_activities_folder(app: &mut PresenceApp) {
    let folder = app.activities_directory.clone();
    let result = ensure_directory(&folder).and_then(|()| {
        Command::new(FILE_EXPLORER)
            .arg(&folder)
            .spawn()
            .map(drop)
            .map_err(crate::error::io_error_at(&folder))
    });
    if let Err(error) = result {
        show_error(app, error);
    }
}
