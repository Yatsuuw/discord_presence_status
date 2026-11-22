//! Refresh of the tray menu and tooltip.

use crate::i18n::{fill_placeholders, translations_for};
use crate::ui::actions::show_error;
use crate::ui::state::PresenceApp;
use crate::ui::tray::{build_tray_menu, build_tray_menu_model};

/// Updates the tray menu and tooltip when the activities or the displayed
/// activity changed since they were last installed.
///
/// Does nothing when the application runs without a tray icon.
pub fn sync_tray_menu(app: &mut PresenceApp) {
    let model = build_tray_menu_model(&app.activities, app.active.as_ref(), app.language);
    let Some(tray) = app.tray.as_mut() else {
        return;
    };
    if tray.shown_model.as_ref() == Some(&model) {
        return;
    }

    let result = build_tray_menu(&model).map(|menu| {
        tray.icon.set_menu(Some(Box::new(menu)));
    });
    let _ = tray.icon.set_tooltip(Some(&model.tooltip));
    // Remember the model even on failure, to avoid retrying every frame.
    tray.shown_model = Some(model);

    if let Err(error) = result {
        let texts = translations_for(app.language);
        let text = fill_placeholders(texts.error_tray_menu, &[("detail", &error.to_string())]);
        show_error(app, text);
    }
}
