//! Definition of [`ActivityKind`], the verb Discord shows before a name.

use serde::{Deserialize, Serialize};

/// The kind of activity, which decides the verb displayed by Discord.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActivityKind {
    /// Displayed as "Playing …".
    #[default]
    Playing,
    /// Displayed as "Listening to …".
    Listening,
    /// Displayed as "Watching …".
    Watching,
    /// Displayed as "Competing in …".
    Competing,
}

impl ActivityKind {
    /// Every kind, in the order they are offered to the user.
    pub const ALL: [Self; 4] = [
        Self::Playing,
        Self::Listening,
        Self::Watching,
        Self::Competing,
    ];
}
