//! Definition of [`Notice`], the message shown in the footer.

use crate::ui::state::NoticeKind;

/// A short message telling the user what just happened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notice {
    /// Whether the message reports a success or a failure.
    pub kind: NoticeKind,
    /// The message itself.
    pub text: String,
}
