//! Sending of an activity over an open connection.

use discord_rich_presence::DiscordIpc;

use crate::presence::{
    DiscordConnection, PresenceError, PresenceRequest, build_discord_activity, read_discord_reply,
};

/// Asks Discord to display `request` and checks its answer.
///
/// # Errors
///
/// * [`PresenceError::Unavailable`] when the connection is broken;
/// * [`PresenceError::Rejected`] when Discord refuses the activity.
pub fn send_activity(
    connection: &mut DiscordConnection,
    request: &PresenceRequest,
) -> Result<(), PresenceError> {
    connection
        .client
        .set_activity(build_discord_activity(request))
        .map_err(|error| PresenceError::Unavailable(error.to_string()))?;
    read_discord_reply(&mut connection.client)
}
