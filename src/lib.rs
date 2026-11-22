//! Discord Presence Status — core library.
//!
//! The executable (`src/main.rs`) is only a thin entry point: every feature
//! lives in one of the modules below so that it can be read, tested and
//! maintained on its own.
//!
//! | Module       | Responsibility                                            |
//! |--------------|-----------------------------------------------------------|
//! | [`activity`] | The activity model and its validation rules.              |
//! | [`constants`]| Application-wide constant values.                         |
//! | [`error`]    | The single error type shared by the whole application.    |
//! | [`i18n`]     | Translation of every text of the interface.               |
//! | [`paths`]    | Where the application and its data live on disk.          |
//! | [`storage`]  | Reading and writing activities in `user_activity/`.       |
//! | [`presence`] | The background worker talking to the Discord client.      |
//! | [`settings`] | User preferences kept between launches (language).        |
//! | [`ui`]       | The desktop user interface.                               |

pub mod activity;
pub mod constants;
pub mod error;
pub mod i18n;
pub mod paths;
pub mod presence;
pub mod settings;
pub mod storage;
pub mod ui;
