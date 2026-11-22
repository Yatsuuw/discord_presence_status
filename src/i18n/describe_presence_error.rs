//! Translation of the errors of the Discord connection.

use crate::i18n::{Translations, fill_placeholders};
use crate::presence::PresenceError;

/// Explains `error` to the user in the interface language.
///
/// The technical detail (or Discord's own message) is kept as is, because it
/// is what to search for when troubleshooting.
#[must_use]
pub fn describe_presence_error(error: &PresenceError, texts: &Translations) -> String {
    let (template, detail) = match error {
        PresenceError::Unavailable(detail) => (texts.error_discord_unavailable, detail),
        PresenceError::Rejected(detail) => (texts.error_discord_rejected, detail),
    };
    fill_placeholders(template, &[("detail", detail)])
}
