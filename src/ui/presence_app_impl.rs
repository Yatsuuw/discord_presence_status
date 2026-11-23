//! The per-frame entry point of the user interface.

use std::time::Duration;

use eframe::egui::{self, CentralPanel, Panel};

use crate::i18n::translations_for;
use crate::presence::PresenceStatus;
use crate::ui::UiAction;
use crate::ui::actions::{apply_action, poll_presence_status, poll_tray_commands, sync_tray_menu};
use crate::ui::state::{Dialog, PresenceApp};
use crate::ui::views::{
    detect_hide_to_tray, draw_about_dialog, draw_activity_editor, draw_activity_list,
    draw_delete_dialog, draw_footer, draw_header, draw_welcome,
};

/// Width of the activity list when the window opens, in logical pixels.
const ACTIVITY_LIST_WIDTH: f32 = 230.0;

/// How often the window refreshes while the Discord timer is running.
const TIMER_REFRESH: Duration = Duration::from_secs(1);

impl eframe::App for PresenceApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        poll_presence_status(self);

        let mut actions: Vec<UiAction> = poll_tray_commands(self);
        actions.extend(detect_hide_to_tray(ui.ctx(), self));
        let active_folder = self.active.as_ref().map(|active| active.folder.clone());
        let texts = translations_for(self.language);

        Panel::top("header").show(ui, |ui| {
            actions.extend(draw_header(ui, &self.status, self.language, texts));
        });

        Panel::bottom("footer").show(ui, |ui| {
            actions.extend(draw_footer(
                ui,
                self.notice.as_ref(),
                &self.activities_directory,
                texts,
            ));
        });

        Panel::left("activities")
            .default_size(ACTIVITY_LIST_WIDTH)
            .show(ui, |ui| {
                let selected = self
                    .draft
                    .as_ref()
                    .and_then(|draft| draft.folder.as_deref());
                actions.extend(draw_activity_list(
                    ui,
                    &self.activities,
                    &self.load_errors,
                    selected,
                    active_folder.as_deref(),
                    texts,
                ));
            });

        CentralPanel::default_margins().show(ui, |ui| match &mut self.draft {
            Some(draft) => {
                let is_active = draft.folder.is_some() && draft.folder == active_folder;
                actions.extend(draw_activity_editor(ui, draft, is_active, texts));
            }
            None => actions.extend(draw_welcome(ui, texts)),
        });

        if self.dialog == Some(Dialog::DeleteConfirmation) {
            let name = self.draft.as_ref().map_or("", |draft| draft.name.as_str());
            actions.extend(draw_delete_dialog(ui.ctx(), name, texts));
        }

        if self.dialog == Some(Dialog::About) {
            actions.extend(draw_about_dialog(ui.ctx(), texts));
        }

        for action in actions {
            apply_action(self, action, ui.ctx());
        }

        sync_tray_menu(self);

        if matches!(self.status, PresenceStatus::Active { .. }) {
            ui.ctx().request_repaint_after(TIMER_REFRESH);
        }
    }
}
