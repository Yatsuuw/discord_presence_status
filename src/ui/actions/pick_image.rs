//! Choice of a local image for the edited activity.

use crate::constants::SUPPORTED_IMAGE_EXTENSIONS;
use crate::ui::state::PresenceApp;

/// Opens a file dialog and attaches the chosen image to the edited activity.
///
/// The image is only copied into the activity folder when the activity is
/// saved, so cancelling the edit leaves the folder untouched.
pub fn pick_image(app: &mut PresenceApp) {
    let Some(draft) = app.draft.as_mut() else {
        return;
    };
    let chosen = rfd::FileDialog::new()
        .set_title("Choose the activity image")
        .add_filter("Images", SUPPORTED_IMAGE_EXTENSIONS)
        .pick_file();

    if let Some(path) = chosen {
        draft.preview_image = Some(path.clone());
        draft.new_image = Some(path);
    }
}
