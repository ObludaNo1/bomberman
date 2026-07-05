use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::{
    assets::{
        bomb_explosion_tileset::{self, BombExplosionTileType},
        material::ColouringMaterial,
    },
    bomb::{BombTiming, ExplosionOrientation, ExplosionPathType, ExplosionTileVariant},
    util::RenderScale,
    world_entities::{Bomb, Explosion},
};

pub fn animate_bomb(mut query: Query<(&BombTiming, &mut RenderScale), With<Bomb>>) {
    for (bomb_timing, mut render_scale) in query.iter_mut() {
        let scale: f32 = if bomb_timing.is_on_final_tick() {
            1.1
        } else if bomb_timing.ticks % 2 == 0 {
            1.0
        } else {
            0.9
        };
        render_scale.0 = scale;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExplosionAnimationVariant {
    V1,
    V2,
    V3,
    V4,
}

impl ExplosionAnimationVariant {
    fn from_tick(timing: &BombTiming) -> Self {
        if timing.ticks == 0 {
            return ExplosionAnimationVariant::V1;
        } else if timing.ticks == 1 {
            return ExplosionAnimationVariant::V2;
        } else if timing.ticks % 2 == 0 {
            return ExplosionAnimationVariant::V3;
        } else {
            return ExplosionAnimationVariant::V4;
        }
    }
}

fn get_asset_variant(
    anim_var: ExplosionAnimationVariant,
    dir_var: ExplosionTileVariant,
) -> BombExplosionTileType {
    use BombExplosionTileType as TT;
    use ExplosionAnimationVariant as AV;
    use ExplosionPathType as PT;

    match (anim_var, dir_var.kind) {
        (AV::V1, PT::Center) => TT::ExplosionCenter1,
        (AV::V2, PT::Center) => TT::ExplosionCenter2,
        (AV::V3, PT::Center) => TT::ExplosionCenter3,
        (AV::V4, PT::Center) => TT::ExplosionCenter4,
        (AV::V1, PT::Straight) => TT::ExplosionStraight1,
        (AV::V2, PT::Straight) => TT::ExplosionStraight2,
        (AV::V3, PT::Straight) => TT::ExplosionStraight3,
        (AV::V4, PT::Straight) => TT::ExplosionStraight4,
        (AV::V1, PT::End) => TT::ExplosionEnd1,
        (AV::V2, PT::End) => TT::ExplosionEnd2,
        (AV::V3, PT::End) => TT::ExplosionEnd3,
        (AV::V4, PT::End) => TT::ExplosionEnd4,
    }
}

pub fn animate_explosion(
    mut query: Query<
        (
            &mut BombTiming,
            &ExplosionTileVariant,
            &MeshMaterial2d<ColouringMaterial>,
            &mut Transform,
        ),
        With<Explosion>,
    >,
    mut materials: ResMut<Assets<ColouringMaterial>>,
    time: Res<Time>,
) {
    let delta_time = time.delta();

    for (mut timing, dir_var, material_handle, mut transform) in query.iter_mut() {
        timing.update(delta_time);

        let anim_var = ExplosionAnimationVariant::from_tick(&timing);

        let explosion_type = get_asset_variant(anim_var, *dir_var);
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.set_uv_rect(bomb_explosion_tileset::TILEMAP.sprite_uv_rect(explosion_type));
            material.set_flip_x(false);
        }

        let angle = match dir_var.orientation {
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
