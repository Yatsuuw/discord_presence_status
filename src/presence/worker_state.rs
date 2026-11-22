//! Definition of [`WorkerState`], the memory of the presence worker.

use crate::presence::{DiscordConnection, PresenceRequest};

/// Everything the presence worker remembers between two wake-ups.
#[derive(Debug, Default)]
pub struct WorkerState {
    /// The open Discord connection, if any.
    pub connection: Option<DiscordConnection>,
    /// The activity that should currently be displayed, if any.
    pub desired: Option<PresenceRequest>,
    /// `true` while `desired` has not been successfully sent to Discord.
    pub pending: bool,
    /// `true` when Discord refused `desired`; no retry until a new command.
    pub rejected: bool,
}
