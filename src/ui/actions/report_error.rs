//! Display of an application error in the user's language.

use crate::error::AppError;
use crate::i18n::{describe_app_error, translations_for};
use crate::ui::actions::show_error;
use crate::ui::state::PresenceApp;

/// Shows `error` in the footer, translated into the interface language.
pub fn report_error(app: &mut PresenceApp, error: &AppError) {
    let text = describe_app_error(error, translations_for(app.language));
    show_error(app, text);
}
