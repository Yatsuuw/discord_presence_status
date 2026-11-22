//! End-to-end tests of the user interface, run headless with `egui_kittest`.

use discord_presence_status::storage::read_activity_file;
use discord_presence_status::ui::build_presence_app;
use discord_presence_status::ui::state::PresenceApp;
use eframe::egui;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;

fn harness(activities: &std::path::Path) -> Harness<'static, PresenceApp> {
    let folder = activities.to_path_buf();
    Harness::builder()
        .with_size([900.0, 640.0])
        .build_eframe(move |creation_context| {
            build_presence_app(&creation_context.egui_ctx, folder).expect("build the app")
        })
}

#[test]
fn creates_saves_and_deletes_an_activity() {
    let root = tempfile::tempdir().expect("temporary folder");
    let mut harness = harness(root.path());
    harness.run();

    harness.get_by_label("Show the world what you are up to");
    harness
        .get_all_by_label("+ New activity")
        .next()
        .expect("button")
        .click();
    harness.run();

    let fields = [
        ("Name *", "Rust programming"),
        ("Application ID *", "123456789012345678"),
        ("Description", "Building a desktop app"),
    ];
    for (label, text) in fields {
        let field = harness.get_by_role_and_label(egui::accesskit::Role::TextInput, label);
        field.focus();
        field.type_text(text);
        harness.run();
    }

    harness.get_by_label("Save").click();
    harness.run();

    let folder = root.path().join("Rust programming");
    let saved = read_activity_file(&folder).expect("activity saved on disk");
    assert_eq!(saved.application_id, "123456789012345678");
    assert_eq!(saved.details.as_deref(), Some("Building a desktop app"));
    harness.get_by_label("\"Rust programming\" saved.");

    harness.get_by_label("Delete").click();
    harness.run();
    harness.get_by_label("Delete this activity?");
    harness
        .get_all_by_label("Delete")
        .last()
        .expect("confirm button")
        .click();
    harness.run();

    assert!(!folder.exists());
    assert_eq!(
        harness.state().activities.len(),
        0,
        "the list must be empty"
    );
}

#[test]
fn shows_validation_errors_instead_of_saving() {
    let root = tempfile::tempdir().expect("temporary folder");
    let mut harness = harness(root.path());
    harness.run();

    harness
        .get_all_by_label("+ New activity")
        .next()
        .expect("button")
        .click();
    harness.run();
    harness.get_by_label("Save").click();
    harness.run();

    harness.get_by_label("the activity name is required");
    assert_eq!(std::fs::read_dir(root.path()).expect("list").count(), 0);
}
