//! Tests of the activity validation rules.

use discord_presence_status::activity::{Activity, ValidationError, validate_activity};

fn valid_activity() -> Activity {
    Activity {
        name: "Rust programming".to_owned(),
        application_id: "123456789012345678".to_owned(),
        details: Some("Building a desktop app".to_owned()),
        large_image: Some("https://example.com/rust.png".to_owned()),
        ..Activity::default()
    }
}

#[test]
fn accepts_a_complete_activity() {
    assert_eq!(validate_activity(&valid_activity()), Ok(()));
}

#[test]
fn accepts_an_activity_with_only_required_fields() {
    let activity = Activity {
        details: None,
        large_image: None,
        ..valid_activity()
    };
    assert_eq!(validate_activity(&activity), Ok(()));
}

#[test]
fn rejects_a_blank_name() {
    let activity = Activity {
        name: "   ".to_owned(),
        ..valid_activity()
    };
    assert_eq!(
        validate_activity(&activity),
        Err(ValidationError::MissingName)
    );
}

#[test]
fn rejects_a_name_unusable_as_a_folder() {
    let activity = Activity {
        name: "...".to_owned(),
        ..valid_activity()
    };
    assert_eq!(
        validate_activity(&activity),
        Err(ValidationError::UnusableName)
    );
}

#[test]
fn rejects_malformed_application_ids() {
    for application_id in ["", "12345", "12345678901234567890123", "1234567890123456a8"] {
        let activity = Activity {
            application_id: application_id.to_owned(),
            ..valid_activity()
        };
        assert_eq!(
            validate_activity(&activity),
            Err(ValidationError::InvalidApplicationId),
            "{application_id:?} should be rejected"
        );
    }
}

#[test]
fn rejects_texts_discord_would_refuse() {
    let too_short = Activity {
        details: Some("x".to_owned()),
        ..valid_activity()
    };
    let too_long = Activity {
        state: Some("x".repeat(129)),
        ..valid_activity()
    };
    assert!(matches!(
        validate_activity(&too_short),
        Err(ValidationError::InvalidLength { .. })
    ));
    assert!(matches!(
        validate_activity(&too_long),
        Err(ValidationError::InvalidLength { .. })
    ));
}

#[test]
fn counts_characters_rather_than_bytes() {
    let activity = Activity {
        details: Some("é".repeat(128)),
        ..valid_activity()
    };
    assert_eq!(validate_activity(&activity), Ok(()));
}

#[test]
fn accepts_asset_keys_and_https_urls_only() {
    for (reference, expected_valid) in [
        ("rust_logo", true),
        ("https://example.com/a.png", true),
        ("http://example.com/a.png", false),
        ("file:///C:/image.png", false),
        ("has space", false),
    ] {
        let activity = Activity {
            large_image: Some(reference.to_owned()),
            ..valid_activity()
        };
        assert_eq!(
            validate_activity(&activity).is_ok(),
            expected_valid,
            "unexpected result for {reference:?}"
        );
    }
}
