//! Conversion of a [`PresenceRequest`] into a Discord IPC payload.

use discord_rich_presence::activity::{Activity as DiscordActivity, Assets, Timestamps};

use crate::presence::{PresenceRequest, to_discord_activity_type};

/// Builds the Discord payload for `request`.
///
/// The payload always contains a start timestamp, which makes Discord show
/// its "elapsed" timer. Optional fields are only sent when they are set.
#[must_use]
pub fn build_discord_activity(request: &PresenceRequest) -> DiscordActivity<'_> {
    let activity = &request.activity;

    let mut payload = DiscordActivity::new()
        .name(activity.name.as_str())
        .activity_type(to_discord_activity_type(activity.kind))
        .timestamps(Timestamps::new().start(request.started_at_millis));

    if let Some(details) = &activity.details {
        payload = payload.details(details.as_str());
    }
    if let Some(state) = &activity.state {
        payload = payload.state(state.as_str());
    }
    if let Some(image) = &activity.large_image {
        let mut assets = Assets::new().large_image(image.as_str());
        if let Some(tooltip) = &activity.large_text {
            assets = assets.large_text(tooltip.as_str());
        }
        payload = payload.assets(assets);
    }
    payload
}
