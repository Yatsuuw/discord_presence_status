//! User preferences kept between launches.
//!
//! They are stored in `settings.toml`, in the same data root as
//! `user_activity/` (see [`crate::paths`]):
//!
//! ```toml
//! language = "fr"
//! ```

mod load_settings;
mod model;
mod save_settings;
mod settings_file_path;

pub use load_settings::load_settings;
pub use model::Settings;
pub use save_settings::save_settings;
pub use settings_file_path::settings_file_path;
