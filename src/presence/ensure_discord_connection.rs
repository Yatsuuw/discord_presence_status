//! Reuse or (re)opening of the Discord connection.

use crate::presence::{
    DiscordConnection, PresenceError, WorkerState, close_discord_connection,
    open_discord_connection,
};

/// Returns a connection opened for `application_id`, opening one if needed.
///
/// An existing connection bound to another Discord application is closed
/// first, because a connection can only display activities of the
/// application it was opened for.
///
/// # Errors
///
/// Returns the [`PresenceError`] raised while opening the connection.
pub fn ensure_discord_connection<'state>(
    state: &'state mut WorkerState,
    application_id: &str,
) -> Result<&'state mut DiscordConnection, PresenceError> {
    let connection = match state.connection.take() {
        Some(connection) if connection.application_id == application_id => connection,
        Some(other_application) => {
            close_discord_connection(other_application);
            open_discord_connection(application_id)?
        }
        None => open_discord_connection(application_id)?,
    };
    Ok(state.connection.insert(connection))
}
