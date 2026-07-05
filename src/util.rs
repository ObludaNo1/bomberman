use bevy::{
    prelude::*,
    window::{
        Monitor, PrimaryMonitor, PrimaryWindow, WindowMode, WindowResized, WindowScaleFactorChanged,
    },
};

use crate::{
    assets::TILESET_TILE_SIZE,
    map::{MAP_HEIGHT, MAP_WIDTH},
    position::WorldPosition,
};

#[derive(Resource, Deref, DerefMut)]
pub struct CameraScale(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct RenderScale(pub f32);

fn calculate_scale(window: &Window, monitor: Option<&Monitor>) -> f32 {
    let (target_width, target_height) = match window.mode {
        WindowMode::Fullscreen(..) | WindowMode::BorderlessFullscreen(_) => {
            if let Some(monitor) = monitor {
                (
                    monitor.physical_width as f32,
                    monitor.physical_height as f32,
                )
            } else {
                (window.width(), window.height())
            }
        }
        WindowMode::Windowed => (window.width(), window.height()),
    };

    let tile_width = TILESET_TILE_SIZE.x as f32;
    let tile_height = TILESET_TILE_SIZE.y as f32;

    let map_width_px = MAP_WIDTH as f32 * tile_width;
    let map_height_px = MAP_HEIGHT as f32 * tile_height;

    (target_width / map_width_px).min(target_height / map_height_px)
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

    let scale = calculate_scale(window, monitor.single().ok());

    commands.insert_resource(CameraScale(scale));
}

fn recompute_scale_on_window_change(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    monitor: Query<&Monitor, With<PrimaryMonitor>>,
    mut resized_events: MessageReader<WindowResized>,
    mut scale_factor_events: MessageReader<WindowScaleFactorChanged>,
) {
    let resized = resized_events.read().next().is_some();
    let scale_changed = scale_factor_events.read().next().is_some();

    if !resized && !scale_changed {
        return;
    }

    let Ok(window) = windows.single() else {
        eprintln!("No primary window found, cannot compute camera scale.");
        return;
    };

    let scale = calculate_scale(window, monitor.single().ok());

    commands.insert_resource(CameraScale(scale));
}

pub fn update_transformations(
    mut query: Query<(&mut Transform, &WorldPosition, Option<&RenderScale>)>,
    scale: Res<CameraScale>,
) {
    let scale = scale.0;
    for (mut transform, world_position, render_scale) in query.iter_mut() {
        let render_scale = render_scale.map(|rs| rs.0).unwrap_or(1.0);
        *transform = Transform {
            translation: Vec3::new(
                world_position.x * TILESET_TILE_SIZE.x as f32 * scale,
                world_position.y * TILESET_TILE_SIZE.y as f32 * scale,
                transform.translation.z,
            ),
            scale: Vec3::splat(scale * render_scale),
            rotation: transform.rotation,
        };
    }
}

pub struct CameraScalePlugin;

impl Plugin for CameraScalePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, compute_scale).add_systems(
            PostUpdate,
            (recompute_scale_on_window_change, update_transformations).chain(),
        );
    }
}
