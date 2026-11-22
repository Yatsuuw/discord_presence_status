//! Tests of the activity storage (folders, `activity.toml`, local images).

use std::path::Path;

use discord_presence_status::activity::Activity;
use discord_presence_status::error::AppError;
use discord_presence_status::storage::{
    delete_activity, load_activities, local_image_path, read_activity_file, save_activity,
};

fn activity(name: &str) -> Activity {
    Activity {
        name: name.to_owned(),
        application_id: "123456789012345678".to_owned(),
        details: Some("A description".to_owned()),
        ..Activity::default()
    }
}

fn write_image(path: &Path) {
    std::fs::write(path, b"\x89PNG fake image").expect("write test image");
}

#[test]
fn saves_an_activity_in_its_own_folder() {
    let root = tempfile::tempdir().expect("temporary folder");
    let stored = save_activity(root.path(), activity("Coding"), None, None).expect("save");

    assert_eq!(stored.folder, root.path().join("Coding"));
    assert_eq!(
        read_activity_file(&stored.folder).expect("read"),
        activity("Coding")
    );
}

#[test]
fn loads_every_activity_sorted_by_name() {
    let root = tempfile::tempdir().expect("temporary folder");
    for name in ["zebra", "Alpha", "middle"] {
        save_activity(root.path(), activity(name), None, None).expect("save");
    }
    std::fs::create_dir(root.path().join("not an activity")).expect("create folder");

    let names: Vec<String> = load_activities(root.path())
        .expect("load")
        .into_iter()
        .map(|result| result.expect("valid activity").activity.name)
        .collect();
    assert_eq!(names, ["Alpha", "middle", "zebra"]);
}

#[test]
fn reports_broken_files_without_hiding_the_others() {
    let root = tempfile::tempdir().expect("temporary folder");
    save_activity(root.path(), activity("Valid"), None, None).expect("save");
    let broken = root.path().join("Broken");
    std::fs::create_dir(&broken).expect("create folder");
    std::fs::write(broken.join("activity.toml"), "this is = = not toml").expect("write");

    let results = load_activities(root.path()).expect("load");
    assert_eq!(results.len(), 2);
    assert!(results[0].is_ok());
    assert!(matches!(results[1], Err(AppError::ActivityParse { .. })));
}

#[test]
fn refuses_two_activities_with_the_same_folder() {
    let root = tempfile::tempdir().expect("temporary folder");
    save_activity(root.path(), activity("Coding"), None, None).expect("save");

    let duplicate = save_activity(root.path(), activity("Coding"), None, None);
    assert!(matches!(duplicate, Err(AppError::ActivityAlreadyExists(_))));
}

#[test]
fn renames_the_folder_when_the_name_changes() {
    let root = tempfile::tempdir().expect("temporary folder");
    let source = root.path().join("source.png");
    write_image(&source);
    let first =
        save_activity(root.path(), activity("Old name"), None, Some(&source)).expect("save");

    let renamed = save_activity(
        root.path(),
        first.activity.clone().with_name("New name"),
        Some(&first.folder),
        None,
    )
    .expect("rename");

    assert!(!first.folder.exists());
    assert_eq!(renamed.folder, root.path().join("New name"));
    assert!(
        local_image_path(&renamed).is_some(),
        "the image follows the activity"
    );
}

#[test]
fn imports_replaces_and_removes_the_local_image() {
    let root = tempfile::tempdir().expect("temporary folder");
    let png = root.path().join("picture.PNG");
    let jpeg = root.path().join("photo.jpg");
    write_image(&png);
    write_image(&jpeg);

    let stored = save_activity(root.path(), activity("Art"), None, Some(&png)).expect("save");
    assert_eq!(stored.activity.local_image.as_deref(), Some("image.png"));

    let replaced = save_activity(
        root.path(),
        stored.activity.clone(),
        Some(&stored.folder),
        Some(&jpeg),
    )
    .expect("replace image");
    assert_eq!(replaced.activity.local_image.as_deref(), Some("image.jpg"));
    assert!(!replaced.folder.join("image.png").exists());

    let without_image = Activity {
        local_image: None,
        ..replaced.activity.clone()
    };
    let removed = save_activity(root.path(), without_image, Some(&replaced.folder), None)
        .expect("remove image");
    assert!(!removed.folder.join("image.jpg").exists());
}

#[test]
fn rejects_unsupported_images() {
    let root = tempfile::tempdir().expect("temporary folder");
    let text_file = root.path().join("notes.txt");
    std::fs::write(&text_file, "hello").expect("write");

    let result = save_activity(root.path(), activity("Notes"), None, Some(&text_file));
    assert!(matches!(result, Err(AppError::UnsupportedImageFormat(_))));
}

#[test]
fn ignores_local_image_names_escaping_the_folder() {
    let root = tempfile::tempdir().expect("temporary folder");
    write_image(&root.path().join("secret.png"));
    let mut stored = save_activity(root.path(), activity("Sneaky"), None, None).expect("save");
    stored.activity.local_image = Some("../secret.png".to_owned());

    assert_eq!(local_image_path(&stored), None);
}

#[test]
fn deletes_an_activity_folder() {
    let root = tempfile::tempdir().expect("temporary folder");
    let stored = save_activity(root.path(), activity("Temporary"), None, None).expect("save");

    delete_activity(root.path(), &stored.folder).expect("delete");
    assert!(!stored.folder.exists());
}

#[test]
fn never_deletes_outside_the_activities_folder() {
    let root = tempfile::tempdir().expect("temporary folder");
    let activities = root.path().join("user_activity");
    std::fs::create_dir(&activities).expect("create folder");

    let result = delete_activity(&activities, root.path());
    assert!(matches!(result, Err(AppError::OutsideActivitiesFolder(_))));
    assert!(activities.exists());
}

/// Small helper keeping the rename test readable.
trait WithName {
    fn with_name(self, name: &str) -> Self;
}

impl WithName for Activity {
    fn with_name(self, name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..self
        }
    }
}
