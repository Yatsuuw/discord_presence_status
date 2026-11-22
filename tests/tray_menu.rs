//! Tests of the tray menu description, independent of the native tray.

use std::path::PathBuf;

use discord_presence_status::activity::Activity;
use discord_presence_status::constants::APP_DISPLAY_NAME;
use discord_presence_status::storage::StoredActivity;
use discord_presence_status::ui::state::ActivePresence;
use discord_presence_status::ui::tray::{
    TrayCommand, TrayMenuEntry, build_tray_menu, build_tray_menu_model, escape_menu_label,
    tray_tooltip,
};

fn stored(name: &str) -> StoredActivity {
    StoredActivity {
        folder: PathBuf::from("user_activity").join(name),
        activity: Activity {
            name: name.to_owned(),
            application_id: "123456789012345678".to_owned(),
            ..Activity::default()
        },
    }
}

#[test]
fn menu_identifiers_round_trip() {
    let commands = [
        TrayCommand::ShowWindow,
        TrayCommand::StartActivity(0),
        TrayCommand::StartActivity(42),
        TrayCommand::StopActivity,
        TrayCommand::Quit,
    ];
    for command in commands {
        assert_eq!(TrayCommand::from_menu_id(&command.menu_id()), Some(command));
    }
}

#[test]
fn unknown_menu_identifiers_are_ignored() {
    for id in [
        "",
        "other",
        "start-activity:",
        "start-activity:abc",
        "quit ",
    ] {
        assert_eq!(TrayCommand::from_menu_id(id), None, "{id:?}");
    }
}

#[test]
fn lists_every_activity_and_marks_the_displayed_one() {
    let activities = [stored("Coding"), stored("Gaming")];
    let active = ActivePresence {
        folder: activities[1].folder.clone(),
        started_at_millis: 0,
    };

    let model = build_tray_menu_model(&activities, Some(&active));

    assert_eq!(
        model.activities,
        vec![
            TrayMenuEntry {
                name: "Coding".to_owned(),
                folder: activities[0].folder.clone(),
                is_active: false,
            },
            TrayMenuEntry {
                name: "Gaming".to_owned(),
                folder: activities[1].folder.clone(),
                is_active: true,
            },
        ]
    );
    assert!(model.can_stop);
    assert_eq!(model.tooltip, tray_tooltip(Some("Gaming")));
}

#[test]
fn cannot_stop_when_nothing_is_displayed() {
    let model = build_tray_menu_model(&[stored("Coding")], None);

    assert!(!model.can_stop);
    assert!(model.activities.iter().all(|entry| !entry.is_active));
    assert_eq!(model.tooltip, APP_DISPLAY_NAME);
}

#[test]
fn tooltip_names_the_displayed_activity_and_stays_short() {
    assert_eq!(
        tray_tooltip(Some("Coding")),
        format!("{APP_DISPLAY_NAME}\nDisplaying \"Coding\"")
    );

    let long_name = "x".repeat(200);
    assert_eq!(tray_tooltip(Some(&long_name)).chars().count(), 120);
}

#[test]
fn ampersands_are_shown_as_typed() {
    assert_eq!(escape_menu_label("Rock & Roll"), "Rock && Roll");
    assert_eq!(escape_menu_label("Plain"), "Plain");
}

#[test]
fn builds_the_native_menu() {
    let activities = [stored("Coding"), stored("Rock & Roll")];
    let empty = build_tray_menu_model(&[], None);
    let full = build_tray_menu_model(&activities, None);

    assert!(build_tray_menu(&empty).is_ok());
    let menu = build_tray_menu(&full).expect("menu");
    // Show window, separator, Start activity, Stop activity, separator, Quit.
    assert_eq!(menu.items().len(), 6);
}
