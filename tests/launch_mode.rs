//! Tests of the command-line launch modes.

use std::ffi::OsString;
use std::path::PathBuf;

use discord_presence_status::launch::{
    LaunchMode, PresenceHandover, background_arguments, parse_launch_mode,
};

fn arguments(values: &[&str]) -> Vec<OsString> {
    values.iter().map(OsString::from).collect()
}

#[test]
fn opens_the_window_by_default() {
    assert_eq!(parse_launch_mode(arguments(&[])), LaunchMode::Window);
    assert_eq!(
        parse_launch_mode(arguments(&["--unknown"])),
        LaunchMode::Window
    );
}

#[test]
fn round_trips_a_background_start_with_its_activity() {
    let handover = PresenceHandover {
        folder: PathBuf::from(r"C:\Users\me\Discord Presence Status\user_activity\Rust coding"),
        started_at_millis: 1_700_000_000_123,
    };

    let parsed = parse_launch_mode(background_arguments(Some(&handover)));

    assert_eq!(parsed, LaunchMode::Background(Some(handover)));
}

#[test]
fn starts_in_background_without_activity() {
    assert_eq!(
        parse_launch_mode(background_arguments(None)),
        LaunchMode::Background(None)
    );
}

#[test]
fn ignores_an_invalid_timer_start() {
    let parsed = parse_launch_mode(arguments(&[
        "--background",
        "--activity",
        "folder",
        "--started-at",
        "yesterday",
    ]));
    assert_eq!(parsed, LaunchMode::Background(None));
}
