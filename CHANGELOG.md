# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Ratatui styling macros: `styled_span!` and `styled_line!` for simplified creation of stylized text spans with localization and color support
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
- Added comprehensive key parsing for all common key types (F1-F12, modifiers, etc.)
- Added validation system for localization key bindings
- Added case-insensitive key parsing and better error handling
- Added dirs crate for getting user home directory and packaging assets
- Added rext-core dependency for creating new Rext apps
- Added create_new_app dialog, spawns when TUI is launched and rext_core::check_for_rext_app returns false, calls rext_core::scaffold_rext_app
- Added destroy_rext_app to settings, removes everything from a rext project (for testing!)
- Added generate_sea_orm_entities button to main interface, calls rext_core::generate_sea_orm_entities

### Fixed

- Fixed localization system to properly support arrow keys and navigation keys
- Fixed destroy_rext_app not using a result properly

### Changed

- Replaced counter demo with route management interface
- Removed main border around TUI
- Updated keyboard event handling to support dialog state
- Restructured App state to include dialog and input management
- Replaced all the text with localization system
- Replaced the key controls with localization system
- Moved theme switching from main interface (t key) to settings dialog
- Added more documentation to the codebase
- Overhauled the config system- package configs with binary, but can override with user configs in ~/.rext/

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