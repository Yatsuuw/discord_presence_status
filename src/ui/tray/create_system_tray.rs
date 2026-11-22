//! Creation of the tray icon.

use std::sync::mpsc;

use eframe::egui;
use tray_icon::menu::MenuEvent;
use tray_icon::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

use crate::constants::APP_DISPLAY_NAME;
use crate::error::AppError;
use crate::ui::tray::{SystemTray, TrayCommand, load_tray_icon};

/// Adds the application icon to the notification area.
///
/// Menu clicks, and left clicks on the icon (which show the window), are
/// sent as [`TrayCommand`]s and wake the interface through `context`, even
/// while the window is hidden. The menu itself is installed by
/// [`sync_tray_menu`](crate::ui::actions::sync_tray_menu).
///
/// Must be called on the thread running the window event loop, after it
/// started (for example while creating the application state).
///
/// # Errors
///
/// Returns [`AppError::Tray`] when the operating system refuses to create
/// the icon.
pub fn create_system_tray(context: &egui::Context) -> Result<SystemTray, AppError> {
    let (sender, commands) = mpsc::channel();

    let menu_sender = sender.clone();
    let menu_context = context.clone();
    MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
        if let Some(command) = TrayCommand::from_menu_id(event.id.as_ref()) {
            let _ = menu_sender.send(command);
            menu_context.request_repaint();
        }
    }));

    let icon_context = context.clone();
    TrayIconEvent::set_event_handler(Some(move |event: TrayIconEvent| {
        let shows_window = matches!(
            event,
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } | TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            }
        );
        if shows_window {
            let _ = sender.send(TrayCommand::ShowWindow);
            icon_context.request_repaint();
        }
    }));

    let mut builder = TrayIconBuilder::new()
        .with_tooltip(APP_DISPLAY_NAME)
        .with_menu_on_left_click(false);
    if let Some(icon) = load_tray_icon() {
        builder = builder.with_icon(icon);
    }
    let icon = builder
        .build()
        .map_err(|error| AppError::Tray(error.to_string()))?;

    Ok(SystemTray {
        icon,
        commands,
        shown_model: None,
    })
}
