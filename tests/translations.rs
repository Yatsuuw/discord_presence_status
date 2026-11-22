//! Tests guaranteeing that every language is complete and displayable.

use std::collections::BTreeSet;

use discord_presence_status::i18n::{ENGLISH, Language, Translations, translations_for};
use eframe::egui::{self, FontId};
use serde_json::{Map, Value};

/// Every text of `texts`, keyed by field name.
fn texts_of(texts: &Translations) -> Map<String, Value> {
    match serde_json::to_value(texts).expect("serialize the texts") {
        Value::Object(map) => map,
        other => panic!("unexpected serialization: {other}"),
    }
}

/// The `{placeholders}` used in `text`.
fn placeholders(text: &str) -> BTreeSet<String> {
    text.split('{')
        .skip(1)
        .filter_map(|part| part.split_once('}').map(|(name, _)| name.to_owned()))
        .collect()
}

#[test]
fn no_text_is_empty() {
    for language in Language::ALL {
        for (key, text) in texts_of(translations_for(language)) {
            let text = text.as_str().expect("text");
            assert!(!text.trim().is_empty(), "{language:?}: `{key}` is empty");
        }
    }
}

#[test]
fn every_language_uses_the_same_placeholders_as_english() {
    let english = texts_of(&ENGLISH);
    for language in Language::ALL {
        for (key, text) in texts_of(translations_for(language)) {
            let expected = placeholders(english[&key].as_str().expect("text"));
            let actual = placeholders(text.as_str().expect("text"));
            assert_eq!(actual, expected, "{language:?}: placeholders of `{key}`");
        }
    }
}

#[test]
fn every_character_exists_in_the_interface_font() {
    let context = egui::Context::default();
    let mut output = context.run_ui(egui::RawInput::default(), |_| {});
    output.textures_delta.clear();
    let font = FontId::proportional(14.0);

    for language in Language::ALL {
        for (key, text) in texts_of(translations_for(language)) {
            let text = text.as_str().expect("text");
            let missing: String = text
                .chars()
                .filter(|&character| !context.fonts_mut(|fonts| fonts.has_glyph(&font, character)))
                .collect();
            assert!(
                missing.is_empty(),
                "{language:?}: `{key}` uses characters missing from the font: {missing:?}"
            );
        }
    }
}
