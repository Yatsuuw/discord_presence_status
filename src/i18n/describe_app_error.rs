//! Translation of application errors.

use crate::error::AppError;
use crate::i18n::{Translations, describe_validation_error, fill_placeholders};

/// Explains `error` to the user in the interface language.
///
/// Operating system messages (`{detail}`) are kept as Windows wrote them.
#[must_use]
pub fn describe_app_error(error: &AppError, texts: &Translations) -> String {
    let fill = fill_placeholders;
    match error {
        AppError::ExecutableLocation(source) => fill(
            texts.error_executable_location,
            &[("detail", &source.to_string())],
        ),
        AppError::MissingAppDataFolder => texts.error_missing_app_data.to_owned(),
        AppError::Io { path, source } => fill(
            texts.error_io,
            &[
                ("path", &path.display().to_string()),
                ("detail", &source.to_string()),
            ],
        ),
        AppError::ActivityParse { path, source } => fill(
            texts.error_activity_parse,
            &[
                ("path", &path.display().to_string()),
                ("detail", source.message()),
            ],
        ),
        AppError::SettingsParse { path, source } => fill(
            texts.error_settings_parse,
            &[
                ("path", &path.display().to_string()),
                ("detail", source.message()),
            ],
        ),
        AppError::ActivitySerialize(source) | AppError::SettingsSerialize(source) => {
            fill(texts.error_serialize, &[("detail", &source.to_string())])
        }
        AppError::Validation(validation) => describe_validation_error(validation, texts),
        AppError::ActivityAlreadyExists(name) => {
            fill(texts.error_activity_exists, &[("name", name)])
        }
        AppError::UnsupportedImageFormat(formats) => {
            fill(texts.error_unsupported_image, &[("formats", formats)])
        }
        AppError::ImageTooLarge { max_megabytes } => fill(
            texts.error_image_too_large,
            &[("max", &max_megabytes.to_string())],
        ),
        AppError::OutsideActivitiesFolder(path) => fill(
            texts.error_outside_folder,
            &[("path", &path.display().to_string())],
        ),
        AppError::WorkerSpawn(source) => {
            fill(texts.error_worker_spawn, &[("detail", &source.to_string())])
        }
        AppError::Tray(detail) => fill(texts.error_tray, &[("detail", detail)]),
        AppError::Interface(detail) => fill(texts.error_interface, &[("detail", detail)]),
    }
}
