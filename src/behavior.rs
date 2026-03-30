use std::f32::consts::PI;

use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowPosition};

use crate::assets::{NekoAssets, NekoSprite, SpriteBase};
use crate::config::{LOGICAL_WINDOW_SIZE, NekoConfig};
use crate::platform::cursor::monitor_bounds_for_point;
use crate::state::{Direction, NekoSoundEvent, NekoState};

pub fn scaled_window_size(scale: f32) -> u32 {
    let scale = scale.max(0.1);
    (LOGICAL_WINDOW_SIZE as f32 * scale).round().max(1.0) as u32
}

pub fn fixed_update_neko_behavior(
    config: Res<NekoConfig>,
    mut state: ResMut<NekoState>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut sound_events: MessageWriter<NekoSoundEvent>,
) {
    let Some(cursor_position) = crate::platform::cursor::global_cursor_position() else {
        return;
    };

    let previous_base = state.sprite_base;
    let previous_state = state.state;
    let scale = config.scale.max(0.1);
    let window_size = scaled_window_size(config.scale) as f32;
    let center = state.window_pos + Vec2::splat(window_size / 2.0);
    let offset_pixels = cursor_position - center;
    let offset_logical = offset_pixels / scale;

    state.count += 1;

    if state.state == 10 && state.count == state.min {
        sound_events.write(NekoSoundEvent::IdleYawn);
        bevy::log::info!("Idle yawn sound triggered.");
    }

    state.distance = manhattan_distance(offset_logical);

    if state.distance < LOGICAL_WINDOW_SIZE || state.waiting {
        stay_idle(&mut state);

        if mouse_buttons.just_pressed(MouseButton::Left) {
            state.waiting = !state.waiting;
            bevy::log::info!("Waiting toggled: {}", state.waiting);
        }
    } else {
        if state.state >= 13 {
            sound_events.write(NekoSoundEvent::Wake);
            bevy::log::info!("Waking from sleep.");
        }

        catch_cursor(&mut state, offset_logical, config.speed);
    }

    if state.count > state.max {
        state.count = 0;

        if state.state > 0 {
            state.state += 1;

            if state.state == 13 {
                sound_events.write(NekoSoundEvent::Sleep);
                bevy::log::info!("Entering sleep.");
            }
        }
    }

    let window_side = scaled_window_size(config.scale) as i32;
    if let Some(bounds) = monitor_bounds_for_point(center) {
        state.window_pos = bounds.clamp_window_position(state.window_pos, window_side, window_side);
    }

    if state.sprite_base != previous_base || state.state != previous_state {
        bevy::log::debug!(
            "State update: sprite={:?}, state={}, count={}, distance={}",
            state.sprite_base,
            state.state,
            state.count,
            state.distance
        );
    }
}

pub fn apply_window_position(
    state: Res<NekoState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Some(mut window) = windows.iter_mut().next() else {
        return;
    };

    let position = IVec2::new(
        state.window_pos.x.round() as i32,
        state.window_pos.y.round() as i32,
    );
    window.position = WindowPosition::new(position);
}

pub fn sync_sprite_frame(
    assets: Res<NekoAssets>,
    mut state: ResMut<NekoState>,
    mut sprites: Query<&mut Sprite, With<NekoSprite>>,
) {
    let Some(mut sprite) = sprites.iter_mut().next() else {
        return;
    };

    let frame = state.current_frame();
    if state.last_frame == Some(frame) {
        return;
    }

    sprite.image = assets.frame_handle(frame);
    state.last_frame = Some(frame);
}

pub fn manhattan_distance(offset: Vec2) -> i32 {
    offset.x.abs().round() as i32 + offset.y.abs().round() as i32
}

pub fn stay_idle(state: &mut NekoState) {
    match state.state {
        0 | 1 | 2 | 3 => {
            if state.state == 0 {
                state.state = 1;
            }
            state.sprite_base = SpriteBase::Awake;
        }
        4 | 5 | 6 => {
            state.sprite_base = SpriteBase::Scratch;
        }
        7 | 8 | 9 => {
            state.sprite_base = SpriteBase::Wash;
        }
        10 | 11 | 12 => {
            state.min = 32;
            state.max = 64;
            state.sprite_base = SpriteBase::Yawn;
        }
        _ => {
            state.sprite_base = SpriteBase::Sleep;
        }
    }
}

pub fn catch_cursor(state: &mut NekoState, offset: Vec2, speed: f32) {
    let direction = direction_for_offset(offset);
    let diagonal_speed = speed / 2.0_f32.sqrt();

    state.state = 0;
    state.min = 8;
    state.max = 16;
    state.sprite_base = direction.sprite_base();

    match direction {
        Direction::Up => state.window_pos.y -= speed,
        Direction::UpRight => {
            state.window_pos.x += diagonal_speed;
            state.window_pos.y -= diagonal_speed;
        }
        Direction::Right => state.window_pos.x += speed,
        Direction::DownRight => {
            state.window_pos.x += diagonal_speed;
            state.window_pos.y += diagonal_speed;
        }
        Direction::Down => state.window_pos.y += speed,
        Direction::DownLeft => {
            state.window_pos.x -= diagonal_speed;
            state.window_pos.y += diagonal_speed;
        }
        Direction::Left => state.window_pos.x -= speed,
        Direction::UpLeft => {
            state.window_pos.x -= diagonal_speed;
            state.window_pos.y -= diagonal_speed;
        }
    }
}

pub fn direction_for_offset(offset: Vec2) -> Direction {
    let angle = ((offset.y.atan2(offset.x) / PI * 180.0) + 360.0) % 360.0;

    match angle {
        a if a <= 292.5 && a > 247.5 => Direction::Up,
        a if a <= 337.5 && a > 292.5 => Direction::UpRight,
        a if a <= 22.5 || a > 337.5 => Direction::Right,
        a if a <= 67.5 && a > 22.5 => Direction::DownRight,
        a if a <= 112.5 && a > 67.5 => Direction::Down,
        a if a <= 157.5 && a > 112.5 => Direction::DownLeft,
        a if a <= 202.5 && a > 157.5 => Direction::Left,
        _ => Direction::UpLeft,
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec2;

    use crate::assets::{SpriteBase, SpriteFrameKey};
    use crate::platform::cursor::MonitorBounds;
    use crate::state::{Direction, NekoState, frame_for};

    use super::{
        catch_cursor, direction_for_offset, manhattan_distance, scaled_window_size, stay_idle,
    };

    #[test]
    fn direction_mapping_matches_reference_sectors() {
        assert_eq!(direction_for_offset(Vec2::new(0.0, -20.0)), Direction::Up);
        assert_eq!(
            direction_for_offset(Vec2::new(10.0, -20.0)),
            Direction::UpRight
        );
        assert_eq!(
            direction_for_offset(Vec2::new(20.0, -5.0)),
            Direction::Right
        );
        assert_eq!(
            direction_for_offset(Vec2::new(-10.0, 15.0)),
            Direction::DownLeft
        );
        assert_eq!(
            direction_for_offset(Vec2::new(-10.0, -10.0)),
            Direction::UpLeft
        );
    }

    #[test]
    fn frame_selection_uses_awake_single_frame() {
        assert_eq!(frame_for(SpriteBase::Awake, 0, 8), SpriteFrameKey::Awake);
        assert_eq!(frame_for(SpriteBase::Awake, 99, 8), SpriteFrameKey::Awake);
    }

    #[test]
    fn frame_selection_switches_after_minimum_threshold() {
        assert_eq!(frame_for(SpriteBase::Right, 7, 8), SpriteFrameKey::Right1);
        assert_eq!(frame_for(SpriteBase::Right, 8, 8), SpriteFrameKey::Right2);
    }

    #[test]
    fn idle_state_sets_expected_animation_family() {
        let mut state = NekoState::new(Vec2::ZERO);
        stay_idle(&mut state);
        assert_eq!(state.state, 1);
        assert_eq!(state.sprite_base, SpriteBase::Awake);

        state.state = 5;
        stay_idle(&mut state);
        assert_eq!(state.sprite_base, SpriteBase::Scratch);

        state.state = 10;
        stay_idle(&mut state);
        assert_eq!(state.sprite_base, SpriteBase::Yawn);
        assert_eq!(state.min, 32);
        assert_eq!(state.max, 64);

        state.state = 15;
        stay_idle(&mut state);
        assert_eq!(state.sprite_base, SpriteBase::Sleep);
    }

    #[test]
    fn catch_cursor_resets_idle_timing_and_moves_diagonally() {
        let mut state = NekoState::new(Vec2::new(50.0, 50.0));
        state.state = 14;
        state.min = 32;
        state.max = 64;

        catch_cursor(&mut state, Vec2::new(20.0, -20.0), 2.0);

        assert_eq!(state.state, 0);
        assert_eq!(state.min, 8);
        assert_eq!(state.max, 16);
        assert_eq!(state.sprite_base, SpriteBase::UpRight);
        assert!(state.window_pos.x > 50.0);
        assert!(state.window_pos.y < 50.0);
    }

    #[test]
    fn manhattan_distance_matches_reference_logic() {
        assert_eq!(manhattan_distance(Vec2::new(10.0, -4.0)), 14);
        assert_eq!(manhattan_distance(Vec2::new(-0.4, 0.4)), 0);
    }

    #[test]
    fn scaled_window_size_rounds_to_pixels() {
        assert_eq!(scaled_window_size(2.0), 64);
        assert_eq!(scaled_window_size(1.5), 48);
    }

    #[test]
    fn monitor_bounds_clamp_window_position() {
        let bounds = MonitorBounds {
            left: 0,
            top: 0,
            right: 100,
            bottom: 80,
        };

        let clamped = bounds.clamp_window_position(Vec2::new(95.0, 90.0), 32, 32);
        assert_eq!(clamped, Vec2::new(68.0, 48.0));
    }
}
