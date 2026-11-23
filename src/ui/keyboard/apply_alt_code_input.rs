//! Correction of the keyboard input of a frame.

use eframe::egui::{Event, RawInput};

use crate::ui::keyboard::AltCodeInput;

/// Applies the Alt code corrections gathered since the last frame to
/// `raw_input`, then clears them.
///
/// The keypad digits typed during an Alt code are removed, and the composed
/// characters are inserted, unless egui already received them (should a
/// future winit version fix the problem itself).
pub fn apply_alt_code_input(input: &mut AltCodeInput, raw_input: &mut RawInput) {
    let mut digits_to_remove = std::mem::take(&mut input.digits_to_remove);
    raw_input.events.retain(|event| {
        let is_digit = matches!(event, Event::Text(text)
            if text.len() == 1 && text.bytes().all(|byte| byte.is_ascii_digit()));
        if is_digit && digits_to_remove > 0 {
            digits_to_remove -= 1;
            false
        } else {
            true
        }
    });

    for character in std::mem::take(&mut input.characters) {
        let text = character.to_string();
        let already_received = raw_input
            .events
            .iter()
            .any(|event| matches!(event, Event::Text(existing) if *existing == text));
        if !already_received {
            raw_input.events.push(Event::Text(text));
        }
    }
}
