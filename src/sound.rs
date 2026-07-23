use std::time::Duration;

use bevy::{audio::Volume, prelude::*};

use crate::{assets::audio::AudioAssets, world_entities::GameplaySet};

#[derive(Event, Clone, Copy)]
pub enum EffectKind {
    Victory,
    CharacterDeath,
    EnemyDeath,
    PickUp,
    Explosion,
    Overtime,
}

#[derive(Component, Clone)]
struct DelayedAudio {
    pub delay: Timer,
    pub player: AudioPlayer,
    pub settings: PlaybackSettings,
}

fn default_playback_settings() -> PlaybackSettings {
    PlaybackSettings::DESPAWN.with_volume(Volume::Decibels(-18.0))
}

fn on_sound_trigger(
    trigger: On<EffectKind>,
    audio_assets: Res<AudioAssets>,
    mut commands: Commands,
) {
    let single_effect = match *trigger.event() {
        EffectKind::Explosion => {
            let handles = audio_assets.explosions.clone();

            let bomb_explosion = [
                (handles.2.clone(), 150),
                (handles.2.clone(), 250),
                (handles.2.clone(), 400),
                (handles.2.clone(), 550),
                (handles.2.clone(), 700),
                (handles.0.clone(), 000),
                (handles.0.clone(), 700),
                (handles.1.clone(), 400),
            ];

            for (handle, delay) in bomb_explosion {
                commands.spawn(DelayedAudio {
                    delay: Timer::new(Duration::from_millis(delay), TimerMode::Once),
                    player: AudioPlayer::new(handle.clone()),
                    settings: default_playback_settings().with_volume(Volume::Decibels(-36.0)),
                });
            }
            return;
        }
        EffectKind::EnemyDeath => {
            commands.spawn(DelayedAudio {
                delay: Timer::new(Duration::from_millis(1500), TimerMode::Once),
                player: AudioPlayer::new(audio_assets.enemy_death.clone()),
                settings: default_playback_settings(),
            });
            return;
        }
        EffectKind::Victory => audio_assets.victory.clone(),
        EffectKind::CharacterDeath => {
            commands.spawn(DelayedAudio {
                delay: Timer::new(Duration::from_millis(1500), TimerMode::Once),
                player: AudioPlayer::new(audio_assets.character_death.clone()),
                settings: default_playback_settings(),
            });
            return;
        }
        EffectKind::PickUp => audio_assets.pick_up.clone(),
        EffectKind::Overtime => audio_assets.overtime.clone(),
    };

    commands.spawn((AudioPlayer::new(single_effect), default_playback_settings()));
}

fn play_delayed_effects(
    mut commands: Commands,
    mut delayed_audios: Query<(Entity, &mut DelayedAudio)>,
    time: Res<Time>,
) {
    let delta = time.delta();
    for (entity, mut delayed_audio) in delayed_audios.iter_mut() {
        delayed_audio.delay.tick(delta);
        if delayed_audio.delay.is_finished() {
            commands.spawn((delayed_audio.player.clone(), delayed_audio.settings));
            commands.entity(entity).despawn();
        }
    }
}

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioAssets>()
            .add_observer(on_sound_trigger)
            .add_systems(
                Update,
                play_delayed_effects.in_set(GameplaySet::AnimationAndSound),
            );
    }
}
