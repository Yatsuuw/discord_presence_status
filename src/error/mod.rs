//! The error type shared by the whole application.

mod app_error;
mod io_error_at;

pub use app_error::AppError;
pub use io_error_at::io_error_at;
