//! Removal of the activity from Discord.

use crate::i18n::translations_for;
use crate::presence::PresenceCommand;
use crate::ui::actions::show_success;
use crate::ui::state::PresenceApp;

/// Removes the displayed activity from the Discord profile.
pub fn stop_presence(app: &mut PresenceApp) {
    let _ = app.presence.commands.send(PresenceCommand::Clear);
    app.active = None;
    show_success(app, translations_for(app.language).notice_removed);
}
