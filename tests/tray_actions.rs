//! Tests of the actions behind the tray icon, run without a native tray.

use std::path::Path;
use std::sync::mpsc;

use discord_presence_status::activity::Activity;
use discord_presence_status::presence::{PresenceCommand, PresenceHandle};
use discord_presence_status::storage::save_activity;
use discord_presence_status::ui::actions::{apply_action, poll_tray_commands, reload_activities};
use discord_presence_status::ui::state::{
    ActivePresence, NoticeKind, PresenceApp, keeps_running_in_tray,
};
use discord_presence_status::ui::views::detect_hide_to_tray;
use discord_presence_status::ui::{UiAction, build_presence_app};
use eframe::egui::{self, ViewportCommand, ViewportEvent, ViewportId};

/// Builds the application on `folder` with its presence worker replaced by a
/// channel, so the tests never talk to Discord.
fn app_with_fake_worker(
    context: &egui::Context,
    folder: &Path,
) -> (PresenceApp, mpsc::Receiver<PresenceCommand>) {
    let mut app = build_presence_app(context, folder).expect("build the app");
    let (commands, received) = mpsc::channel();
    let (_statuses_sender, statuses) = mpsc::channel();
    app.presence = PresenceHandle {
        commands,
        statuses,
        thread: None,
    };
    (app, received)
}

/// Applies `action` inside a frame and returns the window commands it sent.
fn apply_in_frame(
    context: &egui::Context,
    app: &mut PresenceApp,
    action: UiAction,
) -> Vec<ViewportCommand> {
    let mut action = Some(action);
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        if let Some(action) = action.take() {
            apply_action(app, action, ui.ctx());
        }
    });
    // No renderer here: drop the font texture instead of uploading it.
    output.textures_delta.clear();
    output
        .viewport_output
        .get(&ViewportId::ROOT)
        .map(|viewport| viewport.commands.clone())
        .unwrap_or_default()
}

#[test]
fn starts_a_saved_activity_without_opening_it() {
    let root = tempfile::tempdir().expect("temporary folder");
    let context = egui::Context::default();
    let (mut app, received) = app_with_fake_worker(&context, root.path());
    let activity = Activity {
        name: "Coding".to_owned(),
        application_id: "123456789012345678".to_owned(),
        ..Activity::default()
    };
    let stored = save_activity(&app.activities_directory, activity, None, None).expect("save");
    reload_activities(&mut app);

    apply_action(
        &mut app,
        UiAction::StartActivity(stored.folder.clone()),
        &context,
    );

    match received.try_recv() {
        Ok(PresenceCommand::Show(request)) => assert_eq!(request.activity, stored.activity),
        other => panic!("expected a Show command, got {other:?}"),
    }
    let active = app.active.as_ref().expect("active activity");
    assert_eq!(active.folder, stored.folder);
    assert!(app.draft.is_none(), "the editor must stay closed");
}

#[test]
fn reports_an_activity_that_no_longer_exists() {
    let root = tempfile::tempdir().expect("temporary folder");
    let context = egui::Context::default();
    let (mut app, received) = app_with_fake_worker(&context, root.path());

    apply_action(
        &mut app,
        UiAction::StartActivity(root.path().join("Missing")),
        &context,
    );

    assert!(received.try_recv().is_err());
    assert!(app.active.is_none());
    let notice = app.notice.as_ref().expect("error message");
    assert_eq!(notice.kind, NoticeKind::Error);
}

#[test]
fn hides_and_shows_the_window() {
    let root = tempfile::tempdir().expect("temporary folder");
    let context = egui::Context::default();
    let (mut app, _received) = app_with_fake_worker(&context, root.path());

    let commands = apply_in_frame(&context, &mut app, UiAction::HideToTray);
    assert!(app.window_hidden);
    assert!(commands.contains(&ViewportCommand::CancelClose));
    assert!(commands.contains(&ViewportCommand::Visible(false)));

    let commands = apply_in_frame(&context, &mut app, UiAction::ShowWindow);
    assert!(!app.window_hidden);
    assert!(commands.contains(&ViewportCommand::Visible(true)));
    assert!(commands.contains(&ViewportCommand::Minimized(false)));
    assert!(commands.contains(&ViewportCommand::Focus));
}

#[test]
fn quit_closes_the_window_for_real() {
    let root = tempfile::tempdir().expect("temporary folder");
    let context = egui::Context::default();
    let (mut app, _received) = app_with_fake_worker(&context, root.path());
    app.active = Some(ActivePresence {
        folder: root.path().join("Coding"),
        started_at_millis: 0,
    });

    let commands = apply_in_frame(&context, &mut app, UiAction::Quit);

    assert!(app.quit_requested);
    assert!(commands.contains(&ViewportCommand::Close));
    assert!(!keeps_running_in_tray(&app));
}

#[test]
fn closing_quits_when_there_is_no_tray_icon() {
    let root = tempfile::tempdir().expect("temporary folder");
    let context = egui::Context::default();
    let (mut app, _received) = app_with_fake_worker(&context, root.path());
    app.active = Some(ActivePresence {
        folder: root.path().join("Coding"),
        started_at_millis: 0,
    });
    assert!(!keeps_running_in_tray(&app));
    assert_eq!(poll_tray_commands(&mut app), Vec::new());

    let mut input = egui::RawInput::default();
    input
        .viewports
        .entry(ViewportId::ROOT)
        .or_default()
        .events
        .push(ViewportEvent::Close);
    let mut detected = None;
    let mut output = context.run_ui(input, |ui| detected = detect_hide_to_tray(ui.ctx(), &app));
    output.textures_delta.clear();

    assert_eq!(detected, None);
}
