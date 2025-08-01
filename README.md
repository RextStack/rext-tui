# Rext TUI

[![crates.io](https://img.shields.io/crates/v/rext-tui.svg)](https://crates.io/crates/rext-tui)
[![docs.rs documentation](https://img.shields.io/docsrs/rext-tui)](https://docs.rs/rext-tui)
[![MIT](https://img.shields.io/crates/l/rext-tui.svg)](./LICENSE)
[![Rust](https://github.com/RextStack/rext-tui/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/RextStack/rext-tui/actions/workflows/rust.yml)

> **Note**
> Build step will fail until the latest rext-core is published to crates.io, works locally where we're using the latest version.

The Rext Terminal User Interface, used to scaffold and build Rext apps.

The Core library or Rext (where all the magic happens) is [Rext Core](https://github.com/RextStack/rext-core)

Rext TUI is built on the fantastic [Ratatui](https://ratatui.rs) library.

## What is Rext?

Rext is a fullstack Rust framework in active development, aimed at providing a batteries included experience for building fullstack web applications.

## What does the TUI do?

The Rext TUI will be responsible for:
- Creating or initializing new projects
- Validating the project
- Running database migrations
- Adding new routes to your API
- Adding new models and other ORM operations
- Running integration tests
- Adding frontend components
- Debug Rext apps
- ...and more?

## Why a TUI and not a CLI?

A TUI is a fantastic way to keep developers efficient while providing them with a friendly, discoverable UI. It's much easier to find all the functionality of a new tool when all the options are available on the screen, versus having to read through help messages from a CLI.

> **Rext loves CLIs too!**
> The Rext CLI is coming as well! This will be the interface used for automation, running commands from scripts or AI Agents, or for those who prefer a CLI over a TUI.

## Maintaing a TUI and CLI seems like a lot of effort!

It is a little extra effort for a relatively niche reason (TUIs are more fun). Rext's TUI/CLI goal is to use the same unopinionated backend- the actual logic that the interfaces are running will all come from [Rext-Core](https://github.com/RextStack/rext-core), the TUI and CLI crates will just be interfaces calling core logic, nothing more.

While we'll still have two interfaces to deal with, ensuring they are using the same API removes a lot of tedium from maintaining both.

# Changelog

Visit [CHANGELOG](CHANGELOG.md)

> **Note**
> There are two changelogs, CHANGELOG.md, which is manually currated, and CLIFF_CHANGELOG.md, which is generated by `git-cliff`. The CLIFF log is used as a quick overview of anything we might have missed in the manually currated changelog when creating a new release. It can be ignored, unless you are helping create a release.

# Installation

run `cargo add rext-tui` or add this to your `Cargo.toml`:

```
[dependencies]
rext-tui = "0.1.0"
```

> **Warning**
> This project is in early development. The API is unstable and subject to change. Not recommended for production use or demos. Only install if you are looking to contribute and test.

# Usage

> **Not Available**

# Contribution Guidelines

Visit [CONTRIBUTING](CONTRIBUTING.md)

# License

Licensed under the MIT License. See [LICENSE](LICENSE.txt) for details.
