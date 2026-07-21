use bevy::prelude::*;

pub const AUDIO_PATH: &str = "audio/sfx";

#[derive(Resource)]
pub struct AudioAssets {
    pub victory: Handle<AudioSource>,
    pub character_death: Handle<AudioSource>,
    pub enemy_death: Handle<AudioSource>,
    pub pick_up: Handle<AudioSource>,
    pub explosions: (
        Handle<AudioSource>,
        Handle<AudioSource>,
        Handle<AudioSource>,
    ),
    pub overtime: Handle<AudioSource>,
}

pub fn load_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        victory: asset_server.load(format!("{AUDIO_PATH}/Jingle_Achievement_00.wav")),
        character_death: asset_server.load(format!("{AUDIO_PATH}/Hero_Death_00.wav")),
        enemy_death: asset_server.load(format!("{AUDIO_PATH}/Shoot_00.wav")),
        pick_up: asset_server.load(format!("{AUDIO_PATH}/Pickup_04.wav")),
        explosions: (
            asset_server.load(format!("{AUDIO_PATH}/Explosion_00.wav")),
            // asset_server.load(format!("{AUDIO_PATH}/Explosion_01.wav")),
            asset_server.load(format!("{AUDIO_PATH}/Explosion_02.wav")),
            // asset_server.load(format!("{AUDIO_PATH}/Explosion_03.wav")),
            asset_server.load(format!("{AUDIO_PATH}/Explosion_04.wav")),
        ),
        overtime: asset_server.load(format!("{AUDIO_PATH}/Pickup_02.wav")),
    });
}
