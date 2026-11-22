//! The system tray (notification area) icon and its menu.
//!
//! Discord only displays a presence while this application runs. The tray
//! icon lets it keep running with the main window hidden: closing or
//! minimising the window while an activity is displayed hides it to the tray
//! instead of quitting.
//!
//! The menu is described by a plain [`TrayMenuModel`], rebuilt from the
//! application state and compared every frame, so the native menu is only
//! recreated when something visible changes. Clicks are reported as
//! [`TrayCommand`]s on a channel and turned into
//! [`UiAction`](crate::ui::UiAction)s by
//! [`poll_tray_commands`](crate::ui::actions::poll_tray_commands).

mod build_tray_menu;
mod build_tray_menu_model;
mod create_system_tray;
mod escape_menu_label;
mod load_tray_icon;
mod system_tray;
mod tray_command;
mod tray_menu_entry;
mod tray_menu_model;
mod tray_tooltip;

pub use build_tray_menu::build_tray_menu;
pub use build_tray_menu_model::build_tray_menu_model;
pub use create_system_tray::create_system_tray;
pub use escape_menu_label::escape_menu_label;
pub use load_tray_icon::load_tray_icon;
pub use system_tray::SystemTray;
pub use tray_command::TrayCommand;
pub use tray_menu_entry::TrayMenuEntry;
pub use tray_menu_model::TrayMenuModel;
pub use tray_tooltip::tray_tooltip;
