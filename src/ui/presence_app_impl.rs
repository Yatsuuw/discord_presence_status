//! The per-frame entry point of the user interface.

use std::time::Duration;

use eframe::egui::{self, CentralPanel, Panel};

use crate::presence::{PresenceStatus, shutdown_presence_worker};
use crate::ui::UiAction;
use crate::ui::actions::{apply_action, poll_presence_status, poll_tray_commands, sync_tray_menu};
use crate::ui::state::PresenceApp;
use crate::ui::views::{
    detect_hide_to_tray, draw_activity_editor, draw_activity_list, draw_delete_dialog, draw_footer,
    draw_header, draw_welcome,
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

        Panel::top("header").show(ui, |ui| draw_header(ui, &self.status));

        Panel::bottom("footer").show(ui, |ui| {
            actions.extend(draw_footer(
                ui,
                self.notice.as_ref(),
                &self.activities_directory,
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
                ));
            });

        CentralPanel::default_margins().show(ui, |ui| match &mut self.draft {
            Some(draft) => {
                let is_active = draft.folder.is_some() && draft.folder == active_folder;
                actions.extend(draw_activity_editor(ui, draft, is_active));
            }
            None => actions.extend(draw_welcome(ui)),
        });

        if self.delete_requested {
            let name = self.draft.as_ref().map_or("", |draft| draft.name.as_str());
            actions.extend(draw_delete_dialog(ui.ctx(), name));
        }

        for action in actions {
            apply_action(self, action, ui.ctx());
        }

        sync_tray_menu(self);

        // The timer is only visible in the window: no need to refresh it
        // while the window is hidden to the tray.
        if matches!(self.status, PresenceStatus::Active { .. }) && !self.window_hidden {
            ui.ctx().request_repaint_after(TIMER_REFRESH);
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        shutdown_presence_worker(&mut self.presence);
        // Remove the icon from the notification area right away.
        self.tray = None;
    }
}
