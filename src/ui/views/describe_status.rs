//! Conversion of a presence status into header text.

use crate::presence::{PresenceStatus, current_unix_millis};
use crate::ui::views::{
    ERROR_COLOR, IDLE_COLOR, SUCCESS_COLOR, StatusDisplay, WARNING_COLOR, format_elapsed,
};

/// Describes `status` for the header, including the running timer.
#[must_use]
pub fn describe_status(status: &PresenceStatus) -> StatusDisplay {
    match status {
        PresenceStatus::Idle => StatusDisplay {
            color: IDLE_COLOR,
            text: "No activity displayed".to_owned(),
            detail: None,
        },
        PresenceStatus::Active {
            activity_name,
            started_at_millis,
        } => StatusDisplay {
            color: SUCCESS_COLOR,
            text: format!(
                "Displaying \"{activity_name}\"  -  {}",
                format_elapsed(current_unix_millis() - started_at_millis)
            ),
            detail: Some("Your Discord profile shows this activity.".to_owned()),
        },
        PresenceStatus::Waiting {
            activity_name,
            reason,
        } => StatusDisplay {
            color: WARNING_COLOR,
            text: format!("Waiting for Discord to display \"{activity_name}\"..."),
            detail: Some(reason.clone()),
        },
        PresenceStatus::Rejected {
            activity_name,
            reason,
        } => StatusDisplay {
            color: ERROR_COLOR,
            text: format!("Discord refused \"{activity_name}\""),
            detail: Some(reason.clone()),
        },
    }
}
