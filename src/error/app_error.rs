//! Definition of [`AppError`], the single error type of the application.

use std::path::PathBuf;

use thiserror::Error;

use crate::activity::ValidationError;

/// Every failure the application can report to the user.
///
/// Each variant carries enough context (paths, names, limits) to produce a
/// message that is understandable without reading the source code.
#[derive(Debug, Error)]
pub enum AppError {
    /// The path of the running executable could not be determined.
    #[error("cannot locate the application executable: {0}")]
    ExecutableLocation(#[source] std::io::Error),

    /// The per-user application data folder could not be determined.
    #[error("cannot locate the user's application data folder")]
    MissingAppDataFolder,

    /// A file system operation failed.
    #[error("file system error on \"{}\": {source}", path.display())]
    Io {
        /// The file or folder involved in the failed operation.
        path: PathBuf,
        /// The underlying operating system error.
        #[source]
        source: std::io::Error,
    },

    /// An `activity.toml` file exists but cannot be understood.
    #[error("invalid activity file \"{}\": {source}", path.display())]
    ActivityParse {
        /// The file that could not be parsed.
        path: PathBuf,
        /// The underlying TOML error.
        #[source]
        source: toml::de::Error,
    },

    /// An activity could not be converted to TOML.
    #[error("cannot serialize the activity: {0}")]
    ActivitySerialize(#[from] toml::ser::Error),

    /// An activity does not respect the validation rules.
    #[error(transparent)]
    Validation(#[from] ValidationError),

    /// Another activity already uses the folder derived from this name.
    #[error("an activity named \"{0}\" already exists")]
    ActivityAlreadyExists(String),

    /// The chosen local image does not use a supported format.
    #[error("unsupported image format; use one of: {0}")]
    UnsupportedImageFormat(String),

    /// The chosen local image exceeds the size limit.
    #[error("the image is too large (maximum {max_megabytes} MB)")]
    ImageTooLarge {
        /// The maximum accepted size, in megabytes.
        max_megabytes: u64,
    },

    /// A destructive operation targeted a path outside the activities folder.
    #[error("refusing to modify \"{}\": it is outside the activities folder", .0.display())]
    OutsideActivitiesFolder(PathBuf),

    /// The background presence worker could not be started.
    #[error("cannot start the presence worker: {0}")]
    WorkerSpawn(#[source] std::io::Error),

    /// The user interface could not be started.
    #[error("cannot start the user interface: {0}")]
    Interface(String),
}
