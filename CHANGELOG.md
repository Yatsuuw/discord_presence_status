# Changelog

All notable changes to this project are documented here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and the project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- System tray icon with a menu to show the window, start a saved activity,
  stop the displayed activity and quit.
- While an activity is displayed, closing or minimising the window hides it
  to the system tray so the Discord presence keeps running.

## [0.1.0] - 2026-11-22

### Added

- Desktop application to create, edit and delete custom Discord activities.
- Activities stored in `user_activity/<name>/activity.toml` with an optional local image.
- Discord Rich Presence with name, description, second line, image, tooltip,
  activity type and elapsed timer.
- Automatic reconnection when Discord starts or restarts.
- Windows installer supporting per-user and machine-wide installation, and a portable archive.
- Continuous integration and automated, attested GitHub releases.

[Unreleased]: https://github.com/Yatsuuw/discord_presence_status/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Yatsuuw/discord_presence_status/releases/tag/v0.1.0
