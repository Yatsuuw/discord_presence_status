//! Definition of [`TrayCommand`], a click in the tray icon menu.

/// Prefix of the menu identifiers of the "Start activity" entries.
const START_ACTIVITY_PREFIX: &str = "start-activity:";

/// Menu identifier of the "Show window" entry.
const SHOW_WINDOW_ID: &str = "show-window";

/// Menu identifier of the "Stop activity" entry.
const STOP_ACTIVITY_ID: &str = "stop-activity";

/// Menu identifier of the "Quit" entry.
const QUIT_ID: &str = "quit";

/// Something the user asked for through the tray icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayCommand {
    /// Bring the main window back (menu entry or click on the icon).
    ShowWindow,
    /// Display the activity at this index of the menu's activity list.
    StartActivity(usize),
    /// Remove the activity from the Discord profile.
    StopActivity,
    /// Clear the presence and close the application.
    Quit,
}

impl TrayCommand {
    /// Returns the identifier given to the menu entry of this command.
    #[must_use]
    pub fn menu_id(self) -> String {
        match self {
            Self::ShowWindow => SHOW_WINDOW_ID.to_owned(),
            Self::StartActivity(index) => format!("{START_ACTIVITY_PREFIX}{index}"),
            Self::StopActivity => STOP_ACTIVITY_ID.to_owned(),
            Self::Quit => QUIT_ID.to_owned(),
        }
    }

    /// Returns the command of the menu entry identified by `id`, or `None`
    /// for an identifier this application did not create.
    #[must_use]
    pub fn from_menu_id(id: &str) -> Option<Self> {
        match id {
            SHOW_WINDOW_ID => Some(Self::ShowWindow),
            STOP_ACTIVITY_ID => Some(Self::StopActivity),
            QUIT_ID => Some(Self::Quit),
            _ => id
                .strip_prefix(START_ACTIVITY_PREFIX)
                .and_then(|index| index.parse().ok())
                .map(Self::StartActivity),
        }
    }
}
