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
- [ ] Test whether Bevy-level window cursor APIs are sufficient
- [ ] If Bevy-level APIs are insufficient, investigate `bevy::winit` backend access
- [x] If backend access is still insufficient, implement a minimal platform cursor path behind the abstraction
- [x] Prove that cursor position is available while the cursor is outside the pet window
- [x] Prove that the native window can be repositioned at runtime
- [x] Prove that cursor tracking still works while the window is moving
- [ ] Document the chosen cursor strategy in code comments where needed

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
- [ ] Verify the window cannot drift off the active bounds

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
- [ ] Confirm wait mode does not corrupt idle state progression

## Milestone 12: Mouse Passthrough

- [x] Initialize hit testing from `NekoConfig.mouse_passthrough`
- [x] Apply the correct `CursorOptions.hit_test` behavior
- [ ] Verify expected interaction when passthrough is off
- [ ] Verify expected interaction when passthrough is on

## Milestone 13: Audio

- [x] Implement sound event emission for `IdleYawn`
- [x] Implement sound event emission for `Sleep`
- [x] Implement sound event emission for `Wake`
- [x] Consume sound events in a dedicated audio system
- [x] Spawn one-shot audio playback entities
- [x] Suppress sound when `quiet == true`
- [ ] Verify WAV assets decode and play correctly

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
- [ ] Verify scaling matches the config value
- [x] Verify cursor chasing matches the Go reference closely
- [ ] Verify diagonal motion speed feels correct
- [ ] Verify the pet stops chasing when close to the cursor
- [ ] Verify idle sequence order matches the reference
- [ ] Verify yawn, sleep, and wake sounds trigger at the right times
- [ ] Verify wait mode behavior manually
- [ ] Verify mouse passthrough behavior manually
- [ ] Verify the window remains inside intended monitor bounds

## Milestone 16: Build Validation

- [x] Run `cargo fmt` if formatting is configured
- [x] Run `cargo check`
- [x] Run `cargo build`
- [x] Resolve all compilation errors
- [x] Resolve all warnings that reflect real correctness or maintenance issues

## Phase 4 Exit Criteria

- [ ] The implementation checklist items required for parity are complete
- [x] The app behavior is acceptably close to the Go reference
- [x] `cargo check` succeeds
- [x] `cargo build` succeeds

## Deferred Items

- [ ] Consider unused footprint assets only after parity
- [ ] Consider unused claw-direction assets only after parity
- [ ] Consider richer config loading only after parity
- [ ] Consider architecture cleanup only after parity
