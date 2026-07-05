use bevy::{prelude::*, sprite_render::Material2dPlugin};

use crate::assets::{
    TILESET_TILE_SIZE,
    material::{ColouringMaterial, ExplosionMaterial},
};

#[derive(Resource, Debug)]
pub struct MeshHandle(pub Handle<Mesh>);

fn add_rendering_mesh(mut meshes: ResMut<Assets<Mesh>>, mut mesh_handle: ResMut<MeshHandle>) {
    let mesh = meshes.add(Rectangle::new(
        TILESET_TILE_SIZE.x as f32,
        TILESET_TILE_SIZE.y as f32,
    ));
    mesh_handle.0 = mesh;
}

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MeshHandle(Handle::default()))
            .add_plugins(Material2dPlugin::<ColouringMaterial>::default())
            .add_plugins(Material2dPlugin::<ExplosionMaterial>::default())
            .add_systems(PreStartup, add_rendering_mesh);
    }
}
