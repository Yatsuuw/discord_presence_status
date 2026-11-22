//! Tests of the presence worker scheduling.

use discord_presence_status::activity::Activity;
use discord_presence_status::constants::{PRESENCE_REFRESH_INTERVAL, PRESENCE_RETRY_INTERVAL};
use discord_presence_status::presence::{
    PresenceRequest, WorkerState, next_wake_delay, sync_presence,
};

fn request() -> PresenceRequest {
    PresenceRequest {
        activity: Activity {
            name: "Testing".to_owned(),
            application_id: "123456789012345678".to_owned(),
            ..Activity::default()
        },
        started_at_millis: 0,
    }
}

#[test]
fn sleeps_until_the_next_command_when_idle() {
    assert_eq!(next_wake_delay(&WorkerState::default()), None);
}

#[test]
fn retries_quickly_while_waiting_for_discord() {
    let state = WorkerState {
        desired: Some(request()),
        pending: true,
        ..WorkerState::default()
    };
    assert_eq!(next_wake_delay(&state), Some(PRESENCE_RETRY_INTERVAL));
}

#[test]
fn refreshes_slowly_once_displayed() {
    let state = WorkerState {
        desired: Some(request()),
        ..WorkerState::default()
    };
    assert_eq!(next_wake_delay(&state), Some(PRESENCE_REFRESH_INTERVAL));
}

#[test]
fn stops_retrying_after_a_refusal() {
    let mut state = WorkerState {
        desired: Some(request()),
        rejected: true,
        ..WorkerState::default()
    };
    assert_eq!(next_wake_delay(&state), None);
    assert_eq!(sync_presence(&mut state), None);
}

#[test]
fn does_nothing_without_a_desired_activity() {
    assert_eq!(sync_presence(&mut WorkerState::default()), None);
}
