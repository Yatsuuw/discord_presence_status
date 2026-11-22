//! Collection of the clicks made in the tray menu.

use crate::ui::UiAction;
use crate::ui::state::PresenceApp;
use crate::ui::tray::TrayCommand;

/// Returns the actions requested through the tray icon since the last frame.
///
/// Returns nothing when the application runs without a tray icon.
pub fn poll_tray_commands(app: &mut PresenceApp) -> Vec<UiAction> {
    let Some(tray) = app.tray.as_mut() else {
        return Vec::new();
    };

    let mut actions = Vec::new();
    while let Ok(command) = tray.commands.try_recv() {
        let action = match command {
            TrayCommand::ShowWindow => Some(UiAction::ShowWindow),
            TrayCommand::StartActivity(index) => tray
                .shown_model
                .as_ref()
                .and_then(|model| model.activities.get(index))
                .map(|entry| UiAction::StartActivity(entry.folder.clone())),
            TrayCommand::StopActivity => Some(UiAction::StopPresence),
            TrayCommand::Quit => Some(UiAction::Quit),
        };
        actions.extend(action);
        // Clicking a check entry toggles it natively: rebuild the menu so it
        // matches the state again.
        tray.shown_model = None;
    }
    actions
}
