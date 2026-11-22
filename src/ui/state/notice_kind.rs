//! Definition of [`NoticeKind`].

/// The tone of a [`Notice`](crate::ui::state::Notice).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoticeKind {
    /// Something worked as expected.
    Success,
    /// Something failed; the text explains why.
    Error,
}
