//! Change of the interface language.

use crate::i18n::Language;
use crate::settings::{Settings, save_settings};
use crate::ui::actions::{reload_activities, report_error};
use crate::ui::state::PresenceApp;

/// Switches the interface to `language` and remembers it for the next launches.
///
/// The interface switches even if the preference cannot be saved; the error
/// is then reported. Messages already displayed are cleared because they are
/// written in the previous language, and the activity list is reloaded so
/// that its error messages are translated too.
pub fn change_language(app: &mut PresenceApp, language: Language) {
    app.language = language;
    app.notice = None;
    reload_activities(app);

    let settings = Settings { language };
    if let Err(error) = save_settings(&app.settings_path, &settings) {
        report_error(app, &error);
    }
}
