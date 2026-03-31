# `req-os1` File-By-File Replacement Plan

This document maps `req-os1` from `_spec/neko_requirements.md` into a concrete file-by-file replacement plan.

Requirement summary:

- target platforms are Windows 11, Linux, and macOS
- step-1 is to identify Windows-only assumptions in the current repository
- step-2 is to replace those assumptions with Windows/Linux/macOS-capable behavior

## Bevy API Baseline

The current Bevy 0.18 APIs that should remain the cross-platform foundation are:

- `Monitor` and `PrimaryMonitor` for monitor topology inside ECS
- `WindowPosition::At(IVec2)` for runtime window movement in desktop-space physical pixels
- `CursorOptions.hit_test` for mouse passthrough
- `transparent: true` plus `ClearColor(Color::NONE)` for transparent windows

Bevy constraints that directly affect `req-os1`:

- `Window::cursor_position()` and `Window::physical_cursor_position()` return `None` when the cursor is outside the window, so they cannot provide global desktop cursor tracking for the pet
- transparent desktop windows on macOS and Linux should use `CompositeAlphaMode`
  - macOS: `CompositeAlphaMode::PostMultiplied`
  - Linux: `CompositeAlphaMode::PreMultiplied`
- `WindowLevel` is documented as unsupported on Wayland, so Linux always-on-top behavior must be treated as best effort rather than guaranteed parity

## Files Requiring Replacement Work

### `Cargo.toml`

Current issue:

- the repository only declares a target-specific dependency for Windows: `windows-sys`

Replacement plan:

- keep the existing Windows dependency path for the Windows cursor backend
- add target-specific native dependency groups for Linux and macOS cursor backends
- keep Bevy as the engine and windowing foundation; only use minimal native bindings where Bevy does not expose the required global cursor behavior
- do not add alternate game or UI frameworks

Expected result:

- the project can compile platform backends for Windows, Linux, and macOS from a single codebase

### `src/platform/mod.rs`

Current issue:

- the platform layer only exports `cursor`

Replacement plan:

- turn `platform` into the facade for all target-specific cursor backends
- keep platform-neutral desktop layout logic shared
- add per-OS backend modules behind `cfg` gates
- keep the public API stable for the rest of the game logic

Suggested module shape:

- `src/platform/cursor.rs` for shared types and the public facade
- `src/platform/windows.rs` for Windows native cursor access
- `src/platform/macos.rs` for macOS native cursor access
- `src/platform/linux.rs` for Linux native cursor access

If Linux requires multiple backend paths:

- keep X11 and Wayland handling behind the same Linux facade
- expose capability or degraded-mode reporting instead of leaking backend differences into gameplay code

### `src/platform/cursor.rs`

Current issue:

- `global_cursor_position()` is implemented only on Windows
- `monitor_bounds_for_point()` is implemented only on Windows
- non-Windows builds return `None`, which means the pet cannot chase the cursor reliably outside Windows

Replacement plan:

- keep `MonitorBounds`, `DesktopMonitor`, and `DesktopMonitorLayout` as platform-neutral shared logic
- move Windows FFI code out of the shared file into a Windows backend module
- replace the non-Windows `None` stubs with real macOS and Linux implementations
- keep Bevy monitor ECS data as the preferred monitor-topology source
- reduce `monitor_bounds_for_point()` to a backend fallback rather than the primary monitor-selection path
- make backend availability explicit in logs or result types so unsupported runtime environments can be diagnosed

Linux-specific planning note:

- Linux support must account for the fact that desktop behavior differs between X11 and Wayland
- if global cursor access or always-on-top semantics cannot be provided on a specific compositor, the backend should report a documented degraded mode instead of silently failing

macOS-specific planning note:

- macOS backend work should focus on global cursor coordinates only
- monitor topology should still come from Bevy ECS unless there is a documented gap

### `src/lib.rs`

Current issue:

- the primary window is configured without macOS/Linux `CompositeAlphaMode`
- `WindowLevel::AlwaysOnTop` is currently applied unconditionally even though Wayland support is documented as unsupported
- startup placement still contains a native fallback shaped around the Windows helper path

Replacement plan:

- add `CompositeAlphaMode` imports behind `cfg(any(target_os = "macos", target_os = "linux"))`
- set:
  - macOS: `CompositeAlphaMode::PostMultiplied`
  - Linux: `CompositeAlphaMode::PreMultiplied`
- keep `transparent: true`, `decorations: false`, and `ClearColor(Color::NONE)`
- treat always-on-top as best effort on Linux and document/log the limitation where needed
- keep startup placement Bevy-first:
  - prefer `Monitor` and `PrimaryMonitor` ECS data
  - use global cursor only to choose the startup monitor
  - if global cursor is unavailable, fall back to primary monitor placement instead of silently degrading into a broken state

Expected result:

- transparent startup behavior is configured correctly for Windows, Linux, and macOS
- startup placement still works even when a backend cannot provide a global cursor immediately

### `src/behavior.rs`

Current issue:

- the fixed update path returns early when `global_cursor_position()` is unavailable
- a Windows-shaped native monitor fallback is still present in the movement path

Replacement plan:

- route all cursor access through the new cross-platform platform facade
- keep Bevy monitor ECS queries as the primary active-monitor selection path
- treat backend cursor failure as an explicit runtime state:
  - either keep the last valid motion state
  - or force the pet into a safe idle state
  - do not silently make one target OS behave as if the game loop is broken
- keep the current desktop layout logic and cross-display clamping tests
- remove assumptions in gameplay code that only the Windows backend can supply native monitor information

Expected result:

- chase behavior remains platform-neutral
- OS-specific behavior is isolated to the platform layer

### `src/config.rs`

Current issue:

- no direct Windows-only code is present, but there is no place yet for platform diagnostics or backend policy if they become necessary

Replacement plan:

- keep this file unchanged unless runtime backend selection or diagnostics become necessary
- if a backend-selection knob is needed, place it here rather than inside gameplay code

Current recommendation:

- do not change this file in the first `req-os1` pass unless implementation proves it necessary

### `src/assets.rs`

Current issue:

- no platform-specific code found

Replacement plan:

- no replacement required
- retest asset loading on all three target OSes after platform changes

### `src/audio.rs`

Current issue:

- no platform-specific code found

Replacement plan:

- no replacement required for `req-os1`
- retest WAV playback on Windows, Linux, and macOS after platform changes

### `src/state.rs`

Current issue:

- no platform-specific code found

Replacement plan:

- no replacement required

### `src/main.rs`

Current issue:

- no platform-specific code found

Replacement plan:

- no replacement required unless startup bootstrapping is later split by platform

## Spec Files To Update After Code Changes

### `_spec/phase3_detailed_porting_plan.md`

Update needed:

- add `req-os1` as a cross-platform implementation track
- document that global cursor acquisition is platform-backed because Bevy cursor APIs are window-local
- document Linux/Wayland always-on-top limitations
- document macOS/Linux transparent window alpha-mode requirements

### `_spec/phase4_implementation_checklist.md`

Update needed:

- add a dedicated `req-os1` checklist section
- separate code-complete items from manual per-OS verification items

## Validation Matrix

### Build And Test

- `cargo test`
- `cargo check`
- `cargo build`
- if toolchains are available, run target-specific checks for:
  - Windows MSVC
  - Linux GNU
  - macOS Apple Silicon and/or Intel, depending on release targets

### Manual Verification

Windows 11:

- cursor chasing works outside the window bounds
- multi-display movement still works
- transparent window works
- always-on-top works
- mouse passthrough works

Linux:

- cursor chasing works on the supported compositor/backend
- transparent window works with the configured alpha mode
- always-on-top behavior is verified and any compositor limitation is documented
- mouse passthrough works

macOS:

- cursor chasing works outside the window bounds
- transparent window works with `CompositeAlphaMode::PostMultiplied`
- always-on-top behavior works as expected
- mouse passthrough works

## Recommended Implementation Order

1. Replace `src/platform/cursor.rs` with a real cross-platform backend facade
2. Update `Cargo.toml` target dependencies to support the new backends
3. Update `src/lib.rs` window configuration for macOS/Linux transparency and best-effort window level behavior
4. Update `src/behavior.rs` to consume the new cursor/backend result model cleanly
5. Run build/test validation
6. Perform Windows, Linux, and macOS manual verification
