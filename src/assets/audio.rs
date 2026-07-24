use bevy::prelude::*;

pub const SFX_PATH: &str = "audio/sfx";
pub const MUSIC_PATH: &str = "audio/music";

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
    pub menu_music: Handle<AudioSource>,
    pub game_music: Handle<AudioSource>,
}

impl FromWorld for AudioAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        AudioAssets {
            victory: asset_server.load(format!("{SFX_PATH}/Jingle_Achievement_00.wav")),
            character_death: asset_server.load(format!("{SFX_PATH}/Hero_Death_00.wav")),
            enemy_death: asset_server.load(format!("{SFX_PATH}/Shoot_00.wav")),
            pick_up: asset_server.load(format!("{SFX_PATH}/Pickup_04.wav")),
            explosions: (
                asset_server.load(format!("{SFX_PATH}/Explosion_00.wav")),
                // asset_server.load(format!("{AUDIO_PATH}/Explosion_01.wav")),
                asset_server.load(format!("{SFX_PATH}/Explosion_02.wav")),
                // asset_server.load(format!("{AUDIO_PATH}/Explosion_03.wav")),
                asset_server.load(format!("{SFX_PATH}/Explosion_04.wav")),
            ),
            overtime: asset_server.load(format!("{SFX_PATH}/Pickup_02.wav")),
            menu_music: asset_server.load(format!("{MUSIC_PATH}/01 - Opening.ogg")),
            game_music: asset_server.load(format!("{MUSIC_PATH}/13 - Danger.ogg")),
        }
    }
}
