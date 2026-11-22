//! Tests of the payload sent to Discord.

use discord_presence_status::activity::{Activity, ActivityKind};
use discord_presence_status::presence::{PresenceRequest, build_discord_activity};
use serde_json::json;

#[test]
fn sends_every_field_with_the_timer_start() {
    let request = PresenceRequest {
        activity: Activity {
            name: "Rust programming".to_owned(),
            application_id: "123456789012345678".to_owned(),
            kind: ActivityKind::Watching,
            details: Some("Line one".to_owned()),
            state: Some("Line two".to_owned()),
            large_image: Some("rust_logo".to_owned()),
            large_text: Some("Ferris".to_owned()),
            local_image: Some("image.png".to_owned()),
        },
        started_at_millis: 1_700_000_000_000,
    };

    let payload = serde_json::to_value(build_discord_activity(&request)).expect("serialize");

    assert_eq!(
        payload,
        json!({
            "name": "Rust programming",
            "type": 3,
            "details": "Line one",
            "state": "Line two",
            "timestamps": { "start": 1_700_000_000_000_i64 },
            "assets": { "large_image": "rust_logo", "large_text": "Ferris" },
        })
    );
}

#[test]
fn omits_optional_fields_that_are_not_set() {
    let request = PresenceRequest {
        activity: Activity {
            name: "Minimal".to_owned(),
            application_id: "123456789012345678".to_owned(),
            ..Activity::default()
        },
        started_at_millis: 42,
    };

    let payload = serde_json::to_value(build_discord_activity(&request)).expect("serialize");

    assert_eq!(
        payload,
        json!({ "name": "Minimal", "type": 0, "timestamps": { "start": 42 } })
    );
}
