//! Reading and writing activities on disk.
//!
//! Layout of the activities folder:
//!
//! ```text
//! user_activity/
//! ├── Rust programming/
//! │   ├── activity.toml   ← the activity itself (see `Activity`)
//! │   └── image.png       ← optional local image
//! └── Watching anime/
//!     └── activity.toml
//! ```
//!
//! Every function here works on explicit paths, which keeps them independent
//! from the installation mode and easy to test with temporary folders.

mod delete_activity;
mod ensure_directory;
mod ensure_inside_activities_directory;
mod import_activity_image;
mod is_same_folder;
mod load_activities;
mod local_image_path;
mod prepare_activity_folder;
mod read_activity_file;
mod remove_activity_images;
mod save_activity;
mod stored_activity;
mod write_activity_file;

pub use delete_activity::delete_activity;
pub use ensure_directory::ensure_directory;
pub use ensure_inside_activities_directory::ensure_inside_activities_directory;
pub use import_activity_image::import_activity_image;
pub use is_same_folder::is_same_folder;
pub use load_activities::load_activities;
pub use local_image_path::local_image_path;
pub use prepare_activity_folder::prepare_activity_folder;
pub use read_activity_file::read_activity_file;
pub use remove_activity_images::remove_activity_images;
pub use save_activity::save_activity;
pub use stored_activity::StoredActivity;
pub use write_activity_file::write_activity_file;
