//! Display of an error message.

use crate::ui::state::{Notice, NoticeKind, PresenceApp};

/// Shows `error` as an error message in the footer.
pub fn show_error(app: &mut PresenceApp, error: impl std::fmt::Display) {
    app.notice = Some(Notice {
        kind: NoticeKind::Error,
        text: error.to_string(),
    });
}
