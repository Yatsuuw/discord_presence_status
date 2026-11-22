//! Conversion of a presence status into header text.

use crate::i18n::{Translations, describe_presence_error, fill_placeholders};
use crate::presence::{PresenceStatus, current_unix_millis};
use crate::ui::views::{
    ERROR_COLOR, IDLE_COLOR, SUCCESS_COLOR, StatusDisplay, WARNING_COLOR, format_elapsed,
};

/// Describes `status` for the header, including the running timer.
#[must_use]
pub fn describe_status(status: &PresenceStatus, texts: &Translations) -> StatusDisplay {
    match status {
        PresenceStatus::Idle => StatusDisplay {
            color: IDLE_COLOR,
            text: texts.status_idle.to_owned(),
            detail: None,
        },
        PresenceStatus::Active {
            activity_name,
            started_at_millis,
        } => StatusDisplay {
            color: SUCCESS_COLOR,
            text: fill_placeholders(
                texts.status_active,
                &[
                    ("name", activity_name),
                    (
                        "elapsed",
                        &format_elapsed(current_unix_millis() - started_at_millis),
                    ),
                ],
            ),
            detail: Some(texts.status_active_detail.to_owned()),
        },
        PresenceStatus::Waiting {
            activity_name,
            error,
        } => StatusDisplay {
            color: WARNING_COLOR,
            text: fill_placeholders(texts.status_waiting, &[("name", activity_name)]),
            detail: Some(describe_presence_error(error, texts)),
        },
        PresenceStatus::Rejected {
            activity_name,
            error,
        } => StatusDisplay {
            color: ERROR_COLOR,
            text: fill_placeholders(texts.status_rejected, &[("name", activity_name)]),
            detail: Some(describe_presence_error(error, texts)),
        },
    }
}
