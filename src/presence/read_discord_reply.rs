//! Reading and interpretation of a Discord IPC reply.

use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use serde_json::Value;

use crate::presence::PresenceError;

/// IPC opcode Discord uses when it closes the connection.
const OPCODE_CLOSE: u32 = 2;

/// Reads the reply to the last message sent and turns errors into [`PresenceError`].
///
/// Discord answers every command. Reading the answer serves two purposes:
/// it surfaces refusals (invalid application ID, invalid field…) to the user,
/// and it keeps the IPC pipe from filling up during long sessions.
///
/// # Errors
///
/// * [`PresenceError::Unavailable`] when the reply cannot be read;
/// * [`PresenceError::Rejected`] when Discord closed the connection or
///   answered with an `ERROR` event.
pub fn read_discord_reply(client: &mut DiscordIpcClient) -> Result<(), PresenceError> {
    let (opcode, payload) = client
        .recv()
        .map_err(|error| PresenceError::Unavailable(error.to_string()))?;

    let reason = || {
        payload
            .pointer("/data/message")
            .or_else(|| payload.get("message"))
            .and_then(Value::as_str)
            .unwrap_or("no reason given")
            .to_owned()
    };

    let is_error_event = payload.get("evt").and_then(Value::as_str) == Some("ERROR");
    if opcode == OPCODE_CLOSE || is_error_event {
        Err(PresenceError::Rejected(reason()))
    } else {
        Ok(())
    }
}
