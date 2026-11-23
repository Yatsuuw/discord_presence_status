//! How the application was started, and how it restarts itself in the
//! background.
//!
//! Once a window has been shown, the graphics driver keeps tens of megabytes
//! of memory until the process ends, even after the window is destroyed. To
//! run behind the tray icon with a minimal footprint, the application
//! therefore hands its displayed activity over to a fresh copy of itself
//! started in background mode, which never touches the graphics card:
//!
//! ```text
//! discord_presence_status.exe --background --activity "<folder>" --started-at <ms>
//! ```

mod background_arguments;
mod launch_mode;
mod parse_launch_mode;
mod presence_handover;
mod spawn_background_process;

pub use background_arguments::background_arguments;
pub use launch_mode::LaunchMode;
pub use parse_launch_mode::parse_launch_mode;
pub use presence_handover::PresenceHandover;
pub use spawn_background_process::spawn_background_process;
