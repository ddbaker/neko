use std::collections::HashMap;

use bevy::audio::AudioSource;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct NekoSprite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteBase {
    Awake,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Scratch,
    Wash,
    Yawn,
    Sleep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteFrameKey {
    Awake,
    Up1,
    Up2,
    UpRight1,
    UpRight2,
    Right1,
    Right2,
    DownRight1,
    DownRight2,
    Down1,
    Down2,
    DownLeft1,
    DownLeft2,
    Left1,
    Left2,
    UpLeft1,
    UpLeft2,
    Scratch1,
    Scratch2,
    Wash1,
    Wash2,
    Yawn1,
    Yawn2,
    Sleep1,
    Sleep2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundKey {
    Awake,
    IdleYawn,
    Sleep,
}

#[derive(Resource, Debug, Clone)]
pub struct NekoAssets {
    frames: HashMap<SpriteFrameKey, Handle<Image>>,
    sounds: HashMap<SoundKey, Handle<AudioSource>>,
}

impl NekoAssets {
    pub fn frame_handle(&self, key: SpriteFrameKey) -> Handle<Image> {
        self.frames
            .get(&key)
            .unwrap_or_else(|| panic!("Missing sprite frame asset: {key:?}"))
            .clone()
    }

    pub fn sound_handle(&self, key: SoundKey) -> Handle<AudioSource> {
        self.sounds
            .get(&key)
            .unwrap_or_else(|| panic!("Missing sound asset: {key:?}"))
            .clone()
    }
}

pub fn load_assets(asset_server: &AssetServer) -> NekoAssets {
    let mut frames = HashMap::new();
    let mut sounds = HashMap::new();

    for (key, path) in sprite_paths() {
        frames.insert(key, asset_server.load(path));
    }

    sounds.insert(SoundKey::Awake, asset_server.load("awake.wav"));
    sounds.insert(SoundKey::IdleYawn, asset_server.load("idle3.wav"));
    sounds.insert(SoundKey::Sleep, asset_server.load("sleep.wav"));

    bevy::log::info!(
        "Loaded {} sprite handles and {} sound handles.",
        frames.len(),
        sounds.len()
    );

    NekoAssets { frames, sounds }
}

fn sprite_paths() -> [(SpriteFrameKey, &'static str); 25] {
    [
        (SpriteFrameKey::Awake, "awake.png"),
        (SpriteFrameKey::Up1, "up1.png"),
        (SpriteFrameKey::Up2, "up2.png"),
        (SpriteFrameKey::UpRight1, "upright1.png"),
        (SpriteFrameKey::UpRight2, "upright2.png"),
        (SpriteFrameKey::Right1, "right1.png"),
        (SpriteFrameKey::Right2, "right2.png"),
        (SpriteFrameKey::DownRight1, "downright1.png"),
        (SpriteFrameKey::DownRight2, "downright2.png"),
        (SpriteFrameKey::Down1, "down1.png"),
        (SpriteFrameKey::Down2, "down2.png"),
        (SpriteFrameKey::DownLeft1, "downleft1.png"),
        (SpriteFrameKey::DownLeft2, "downleft2.png"),
        (SpriteFrameKey::Left1, "left1.png"),
        (SpriteFrameKey::Left2, "left2.png"),
        (SpriteFrameKey::UpLeft1, "upleft1.png"),
        (SpriteFrameKey::UpLeft2, "upleft2.png"),
        (SpriteFrameKey::Scratch1, "scratch1.png"),
        (SpriteFrameKey::Scratch2, "scratch2.png"),
        (SpriteFrameKey::Wash1, "wash1.png"),
        (SpriteFrameKey::Wash2, "wash2.png"),
        (SpriteFrameKey::Yawn1, "yawn1.png"),
        (SpriteFrameKey::Yawn2, "yawn2.png"),
        (SpriteFrameKey::Sleep1, "sleep1.png"),
        (SpriteFrameKey::Sleep2, "sleep2.png"),
    ]
}
