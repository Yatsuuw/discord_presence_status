//! Case-insensitive folder comparison.

use std::path::Path;

/// Tells whether two paths designate the same folder, ignoring letter case.
///
/// Windows file systems are case-insensitive: renaming "coding" to "Coding"
/// must be treated as a rename of the same folder, not as a conflict.
#[must_use]
pub fn is_same_folder(first: &Path, second: &Path) -> bool {
    first.to_string_lossy().to_lowercase() == second.to_string_lossy().to_lowercase()
}
