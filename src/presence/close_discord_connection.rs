//! Clean closing of a Discord connection.

use discord_rich_presence::DiscordIpc;

use crate::presence::{DiscordConnection, read_discord_reply};

/// Clears the displayed activity and closes `connection`.
///
/// Failures are ignored on purpose: if Discord is already gone, there is
/// nothing left to clean up, and the pipe is released when the client drops.
pub fn close_discord_connection(connection: DiscordConnection) {
    let mut client = connection.client;
    if client.clear_activity().is_ok() {
        let _ = read_discord_reply(&mut client);
    }
    let _ = client.close();
}
