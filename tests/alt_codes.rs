//! Tests of the Alt code support (for example Alt+144 = É).

use discord_presence_status::ui::keyboard::{
    AltCodeInput, apply_alt_code_input, observe_keyboard_message,
};
use eframe::egui::{Event, RawInput};

const WM_KEYDOWN: u32 = 0x0100;
const WM_SYSKEYDOWN: u32 = 0x0104;
const WM_CHAR: u32 = 0x0102;
const VK_NUMPAD1: usize = 0x61;
const VK_NUMPAD4: usize = 0x64;
const KEY_PRESSED: isize = 0x0001;
/// lParam of the character sent when Alt is released (scan code of Alt, release bit set).
const KEY_RELEASED: isize = 0xC038_0001_u32.cast_signed() as isize;

fn texts(raw_input: &RawInput) -> Vec<String> {
    raw_input
        .events
        .iter()
        .filter_map(|event| match event {
            Event::Text(text) => Some(text.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn recognises_the_messages_of_alt_144() {
    let mut input = AltCodeInput::default();
    for key in [VK_NUMPAD1, VK_NUMPAD4, VK_NUMPAD4] {
        assert!(observe_keyboard_message(
            &mut input,
            WM_SYSKEYDOWN,
            key,
            KEY_PRESSED
        ));
    }
    assert!(observe_keyboard_message(
        &mut input,
        WM_CHAR,
        0xC9,
        KEY_RELEASED
    ));

    assert_eq!(input.digits_to_remove, 3);
    assert_eq!(input.characters, ['É']);
}

#[test]
fn ignores_normal_typing_and_altgr() {
    let mut input = AltCodeInput::default();
    // A character typed normally (or with AltGr) comes with a key press.
    assert!(!observe_keyboard_message(
        &mut input,
        WM_CHAR,
        '@' as usize,
        KEY_PRESSED
    ));
    // Digits typed without Alt are ordinary key presses.
    assert!(!observe_keyboard_message(
        &mut input,
        WM_KEYDOWN,
        VK_NUMPAD1,
        KEY_PRESSED
    ));
    // Control characters are never inserted.
    assert!(!observe_keyboard_message(
        &mut input,
        WM_CHAR,
        0x08,
        KEY_RELEASED
    ));

    assert_eq!(input, AltCodeInput::default());
}

#[test]
fn replaces_the_typed_digits_with_the_composed_character() {
    let mut input = AltCodeInput {
        digits_to_remove: 3,
        characters: vec!['É'],
    };
    let mut raw_input = RawInput::default();
    for text in ["1", "4", "4"] {
        raw_input.events.push(Event::Text(text.to_owned()));
    }

    apply_alt_code_input(&mut input, &mut raw_input);

    assert_eq!(texts(&raw_input), ["É"]);
    assert_eq!(
        input,
        AltCodeInput::default(),
        "corrections are applied once"
    );
}

#[test]
fn keeps_other_text_and_digits_typed_normally() {
    let mut input = AltCodeInput {
        digits_to_remove: 1,
        characters: Vec::new(),
    };
    let mut raw_input = RawInput::default();
    for text in ["a", "1", "2"] {
        raw_input.events.push(Event::Text(text.to_owned()));
    }

    apply_alt_code_input(&mut input, &mut raw_input);

    assert_eq!(texts(&raw_input), ["a", "2"]);
}

#[test]
fn does_not_insert_a_character_twice() {
    let mut input = AltCodeInput {
        digits_to_remove: 0,
        characters: vec!['É'],
    };
    let mut raw_input = RawInput::default();
    raw_input.events.push(Event::Text("É".to_owned()));

    apply_alt_code_input(&mut input, &mut raw_input);

    assert_eq!(texts(&raw_input), ["É"]);
}
