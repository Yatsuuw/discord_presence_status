//! Start and life cycle of the desktop application.

use crate::error::AppError;
use crate::launch::{LaunchMode, spawn_background_process};
use crate::presence::shutdown_presence_worker;
use crate::ui::actions::resume_presence;
use crate::ui::state::{TrayWakeUp, WindowExit, keeps_running_in_tray, presence_handover_of};
use crate::ui::{create_presence_app, run_window, wait_in_tray};

/// Runs the application until the user quits.
///
/// ```text
///   normal start ─▶ window ── closed while an activity is shown ──▶ new process
///                     ▲                                             in background
///                     └──────────── click on the tray icon ◀────────────┘
/// ```
///
/// Once a window has been shown, the graphics driver keeps its memory until
/// the process ends. Hiding to the tray therefore hands the displayed
/// activity (and its timer) over to a fresh process started with
/// [`LaunchMode::Background`], which only runs the tray icon and the Discord
/// connection and uses a few megabytes. If that process cannot be started,
/// this one keeps running behind the tray icon instead.
///
/// # Errors
///
/// Returns an [`AppError`] when the application state cannot be prepared or
/// the window cannot be created.
pub fn run_app(launch_mode: LaunchMode) -> Result<(), AppError> {
    let mut app = create_presence_app()?;

    // A background start without a tray icon would be invisible: show the
    // window instead.
    let mut window_wanted = match launch_mode {
        LaunchMode::Window => true,
        LaunchMode::Background(handover) => {
            if let Some(handover) = &handover {
                resume_presence(&mut app, handover);
            }
            app.tray.is_none()
        }
    };

    let result = loop {
        if window_wanted {
            if let Err(error) = run_window(&mut app) {
                break Err(error);
            }
            let hides =
                app.window_exit == Some(WindowExit::HideToTray) && keeps_running_in_tray(&app);
            if !hides {
                break Ok(());
            }
            if spawn_background_process(presence_handover_of(&app).as_ref()).is_ok() {
                break Ok(());
            }
        }
        if wait_in_tray(&mut app) == TrayWakeUp::Quit {
            break Ok(());
        }
        window_wanted = true;
    };

    shutdown_presence_worker(&mut app.presence);
    // Remove the icon from the notification area right away.
    app.tray = None;
    result
}
