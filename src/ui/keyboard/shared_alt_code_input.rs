//! Definition of [`SharedAltCodeInput`].

use std::sync::{Arc, Mutex};

use crate::ui::keyboard::AltCodeInput;

/// [`AltCodeInput`] shared between the Windows message hook, which fills it,
/// and the window, which applies it before every frame.
pub type SharedAltCodeInput = Arc<Mutex<AltCodeInput>>;
