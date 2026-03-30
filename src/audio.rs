use bevy::ecs::message::MessageReader;
use bevy::prelude::*;

use crate::assets::{NekoAssets, SoundKey};
use crate::config::{NekoConfig, SOUND_VOLUME};
use crate::state::NekoSoundEvent;

pub fn play_sound_messages(
    mut commands: Commands,
    config: Res<NekoConfig>,
    assets: Res<NekoAssets>,
    mut sound_events: MessageReader<NekoSoundEvent>,
) {
    if config.quiet {
        sound_events.clear();
        return;
    }

    for event in sound_events.read() {
        let sound = match event {
            NekoSoundEvent::Wake => SoundKey::Awake,
            NekoSoundEvent::Sleep => SoundKey::Sleep,
            NekoSoundEvent::IdleYawn => SoundKey::IdleYawn,
        };

        commands.spawn((
            AudioPlayer::new(assets.sound_handle(sound)),
            PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::Linear(SOUND_VOLUME)),
        ));
    }
}
