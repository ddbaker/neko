pub mod assets;
pub mod audio;
pub mod behavior;
pub mod config;
pub mod platform;
pub mod state;

use bevy::prelude::*;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use bevy::window::CompositeAlphaMode;
use bevy::window::{
    CursorOptions, Monitor, PrimaryMonitor, PrimaryWindow, WindowLevel, WindowPlugin,
    WindowPosition, WindowResolution,
};

use assets::{NekoSprite, load_assets};
use audio::play_sound_messages;
use behavior::{
    apply_window_position, fixed_update_neko_behavior, scaled_window_size, sync_sprite_frame,
};
use config::{FIXED_TIMESTEP_HZ, NekoConfig, WINDOW_TITLE};
use platform::cursor::{
    desktop_monitor_layout_from_bevy, global_cursor_position, monitor_bounds_for_point,
    platform_cursor_capabilities,
};
use state::{NekoSoundEvent, NekoState};

pub fn run() {
    let config = NekoConfig::default();
    let window_size = scaled_window_size(config.scale);

    App::new()
        .insert_resource(config)
        .insert_resource(ClearColor(Color::NONE))
        .insert_resource(Time::<Fixed>::from_hz(FIXED_TIMESTEP_HZ))
        .add_message::<NekoSoundEvent>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_TITLE.into(),
                        resolution: WindowResolution::new(window_size, window_size)
                            .with_scale_factor_override(1.0),
                        transparent: true,
                        decorations: false,
                        resizable: false,
                        window_level: WindowLevel::AlwaysOnTop,
                        #[cfg(target_os = "macos")]
                        composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                        #[cfg(target_os = "linux")]
                        composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
                        position: WindowPosition::new(IVec2::ZERO),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup_neko)
        .add_systems(Update, exit_on_shortcuts)
        .add_systems(
            FixedUpdate,
            (
                fixed_update_neko_behavior,
                apply_window_position,
                sync_sprite_frame,
                play_sound_messages,
            )
                .chain(),
        )
        .run();
}

fn setup_neko(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<NekoConfig>,
    mut windows: Query<(&mut Window, &mut CursorOptions), With<PrimaryWindow>>,
    monitors: Query<(Entity, &Monitor)>,
    primary_monitors: Query<Entity, With<PrimaryMonitor>>,
) {
    let assets = load_assets(&asset_server);
    let Some((mut window, mut cursor_options)) = windows.iter_mut().next() else {
        bevy::log::warn!("Primary window was not available during setup.");
        return;
    };

    let cursor_capabilities = platform_cursor_capabilities();
    let window_size = scaled_window_size(config.scale);
    let initial_cursor = global_cursor_position();
    let monitor_layout =
        desktop_monitor_layout_from_bevy(monitors.iter(), primary_monitors.iter().next());
    let initial_pos = monitor_layout
        .initial_window_position(initial_cursor, window_size as i32, window_size as i32)
        .or_else(|| {
            initial_cursor.and_then(|cursor| {
                monitor_bounds_for_point(cursor).map(|bounds| {
                    bounds.centered_window_position(window_size as i32, window_size as i32)
                })
            })
        })
        .unwrap_or(IVec2::new(400, 200));

    if monitor_layout.is_empty() {
        bevy::log::warn!(
            "Bevy monitor topology was unavailable during startup; falling back to conservative placement."
        );
    }

    if !cursor_capabilities.supports_native_monitor_bounds {
        bevy::log::warn!(
            "Cursor backend '{}' does not provide native monitor fallback bounds; relying on Bevy monitor ECS as the primary topology source.",
            cursor_capabilities.backend_name
        );
    }

    #[cfg(target_os = "linux")]
    bevy::log::warn!(
        "Linux window level support is compositor-dependent; Wayland may ignore always-on-top requests."
    );

    window.position = WindowPosition::At(initial_pos);
    cursor_options.hit_test = !config.mouse_passthrough;

    let mut state = NekoState::new(initial_pos.as_vec2());
    let initial_frame = state.current_frame();
    state.last_frame = Some(initial_frame);

    commands.spawn(Camera2d);
    commands.spawn((
        NekoSprite,
        Sprite::from_image(assets.frame_handle(initial_frame)),
        Transform::from_scale(Vec3::splat(config.scale.max(0.1))),
    ));

    commands.insert_resource(state);
    commands.insert_resource(assets);

    bevy::log::info!(
        "Neko started: scale={:.1}, speed={:.1}, quiet={}, mouse_passthrough={}, cursor_backend={}",
        config.scale,
        config.speed,
        config.quiet,
        config.mouse_passthrough,
        cursor_capabilities.backend_name
    );
}

fn exit_on_shortcuts(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    let Some(window) = windows.iter().next() else {
        return;
    };

    if should_exit_from_shortcuts(keyboard_input.just_pressed(KeyCode::Escape), false) {
        bevy::log::info!("Escape pressed, exiting neko.");
        app_exit.write(AppExit::Success);
        return;
    }

    let right_click_on_neko =
        mouse_buttons.just_pressed(MouseButton::Right) && window.cursor_position().is_some();

    if should_exit_from_shortcuts(false, right_click_on_neko) {
        bevy::log::info!("Right-click on neko detected, exiting neko.");
        app_exit.write(AppExit::Success);
    }
}

fn should_exit_from_shortcuts(escape_pressed: bool, right_click_on_neko: bool) -> bool {
    escape_pressed || right_click_on_neko
}

#[cfg(test)]
mod tests {
    use super::should_exit_from_shortcuts;

    #[test]
    fn escape_requests_exit() {
        assert!(should_exit_from_shortcuts(true, false));
    }

    #[test]
    fn right_click_on_neko_requests_exit() {
        assert!(should_exit_from_shortcuts(false, true));
    }

    #[test]
    fn no_shortcut_does_not_exit() {
        assert!(!should_exit_from_shortcuts(false, false));
    }
}
