# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Bevy Text Popup is a Bevy plugin for creating temporary UI text popups with events. The plugin provides a simple event-driven system where users send `TextPopupEvent` to create customizable popups that can include buttons, timeouts, modal backgrounds, and various styling options.

## Architecture

The codebase is organized around three main components:

- **Core Types (`lib.rs`)**: Defines the main event (`TextPopupEvent`), components (`TextPopup`, `TextPopupExpires`, etc.), and configuration structs
- **Text Popup Generation (`text_popup.rs`)**: Contains the logic for converting events into actual UI entities with proper positioning and styling
- **Systems (`systems.rs`)**: Handles event processing, expiration cleanup, and button interactions

The plugin uses Bevy's ECS architecture where popups are entities with components that determine their behavior (expiration, button actions, etc.).

## Development Commands

### Building and Running
```bash
cargo build # Build the library
cargo fmt # Fix formatting issues
cargo clippy # Linter
cargo run --example <name>  # Run specific example (e.g. modal, buttons, border)
```

### Available Examples
All examples are in the `examples/` directory and demonstrate different features:
- `modal` - Modal popup with background overlay
- `buttons` - Confirm/dismiss buttons
- `border` - Custom borders and styling
- `custom_font` - Custom font usage
- `locations` - Different positioning options
- `custom_locations` - Custom pixel coordinates
- `transparency` - Background transparency
- `named` - Using Name component for popup identification
- `expiration` - Frame-based expiration timing

### Testing
```bash
cargo test               # Run all tests
cargo check              # Quick compilation check
```

## Key Implementation Details

- Popups are created by sending `TextPopupEvent` through Bevy's event system
- The plugin supports three expiration modes: Never, Seconds, and Frames
- Button actions are function pointers that receive `Commands` and the root entity ID
- Modal backgrounds are achieved by setting a background color on the root node
- Positioning uses a combination of CSS flexbox and absolute positioning
- All popups use `GlobalZIndex(i32::MAX)` by default to appear on top

## Bevy Compatibility

Currently targets Bevy 0.16. The compatibility table in README.md shows the mapping between Bevy versions and plugin versions.