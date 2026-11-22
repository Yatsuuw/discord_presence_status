//! The data the user interface keeps between frames.

mod active_presence;
mod activity_draft;
mod activity_kind_label;
mod draft_from_stored;
mod draft_to_activity;
mod notice;
mod notice_kind;
mod optional_text;
mod presence_app;

pub use active_presence::ActivePresence;
pub use activity_draft::ActivityDraft;
pub use activity_kind_label::activity_kind_label;
pub use draft_from_stored::draft_from_stored;
pub use draft_to_activity::draft_to_activity;
pub use notice::Notice;
pub use notice_kind::NoticeKind;
pub use optional_text::optional_text;
pub use presence_app::PresenceApp;
