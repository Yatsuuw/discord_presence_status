//! Waiting for the next tray icon event while no window is open.

/// Blocks until the next Windows message of this thread arrives, then
/// dispatches it to the tray icon, which turns clicks into tray commands.
///
/// While a window is open, its event loop does this job; without a window,
/// someone must keep receiving the messages of the tray icon's hidden window.
///
/// Returns `false` when Windows asks the thread to quit (`WM_QUIT`, for
/// example when the session ends) or when receiving fails.
#[cfg(windows)]
#[allow(unsafe_code)]
#[must_use]
pub fn wait_for_tray_message() -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        DispatchMessageW, GetMessageW, MSG, TranslateMessage,
    };

    // SAFETY: an all-zero MSG is a valid value for this plain C structure.
    let mut message: MSG = unsafe { std::mem::zeroed() };
    // SAFETY: `message` is a valid, writable MSG; a null window handle asks
    // for the messages of every window of the calling thread.
    let received = unsafe { GetMessageW(&raw mut message, std::ptr::null_mut(), 0, 0) };
    if received <= 0 {
        return false;
    }
    // SAFETY: `message` was just filled in by GetMessageW.
    unsafe {
        TranslateMessage(&raw const message);
        DispatchMessageW(&raw const message);
    }
    true
}

/// Other systems have no tray loop: stop waiting immediately.
#[cfg(not(windows))]
#[must_use]
pub fn wait_for_tray_message() -> bool {
    false
}
