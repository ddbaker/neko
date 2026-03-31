# Phase 4 Implementation Checklist

## Purpose

This checklist maps the phase-3 detailed porting plan into concrete implementation tasks.

The checklist is intentionally ordered so that high-risk unknowns are resolved before lower-risk polish work.

## Phase 4 Rules

- do not add non-reference features during the first parity pass
- treat global cursor access as the first technical gate
- keep behavior logic close to the Go reference until parity is demonstrated
- run `cargo build` and `cargo check` before phase completion

## Milestone 0: Repository Bootstrap

- [x] Create `Cargo.toml`
- [x] Add Bevy as the primary dependency
- [x] Enable the Bevy `wav` feature for reference sound playback
- [x] Create `src/`
- [x] Create `assets/`
- [x] Create initial source files:
- [x] `src/main.rs`
- [x] `src/config.rs`
- [x] `src/assets.rs`
- [x] `src/state.rs`
- [x] `src/behavior.rs`
- [x] `src/audio.rs`
- [x] `src/platform/mod.rs`
- [x] `src/platform/cursor.rs`

## Milestone 1: Asset Preparation

- [x] Copy required sprite assets from the Go reference repo into local `assets/`
- [x] Copy required sound assets from the Go reference repo into local `assets/`
- [x] Exclude unused footprint assets for the first pass
- [x] Exclude unused claw-direction assets for the first pass
- [x] Verify asset filenames match the planned loader keys

## Milestone 2: Window And Rendering Baseline

- [x] Configure `DefaultPlugins`
- [x] Set `ImagePlugin::default_nearest()`
- [x] Configure a transparent primary window
- [x] Set window decorations off
- [x] Set window level to always-on-top
- [x] Set a transparent clear color
- [x] Set initial window resolution from `32x32 * scale`
- [x] Spawn `Camera2d`
- [x] Spawn one visible cat sprite entity
- [x] Confirm the sprite is crisp rather than blurred

## Milestone 3: Cursor And Native Window Spike

- [x] Implement an initial cursor provider abstraction in `src/platform/cursor.rs`
- [x] Test whether Bevy-level window cursor APIs are sufficient
- [x] If Bevy-level APIs are insufficient, investigate `bevy::winit` backend access
- [x] If backend access is still insufficient, implement a minimal platform cursor path behind the abstraction
- [x] Prove that cursor position is available while the cursor is outside the pet window
- [x] Prove that the native window can be repositioned at runtime
- [x] Prove that cursor tracking still works while the window is moving
- [x] Document the chosen cursor strategy in code comments where needed

Stop condition:

- [ ] If global cursor access cannot be made reliable enough for parity, stop further behavior work and document the blocker

## Milestone 4: Core Data Model

- [x] Implement `NekoConfig`
- [x] Set default values to match the Go reference
- [x] Implement `SpriteBase`
- [x] Implement `SpriteFrameKey`
- [x] Implement `SoundKey`
- [x] Implement `Direction`
- [x] Implement `NekoSoundEvent`
- [x] Implement `NekoAssets`
- [x] Implement `NekoState`

## Milestone 5: Asset Loading

- [x] Load all required sprite images through `AssetServer`
- [x] Load all required WAV files through `AssetServer`
- [x] Map logical sprite bases to typed frame keys
- [x] Store loaded handles in `NekoAssets`
- [x] Verify that all required handles resolve successfully at runtime

## Milestone 6: Fixed Timestep Setup

- [x] Insert `Time::<Fixed>::from_hz(50.0)`
- [x] Register fixed-update systems
- [x] Keep authoritative pet behavior inside `FixedUpdate`
- [x] Keep non-authoritative startup or support systems outside `FixedUpdate` as appropriate

## Milestone 7: Native Window Position Management

- [x] Store window position in `NekoState`
- [x] Implement runtime `WindowPosition::At(...)` updates
- [x] Compute the pet window center from window position and scaled size
- [x] Clamp movement to the intended monitor bounds
- [x] Verify the window cannot drift off the active bounds

## Milestone 8: Chase Behavior

- [x] Compute cursor offset relative to the pet window center
- [x] Compute Manhattan distance like the Go reference
- [x] Detect the near-cursor idle threshold
- [x] Compute movement angle with `atan2`
- [x] Normalize angle into `[0, 360)`
- [x] Map angle to 8 direction buckets
- [x] Apply straight movement using `speed`
- [x] Apply diagonal movement using `speed / sqrt(2)`
- [x] Select the correct directional sprite base
- [x] Reset idle timing fields during chase

## Milestone 9: Idle State Machine

- [x] Port state `0` idle-entry behavior
- [x] Port states `1..=3` as `awake`
- [x] Port states `4..=6` as `scratch`
- [x] Port states `7..=9` as `wash`
- [x] Port states `10..=12` as `yawn`
- [x] Port states `13+` as `sleep`
- [x] Port `count`, `min`, and `max` timing behavior
- [x] Reset `count` when it exceeds `max`
- [x] Advance idle state after each completed idle timing cycle
- [x] Preserve the longer yawn timing window

## Milestone 10: Animation Frame Selection

- [x] Implement single-image handling for `awake`
- [x] Implement frame `1` and frame `2` selection for other used animations
- [x] Select frame `1` when `count < min`
- [x] Select frame `2` otherwise
- [x] Sync the current renderable frame to the sprite entity
- [x] Avoid unnecessary sprite handle updates if the frame did not change

## Milestone 11: Wait Mode And Mouse Input

- [x] Detect left mouse button edge input
- [x] Only toggle waiting from the idle path
- [x] Keep the pet idling while `waiting == true`
- [x] Confirm wait mode does not corrupt idle state progression

## Milestone 12: Mouse Passthrough

- [x] Initialize hit testing from `NekoConfig.mouse_passthrough`
- [x] Apply the correct `CursorOptions.hit_test` behavior
- [x] Verify expected interaction when passthrough is off
- [x] Verify expected interaction when passthrough is on

## Milestone 13: Audio

- [x] Implement sound event emission for `IdleYawn`
- [x] Implement sound event emission for `Sleep`
- [x] Implement sound event emission for `Wake`
- [x] Consume sound events in a dedicated audio system
- [x] Spawn one-shot audio playback entities
- [x] Suppress sound when `quiet == true`
- [x] Verify WAV assets decode and play correctly

## Milestone 14: Logic Tests

- [x] Add tests for direction bucket mapping
- [x] Add tests for idle state progression
- [x] Add tests for frame selection
- [x] Add tests for movement clamping
- [x] Keep tests focused on pure logic rather than full window integration

## Milestone 15: Manual Verification

- [x] Verify the window starts transparent
- [x] Verify the window is undecorated
- [x] Verify the window stays above normal windows
- [x] Verify scaling matches the config value
- [x] Verify cursor chasing matches the Go reference closely
- [x] Verify diagonal motion speed feels correct
- [x] Verify the pet stops chasing when close to the cursor
- [x] Verify idle sequence order matches the reference
- [x] Verify yawn, sleep, and wake sounds trigger at the right times
- [x] Verify wait mode behavior manually
- [x] Verify pressing `Esc` exits `neko` when the app window has keyboard focus
- [x] Verify right-clicking on `neko` exits the app when mouse passthrough is disabled
- [x] Verify mouse passthrough behavior manually
- [x] Verify the window remains inside intended monitor bounds

## Milestone 16: Build Validation

- [x] Run `cargo fmt` if formatting is configured
- [x] Run `cargo check`
- [x] Run `cargo build`
- [x] Resolve all compilation errors
- [x] Resolve all warnings that reflect real correctness or maintenance issues

## Phase 4 Exit Criteria

- [x] The implementation checklist items required for parity are complete
- [x] The app behavior is acceptably close to the Go reference
- [x] `cargo check` succeeds
- [x] `cargo build` succeeds

## Post-Parity Extension: `req-md1` Multi-Display Support

This section maps `req-md1` from `_spec/neko_requirements.md` into concrete follow-up work.
It supersedes the current single-monitor clamp in `src/platform/cursor.rs` and `src/behavior.rs`
when multi-display support is in scope.

### Implementation

- [x] Add a monitor-topology abstraction that can return every connected monitor as a desktop-space rectangle in physical pixels
- [x] Prefer Bevy monitor ECS data (`Monitor`, `PrimaryMonitor`) as the engine-level source of monitor topology
- [x] Keep the existing Windows backend path only for global cursor acquisition unless Bevy monitor data proves insufficient
- [x] Preserve negative desktop coordinates for monitors placed left of or above the primary monitor
- [x] Represent multi-monitor topology separately from the current single `MonitorBounds` helper
- [x] Add a helper that finds the monitor containing a point in desktop space
- [x] Add a helper that finds the nearest monitor when the point falls in a gap between displays
- [x] Add a helper that clamps a window position against a chosen monitor rectangle
- [x] Update startup placement so the pet centers on the monitor containing the current cursor, with primary-monitor fallback only when cursor or monitor data is unavailable
- [x] Update `fixed_update_neko_behavior` so the clamp target is chosen from the cursor's current monitor instead of the pet's current monitor
- [x] Allow `state.window_pos` to cross a display border when the cursor has already crossed onto another monitor
- [x] Prevent the current stuck-running case by switching the active clamp target as soon as the cursor's monitor changes
- [x] Keep runtime movement on `WindowPosition::At(IVec2)` because Bevy defines `At` in physical screen-space pixels
- [x] Preserve the current behavior when only one monitor is detected
- [x] Define fallback behavior when monitor topology cannot be queried on a platform; log the fidelity gap and keep the conservative single-monitor clamp instead of failing

### Tests

- [x] Add pure logic tests for left-to-right crossing across adjacent monitors
- [x] Add pure logic tests for right-to-left crossing across adjacent monitors
- [x] Add pure logic tests for stacked vertical monitor layouts
- [x] Add pure logic tests for monitors with negative desktop coordinates
- [x] Add pure logic tests for uneven monitor sizes and monitor gaps
- [x] Add a regression test that reproduces the current boundary-stuck scenario and proves the active monitor switches correctly
- [x] Keep multi-display tests independent from live OS monitor enumeration

### Manual Verification

- [ ] Verify crossing from display 1 to display 2 on a side-by-side layout
- [ ] Verify crossing from display 2 back to display 1 on the same layout
- [ ] Verify crossing on a vertically stacked monitor layout
- [ ] Verify behavior when the secondary monitor uses a different resolution or DPI scale
- [ ] Verify the pet does not stall in running animation when the cursor crosses a display border
- [ ] Verify the pet remains fully visible after a cross-display move
- [ ] Verify startup placement chooses the monitor containing the cursor
- [ ] Verify single-monitor behavior is unchanged when only one display is active

### Validation

- [x] Run `cargo test`
- [x] Run `cargo check`
- [x] Run `cargo build`

## Post-Parity Extension: `req-os1` Windows, Linux and macOS Support

This section maps `_spec/req_os1_file_by_file_replacement_plan.md` into concrete follow-up work.
It covers the Windows/Linux/macOS replacement track required by `_spec/neko_requirements.md`.

### Audit

- [x] Verify the current repository for Windows-only code paths in `Cargo.toml`, `src/platform/cursor.rs`, `src/lib.rs`, and `src/behavior.rs`
- [x] Record the current non-Windows degraded behavior so replacement work can be verified against it

### `Cargo.toml`

- [x] Keep the existing Windows-specific `windows-sys` dependency path for the Windows backend
- [x] Add target-specific native dependencies for a Linux global-cursor backend
- [x] Add target-specific native dependencies for a macOS global-cursor backend
- [x] Keep Bevy as the engine/windowing foundation rather than introducing an alternate framework

### `src/platform/mod.rs`

- [x] Expand the platform facade beyond a single `cursor` module if backend separation makes the implementation clearer
- [x] Add per-OS backend modules behind `cfg` gates for Windows, Linux, and macOS
- [x] Keep desktop layout and monitor-topology logic platform-neutral and shared

### `src/platform/cursor.rs`

- [x] Keep `MonitorBounds`, `DesktopMonitor`, and `DesktopMonitorLayout` as shared platform-neutral logic
- [x] Move Windows FFI cursor code into a Windows backend module
- [x] Replace the non-Windows `global_cursor_position()` stub with a real Linux backend
- [x] Replace the non-Windows `global_cursor_position()` stub with a real macOS backend
- [x] Replace the non-Windows `monitor_bounds_for_point()` stub with real backend support or explicit capability reporting
- [x] Keep Bevy monitor ECS data (`Monitor`, `PrimaryMonitor`) as the primary monitor-topology source
- [x] Reduce `monitor_bounds_for_point()` to a backend fallback rather than the primary path
- [x] Add runtime logging or capability reporting for unsupported compositor/backend cases instead of silently returning broken behavior

### `src/lib.rs`

- [x] Add `CompositeAlphaMode` imports behind `cfg(any(target_os = "macos", target_os = "linux"))`
- [x] Set macOS transparent windows to `CompositeAlphaMode::PostMultiplied`
- [x] Set Linux transparent windows to `CompositeAlphaMode::PreMultiplied`
- [x] Keep `transparent: true` and `ClearColor(Color::NONE)` so cross-platform transparency stays correct
- [x] Treat `WindowLevel::AlwaysOnTop` as best effort on Linux/Wayland and document or log the platform limitation
- [x] Keep startup placement Bevy-first: prefer `Monitor` and `PrimaryMonitor`, use global cursor only to choose the startup monitor
- [x] Fall back to primary-monitor startup placement when global cursor data is unavailable

### `src/behavior.rs`

- [x] Route all cursor access through the cross-platform platform facade
- [x] Remove Windows-shaped native monitor assumptions from the movement path
- [x] Define explicit behavior when global cursor data is temporarily unavailable
- [x] Keep chase and idle gameplay logic platform-neutral after backend replacement

### Spec Follow-Up

- [ ] Update `_spec/phase3_detailed_porting_plan.md` with `req-os1` cross-platform notes after implementation decisions are final
- [ ] Update this checklist as `req-os1` implementation work is completed and verified

### Tests

- [x] Add unit tests for any new platform-neutral cursor capability or fallback logic
- [ ] Keep existing multi-display and movement logic tests passing on non-Windows builds
- [ ] Verify Linux/macOS cfg-split code compiles cleanly without breaking Windows builds

### Manual Verification

- [ ] On Windows 11, verify cursor chasing works outside the pet window bounds
- [ ] On Windows 11, verify multi-display movement still works
- [ ] On Windows 11, verify transparent window, always-on-top, and mouse passthrough behavior
- [ ] On Linux, verify cursor chasing works on the supported compositor/backend
- [ ] On Linux, verify transparent window behavior with the configured `CompositeAlphaMode`
- [ ] On Linux, verify always-on-top behavior and document any Wayland/compositor limitation
- [ ] On Linux, verify mouse passthrough behavior
- [ ] On macOS, verify cursor chasing works outside the pet window bounds
- [ ] On macOS, verify transparent window behavior with `CompositeAlphaMode::PostMultiplied`
- [ ] On macOS, verify always-on-top and mouse passthrough behavior

### Validation

- [x] Run `cargo test`
- [x] Run `cargo check`
- [x] Run `cargo build`
- [x] If the toolchains are available, run target-specific validation for Windows MSVC
- [ ] If the toolchains are available, run target-specific validation for Linux GNU
- [ ] If the toolchains are available, run target-specific validation for macOS targets

## Deferred Items

- [ ] Consider unused footprint assets only after parity
- [ ] Consider unused claw-direction assets only after parity
- [ ] Consider richer config loading only after parity
- [ ] Consider architecture cleanup only after parity
