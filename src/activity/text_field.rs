//! Definition of [`TextField`], the text fields subject to length limits.

use std::fmt;

/// A text field of an activity whose length Discord limits.
///
/// Using an enum rather than a sentence lets the user interface name the
/// field in the user's language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextField {
    /// The activity name.
    Name,
    /// The description (first line).
    Details,
    /// The second line.
    State,
    /// The tooltip of the Discord image.
    ImageTooltip,
}

impl fmt::Display for TextField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Name => "The activity name",
            Self::Details => "The description",
            Self::State => "The second line",
            Self::ImageTooltip => "The image tooltip",
        })
    }
}
