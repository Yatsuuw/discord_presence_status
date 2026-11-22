//! Reloading of the activity list from disk.

use crate::i18n::{describe_app_error, translations_for};
use crate::storage::load_activities;
use crate::ui::actions::report_error;
use crate::ui::state::PresenceApp;

/// Reads the activities folder again and refreshes the list.
///
/// Activities that cannot be read are listed in `load_errors` instead of
/// hiding the others.
pub fn reload_activities(app: &mut PresenceApp) {
    match load_activities(&app.activities_directory) {
        Ok(results) => {
            let (loaded, failed): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
            app.activities = loaded.into_iter().filter_map(Result::ok).collect();
            app.load_errors = failed
                .into_iter()
                .filter_map(Result::err)
                .map(|error| describe_app_error(&error, translations_for(app.language)))
                .collect();
        }
        Err(error) => report_error(app, &error),
    }
}
