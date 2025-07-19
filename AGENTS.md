# AGENTS.md - Project Context

## Project Overview

**rext-tui** - The Rext Terminal User Interface

A TUI development tool for scaffolding and managing Rext fullstack web applications.

## Current Status

- **Development Stage**: Very early development (~5% complete)
- **Version**: 0.1.0
- **Rust Edition**: 2024
- **Current State**: Basic TUI demo with counter functionality

## Architecture

### Architecture Goal

rext-tui aims to provide a friendly, discoverable interface for developers to manage Rext projects. It will serve as a frontend to rext-core functionality, handling tasks like project creation, database migrations, route generation, and more.

### Core Technologies
- **TUI Framework**: Ratatui (0.29.0) - Modern terminal user interface framework
- **Terminal Handling**: Crossterm (0.29.0) - Cross-platform terminal manipulation
- **Error Reporting**: color-eyre (0.6.3) - Beautiful error reports
- **Error Handling**: thiserror (2.0.12) - Structured error handling

### Key Components

1. **App Structure**
   - `App` struct manages application state and main loop
   - Event-driven architecture using crossterm events
   - Ratatui rendering system for UI components

2. **Error Handling**
   - `RextTuiError` enum with structured error types
   - Proper error propagation from crossterm operations

### Project Structure
```
rext-tui/
├── src/
│   ├── main.rs         # Application entry point
│   ├── lib.rs          # Main TUI application logic
│   └── error.rs        # Custom error types
├── Cargo.toml          # Dependencies and metadata
├── hooks/pre-commit    # Git pre-commit hooks
└── tests/              # Integration tests
```

## Planned Features

The Rext TUI will be responsible for:
- Creating or initializing new Rext projects
- Validating project structure and configuration
- Running database migrations
- Adding new API routes
- Adding new models and ORM operations
- Running integration tests
- Adding frontend components
- Debugging Rext applications
- Project management and monitoring

## Development Notes

- Built with Ratatui for cross-platform terminal UI
- Uses event-driven architecture for responsive user interaction
- Currently implements a basic counter demo as foundation
- Follows Rust 2024 edition conventions
- Pre-commit hooks ensure code quality (fmt, clippy, test)
- Will interface with rext-core for actual functionality

## Why a TUI vs CLI?

A TUI provides a discoverable, friendly interface where developers can see all available options on screen, rather than memorizing CLI commands or reading help text. This makes the tool more approachable and efficient for interactive development workflows.

## Goals

The Rext TUI aims to be the primary development interface for Rext applications, providing an intuitive way to scaffold, manage, and debug fullstack Rust web applications.

Visit [Rext Stack](https://rextstack.org) for more information.

## For AI Agents

When working on this project:
- Follow Rust 2024 edition conventions
- Use Ratatui widgets and patterns for UI components
- Maintain event-driven architecture with crossterm
- Use structured error handling with `RextTuiError`
- Ensure all UI components are responsive and accessible
- Keep the interface intuitive and discoverable
- Focus on developer experience and workflow efficiency
- Test TUI components thoroughly
- Never add new features that were not requested
- Update Changelog with changes, following `Keep a Changelog` formatting
- Write a commit message, but NEVER commit changes

### TUI Development Guidelines
- Use Ratatui's widget system for all UI components
- Handle keyboard events through the event system
- Ensure proper terminal cleanup on exit
- Design for various terminal sizes and capabilities
- Follow accessibility best practices for terminal applications