//! Typing of special characters with Alt codes (for example Alt+144 = É).
//!
//! winit, which receives the keyboard for the window, mishandles Windows
//! Alt codes: it inserts the keypad digits typed while Alt is held ("144")
//! and drops the composed character, which Windows sends only when Alt is
//! released. This module fixes both before egui sees the input:
//!
//! ```text
//!  Windows message ── install_alt_code_hook ──▶ observe_keyboard_message ──▶ AltCodeInput
//!  egui RawInput   ◀── apply_alt_code_input ◀───────────────────────────────┘
//! ```

mod alt_code_input;
mod apply_alt_code_input;
mod install_alt_code_hook;
mod observe_keyboard_message;
mod shared_alt_code_input;

pub use alt_code_input::AltCodeInput;
pub use apply_alt_code_input::apply_alt_code_input;
pub use install_alt_code_hook::install_alt_code_hook;
pub use observe_keyboard_message::observe_keyboard_message;
pub use shared_alt_code_input::SharedAltCodeInput;
