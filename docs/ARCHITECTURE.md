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
├── paths/         Where the application and its data live.
├── storage/       Reading/writing activities in user_activity/.
├── presence/      Background worker talking to Discord over IPC.
└── ui/            The egui interface.
    ├── state/     Data kept between frames (PresenceApp, ActivityDraft…).
    ├── views/     Drawing; returns UiActions, never has side effects.
    └── actions/   Applies UiActions: the only UI code touching disk or Discord.
build.rs           Embeds the icon and version information into the .exe.
installer/         Inno Setup script (per-user and machine-wide installation).
scripts/           PowerShell scripts used locally and by CI.
tests/             Integration tests, including headless UI tests.
```

Dependencies only point "downwards": `ui` uses `presence`, `storage`,
`paths` and `activity`; `storage` and `presence` use `activity`; nothing below
`ui` knows about the interface toolkit.

## Conventions

- **One function or one type per file.** The file is named after its content
  (`save_activity.rs` contains `save_activity`). Each `mod.rs` only declares
  and re-exports, and its `//!` comment describes the module.
- **Every item is documented** in English. `missing_docs` is a warning and
  Clippy runs in pedantic mode; CI treats warnings as errors.
- **No `unsafe`** (`unsafe_code = "forbid"`), no `unwrap()`/`expect()` in
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

`build_presence_app` takes the activities folder as a parameter so the real
interface can be driven in tests on a temporary folder
(`tests/user_interface.rs`).

## Adding a field to activities

1. Add the field to `Activity` (`src/activity/model.rs`) with
   `#[serde(default)]` so existing files keep loading.
2. Validate it in `validate_activity` if Discord constrains it.
3. Send it in `build_discord_activity`.
4. Add it to `ActivityDraft`, `draft_from_stored`, `draft_to_activity` and
   `draw_activity_form`.
5. Cover it in `tests/` and document it in `docs/USER_GUIDE.md`.
