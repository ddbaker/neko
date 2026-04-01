# neko ![neko righ2](assets/right2.png)

`neko` is a Rust + Bevy port of the classic desktop cat that chases your mouse cursor.

This repository aims to preserve the feel of the older `neko` implementations while rebuilding the app in modern Rust with the Bevy game engine. The current app runs as a small transparent always-on-top window, animates the cat sprite, follows the desktop cursor, and plays the reference sounds.

Note: the code in this repository is authored with the help of AI coding agents and reviewed through the project's phased planning and verification process.

## Status

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

## Configuration

Runtime defaults are currently defined in [src/config.rs](src/config.rs):

- `speed = 2.0`
- `scale = 2.0`
- `quiet = false`
- `mouse_passthrough = false`

There is no external config file or CLI layer yet. The current goal is behavior parity first.

## Reference

* [Go implementation neko](https://github.com/crgimenes/neko)
* [eliot-akira neko](https://github.com/eliot-akira/neko)
