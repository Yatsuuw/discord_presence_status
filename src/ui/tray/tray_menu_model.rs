//! Definition of [`TrayMenuModel`], what the tray icon shows.

use crate::i18n::Language;
use crate::ui::tray::TrayMenuEntry;

/// Everything the tray icon displays, independent of the native menu API.
///
/// Comparing two models tells whether the native menu must be rebuilt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrayMenuModel {
    /// Language of the menu labels.
    pub language: Language,
    /// Text shown when hovering the tray icon.
    pub tooltip: String,
    /// The activities listed in the "Start activity" sub-menu, in order.
    pub activities: Vec<TrayMenuEntry>,
    /// `true` when an activity is displayed and can be stopped.
    pub can_stop: bool,
}
