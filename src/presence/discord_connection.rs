//! Definition of [`DiscordConnection`], an open IPC session.

use discord_rich_presence::DiscordIpcClient;

/// An IPC connection to Discord, bound to one Discord application.
pub struct DiscordConnection {
    /// The Discord application the connection was opened for.
    pub application_id: String,
    /// The underlying IPC client.
    pub client: DiscordIpcClient,
}

impl std::fmt::Debug for DiscordConnection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DiscordConnection")
            .field("application_id", &self.application_id)
            .finish_non_exhaustive()
    }
}
