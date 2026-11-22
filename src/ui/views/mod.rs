//! Drawing of the user interface.
//!
//! Views only read the state (and edit form fields in place); they report
//! everything else as [`UiAction`](crate::ui::UiAction)s.

mod describe_status;
mod draw_activity_editor;
mod draw_activity_form;
mod draw_activity_list;
mod draw_delete_dialog;
mod draw_editor_buttons;
mod draw_footer;
mod draw_header;
mod draw_image_picker;
mod draw_status_dot;
mod draw_text_field;
mod draw_welcome;
mod format_elapsed;
mod palette;
mod status_display;

pub use describe_status::describe_status;
pub use draw_activity_editor::draw_activity_editor;
pub use draw_activity_form::draw_activity_form;
pub use draw_activity_list::draw_activity_list;
pub use draw_delete_dialog::draw_delete_dialog;
pub use draw_editor_buttons::draw_editor_buttons;
pub use draw_footer::draw_footer;
pub use draw_header::draw_header;
pub use draw_image_picker::draw_image_picker;
pub use draw_status_dot::draw_status_dot;
pub use draw_text_field::draw_text_field;
pub use draw_welcome::draw_welcome;
pub use format_elapsed::format_elapsed;
pub use palette::{ERROR_COLOR, IDLE_COLOR, SUCCESS_COLOR, WARNING_COLOR};
pub use status_display::StatusDisplay;
