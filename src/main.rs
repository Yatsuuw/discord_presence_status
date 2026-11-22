//! Executable entry point of Discord Presence Status.
//!
//! All the logic lives in the library crate; this file only starts the user
//! interface and turns a fatal start-up error into a non-zero exit code.

// Release builds are GUI applications: do not open a console window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::ExitCode;

fn main() -> ExitCode {
    match discord_presence_status::ui::run_app() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Discord Presence Status failed to start: {error}");
            ExitCode::FAILURE
        }
    }
}
