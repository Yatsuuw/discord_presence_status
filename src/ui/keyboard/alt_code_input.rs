//! Definition of [`AltCodeInput`], what Alt codes changed since the last frame.

/// The corrections to apply to the keyboard input of the next frame.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AltCodeInput {
    /// Keypad digits typed while Alt was held: winit turns them into text
    /// that must be removed.
    pub digits_to_remove: usize,
    /// Characters composed by Windows when Alt was released, to insert.
    pub characters: Vec<char>,
}
