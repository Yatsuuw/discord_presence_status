//! The activity model and its validation rules.
//!
//! An *activity* is what the user wants Discord to display: a name, a
//! description, an image and the kind of activity ("Playing", "Watching"…).

mod activity_kind;
mod is_valid_application_id;
mod model;
mod sanitize_folder_name;
mod text_field;
mod validate_activity;
mod validate_image_reference;
mod validate_text_length;
mod validation_error;

pub use activity_kind::ActivityKind;
pub use is_valid_application_id::is_valid_application_id;
pub use model::Activity;
pub use sanitize_folder_name::sanitize_folder_name;
pub use text_field::TextField;
pub use validate_activity::validate_activity;
pub use validate_image_reference::validate_image_reference;
pub use validate_text_length::validate_text_length;
pub use validation_error::ValidationError;
