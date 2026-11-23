# Architecture

This document explains how the code is organised and the conventions that keep
it easy to maintain.

## Overview

```text
src/
├── main.rs        Thin entry point: starts the UI, maps errors to an exit code.
├── lib.rs         Declares the modules below.
├── constants.rs   Every tunable value (names, limits, intervals).
├── activity/      The activity model and its validation rules.
├── error/         AppError, the single error type.
├── i18n/          Every text of the interface, in each supported language.
├── launch/        Launch modes and the restart in the background (--background).
├── paths/         Where the application and its data live.
├── settings/      User preferences kept between launches (settings.toml).
├── storage/       Reading/writing activities in user_activity/.
├── presence/      Background worker talking to Discord over IPC.
└── ui/            The egui interface.
    ├── state/     Data kept between frames (PresenceApp, ActivityDraft…).
    ├── views/     Drawing; returns UiActions, never has side effects.
    ├── actions/   Applies UiActions: the only UI code touching disk or Discord.
    └── tray/      System tray icon and menu (keeps the presence running hidden).
build.rs           Embeds the icon and version information into the .exe.
installer/         Inno Setup script (per-user and machine-wide installation).
scripts/           PowerShell scripts used locally and by CI.
tests/             Integration tests, including headless UI tests.
```

Dependencies only point "downwards": `ui` uses `i18n`, `settings`,
`presence`, `storage`, `paths` and `activity`; `storage` and `presence` use
`activity`; nothing below `ui` knows about the interface toolkit, and only
`ui` and `i18n` deal with languages.

## Conventions

- **One function or one type per file.** The file is named after its content
  (`save_activity.rs` contains `save_activity`). Each `mod.rs` only declares
  and re-exports, and its `//!` comment describes the module.
- **Every item is documented** in English. `missing_docs` is a warning and
  Clippy runs in pedantic mode; CI treats warnings as errors.
- **No `unsafe`** (`unsafe_code = "deny"`, with one documented exception:
  `wait_for_tray_message`), no `unwrap()`/`expect()` in
  `src/`: failures become an `AppError` shown to the user.
- **Explicit paths.** Storage functions receive the folders they work on,
  which makes them independent of the installation mode and easy to test with
  temporary folders.

## Data model

An activity is the `Activity` struct, serialised as TOML in
`user_activity/<folder>/activity.toml`. The folder name is derived from the
activity name by `sanitize_folder_name` (forbidden characters replaced,
reserved Windows names protected). Renaming an activity renames its folder.

`validate_activity` enforces Discord's limits before saving: required name and
Application ID (17–20 digits), texts between 2 and 128 characters, image as an
asset key or an `https://` URL.

## Data location

`paths::data_root_directory` decides where `user_activity/` lives:

- next to the executable for per-user, portable and development builds;
- in `%APPDATA%\Discord Presence Status` when the executable is inside a
  `Program Files` folder (machine-wide installation).

The uninstaller (`installer/discord_presence_status.iss`, `UserDataFolder`)
mirrors this rule; keep both in sync.

## Discord presence

Discord displays a Rich Presence only while the process that set it keeps its
IPC connection open. A dedicated thread owns that connection:

```text
 UI thread ── PresenceCommand (Show / Clear / Shutdown) ──▶ presence worker ── IPC ──▶ Discord
           ◀── PresenceStatus (Idle / Active / Waiting / Rejected) ──
```

- `run_presence_worker` sleeps until a command arrives or until it is time to
  retry (Discord not running, every 5 s) or refresh (every 20 s, to notice a
  Discord restart). See `next_wake_delay`.
- `sync_presence` connects if needed (`ensure_discord_connection`), sends the
  activity (`send_activity`) and reads Discord's reply (`read_discord_reply`)
  so refusals such as an invalid Application ID reach the user.
- The timer start (`PresenceRequest::started_at_millis`) is fixed when the user
  starts an activity, so reconnections and edits keep the timer running.
- The worker wakes the UI through a `RepaintNotifier` callback, which keeps
  the presence module independent of egui.

## User interface

Each frame (`ui/presence_app_impl.rs`):

1. collects worker statuses (`poll_presence_status`);
2. draws the header, footer, activity list and editor — views edit the form
   (`ActivityDraft`) in place and return `UiAction`s;
3. applies the actions with `apply_action`, which routes each one to its file
   in `ui/actions/`.

`build_presence_app` takes the data root as a parameter so the real
interface can be driven in tests on a temporary folder
(`tests/user_interface.rs`).

Form labels are drawn with `draw_field_label`, which adds a "(?)" icon
(`draw_help_marker`) showing the field's help text; screen readers announce
that text too.

## Languages

Every text shown to the user is a field of `i18n::Translations`. Each
language is one complete constant of that struct (`ENGLISH`, `FRENCH`), so a
missing translation is a compile error. Views receive `&Translations`
(from `translations_for(app.language)`) and never contain literal text.

- Placeholders such as `{name}` are replaced with `fill_placeholders`.
- Errors keep English `Display` messages for logs; the interface translates
  them with `describe_app_error`, `describe_validation_error` and
  `describe_presence_error`. `PresenceStatus` therefore carries the
  `PresenceError` itself rather than a sentence.
- The chosen language is stored in `settings.toml` (`settings` module) next to
  `user_activity/`, and applied by the `change_language` action.
- `tests/translations.rs` checks that no text is empty, that every language
  uses the same placeholders as English, and that every character exists in
  the interface font.

### Adding a language

1. Add a variant to `Language` (`src/i18n/language.rs`) with its code as
   `#[serde(rename = "...")]`, and add it to `Language::ALL`.
2. Give its native name in `language_name`.
3. Create `src/i18n/<language>.rs` with a complete `Translations` constant
   (copy `english.rs`), declare it in `src/i18n/mod.rs` and return it from
   `translations_for`.
4. Run `cargo test`: the translation tests list any problem.

## System tray and background mode

Because Discord drops the presence when the process exits, the application
keeps running behind a notification area icon, built with the
[`tray-icon`](https://crates.io/crates/tray-icon) crate (default features
disabled: on Windows it only needs the Win32 API).

### Keeping the memory use minimal

The window is drawn with OpenGL. Once a window has been shown, the graphics
driver keeps tens of megabytes until the process ends, **even after the
window and its context are destroyed** (measured: about 40 MB with an NVIDIA
driver). Hiding the window, or even destroying it, is therefore not enough.
`run_app` instead works like this:

```text
  normal start ─▶ window ── closed while an activity is shown ──▶ new process
                    ▲                                             --background
                    └──────────── click on the tray icon ◀────────────┘
```

- When the window is closed while an activity is displayed
  (`WindowExit::HideToTray`), `spawn_background_process` starts a new copy of
  the executable with `--background --activity <folder> --started-at <ms>`
  (`launch` module), then the current process exits.
- The background process (`LaunchMode::Background`) never creates a window:
  it loads the activities, resumes the activity with the same timer start
  (`resume_presence`) and waits in `wait_in_tray`. It uses about 2 MB.
- A click on the icon opens the window in that same process (`run_window`);
  closing it again hands over to yet another fresh process.
- If the new process cannot be started, the current one simply keeps
  running behind the icon (`wait_in_tray`), with a higher memory use.

Each window is a `WindowSession` borrowing the `PresenceApp`, with its own
egui context. The context is published in `PresenceApp::repaint_target` so
the presence worker and the tray icon can wake the window up
(`repaint_window`); it is `None` while no window exists.

### Tray events

```text
 tray icon ── TrayCommand (channel + repaint_window) ──▶ poll_tray_commands ──▶ UiAction
 PresenceApp ── build_tray_menu_model ──▶ TrayMenuModel ── (changed?) ──▶ build_tray_menu
```

- `create_presence_app` adds the icon (`create_system_tray`).
  `build_presence_app` does not, so tests never touch the notification area;
  `PresenceApp::tray` is `None` there, and also when Windows refuses the
  icon, in which case closing the window simply quits.
- Menu clicks and left clicks on the icon only send a `TrayCommand` on a
  channel and wake the window. `poll_tray_commands` turns them into ordinary
  `UiAction`s (`ShowWindow`, `StartActivity`, `StopPresence`, `Quit`). With
  a window, `apply_action` applies them; without one, `handle_tray_action`
  does.
- Without a window there is no event loop: `wait_for_tray_message` receives
  the Windows messages of the icon's hidden window (`GetMessageW` /
  `DispatchMessageW`). It is the only `unsafe` code of the project, allowed
  in that single function and documented with `SAFETY` comments.
- `sync_tray_menu` describes the menu as a plain `TrayMenuModel`
  (activities, displayed one, tooltip, language) and rebuilds the native
  menu only when the description changed.
- `detect_hide_to_tray` turns a close or minimise request into
  `UiAction::HideToTray` when `keeps_running_in_tray` holds: a tray icon
  exists, an activity is displayed and the user did not choose **Quit**.
- **Quit** sets `WindowExit::Quit` and closes the window; `run_app` then
  runs `shutdown_presence_worker` (clearing the presence) and drops the icon.

## Adding a field to activities

1. Add the field to `Activity` (`src/activity/model.rs`) with
   `#[serde(default)]` so existing files keep loading.
2. Validate it in `validate_activity` if Discord constrains it.
3. Send it in `build_discord_activity`.
4. Add it to `ActivityDraft`, `draft_from_stored`, `draft_to_activity` and
   `draw_activity_form`, with its label, placeholder and help text in every
   `Translations` constant.
5. Cover it in `tests/` and document it in `docs/USER_GUIDE.md`.
