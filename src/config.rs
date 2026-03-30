use bevy::prelude::Resource;

pub const LOGICAL_WINDOW_SIZE: i32 = 32;
pub const FIXED_TIMESTEP_HZ: f64 = 50.0;
pub const WINDOW_TITLE: &str = "Neko";
pub const SOUND_VOLUME: f32 = 0.3;

#[derive(Resource, Debug, Clone)]
pub struct NekoConfig {
    pub speed: f32,
    pub scale: f32,
    pub quiet: bool,
    pub mouse_passthrough: bool,
}

impl Default for NekoConfig {
    fn default() -> Self {
        Self {
            speed: 2.0,
            scale: 2.0,
            quiet: false,
            mouse_passthrough: false,
        }
    }
}
