//! Protection of user text shown in native menus.

/// Returns `label` ready to be used as a native menu entry text.
///
/// Windows menus treat `&` as a keyboard shortcut marker; doubling it shows
/// the character as typed.
#[must_use]
pub fn escape_menu_label(label: &str) -> String {
    label.replace('&', "&&")
}
