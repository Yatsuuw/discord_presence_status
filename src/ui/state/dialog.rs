//! Definition of [`Dialog`], the modal dialogs of the interface.

/// A modal dialog shown over the main window. Only one can be open at a time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dialog {
    /// Confirmation before deleting the activity open in the editor.
    DeleteConfirmation,
    /// Version, copyright and license.
    About,
}
