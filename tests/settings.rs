//! Tests of the preferences file.

use discord_presence_status::error::AppError;
use discord_presence_status::i18n::Language;
use discord_presence_status::settings::{
    Settings, load_settings, save_settings, settings_file_path,
};

#[test]
fn uses_the_defaults_on_first_launch() {
    let root = tempfile::tempdir().expect("temporary folder");
    let settings = load_settings(&settings_file_path(root.path())).expect("load");
    assert_eq!(settings, Settings::default());
    assert_eq!(settings.language, Language::English);
}

#[test]
fn remembers_the_language() {
    let root = tempfile::tempdir().expect("temporary folder");
    let path = settings_file_path(root.path());
    let settings = Settings {
        language: Language::French,
    };

    save_settings(&path, &settings).expect("save");

    assert_eq!(load_settings(&path).expect("load"), settings);
    let content = std::fs::read_to_string(&path).expect("read");
    assert!(content.contains("language = \"fr\""), "{content}");
}

#[test]
fn creates_the_data_folder_when_needed() {
    let root = tempfile::tempdir().expect("temporary folder");
    let path = settings_file_path(&root.path().join("not yet created"));
    save_settings(&path, &Settings::default()).expect("save");
    assert!(path.is_file());
}

#[test]
fn ignores_unknown_fields() {
    let root = tempfile::tempdir().expect("temporary folder");
    let path = settings_file_path(root.path());
    std::fs::write(&path, "language = \"fr\"\nfuture_option = true\n").expect("write");

    assert_eq!(
        load_settings(&path).expect("load").language,
        Language::French
    );
}

#[test]
fn reports_an_invalid_file() {
    let root = tempfile::tempdir().expect("temporary folder");
    let path = settings_file_path(root.path());
    std::fs::write(&path, "language = \"klingon\"\n").expect("write");

    assert!(matches!(
        load_settings(&path),
        Err(AppError::SettingsParse { .. })
    ));
}
