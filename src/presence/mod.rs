//! The background worker that talks to the Discord desktop client.
//!
//! Discord displays a Rich Presence only while the process that set it keeps
//! its IPC connection open. A dedicated thread therefore owns the connection;
//! the user interface sends it [`PresenceCommand`]s and receives
//! [`PresenceStatus`] updates through channels, so it never blocks.
//!
//! ```text
//!  UI thread ── PresenceCommand ──▶ worker thread ── IPC ──▶ Discord client
//!            ◀── PresenceStatus ──
//! ```

mod build_discord_activity;
mod close_discord_connection;
mod current_unix_millis;
mod discord_connection;
mod ensure_discord_connection;
mod next_wake_delay;
mod open_discord_connection;
mod presence_command;
mod presence_error;
mod presence_handle;
mod presence_request;
mod presence_status;
mod read_discord_reply;
mod repaint_notifier;
mod run_presence_worker;
mod send_activity;
mod shutdown_presence_worker;
mod spawn_presence_worker;
mod sync_presence;
mod to_discord_activity_type;
mod worker_state;

pub use build_discord_activity::build_discord_activity;
pub use close_discord_connection::close_discord_connection;
pub use current_unix_millis::current_unix_millis;
pub use discord_connection::DiscordConnection;
pub use ensure_discord_connection::ensure_discord_connection;
pub use next_wake_delay::next_wake_delay;
pub use open_discord_connection::open_discord_connection;
pub use presence_command::PresenceCommand;
pub use presence_error::PresenceError;
pub use presence_handle::PresenceHandle;
pub use presence_request::PresenceRequest;
pub use presence_status::PresenceStatus;
pub use read_discord_reply::read_discord_reply;
pub use repaint_notifier::RepaintNotifier;
pub use run_presence_worker::run_presence_worker;
pub use send_activity::send_activity;
pub use shutdown_presence_worker::shutdown_presence_worker;
pub use spawn_presence_worker::spawn_presence_worker;
pub use sync_presence::sync_presence;
pub use to_discord_activity_type::to_discord_activity_type;
pub use worker_state::WorkerState;
