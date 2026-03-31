# neko

`neko` is a Rust + Bevy port of the classic desktop cat that chases your mouse cursor.

This repository aims to preserve the feel of the older `neko` implementations while rebuilding the app in modern Rust with the Bevy game engine. The current app runs as a small transparent always-on-top window, animates the cat sprite, follows the desktop cursor, and plays the reference sounds.

Note: the code in this repository is authored with the help of AI coding agents and reviewed through the project's phased planning and verification process.

## Status

The project is working and actively being ported in phases.

Implemented now:

- transparent borderless Bevy window
- fixed-step behavior loop at 50 Hz
- 8-direction cursor chasing
- idle animation state machine
- wake, yawn, and sleep sounds
- wait mode and mouse passthrough support
- multi-display movement logic
- platform cursor backends for Windows, Linux, and macOS

Current platform reality:

- Windows: validated locally with `cargo test`, `cargo check`, and `cargo build`
- Linux: backend code is present, but manual verification is still pending
- macOS: backend code is present, but manual verification is still pending

Known caveats:

- Linux currently uses an X11 best-effort global cursor path
- Wayland/compositor behavior may limit always-on-top behavior
- macOS and Linux transparent window behavior depends on platform alpha compositing support

## Features

- Rust 100%
- Bevy 0.18
- transparent desktop-pet style window
- nearest-neighbor pixel-art rendering
- monitor-aware movement using Bevy monitor topology
- runtime window movement via desktop-space coordinates
- typed asset, state, and sound-event structure
- unit tests for movement, animation, and multi-display logic

## Controls

- Move the mouse: `neko` chases it
- Left click while idle: toggle wait mode
- Right click on `neko`: exit when mouse passthrough is disabled
- `Esc`: exit when the window has keyboard focus

## Run

Requirements:

- Rust toolchain
- a desktop environment that supports transparent windows

Commands:

```bash
cargo run
```

Validation:

```bash
cargo test
cargo check
cargo build
```

## Configuration

Runtime defaults are currently defined in [src/config.rs](src/config.rs):

- `speed = 2.0`
- `scale = 2.0`
- `quiet = false`
- `mouse_passthrough = false`

There is no external config file or CLI layer yet. The current goal is behavior parity first.

## Architecture

Main modules:

- [src/lib.rs](src/lib.rs): app bootstrap, window setup, startup placement
- [src/behavior.rs](src/behavior.rs): fixed-step movement, idle logic, animation selection
- [src/state.rs](src/state.rs): typed runtime state and direction enums
- [src/assets.rs](src/assets.rs): sprite and sound handles
- [src/audio.rs](src/audio.rs): sound playback
- [src/platform/cursor.rs](src/platform/cursor.rs): shared desktop monitor and cursor facade
- [src/platform/windows.rs](src/platform/windows.rs): Windows cursor backend
- [src/platform/linux.rs](src/platform/linux.rs): Linux cursor backend
- [src/platform/macos.rs](src/platform/macos.rs): macOS cursor backend

Bevy-specific implementation notes:

- fixed-step updates use `Time::<Fixed>::from_hz(...)`
- transparent windows use `transparent: true` with `ClearColor(Color::NONE)`
- multi-display placement uses `Monitor` and `PrimaryMonitor`
- movement updates the window with `WindowPosition::At(IVec2)`
- mouse passthrough uses `CursorOptions.hit_test`

## Project Layout

```text
.
|-- assets/
|-- src/
|   |-- platform/
|   |-- assets.rs
|   |-- audio.rs
|   |-- behavior.rs
|   |-- config.rs
|   |-- lib.rs
|   |-- main.rs
|   `-- state.rs
`-- _spec/
```

## Specs And Planning Docs

Project planning lives under [_spec](_spec):

- [neko_requirements.md](_spec/neko_requirements.md)
- [phase2_bevy_porting_plan.md](_spec/phase2_bevy_porting_plan.md)
- [phase3_detailed_porting_plan.md](_spec/phase3_detailed_porting_plan.md)
- [phase4_implementation_checklist.md](_spec/phase4_implementation_checklist.md)
- [req_os1_file_by_file_replacement_plan.md](_spec/req_os1_file_by_file_replacement_plan.md)

## Roadmap

Near-term work:

- manual Linux verification
- manual macOS verification
- stronger Linux backend handling beyond best-effort X11
- finish the `req-os1` checklist
- clean up remaining platform-specific caveats

## Reference

This project is a porting effort based on an existing Go implementation of [neko](https://github.com/crgimenes/neko), with the Bevy version intentionally kept close to the reference behavior before broader refactors or extra features are considered.
