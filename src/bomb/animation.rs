use std::{
    f32::consts::{FRAC_PI_2, PI},
    time::Duration,
};

use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    assets::{
        bomb_explosion_tileset::{self, BombExplosionTileType},
        map_tileset::{self, MapTileType},
        material::{ColouringMaterial, ExplosionMaterial},
    },
    bomb::{BOMB_TICKS, BombAssets, EXPLOSION_TICKS},
    constants::BOMB_EXPLOSION_DURATION,
    map::{BaseTile, WorldMap},
    position::TilePosition,
    rendering::MeshHandle,
    util::EntityScale,
    world_entities::{
        Bomb, DestructibleWall, Explosion, ExplosionNeedsSetup, ExplosionOrientation,
        ExplosionVariant,
    },
};

fn timer_tick(fraction: f32, max_ticks: u32) -> u32 {
    ((max_ticks as f32 * fraction).floor() as u32).min(max_ticks - 1)
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct ExplosionTiming {
    timer: Timer,
    max_ticks: u32,
}

impl ExplosionTiming {
    pub fn new(duration: Duration, max_ticks: u32) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
            max_ticks,
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.timer.tick(delta_time);
    }

    pub fn current_tick(&self) -> u32 {
        timer_tick(self.timer.fraction(), self.max_ticks)
    }

    pub fn is_finished(&self) -> bool {
        self.timer.is_finished()
    }
}

fn explosion_variant_to_tile_type(variant: ExplosionVariant) -> BombExplosionTileType {
    match variant {
        ExplosionVariant::Center => BombExplosionTileType::ExplosionCenter1,
        ExplosionVariant::Straight(_) => BombExplosionTileType::ExplosionStraight1,
        ExplosionVariant::End(_) => BombExplosionTileType::ExplosionEnd1,
    }
}

pub fn spawn_explosion_visuals(
    mut commands: Commands,
    query: Query<(Entity, &ExplosionVariant), (With<Explosion>, With<ExplosionNeedsSetup>)>,
    mut explosion_materials: ResMut<Assets<ExplosionMaterial>>,
    bomb_assets: Res<BombAssets>,
    mesh_handle: Res<MeshHandle>,
) {
    for (entity, variant) in query {
        let Some(mut explosion_material) = explosion_materials
            .get(&bomb_assets.bomb_explosion_handles.0)
            .cloned()
        else {
            return;
        };
        explosion_material.set_uv_rect(
            bomb_explosion_tileset::TILEMAP
                .sprite_uv_rect(explosion_variant_to_tile_type(*variant)),
        );
        explosion_material.set_flip_x(false);
        let explosion_material = explosion_materials.add(explosion_material);

        commands
            .entity(entity)
            .remove::<ExplosionNeedsSetup>()
            .insert((
                Mesh2d(mesh_handle.0.clone()),
                MeshMaterial2d(explosion_material),
                Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                *variant,
                ExplosionTiming::new(BOMB_EXPLOSION_DURATION, EXPLOSION_TICKS),
                RenderLayers::layer(1),
            ));
    }
}

pub fn animate_bomb(
    mut query: Query<(&mut EntityScale, &TilePosition), With<Bomb>>,
    map: Res<WorldMap>,
) {
    for (mut render_scale, pos) in query.iter_mut() {
        if let Some(timer) = map
            .get_tile(*pos)
            .and_then(|t| t.bomb_or_explosion())
            .and_then(|v| v.bomb())
            .map(|b| &b.timer)
        {
            let tick = timer_tick(timer.fraction(), BOMB_TICKS);
            let scale: f32 = if tick >= BOMB_TICKS - 1 {
                1.1
            } else if tick % 2 == 0 {
                1.0
            } else {
                0.9
            };
            render_scale.0 = scale;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnimationStep {
    V1,
    V2,
    V3,
    V4,
}

impl AnimationStep {
    fn from_tick(timing: &ExplosionTiming) -> Self {
        if timing.current_tick() == 0 {
            return AnimationStep::V1;
        } else if timing.current_tick() == 1 {
            return AnimationStep::V2;
        } else if timing.current_tick() % 2 == 0 {
            return AnimationStep::V3;
        } else {
            return AnimationStep::V4;
        }
    }
}

fn get_asset_variant(anim_step: AnimationStep, variant: ExplosionVariant) -> BombExplosionTileType {
    use AnimationStep as AS;
    use BombExplosionTileType as TT;
    use ExplosionVariant as EV;

    match (anim_step, variant) {
        (AS::V1, EV::Center) => TT::ExplosionCenter1,
        (AS::V2, EV::Center) => TT::ExplosionCenter2,
        (AS::V3, EV::Center) => TT::ExplosionCenter3,
        (AS::V4, EV::Center) => TT::ExplosionCenter4,
        (AS::V1, EV::Straight(_)) => TT::ExplosionStraight1,
        (AS::V2, EV::Straight(_)) => TT::ExplosionStraight2,
        (AS::V3, EV::Straight(_)) => TT::ExplosionStraight3,
        (AS::V4, EV::Straight(_)) => TT::ExplosionStraight4,
        (AS::V1, EV::End(_)) => TT::ExplosionEnd1,
        (AS::V2, EV::End(_)) => TT::ExplosionEnd2,
        (AS::V3, EV::End(_)) => TT::ExplosionEnd3,
        (AS::V4, EV::End(_)) => TT::ExplosionEnd4,
    }
}

pub fn advance_and_despawn_explosions(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ExplosionTiming), With<Explosion>>,
    time: Res<Time>,
) {
    for (entity, mut timing) in query.iter_mut() {
        timing.update(time.delta());

        if timing.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn animate_explosion(
    mut query: Query<
        (
            &ExplosionTiming,
            &ExplosionVariant,
            &MeshMaterial2d<ExplosionMaterial>,
            &mut Transform,
        ),
        With<Explosion>,
    >,
    mut materials: ResMut<Assets<ExplosionMaterial>>,
) {
    for (timing, variant, material_handle, mut transform) in query.iter_mut() {
        let anim_step = AnimationStep::from_tick(&timing);

        let explosion_type = get_asset_variant(anim_step, *variant);
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.set_uv_rect(bomb_explosion_tileset::TILEMAP.sprite_uv_rect(explosion_type));
            material.set_flip_x(false);
        }

        let orientation = match *variant {
            ExplosionVariant::Straight(o) | ExplosionVariant::End(o) => o,
            ExplosionVariant::Center => ExplosionOrientation::Up,
        };
        let angle = match orientation {
            ExplosionOrientation::Up => 0.0,
            ExplosionOrientation::Left => FRAC_PI_2,
            ExplosionOrientation::Down => PI,
            ExplosionOrientation::Right => 3.0 * FRAC_PI_2,
        };
        *transform = Transform {
            translation: transform.translation,
            scale: transform.scale,
            rotation: Quat::from_rotation_z(angle),
        };
    }
}

pub fn animate_exploding_walls(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &mut MeshMaterial2d<ColouringMaterial>,
            &TilePosition,
        ),
        With<DestructibleWall>,
    >,
    mut materials: ResMut<Assets<ColouringMaterial>>,
    world_map: Res<WorldMap>,
) {
    for (entity, material_handle, tile_position) in query.iter() {
        if let Some(tile) = world_map.get_tile(*tile_position).map(|t| t.base_type()) {
            match tile {
                BaseTile::BreakingWall(timer) => {
                    let tile = match (timer.fraction() * 4.0) as u32 {
                        0 => MapTileType::WallFade1,
                        1 => MapTileType::WallFade2,
                        2 => MapTileType::WallFade3,
                        _ => MapTileType::WallFade4,
                    };

                    if let Some(material) = materials.get_mut(&material_handle.0) {
                        material.set_uv_rect(map_tileset::BASIC_TILEMAP.sprite_uv_rect(tile));
                    }
                }
                BaseTile::Floor => {
                    // If the wall has finished breaking, despawn it.
                    commands.entity(entity).despawn();
                }
                _ => {}
            }
        }
    }
}
