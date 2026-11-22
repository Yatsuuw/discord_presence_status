//! Creation of the native tray menu from its description.

use tray_icon::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};

use crate::i18n::translations_for;
use crate::ui::tray::{TrayCommand, TrayMenuModel, escape_menu_label};

/// Builds the native context menu described by `model`, in its language:
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
    let texts = translations_for(model.language);
    let activities = Submenu::new(texts.tray_start_activity, !model.activities.is_empty());
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
        &MenuItem::with_id(
            TrayCommand::ShowWindow.menu_id(),
            texts.tray_show_window,
            true,
            None,
        ),
        &PredefinedMenuItem::separator(),
        &activities,
        &MenuItem::with_id(
            TrayCommand::StopActivity.menu_id(),
            texts.tray_stop_activity,
            model.can_stop,
            None,
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(TrayCommand::Quit.menu_id(), texts.tray_quit, true, None),
    ])?;
    Ok(menu)
}
