use bevy::{
    prelude::*,
    window::{
        Monitor, PrimaryMonitor, PrimaryWindow, WindowMode, WindowResized, WindowScaleFactorChanged,
    },
};

use crate::{
    assets::TILESET_TILE_SIZE,
    constants::{TOP_MENU_BAR_HEIGHT, TOTAL_MAP_HEIGHT, TOTAL_MAP_WIDTH},
    game_state::GameState,
    position::{TilePosition, WorldPosition},
    world_entities::RenderedAreaWidth,
};

#[derive(Resource, Deref, DerefMut)]
struct TileScale(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct EntityScale(pub f32);

/// Returns the size of the window in pixels. Fullscreen and windowless fullscreen modes ignore
/// window size and for those cases the actual window size is retrieved from used monitor.
pub fn get_window_size(window: &Window, monitor: Option<&Monitor>) -> (f32, f32) {
    match window.mode {
        WindowMode::Fullscreen(..) | WindowMode::BorderlessFullscreen(_) => {
            if let Some(monitor) = monitor {
                (
                    monitor.physical_width as f32,
                    monitor.physical_height as f32,
                )
                    .into()
            } else {
                (window.width(), window.height()).into()
            }
        }
        WindowMode::Windowed => (window.width(), window.height()).into(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Scaling {
    pub tile_scale: f32,
    pub total_width_px: f32,
}

fn recompute_scaling(window: &Window, monitor: Option<&Monitor>) -> Scaling {
    let (target_width, target_height) = get_window_size(window, monitor);

    let tile_width = TILESET_TILE_SIZE.x as f32;
    let tile_height = TILESET_TILE_SIZE.y as f32;

    let map_width_px = TOTAL_MAP_WIDTH as f32 * tile_width;
    let map_height_px = TOTAL_MAP_HEIGHT as f32 * tile_height;

    let tile_scale =
        (target_width / map_width_px).min((target_height - TOP_MENU_BAR_HEIGHT) / map_height_px);

    Scaling {
        tile_scale,
        total_width_px: map_width_px * tile_scale,
    }
}

fn compute_scale(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    monitor: Query<&Monitor, With<PrimaryMonitor>>,
) {
    let Ok(window) = windows.single() else {
        eprintln!("No primary window found, cannot compute camera scale.");
        return;
    };

    let Scaling {
        tile_scale,
        total_width_px,
    } = recompute_scaling(window, monitor.single().ok());

    commands.insert_resource(TileScale(tile_scale));
    commands.insert_resource(RenderedAreaWidth(total_width_px));
}

fn recompute_scale_on_window_change(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    monitor: Query<&Monitor, With<PrimaryMonitor>>,
    mut resized_events: MessageReader<WindowResized>,
    mut scale_factor_events: MessageReader<WindowScaleFactorChanged>,
) {
    let resized = resized_events.read().last().is_some();
    let scale_changed = scale_factor_events.read().last().is_some();

    if !resized && !scale_changed {
        return;
    }

    let Ok(window) = windows.single() else {
        eprintln!("No primary window found, cannot compute camera scale.");
        return;
    };

    let Scaling {
        tile_scale,
        total_width_px,
    } = recompute_scaling(window, monitor.single().ok());

    commands.insert_resource(TileScale(tile_scale));
    commands.insert_resource(RenderedAreaWidth(total_width_px));
}

fn update_transformation(
    transform: &mut Transform,
    world_position: WorldPosition,
    render_scale: Option<&EntityScale>,
    scale: f32,
) {
    let render_scale = render_scale.map(|rs| rs.0).unwrap_or(1.0);
    *transform = Transform {
        translation: Vec3::new(
            world_position.x * TILESET_TILE_SIZE.x as f32 * scale,
            world_position.y * TILESET_TILE_SIZE.y as f32 * scale - TOP_MENU_BAR_HEIGHT * 0.5,
            transform.translation.z,
        ),
        scale: Vec3::splat(scale * render_scale),
        rotation: transform.rotation,
    };
}

fn update_world_transformations(
    non_tile_entities: Query<(&mut Transform, &WorldPosition, Option<&EntityScale>)>,
    scale: Res<TileScale>,
) {
    let scale = scale.0;
    for (mut transform, world_position, render_scale) in non_tile_entities {
        update_transformation(&mut transform, *world_position, render_scale, scale);
    }
}

fn update_tile_transformations(
    tile_entities: Query<(&mut Transform, &TilePosition, Option<&EntityScale>)>,
    scale: Res<TileScale>,
) {
    let scale = scale.0;
    for (mut transform, tile_position, render_scale) in tile_entities {
        let world_position = tile_position.to_world_position();
        update_transformation(&mut transform, world_position, render_scale, scale);
    }
}

pub struct CameraScalePlugin;

impl Plugin for CameraScalePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RenderedAreaWidth(1.0))
            .insert_resource(TileScale(1.0))
            .add_systems(PreStartup, compute_scale)
            .add_systems(PreUpdate, recompute_scale_on_window_change)
            .add_systems(
                PostUpdate,
                (update_world_transformations, update_tile_transformations)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
