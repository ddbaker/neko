# Phase 2 Bevy Porting Plan

## Purpose

This document defines how the Go-language `neko` reference application at `D:\devel\inetsrc\neko` will be ported into a Rust application using the Bevy game engine.

This is a planning deliverable, not an implementation checklist. The goal is to make the architecture, behavior mapping, risks, and implementation direction explicit before code is written.

## Scope

The Rust version must:

- use Rust only
- use Bevy only as the application/game framework
- preserve the core desktop-pet behavior of the Go reference
- remain small and direct unless a more modular structure is needed for correctness or maintainability

The initial port target is feature parity with the current Go reference behavior, not historical parity with older Neko variants.

## Reference Behavior To Preserve

The Go reference currently does the following:

- creates a small transparent native window that visually contains the cat sprite
- keeps the window undecorated and above normal windows
- optionally enables mouse passthrough
- scales the 32x32 cat sprite window by a configurable factor
- moves the native window toward the mouse cursor
- clamps movement to the monitor bounds
- idles when the cursor is sufficiently close or when wait mode is active
- toggles wait mode with left click while idling
- animates the cat in 8 chase directions
- runs an idle sequence of `awake -> scratch -> wash -> yawn -> sleep`
- plays short sounds on specific state transitions

## Confirmed Reference Assets

### Sprites used by current Go logic

- `awake`
- `up`
- `upright`
- `right`
- `downright`
- `down`
- `downleft`
- `left`
- `upleft`
- `scratch`
- `wash`
- `yawn`
- `sleep`

### Sounds used by current Go logic

- `awake.wav`
- `idle3.wav`
- `sleep.wav`

### Assets present but not currently used in Go logic

- `idle1.wav`
- `idle2.wav`
- `fp_*`
- directional `*claw*` sprites

The port should initially preserve only what the reference app actually uses. Unused assets can be documented for later expansion.

## Reference Runtime Parameters

The Go reference exposes these runtime parameters:

- `speed`
- `scale`
- `quiet`
- `mousepassthrough`

The Rust port should preserve these parameters with equivalent defaults and semantics as closely as possible.

## Bevy Runtime Model

## Windowing

The app should use a single primary Bevy window configured as follows:

- transparent background enabled
- no window decorations
- not resizable
- title set to `Neko`
- always-on-top window level
- fully transparent clear color
- scaled from the logical cat size of 32x32

The native window itself represents the cat's world position. The sprite should not move freely inside a larger scene; instead, the window should be repositioned to follow the cursor, matching the Go reference model.

## Rendering

Rendering should stay intentionally minimal:

- one `Camera2d`
- one sprite entity for the cat
- transparent background
- image assets loaded through Bevy's asset system

The cat animation should be expressed as sprite swaps, not as a texture atlas unless that later proves materially cleaner. The current asset layout already matches file-per-frame animation.

## Timing

The reference app uses a fixed tick rate of 50 TPS. The Rust port should preserve that with Bevy fixed timestep scheduling.

Planned fixed-step configuration:

- insert `Time::<Fixed>::from_hz(50.0)`
- run movement and state-machine systems in `FixedUpdate`
- keep rendering and any input edge handling in Bevy's normal frame schedule if needed

This keeps the port behavior close to the Go implementation and avoids tying animation/state changes to variable render frame rate.

## Application Structure

The repository is currently spec-only. When implementation starts, create a minimal but explicit source layout:

- `src/main.rs`
- `src/config.rs`
- `src/assets.rs`
- `src/state.rs`
- `src/behavior.rs`
- `src/audio.rs`
- `src/platform/mod.rs`
- `src/platform/cursor.rs`

This is not a requirement for deep abstraction. It is a way to keep platform concerns, behavior logic, and Bevy bootstrapping separated from the start.

## Planned Responsibilities By Module

### `src/main.rs`

- build the Bevy app
- configure plugins
- configure the primary window
- register resources, events, and systems

### `src/config.rs`

- define `NekoConfig`
- parse or initialize runtime configuration
- hold defaults for `speed`, `scale`, `quiet`, and `mouse_passthrough`

The first implementation can hardcode defaults if needed, but the structure should leave room for CLI or file-based configuration.

### `src/assets.rs`

- define strongly named asset handles
- load image and sound assets through `AssetServer`
- provide a resource that maps animation keys to handles

### `src/state.rs`

- define a `NekoState` resource
- keep the port close to the Go field model in the first iteration

Suggested initial fields:

- `waiting: bool`
- `window_pos: Vec2`
- `distance: i32`
- `count: i32`
- `min: i32`
- `max: i32`
- `state: i32`
- `sprite_base: SpriteKey`
- `last_sprite: Option<SpriteKey>`

This is intentionally conservative. The first port should favor fidelity over premature redesign.

### `src/behavior.rs`

- implement the fixed-step state machine
- compute cursor direction
- clamp the window position to monitor bounds
- update the selected animation
- detect wake/sleep/yawn sound transitions

### `src/audio.rs`

- define small sound events such as `Wake`, `Sleep`, and `IdleYawn`
- play one-shot audio assets
- suppress audio when `quiet` is enabled

### `src/platform/cursor.rs`

- own the cursor-position acquisition strategy
- isolate platform/backend-specific code needed for a faithful port

This module exists because cursor access is the main risk area for the Bevy port.

## Data Model Plan

## Config Resource

Define a `NekoConfig` resource with:

- `speed: f32`
- `scale: f32`
- `quiet: bool`
- `mouse_passthrough: bool`

## Assets Resource

Define a `NekoAssets` resource that contains:

- image handles for every used sprite frame
- audio handles for used one-shot sounds

The keys should be explicit enums rather than raw strings after loading.

## State Resource

Define a `NekoState` resource that keeps the runtime state close to the Go implementation.

Porting rule:

- keep the original state semantics first
- refactor only after behavior parity is demonstrated

This avoids introducing behavioral drift during the first implementation pass.

## Entity Plan

The initial app should only need:

- one camera entity
- one cat sprite entity

If diagnostics or debug UI are needed later, they should be optional and removable.

## Animation Plan

The Go reference uses:

- a single image for `awake`
- a two-frame swap for movement and most idle animations
- `count`, `min`, and `max` thresholds to choose frame 1 or frame 2 and to advance idle states

The Bevy port should preserve this logic directly:

1. increment `count` each fixed tick
2. choose sprite frame from `sprite_base` and `count`
3. reset `count` when it exceeds `max`
4. advance the idle state when required

Do not replace this with generic animation blending or a more abstract animation graph in the first implementation.

## Idle State Machine Plan

The idle behavior should be ported directly from the Go reference:

- state `0` transitions into active idle state handling
- states `1..=3` display `awake`
- states `4..=6` display `scratch`
- states `7..=9` display `wash`
- states `10..=12` display `yawn` with longer timing
- states `13+` display `sleep`

Additional rules to preserve:

- while chasing, reset `state = 0`, `min = 8`, `max = 16`
- when entering the yawn phase and `count == min`, play the idle yawn sound
- when progressing into sleep, play the sleep sound
- when waking from a sleep state and re-entering chase, play the awake sound

## Chase Movement Plan

The chase logic should remain direction-bucket based rather than physics-based.

Porting steps:

1. compute cursor vector relative to the cat window center
2. compute angle with `atan2`
3. normalize to degrees in `[0, 360)`
4. map that angle into one of 8 direction sectors
5. move the window using `speed` or `speed / sqrt(2)` for diagonals
6. set the matching sprite base for the selected direction

This preserves the reference motion style and avoids accidental smoothing or inertia not present in the Go version.

## Native Window Position Plan

The cat's position is the native window position, not an in-world sprite transform.

Planned behavior:

- store current window position in `NekoState`
- each fixed tick, compute the next desired position
- clamp to the current monitor's usable bounds
- update the Bevy window position

Window size should derive from:

- logical sprite size `32x32`
- multiplied by config `scale`

## Input Plan

The reference behavior needs two kinds of input:

- cursor position for chase behavior
- left-click edge detection to toggle waiting while idle

For the wait toggle:

- use Bevy mouse button input resources/events
- only toggle waiting when the app is in an idle path equivalent to the Go behavior

For mouse passthrough:

- configure initial behavior from `NekoConfig`
- use Bevy `CursorOptions.hit_test` semantics

## Audio Plan

Audio should remain simple and event-driven.

Rules:

- only play one-shot sounds used by the current reference behavior
- do not introduce background loops
- respect `quiet`

Initial sound events should be:

- `Wake`
- `Sleep`
- `IdleYawn`

## Major Technical Risk

## Global Cursor Access

This is the most important planning issue.

The Go reference uses global cursor information relative to the moving pet window. In official Bevy window APIs, `Window::cursor_position()` and `Window::physical_cursor_position()` return `None` when the cursor is outside the window.

That means a faithful port cannot assume Bevy's high-level cursor API alone is sufficient.

### Required implementation spike

The first implementation milestone must validate how the Rust app will obtain cursor data suitable for chase behavior.

Investigation order:

1. use Bevy-supported backend access through `bevy::winit` integration, if it can provide the needed native window or monitor data cleanly
2. if needed, isolate a minimal platform-specific cursor query behind `src/platform/cursor.rs`
3. keep the rest of the app independent from whichever backend method is chosen

Acceptance condition for this spike:

- the app can consistently obtain cursor position even when the cursor is not inside the pet window

If this is not solved, the port cannot match the Go behavior correctly.

## Secondary Risks

### Transparent Window Differences By Platform

Transparent windows and composition behavior can vary by platform. The Bevy implementation should follow the current official transparent-window guidance and keep platform-specific adjustments localized.

### Monitor Bounds And Multi-Monitor Behavior

The Go reference clamps the window to the current monitor dimensions and comments that this avoids accidental travel to other monitors. The Rust port should replicate this conservative behavior first before attempting multi-monitor roaming.

### Asset Filtering And Pixel Fidelity

The cat is pixel art. Texture filtering and scale behavior must preserve crisp edges.

## Planned Implementation Strategy

Implementation should proceed in this order:

1. bootstrap the Bevy app and transparent always-on-top window
2. load and display a single cat sprite on a transparent background
3. implement fixed-step timing at 50 Hz
4. implement native window movement with a temporary test target
5. complete the cursor-access spike
6. port the 8-direction chase behavior
7. port idle sequencing and animation frame selection
8. port wait toggling
9. port one-shot audio transitions
10. polish config loading and asset organization

This order keeps the main unknowns early and avoids spending time on secondary features before the cursor and window behavior are proven.

## Verification Plan

The implementation phase should verify the following behaviors manually and, where practical, with tests around pure logic:

- the window starts transparent and undecorated
- the cat sprite is visible and scaled correctly
- the window remains above normal windows
- the cat follows the cursor in the expected 8 directions
- diagonal movement speed matches reference behavior
- the cat stops chasing when close to the cursor
- idle animation phases advance in the same order as the Go reference
- wait mode toggles correctly
- `quiet` disables sound
- `mouse_passthrough` changes hit-testing behavior
- the window does not drift outside the intended monitor bounds

## Out Of Scope For Initial Port

These items should not be added during the first parity pass unless required for correctness:

- new gameplay or interaction features
- alternate sprite sets
- footprint effects from unused `fp_*` assets
- claw attack variants from unused directional claw sprites
- architectural generalization beyond what the port needs

## Phase 2 Exit Criteria

Phase 2 is complete when:

- the Go reference behavior is mapped to concrete Bevy systems/resources/components
- the main technical risks are identified
- the implementation order is defined
- the result is specific enough to be converted into a detailed markdown porting plan and then an implementation checklist

This document satisfies that requirement and should be used as the baseline for phase 3 and phase 4.
