//! Watching of the Windows keyboard messages of the window.

use std::sync::Arc;

use crate::ui::keyboard::{SharedAltCodeInput, observe_keyboard_message};
use crate::ui::repaint_window;
use crate::ui::state::RepaintTarget;

/// Asks winit to show every Windows message to [`observe_keyboard_message`]
/// before handling it, so Alt codes can be corrected.
///
/// The messages are only observed, never consumed: winit and Windows keep
/// handling them as usual. The window is woken up when a composed character
/// arrives, because winit itself produces no event for it.
#[cfg(windows)]
#[allow(unsafe_code)]
pub fn install_alt_code_hook<T>(
    builder: &mut winit::event_loop::EventLoopBuilder<T>,
    input: &SharedAltCodeInput,
    repaint_target: &RepaintTarget,
) {
    use windows_sys::Win32::UI::WindowsAndMessaging::MSG;
    use winit::platform::windows::EventLoopBuilderExtWindows;

    let input = Arc::clone(input);
    let repaint_target = Arc::clone(repaint_target);
    builder.with_msg_hook(move |message| {
        // SAFETY: winit documents that the hook receives a pointer to the
        // `MSG` it is about to dispatch, valid for the duration of the call.
        let message = unsafe { &*message.cast::<MSG>() };
        let changed = input.lock().is_ok_and(|mut input| {
            observe_keyboard_message(&mut input, message.message, message.wParam, message.lParam)
        });
        if changed {
            repaint_window(&repaint_target);
        }
        false
    });
}

/// Other systems deliver composed characters normally: nothing to watch.
#[cfg(not(windows))]
pub fn install_alt_code_hook<T>(
    _builder: &mut T,
    _input: &SharedAltCodeInput,
    _repaint_target: &RepaintTarget,
) {
}
