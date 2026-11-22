//! Definition of [`Translations`], every text of the user interface.

use serde::Serialize;

/// Every text shown to the user, in one language.
///
/// Placeholders written `{name}` are replaced with
/// [`fill_placeholders`](crate::i18n::fill_placeholders); the documentation
/// of each field lists them. `Serialize` lets tests check every text of
/// every language at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Translations {
    /// Label of the language selector.
    pub language_label: &'static str,
    /// Tooltip of the language selector.
    pub language_help: &'static str,
    /// Header when nothing is displayed.
    pub status_idle: &'static str,
    /// Header while an activity is displayed. `{name}`, `{elapsed}`.
    pub status_active: &'static str,
    /// Tooltip of the header while an activity is displayed.
    pub status_active_detail: &'static str,
    /// Header while Discord is not reachable. `{name}`.
    pub status_waiting: &'static str,
    /// Header when Discord refused the activity. `{name}`.
    pub status_rejected: &'static str,
    /// Title of the activity list.
    pub activities_heading: &'static str,
    /// Button reading the activities folder again.
    pub reload_button: &'static str,
    /// Tooltip of the reload button.
    pub reload_help: &'static str,
    /// Button creating an activity.
    pub new_activity_button: &'static str,
    /// Tooltip of the new activity button.
    pub new_activity_help: &'static str,
    /// Shown when the list is empty.
    pub no_activity_yet: &'static str,
    /// Marker next to the displayed activity.
    pub live_marker: &'static str,
    /// Title shown when no activity is open.
    pub welcome_title: &'static str,
    /// Text shown when no activity is open.
    pub welcome_text: &'static str,
    /// Editor title for a new activity.
    pub editor_title_new: &'static str,
    /// Editor title for an existing activity.
    pub editor_title_edit: &'static str,
    /// Badge shown in the editor of the displayed activity.
    pub live_badge: &'static str,
    /// Label of the name field.
    pub field_name: &'static str,
    /// Placeholder of the name field.
    pub hint_name: &'static str,
    /// Help of the name field.
    pub help_name: &'static str,
    /// Label of the application ID field.
    pub field_application_id: &'static str,
    /// Placeholder of the application ID field.
    pub hint_application_id: &'static str,
    /// Help of the application ID field.
    pub help_application_id: &'static str,
    /// Label of the activity type selector.
    pub field_kind: &'static str,
    /// Help of the activity type selector.
    pub help_kind: &'static str,
    /// Activity type "Playing".
    pub kind_playing: &'static str,
    /// Activity type "Listening to".
    pub kind_listening: &'static str,
    /// Activity type "Watching".
    pub kind_watching: &'static str,
    /// Activity type "Competing in".
    pub kind_competing: &'static str,
    /// Label of the description field.
    pub field_details: &'static str,
    /// Placeholder of the description field.
    pub hint_details: &'static str,
    /// Help of the description field.
    pub help_details: &'static str,
    /// Label of the second line field.
    pub field_state: &'static str,
    /// Placeholder of the second line field.
    pub hint_state: &'static str,
    /// Help of the second line field.
    pub help_state: &'static str,
    /// Label of the Discord image field.
    pub field_large_image: &'static str,
    /// Placeholder of the Discord image field.
    pub hint_large_image: &'static str,
    /// Help of the Discord image field.
    pub help_large_image: &'static str,
    /// Label of the image tooltip field.
    pub field_large_text: &'static str,
    /// Placeholder of the image tooltip field.
    pub hint_large_text: &'static str,
    /// Help of the image tooltip field.
    pub help_large_text: &'static str,
    /// Label of the local image section.
    pub field_local_image: &'static str,
    /// Help of the local image section.
    pub help_local_image: &'static str,
    /// Shown when there is no local image.
    pub no_image: &'static str,
    /// Button choosing a local image.
    pub choose_image_button: &'static str,
    /// Button removing the local image.
    pub remove_image_button: &'static str,
    /// Tooltip of the remove image button.
    pub remove_image_help: &'static str,
    /// Save button.
    pub save_button: &'static str,
    /// Tooltip of the save button.
    pub save_help: &'static str,
    /// Tooltip of the save button while the activity is displayed.
    pub save_help_active: &'static str,
    /// Button displaying the activity on Discord.
    pub start_button: &'static str,
    /// Tooltip of the start button.
    pub start_help: &'static str,
    /// Button removing the activity from Discord.
    pub stop_button: &'static str,
    /// Tooltip of the stop button.
    pub stop_help: &'static str,
    /// Delete button.
    pub delete_button: &'static str,
    /// Tooltip of the delete button.
    pub delete_help: &'static str,
    /// Note under the editor.
    pub required_hint: &'static str,
    /// Link to the Discord Developer Portal.
    pub portal_link: &'static str,
    /// Button hiding the footer message.
    pub dismiss_button: &'static str,
    /// Button opening the activities folder.
    pub open_folder_button: &'static str,
    /// Tooltip of the open folder button. `{path}`.
    pub open_folder_help: &'static str,
    /// Title of the delete confirmation.
    pub delete_dialog_title: &'static str,
    /// Text of the delete confirmation. `{name}`.
    pub delete_dialog_message: &'static str,
    /// Name used for a never saved activity.
    pub delete_dialog_unnamed: &'static str,
    /// Cancel button.
    pub cancel_button: &'static str,
    /// Title of the image file dialog.
    pub image_dialog_title: &'static str,
    /// Name of the file filter of the image dialog.
    pub image_dialog_filter: &'static str,
    /// After saving. `{name}`.
    pub notice_saved: &'static str,
    /// After deleting. `{name}`.
    pub notice_deleted: &'static str,
    /// After starting an activity. `{name}`.
    pub notice_sent: &'static str,
    /// After stopping the activity.
    pub notice_removed: &'static str,
    /// When the presence worker died.
    pub notice_worker_stopped: &'static str,
    /// When the tray starts an activity that no longer exists.
    pub notice_activity_missing: &'static str,
    /// Tray menu: show the window.
    pub tray_show_window: &'static str,
    /// Tray menu: sub-menu of activities.
    pub tray_start_activity: &'static str,
    /// Tray menu: stop the activity.
    pub tray_stop_activity: &'static str,
    /// Tray menu: quit.
    pub tray_quit: &'static str,
    /// Tray tooltip second line. `{name}`.
    pub tray_displaying: &'static str,
    /// Field name used in length errors.
    pub length_field_name: &'static str,
    /// Field name used in length errors.
    pub length_field_details: &'static str,
    /// Field name used in length errors.
    pub length_field_state: &'static str,
    /// Field name used in length errors.
    pub length_field_image_tooltip: &'static str,
    /// Missing name.
    pub validation_missing_name: &'static str,
    /// Name without usable characters.
    pub validation_unusable_name: &'static str,
    /// Malformed application ID.
    pub validation_invalid_application_id: &'static str,
    /// Text too short or too long. `{field}`, `{min}`, `{max}`.
    pub validation_invalid_length: &'static str,
    /// Invalid Discord image. `{max}`.
    pub validation_invalid_image: &'static str,
    /// Discord not reachable. `{detail}`.
    pub error_discord_unavailable: &'static str,
    /// Discord refused the activity. `{detail}`.
    pub error_discord_rejected: &'static str,
    /// Executable not found. `{detail}`.
    pub error_executable_location: &'static str,
    /// User data folder not found.
    pub error_missing_app_data: &'static str,
    /// File system error. `{path}`, `{detail}`.
    pub error_io: &'static str,
    /// Unreadable activity file. `{path}`, `{detail}`.
    pub error_activity_parse: &'static str,
    /// Unreadable settings file. `{path}`, `{detail}`.
    pub error_settings_parse: &'static str,
    /// Data that cannot be written. `{detail}`.
    pub error_serialize: &'static str,
    /// Name already used. `{name}`.
    pub error_activity_exists: &'static str,
    /// Unsupported image. `{formats}`.
    pub error_unsupported_image: &'static str,
    /// Image too large. `{max}`.
    pub error_image_too_large: &'static str,
    /// Path outside the activities folder. `{path}`.
    pub error_outside_folder: &'static str,
    /// Presence worker not started. `{detail}`.
    pub error_worker_spawn: &'static str,
    /// Tray icon not created. `{detail}`.
    pub error_tray: &'static str,
    /// Tray menu not updated. `{detail}`.
    pub error_tray_menu: &'static str,
    /// Interface not started. `{detail}`.
    pub error_interface: &'static str,
}
