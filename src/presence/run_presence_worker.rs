//! Main loop of the presence worker thread.

use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};

use crate::presence::{
    PresenceCommand, PresenceStatus, RepaintNotifier, WorkerState, close_discord_connection,
    next_wake_delay, sync_presence,
};

/// Runs the presence worker until it receives [`PresenceCommand::Shutdown`].
///
/// The loop sleeps until a command arrives or until it is time to retry or
/// refresh (see [`next_wake_delay`]), then brings Discord in line with the
/// desired activity. The worker also stops, after clearing the presence, if
/// the user interface disappears without sending `Shutdown`.
pub fn run_presence_worker(
    commands: &Receiver<PresenceCommand>,
    statuses: &Sender<PresenceStatus>,
    notify: &RepaintNotifier,
) {
    let mut state = WorkerState::default();
    let report = |status: PresenceStatus| {
        if statuses.send(status).is_ok() {
            notify();
        }
    };

    loop {
        let received = match next_wake_delay(&state) {
            Some(delay) => commands.recv_timeout(delay),
            None => commands.recv().map_err(|_| RecvTimeoutError::Disconnected),
        };

        match received {
            Ok(PresenceCommand::Show(request)) => {
                state.desired = Some(request);
                state.pending = true;
                state.rejected = false;
            }
            Ok(PresenceCommand::Clear) => {
                state.desired = None;
                if let Some(connection) = state.connection.take() {
                    close_discord_connection(connection);
                }
                report(PresenceStatus::Idle);
                continue;
            }
            Ok(PresenceCommand::Shutdown) | Err(RecvTimeoutError::Disconnected) => {
                if let Some(connection) = state.connection.take() {
                    close_discord_connection(connection);
                }
                break;
            }
            Err(RecvTimeoutError::Timeout) => {}
        }

        if let Some(status) = sync_presence(&mut state) {
            report(status);
        }
    }
}
