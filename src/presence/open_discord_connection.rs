//! Opening of an IPC connection to the Discord client.

use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use serde_json::json;

use crate::presence::{DiscordConnection, PresenceError, read_discord_reply};

/// IPC opcode of the handshake message.
const OPCODE_HANDSHAKE: u8 = 0;

/// IPC protocol version expected by Discord.
const IPC_VERSION: u8 = 1;

/// Connects to the local Discord client on behalf of `application_id`.
///
/// The handshake is performed here rather than with `DiscordIpc::connect` so
/// that Discord's answer is checked: an unknown application ID is reported as
/// a refusal instead of being silently ignored.
///
/// # Errors
///
/// * [`PresenceError::Unavailable`] when Discord is not running;
/// * [`PresenceError::Rejected`] when Discord refuses the application ID.
pub fn open_discord_connection(application_id: &str) -> Result<DiscordConnection, PresenceError> {
    let unavailable =
        |error: discord_rich_presence::error::Error| PresenceError::Unavailable(error.to_string());

    let mut client = DiscordIpcClient::new(application_id);
    client.connect_ipc().map_err(unavailable)?;
    client
        .send(
            json!({ "v": IPC_VERSION, "client_id": application_id }),
            OPCODE_HANDSHAKE,
        )
        .map_err(unavailable)?;
    read_discord_reply(&mut client)?;

    Ok(DiscordConnection {
        application_id: application_id.to_owned(),
        client,
    })
}
