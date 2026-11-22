//! Creation of the native tray menu from its description.

use tray_icon::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};

use crate::ui::tray::{TrayCommand, TrayMenuModel, escape_menu_label};

/// Builds the native context menu described by `model`:
///
/// ```text
/// Show window
/// ───────────────
/// Start activity ▸  (one entry per activity, the displayed one checked)
/// Stop activity
/// ───────────────
/// Quit
/// ```
///
/// # Errors
///
/// Returns the menu library error when the operating system refuses to
/// create an entry.
pub fn build_tray_menu(model: &TrayMenuModel) -> Result<Menu, tray_icon::menu::Error> {
    let activities = Submenu::new("Start activity", !model.activities.is_empty());
    for (index, entry) in model.activities.iter().enumerate() {
        activities.append(&CheckMenuItem::with_id(
            TrayCommand::StartActivity(index).menu_id(),
            escape_menu_label(&entry.name),
            true,
            entry.is_active,
            None,
        ))?;
    }

    let menu = Menu::new();
    menu.append_items(&[
        &MenuItem::with_id(TrayCommand::ShowWindow.menu_id(), "Show window", true, None),
        &PredefinedMenuItem::separator(),
        &activities,
        &MenuItem::with_id(
            TrayCommand::StopActivity.menu_id(),
            "Stop activity",
            model.can_stop,
            None,
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(TrayCommand::Quit.menu_id(), "Quit", true, None),
    ])?;
    Ok(menu)
}
