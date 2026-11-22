//! Formatting of the elapsed time shown next to the active activity.

/// Formats a duration in milliseconds like Discord does: `MM:SS` or `H:MM:SS`.
///
/// Negative durations (clock changes) are shown as `00:00`.
///
/// ```
/// use discord_presence_status::ui::views::format_elapsed;
///
/// assert_eq!(format_elapsed(65_000), "01:05");
/// assert_eq!(format_elapsed(3_725_000), "1:02:05");
/// ```
#[must_use]
pub fn format_elapsed(milliseconds: i64) -> String {
    let total_seconds = milliseconds.max(0) / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes:02}:{seconds:02}")
    }
}
