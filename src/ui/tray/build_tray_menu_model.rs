//! Description of the tray menu from the application state.

use crate::storage::StoredActivity;
use crate::ui::state::ActivePresence;
use crate::ui::tray::{TrayMenuEntry, TrayMenuModel, tray_tooltip};

/// Describes the tray menu for these `activities`, marking the `active` one.
#[must_use]
pub fn build_tray_menu_model(
    activities: &[StoredActivity],
    active: Option<&ActivePresence>,
) -> TrayMenuModel {
    let entries: Vec<TrayMenuEntry> = activities
        .iter()
        .map(|stored| TrayMenuEntry {
            name: stored.activity.name.clone(),
            folder: stored.folder.clone(),
            is_active: active.is_some_and(|active| active.folder == stored.folder),
        })
        .collect();

    let active_name = entries
        .iter()
        .find(|entry| entry.is_active)
        .map(|entry| entry.name.as_str());

    TrayMenuModel {
        tooltip: tray_tooltip(active_name),
        can_stop: active.is_some(),
        activities: entries,
    }
}
