//! Human-readable labels of activity kinds.

use crate::activity::ActivityKind;

/// Returns the verb Discord displays for `kind`, as shown in the editor.
#[must_use]
pub const fn activity_kind_label(kind: ActivityKind) -> &'static str {
    match kind {
        ActivityKind::Playing => "Playing",
        ActivityKind::Listening => "Listening to",
        ActivityKind::Watching => "Watching",
        ActivityKind::Competing => "Competing in",
    }
}
