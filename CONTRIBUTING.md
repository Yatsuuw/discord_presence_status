# Contributing

Thank you for your interest in improving Discord Presence Status!

## Before you start

- For anything bigger than a small fix, open an issue first so we can agree on
  the approach.
- Security problems must be reported privately: see [SECURITY.md](SECURITY.md).

## Development setup

1. Install [Rust](https://rustup.rs) (1.95 or newer) and the Visual Studio
   Build Tools (*Desktop development with C++*).
2. Clone the repository and run the application:

   ```powershell
   cargo run
   ```

   Development builds store their activities in `target/debug/user_activity/`.

## Code conventions

The project favours readability over cleverness. Please follow these rules,
described in more detail in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md):

- **One function or one type per file**, named after it
  (`save_activity` lives in `save_activity.rs`). Each file has one purpose.
- Every file starts with a `//!` comment saying what it contains; every public
  item has a `///` documentation comment.
- Code, comments and documentation are written in **English**.
- Views never touch the disk or Discord: they return a `UiAction`, applied in
  `src/ui/actions/`.
- No `unsafe` code, no `unwrap()`/`expect()` outside tests.

## Checks

Your change must pass the same checks as the CI:

```powershell
cargo fmt --all
cargo clippy --all-targets
cargo test
```

Tests live in `tests/`. The user interface is tested headless with
[`egui_kittest`](https://crates.io/crates/egui_kittest), see
`tests/user_interface.rs`.

## Commits and pull requests

- Write commit messages in the imperative mood: "Add image tooltip field".
- Keep pull requests focused on one change and fill in the template.
- Update `CHANGELOG.md` under *Unreleased* when the behaviour changes.
