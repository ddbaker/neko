use bevy::prelude::*;

use crate::assets::{SpriteBase, SpriteFrameKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn sprite_base(self) -> SpriteBase {
        match self {
            Self::Up => SpriteBase::Up,
            Self::UpRight => SpriteBase::UpRight,
            Self::Right => SpriteBase::Right,
            Self::DownRight => SpriteBase::DownRight,
            Self::Down => SpriteBase::Down,
            Self::DownLeft => SpriteBase::DownLeft,
            Self::Left => SpriteBase::Left,
            Self::UpLeft => SpriteBase::UpLeft,
        }
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NekoSoundEvent {
    Wake,
    Sleep,
    IdleYawn,
}

#[derive(Resource, Debug, Clone)]
pub struct NekoState {
    pub waiting: bool,
    pub window_pos: Vec2,
    pub distance: i32,
    pub count: i32,
    pub min: i32,
    pub max: i32,
    pub state: i32,
    pub current_direction: Option<Direction>,
    pub sprite_base: SpriteBase,
    pub last_frame: Option<SpriteFrameKey>,
}

impl NekoState {
    pub fn new(window_pos: Vec2) -> Self {
        Self {
            waiting: false,
            window_pos,
            distance: 0,
            count: 0,
            min: 8,
            max: 16,
            state: 0,
            current_direction: None,
            sprite_base: SpriteBase::Awake,
            last_frame: None,
        }
    }

    pub fn current_frame(&self) -> SpriteFrameKey {
        frame_for(self.sprite_base, self.count, self.min)
    }
}

pub fn frame_for(base: SpriteBase, count: i32, min: i32) -> SpriteFrameKey {
    match base {
        SpriteBase::Awake => SpriteFrameKey::Awake,
        SpriteBase::Up => pick_two_frame(count, min, SpriteFrameKey::Up1, SpriteFrameKey::Up2),
        SpriteBase::UpRight => pick_two_frame(
            count,
            min,
            SpriteFrameKey::UpRight1,
            SpriteFrameKey::UpRight2,
        ),
        SpriteBase::Right => {
            pick_two_frame(count, min, SpriteFrameKey::Right1, SpriteFrameKey::Right2)
        }
        SpriteBase::DownRight => pick_two_frame(
            count,
            min,
            SpriteFrameKey::DownRight1,
            SpriteFrameKey::DownRight2,
        ),
        SpriteBase::Down => {
            pick_two_frame(count, min, SpriteFrameKey::Down1, SpriteFrameKey::Down2)
        }
        SpriteBase::DownLeft => pick_two_frame(
            count,
            min,
            SpriteFrameKey::DownLeft1,
            SpriteFrameKey::DownLeft2,
        ),
        SpriteBase::Left => {
            pick_two_frame(count, min, SpriteFrameKey::Left1, SpriteFrameKey::Left2)
        }
        SpriteBase::UpLeft => {
            pick_two_frame(count, min, SpriteFrameKey::UpLeft1, SpriteFrameKey::UpLeft2)
        }
        SpriteBase::Scratch => pick_two_frame(
            count,
            min,
            SpriteFrameKey::Scratch1,
            SpriteFrameKey::Scratch2,
        ),
        SpriteBase::Wash => {
            pick_two_frame(count, min, SpriteFrameKey::Wash1, SpriteFrameKey::Wash2)
        }
        SpriteBase::Yawn => {
            pick_two_frame(count, min, SpriteFrameKey::Yawn1, SpriteFrameKey::Yawn2)
        }
        SpriteBase::Sleep => {
            pick_two_frame(count, min, SpriteFrameKey::Sleep1, SpriteFrameKey::Sleep2)
        }
    }
}

fn pick_two_frame(
    count: i32,
    min: i32,
    frame_one: SpriteFrameKey,
    frame_two: SpriteFrameKey,
) -> SpriteFrameKey {
    if count < min { frame_one } else { frame_two }
}
