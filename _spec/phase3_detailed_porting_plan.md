# Phase 3 Detailed Porting Plan

## Purpose

This document converts the phase-2 Bevy architecture plan into an implementation-oriented porting plan for the Rust version of `neko`.

This file is intended to be specific enough to guide implementation, testing, and verification, while still remaining a plan rather than a completion checklist.

## Planning Inputs

This plan is based on:

- the reference Go implementation at `D:\devel\inetsrc\neko`
- the phase-1 behavior analysis
- the phase-2 Bevy architecture plan
- current Bevy API references verified against official docs

## Project Constraints

- Rust only
- Bevy only as the application framework
- preserve the current Go reference behavior first
- avoid introducing non-reference features during the first parity pass

## Desired End State

At the end of the first implementation pass, this repository should contain a small Bevy desktop application that:

- opens a transparent undecorated cat window
- remains above normal windows
- loads the reference sprite and sound assets
- follows the cursor using the same 8-direction logic as the Go version
- idles, yawns, sleeps, and wakes using the same state progression
- supports `speed`, `scale`, `quiet`, and mouse passthrough behavior
- passes `cargo build` and `cargo check`

## Confirmed Bevy API Assumptions

The following Bevy APIs have been re-verified and should be used as the baseline:

- `WindowPlugin` for primary window setup
- `WindowLevel::AlwaysOnTop` for the pet window level
- `ClearColor(Color::NONE)` for actual window transparency
- `CursorOptions.hit_test` for mouse passthrough behavior
- `WindowPosition::At(...)` for runtime native window movement
- `Time::<Fixed>::from_hz(50.0)` with `FixedUpdate`
- `ImagePlugin::default_nearest()` for pixel-art rendering
- `Sprite::from_image(...)` and standard `AssetServer` image loading
- `AudioPlayer::new(...)` for one-shot audio playback

Important dependency note:

- WAV playback requires enabling Bevy's `wav` Cargo feature

## Planned Repository Layout

When implementation starts, the repository should move from spec-only state into the following minimal source layout:

- `Cargo.toml`
- `src/main.rs`
- `src/config.rs`
- `src/assets.rs`
- `src/state.rs`
- `src/behavior.rs`
- `src/audio.rs`
- `src/platform/mod.rs`
- `src/platform/cursor.rs`
- `assets/`
- `tests/` only if integration tests become useful

## Planned Asset Layout

Create a local `assets/` directory and copy only the reference assets required by the current Go behavior:

- `awake.png`
- `up1.png`
- `up2.png`
- `upright1.png`
- `upright2.png`
- `right1.png`
- `right2.png`
- `downright1.png`
- `downright2.png`
- `down1.png`
- `down2.png`
- `downleft1.png`
- `downleft2.png`
- `left1.png`
- `left2.png`
- `upleft1.png`
- `upleft2.png`
- `scratch1.png`
- `scratch2.png`
- `wash1.png`
- `wash2.png`
- `yawn1.png`
- `yawn2.png`
- `sleep1.png`
- `sleep2.png`
- `awake.wav`
- `idle3.wav`
- `sleep.wav`

Do not add unused footprint or claw assets in the first pass unless implementation later shows that they are unexpectedly required.

## Planned Cargo Setup

The first implementation step should create a Bevy application with the minimum features needed for this port.

Requirements:

- enable Bevy audio support
- enable Bevy WAV decoding support
- keep the dependency set narrow

Guideline:

- prefer Bevy defaults unless a specific default feature conflicts with the transparent desktop-pet use case

## Module-Level Plan

## `src/main.rs`

Responsibilities:

- build the Bevy app
- configure `DefaultPlugins`
- set `ImagePlugin::default_nearest()`
- configure the primary window
- insert `ClearColor(Color::NONE)`
- set fixed timestep resource
- register startup systems, update systems, and fixed-update systems

The startup app should remain small and explicit. Do not hide the app composition behind unnecessary plugin layers at the beginning.

## `src/config.rs`

Define:

- `NekoConfig`

Initial fields:

- `speed: f32`
- `scale: f32`
- `quiet: bool`
- `mouse_passthrough: bool`

Default values should match the Go reference:

- `speed = 2.0`
- `scale = 2.0`
- `quiet = false`
- `mouse_passthrough = false`

Initial implementation guidance:

- begin with in-code defaults
- structure the module so CLI or file-backed config can be added without redesign

## `src/assets.rs`

Define:

- `SpriteBase`
- `SpriteFrameKey`
- `SoundKey`
- `NekoAssets`

Responsibilities:

- load all required images and sounds through `AssetServer`
- map sprite bases and frame variants to typed keys
- expose helpers for resolving the current sprite handle from state

Planned enum split:

- `SpriteBase` identifies the logical animation family such as `Up`, `Scratch`, or `Sleep`
- `SpriteFrameKey` identifies the final renderable frame such as `Up1`, `Up2`, or `Awake`

This avoids stringly typed sprite selection inside the runtime logic.

## `src/state.rs`

Define:

- `NekoState`
- `Direction`
- `NekoSoundEvent`

Suggested `NekoState` fields:

- `waiting: bool`
- `window_pos: Vec2`
- `distance: i32`
- `count: i32`
- `min: i32`
- `max: i32`
- `state: i32`
- `sprite_base: SpriteBase`
- `last_frame: Option<SpriteFrameKey>`

Optional additional helper fields that are acceptable if they reduce boilerplate:

- `current_direction: Option<Direction>`
- `startup_complete: bool`

Porting rule:

- keep the state model close to the Go version until parity is demonstrated

## `src/behavior.rs`

Responsibilities:

- fixed-step chase and idle behavior
- idle state progression
- animation frame selection
- movement vector and direction bucketing
- sound-event emission
- monitor selection and window clamping

Planned system split:

- `initialize_neko_state`
- `fixed_update_neko_behavior`
- `apply_window_position`
- `sync_sprite_frame`

If the implementation remains clearer with fewer systems, reducing the number of systems is acceptable. Fidelity is more important than theoretical purity.

## `src/audio.rs`

Responsibilities:

- consume `NekoSoundEvent`
- spawn one-shot `AudioPlayer` entities
- skip playback when `quiet` is enabled

Planned sound event enum:

- `Wake`
- `Sleep`
- `IdleYawn`

## `src/platform/cursor.rs`

Responsibilities:

- provide cursor acquisition independent from the rest of the app
- isolate any lower-level window backend access required for global cursor tracking

Suggested interface shape:

- one function or resource method that returns cursor position in desktop coordinates

The rest of the app should not know whether cursor data came from high-level Bevy API access or a lower-level platform path.

## Runtime Behavior Porting Plan

## Startup

At startup, the app should:

1. create the Bevy app
2. configure a transparent primary window
3. size the window to `32 * scale` by `32 * scale`
4. set the window title to `Neko`
5. spawn a `Camera2d`
6. spawn the cat sprite entity
7. initialize `NekoState`
8. load required assets

The initial window position should be centered on the monitor containing the current cursor when monitor topology and global cursor data are available. Bevy monitor ECS data (`Monitor`, `PrimaryMonitor`) should be the preferred source of startup placement, with primary-monitor fallback only when topology or cursor data is unavailable. Exact parity with the Go startup location is desirable but not blocking if later movement works correctly.

## Fixed-Step Behavior

The reference logic should be ported into the Bevy fixed timestep with minimal semantic change.

Each fixed tick should do the following:

1. increment `count`
2. obtain current global cursor position
3. resolve the active monitor from the cursor's desktop-space position
4. compute cursor offset relative to the cat window center
5. compute `distance`
6. if `distance < 32` or `waiting == true`, execute idle logic
7. otherwise execute chase logic
8. clamp the resulting window position against the active monitor, or the nearest monitor if the cursor is in a display gap
9. emit sound events if transitions require them
10. choose the renderable sprite frame for this tick

## Idle Logic Port

The idle path should preserve the Go state semantics:

- state `0` should move into the active idle sequence
- states `1..=3` render `awake`
- states `4..=6` render `scratch`
- states `7..=9` render `wash`
- states `10..=12` render `yawn`
- states `13+` render `sleep`

Timing rules:

- default chase timing uses `min = 8`, `max = 16`
- yawn uses `min = 32`, `max = 64`
- when `count > max`, reset `count = 0`
- if `state > 0`, advance to the next idle state after reset

Sound rules:

- when state is yawn and `count == min`, emit `IdleYawn`
- when the idle sequence progresses into sleep, emit `Sleep`
- when waking from sleep back into chase, emit `Wake`

## Chase Logic Port

The chase logic should not be rewritten as continuous steering. Preserve the current bucketed 8-direction movement.

Steps:

1. compute angle from cursor offset using `atan2`
2. normalize to `[0, 360)`
3. map angle to one of 8 sectors
4. update `window_pos`
5. set `sprite_base`
6. reset idle state timing

Direction mapping should stay equivalent to the Go thresholds so the sprite choice and movement feel the same.

## Animation Frame Selection

Render frame resolution should stay close to the Go `Draw()` logic:

- `awake` uses a single image
- every other used animation family uses frame `1` or `2`
- if `count < min`, select frame `1`
- otherwise select frame `2`

If the frame is unchanged from the previous tick, updating the sprite handle can be skipped as an optimization, but this is not required for correctness.

## Mouse Passthrough Plan

Startup behavior:

- initialize the window hit-test mode from `NekoConfig.mouse_passthrough`

Runtime behavior:

- no extra toggle UI is required for the first parity pass unless needed for debugging

The requirement is to match the reference app's configurable passthrough behavior, not to introduce a new user-facing toggle system.

## Wait Mode Plan

The Go reference toggles waiting when left click occurs while the cat is in the idle path.

The Rust port should:

- detect left mouse button edge transitions
- only toggle `waiting` from the idle path
- continue to idle while `waiting == true`

If mouse passthrough is enabled, the exact interaction model may become platform-sensitive. This should be tested and documented during implementation.

## Monitor Topology And Multi-Display Plan

`req-md1` requires the pet to move across monitor boundaries as seamlessly as possible, so the target plan must support multi-display movement rather than locking the pet to one display.

Preferred monitor-topology source:

- query Bevy monitor ECS data through `Monitor` entities
- use each monitor's desktop-space position via `Monitor.physical_position`
- use each monitor's size via `Monitor.physical_size()` or equivalent physical width/height fields
- use `PrimaryMonitor` only as startup fallback or degraded-mode fallback
- preserve negative desktop coordinates for monitors arranged left of or above the primary monitor

Movement plan:

- resolve the active monitor from the global cursor position each fixed tick
- if the cursor falls in a gap between displays, choose the nearest monitor rather than freezing movement
- clamp the pet window against the active monitor's bounds, not only the pet's current monitor
- allow the pet window to cross a display border once the cursor's active monitor changes
- keep runtime movement on `WindowPosition::At(IVec2)` because Bevy defines `At` in physical screen-space pixels

Intermediate fallback:

- if monitor topology cannot be queried reliably on a platform, a temporary single-monitor clamp may be used as an implementation checkpoint
- any such fallback must be documented as a fidelity gap and must not replace `req-md1` as the target behavior

## Pixel-Art Rendering Plan

Use nearest-neighbor rendering by default.

Implementation guidance:

- configure `ImagePlugin::default_nearest()`
- keep sprite scaling simple
- avoid scaling paths that introduce interpolation blur

## Blocking Technical Spike

## Global Cursor Access Spike

This is the first hard technical gate for implementation.

Problem:

- Bevy's high-level window cursor APIs only report cursor position inside the window
- the reference app requires cursor position even when the cursor is outside the pet window

Spike plan:

1. test whether `bevy::winit::WinitWindows` or adjacent Bevy backend integration provides the needed native access
2. if not sufficient, isolate a minimal platform-specific cursor provider inside `src/platform/cursor.rs`
3. keep all higher-level behavior code independent from the cursor backend

Spike acceptance criteria:

- returns cursor coordinates continuously while the pet window does not contain the cursor
- can be combined with runtime window movement
- does not require abandoning Bevy as the app framework

If this spike fails, implementation should stop and document the blocker before more behavior code is added.

## Testing Strategy

Pure logic should be separated enough to allow lightweight tests around:

- direction bucket mapping
- idle state progression
- frame selection
- monitor selection and cross-display movement clamping

Manual verification is still required for:

- transparent window behavior
- always-on-top behavior
- mouse passthrough
- global cursor tracking
- cross-display border crossing
- mixed monitor resolution or DPI behavior
- sound playback

## Sequenced Milestones

## Milestone 0: Bootstrap

- create the Rust crate
- add Bevy dependency and required audio feature for WAV
- create the `assets/` directory

## Milestone 1: Window And Rendering Baseline

- transparent window
- `Camera2d`
- nearest-neighbor image setup
- one visible cat sprite

## Milestone 2: Cursor And Display Topology Spike

- prove global cursor acquisition
- prove runtime native window movement
- prove monitor topology acquisition through Bevy monitor data, or document why a backend/platform fallback is required
- prove active-monitor selection and cross-display clamping path

This milestone is the first go or no-go checkpoint.

## Milestone 3: Core State Resources

- `NekoConfig`
- `NekoAssets`
- `NekoState`
- sprite key enums
- sound event enum

## Milestone 4: Chase Behavior

- 8-direction angle mapping
- cross-display window movement
- distance checks
- chase sprite switching

## Milestone 5: Idle Behavior

- idle state progression
- awake, scratch, wash, yawn, sleep sequencing
- frame timing thresholds
- wait mode logic

## Milestone 6: Audio

- one-shot sound event playback
- `quiet` handling

## Milestone 7: Verification

- logic tests
- manual platform validation, including multi-display verification
- `cargo test`
- `cargo build`
- `cargo check`

## Exit Criteria

Phase 3 is complete when:

- the implementation order is explicit
- the crate/module plan is explicit
- the blocking technical spike is clearly defined
- data model and system responsibilities are concrete
- the result can be translated directly into a task checklist

This document is the baseline for phase 4.
