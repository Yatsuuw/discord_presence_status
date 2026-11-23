//! Executable entry point of Discord Presence Status.
//!
//! All the logic lives in the library crate; this file only starts the user
//! interface and turns a fatal start-up error into a non-zero exit code.

// Release builds are GUI applications: do not open a console window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::ExitCode;

fn main() -> ExitCode {
    let launch_mode =
        discord_presence_status::launch::parse_launch_mode(std::env::args_os().skip(1));
    match discord_presence_status::ui::run_app(launch_mode) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Discord Presence Status failed to start: {error}");
            ExitCode::FAILURE
        }
    }
}
