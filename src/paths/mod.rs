//! Where the application and its data live on disk.
//!
//! The application supports two installation modes (see `docs/INSTALLATION.md`):
//!
//! * **per-user** (`%LOCALAPPDATA%\Programs\…`): the installation folder is
//!   writable, so activities are stored right next to the executable in
//!   `user_activity/`;
//! * **machine-wide** (`C:\Program Files\…`): the installation folder is
//!   read-only for standard users, so activities are stored in
//!   `%APPDATA%\Discord Presence Status\user_activity\` instead.

mod activities_directory;
mod data_root_directory;
mod executable_directory;
mod is_machine_wide_install;
mod user_data_directory;

pub use activities_directory::activities_directory;
pub use data_root_directory::data_root_directory;
pub use executable_directory::executable_directory;
pub use is_machine_wide_install::is_machine_wide_install;
pub use user_data_directory::user_data_directory;
