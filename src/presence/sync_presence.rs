//! Synchronisation of Discord with the desired activity.

use crate::presence::{
    PresenceError, PresenceStatus, WorkerState, ensure_discord_connection, send_activity,
};

/// Sends the desired activity to Discord and returns the new status, if it changed.
///
/// Called on every worker wake-up. It does nothing when no activity is
/// desired or when Discord already refused it. A successful refresh of an
/// activity that is already displayed returns `None`, so the user interface
/// is only notified of real changes.
pub fn sync_presence(state: &mut WorkerState) -> Option<PresenceStatus> {
    if state.rejected {
        return None;
    }
    let request = state.desired.clone()?;
    let activity_name = request.activity.name.clone();

    let result = ensure_discord_connection(state, &request.activity.application_id)
        .and_then(|connection| send_activity(connection, &request));

    match result {
        Ok(()) => {
            let was_pending = std::mem::replace(&mut state.pending, false);
            was_pending.then_some(PresenceStatus::Active {
                activity_name,
                started_at_millis: request.started_at_millis,
            })
        }
        Err(error @ PresenceError::Unavailable(_)) => {
            state.connection = None;
            state.pending = true;
            Some(PresenceStatus::Waiting {
                activity_name,
                reason: error.to_string(),
            })
        }
        Err(error @ PresenceError::Rejected(_)) => {
            state.connection = None;
            state.rejected = true;
            Some(PresenceStatus::Rejected {
                activity_name,
                reason: error.to_string(),
            })
        }
    }
}
