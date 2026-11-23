//! Recognition of the Windows messages involved in an Alt code.

use crate::ui::keyboard::AltCodeInput;

/// `WM_SYSKEYDOWN`: a key pressed while Alt is held (without Ctrl).
const WM_SYSKEYDOWN: u32 = 0x0104;

/// `WM_CHAR`: a character produced by the keyboard.
const WM_CHAR: u32 = 0x0102;

/// Virtual-key codes of the keypad digits 0 to 9 (`VK_NUMPAD0`..`VK_NUMPAD9`).
const KEYPAD_DIGITS: std::ops::RangeInclusive<usize> = 0x60..=0x69;

/// Bit of `lParam` set when a message comes from a key release.
const KEY_RELEASED_BIT: isize = 1 << 31;

/// Updates `input` from one Windows keyboard message and tells whether it
/// changed.
///
/// * A keypad digit pressed while Alt is held (`WM_SYSKEYDOWN`) is part of
///   an Alt code: the digit winit types for it must be removed.
/// * A character sent on a key release (`WM_CHAR` from the Alt release) is
///   the composed character: it must be inserted. Normal typing and `AltGr`
///   produce their characters on key presses and are left alone.
pub fn observe_keyboard_message(
    input: &mut AltCodeInput,
    message: u32,
    wparam: usize,
    lparam: isize,
) -> bool {
    match message {
        WM_SYSKEYDOWN if KEYPAD_DIGITS.contains(&wparam) => {
            input.digits_to_remove += 1;
            true
        }
        WM_CHAR if lparam & KEY_RELEASED_BIT != 0 => {
            let character = u32::try_from(wparam).ok().and_then(char::from_u32);
            match character {
                Some(character) if !character.is_control() => {
                    input.characters.push(character);
                    true
                }
                _ => false,
            }
        }
        _ => false,
    }
}
