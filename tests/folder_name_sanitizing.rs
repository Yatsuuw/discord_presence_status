//! Tests of the conversion of activity names into folder names.

use discord_presence_status::activity::sanitize_folder_name;

#[test]
fn keeps_ordinary_names_untouched() {
    assert_eq!(sanitize_folder_name("Watching anime"), "Watching anime");
    assert_eq!(sanitize_folder_name("Café ☕"), "Café ☕");
}

#[test]
fn replaces_characters_forbidden_by_windows() {
    assert_eq!(
        sanitize_folder_name(r#"a<b>c:d"e/f\g|h?i*j"#),
        "a_b_c_d_e_f_g_h_i_j"
    );
}

#[test]
fn removes_trailing_dots_and_surrounding_spaces() {
    assert_eq!(sanitize_folder_name("  Name. . "), "Name");
}

#[test]
fn protects_reserved_device_names() {
    assert_eq!(sanitize_folder_name("CON"), "_CON");
    assert_eq!(sanitize_folder_name("lpt1.txt"), "_lpt1.txt");
    assert_eq!(sanitize_folder_name("Console"), "Console");
}

#[test]
fn limits_the_length() {
    assert_eq!(sanitize_folder_name(&"a".repeat(200)).chars().count(), 64);
}

#[test]
fn returns_an_empty_string_when_nothing_is_usable() {
    assert_eq!(sanitize_folder_name(" ... "), "");
}
