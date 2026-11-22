//! Detection of a machine-wide ("Program Files") installation.

use std::path::{Path, PathBuf};

/// Environment variables pointing to the system program folders on Windows.
const PROGRAM_FILES_VARIABLES: &[&str] = &["ProgramFiles", "ProgramFiles(x86)", "ProgramW6432"];

/// Tells whether `executable_directory` is inside a system program folder.
///
/// Such folders are read-only for standard users, so user data must be kept
/// elsewhere. The comparison ignores letter case, like Windows does.
#[must_use]
pub fn is_machine_wide_install(executable_directory: &Path) -> bool {
    let lowercase = |path: &Path| PathBuf::from(path.to_string_lossy().to_lowercase());
    let executable_directory = lowercase(executable_directory);

    PROGRAM_FILES_VARIABLES
        .iter()
        .filter_map(std::env::var_os)
        .filter(|folder| !folder.is_empty())
        .any(|folder| executable_directory.starts_with(lowercase(Path::new(&folder))))
}
