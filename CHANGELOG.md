# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- New UI design with color scheme (#1a1a1a background, #ff6b35 accent, #cccccc regular text)
- "Add API endpoint" button in top-left corner with keyboard shortcut (e)
- Endpoint creation dialog with text input functionality
- Centered dialog box with accent-colored border
- Text input handling for route names
- Quit instructions displayed at bottom of screen
- Added serde and toml to dependencies
- Added config file for color schemes
- Added error handling for config file
- Added config.rs for reading config file
- Added current_theme.toml for storing current theme
- Added theme switching functionality
- Added current_localization.toml for storing current localization
- Added localization system with english as fallback
- Settings dialog with keyboard shortcut (s) to access theme and language options
- Language selection dialog with search functionality
- Language filtering with up/down arrow navigation
- Dynamic language switching with localization reload
- Theme switching moved to settings dialog from main interface


### Changed

- Replaced counter demo with route management interface
- Removed main border around TUI
- Updated keyboard event handling to support dialog state
- Restructured App state to include dialog and input management
- Replaced all the text with localization system
- Replaced the key controls with localization system
- Moved theme switching from main interface (t key) to settings dialog

### Removed

- Counter increment/decrement functionality
- Bordered layout from previous demo

## [0.1.0] - 2025-07-18

### Added

- CHANGELOG.md file
- README.md file
- CONTRIBUTION.md guide
- AGENTS.md file for helping AI agents (to be used with good intentions)
- lib.rs, core of the rext-tui binary
- A pre-commit hook for running cargo commands before commiting changes
- A code-workspace file with some workspace support
- A github workflow, tests and builds commits to main, caches assets
- git-cliff pre-commit hook
- CLIFF_CHANGELOG.md, a git-cliff generated changelog for reference
- A bootstrap script to bootstrap development environment quickly
- Cargo.toml package info
- This initial release is to just jump-start the changelog and releases, nothing decent in it

### Fixed

- Workspace cleanup (removed py pre-commit)

[unreleased]: https://github.com/RextStack/rext-tui/releases/tag/v0.1.0