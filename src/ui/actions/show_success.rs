//! Display of a success message.

use crate::ui::state::{Notice, NoticeKind, PresenceApp};

/// Shows `text` as a success message in the footer.
pub fn show_success(app: &mut PresenceApp, text: impl Into<String>) {
    app.notice = Some(Notice {
        kind: NoticeKind::Success,
        text: text.into(),
    });
}
