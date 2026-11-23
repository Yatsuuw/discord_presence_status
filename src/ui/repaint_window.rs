//! Waking up of the main window from any thread.

use crate::ui::state::RepaintTarget;

/// Asks the open window, if any, to refresh.
///
/// Does nothing while the window is closed: the tray loop then reacts to
/// tray events directly.
pub fn repaint_window(target: &RepaintTarget) {
    if let Ok(context) = target.lock()
        && let Some(context) = context.as_ref()
    {
        context.request_repaint();
    }
}
