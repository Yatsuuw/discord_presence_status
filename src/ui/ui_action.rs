//! Definition of [`UiAction`], everything the user can ask for.

use std::path::PathBuf;

use crate::i18n::Language;

/// An action triggered by the user, produced by the views and applied by
/// [`apply_action`](crate::ui::actions::apply_action).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiAction {
    /// Open an empty editor to create a new activity.
    NewActivity,
    /// Open the activity stored in this folder in the editor.
    Select(PathBuf),
    /// Save the activity being edited.
    Save,
    /// Save the activity being edited and display it on Discord.
    StartPresence,
    /// Display the saved activity stored in this folder on Discord, without
    /// opening it in the editor (tray menu).
    StartActivity(PathBuf),
    /// Remove the activity from the Discord profile.
    StopPresence,
    /// Choose a local image for the activity being edited.
    PickImage,
    /// Ask for confirmation before deleting the activity being edited.
    RequestDelete,
    /// Delete the activity being edited (after confirmation).
    ConfirmDelete,
    /// Close the delete confirmation without deleting anything.
    CancelDelete,
    /// Open the activities folder in the system file explorer.
    OpenActivitiesFolder,
    /// Read the activities folder again (after manual edits).
    Reload,
    /// Hide the message shown in the footer.
    DismissNotice,
    /// Show the main window again (tray icon).
    ShowWindow,
    /// Hide the main window, keeping the presence running behind the tray icon.
    HideToTray,
    /// Clear the presence and close the application (tray menu).
    Quit,
    /// Display the interface in this language and remember the choice.
    ChangeLanguage(Language),
}
