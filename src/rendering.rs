use bevy::{
    asset::RenderAssetUsages,
    camera::{ImageRenderTarget, RenderTarget, visibility::RenderLayers},
    image::ImageSampler,
    prelude::*,
    render::render_resource::{
        Extent3d, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        TextureViewDescriptor, TextureViewDimension,
    },
    sprite_render::Material2dPlugin,
    window::{Monitor, PrimaryMonitor, PrimaryWindow, WindowResized, WindowScaleFactorChanged},
};

use crate::{
    assets::{
        TILESET_TILE_SIZE,
        material::{ColouringMaterial, ExplosionMaterial, SecondPassMaterial},
    },
    util::get_window_size,
};

#[derive(Component)]
struct FirstPassCamera;

#[derive(Component)]
struct SecondPassCamera;

#[derive(Component)]
struct FullScreenRect;

#[derive(Resource, Debug)]
pub struct MeshHandle(pub Handle<Mesh>);

fn create_render_target_image(window_size: (f32, f32)) -> Image {
    Image {
        texture_view_descriptor: Some(TextureViewDescriptor {
            label: None,
            aspect: TextureAspect::All,
            dimension: Some(TextureViewDimension::D2),
            format: Some(TextureFormat::Rgba8Unorm),
            usage: Some(TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT),
            ..Default::default()
        }),
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d {
                width: window_size.0 as u32,
                height: window_size.1 as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[TextureFormat::Rgba8Unorm, TextureFormat::Rgba8UnormSrgb],
        },
        asset_usage: RenderAssetUsages::RENDER_WORLD,
        copy_on_resize: false,
        data: None,
        sampler: ImageSampler::nearest(),
        ..Default::default()
    }
}

/// Spawn the main 2D camera.
fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_handle: ResMut<MeshHandle>,
    mut second_pass_material: ResMut<Assets<SecondPassMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    monitor: Query<&Monitor, With<PrimaryMonitor>>,
) {
    let Ok(window) = windows.single() else {
        eprintln!("No primary window found, cannot create matching render target.");
        return;
    };

    let window_size = get_window_size(window, monitor.single().ok());

    let image = create_render_target_image(window_size);
    let image_handle = images.add(image);

    let mesh = meshes.add(Rectangle::new(
        TILESET_TILE_SIZE.x as f32,
        TILESET_TILE_SIZE.y as f32,
    ));
    mesh_handle.0 = mesh;

    let second_pass_material =
        second_pass_material.add(SecondPassMaterial::new(image_handle.clone()));

    let fullscreen_mesh = meshes.add(Rectangle::new(1.0, 1.0));
    commands.spawn((
        Mesh2d(fullscreen_mesh),
        MeshMaterial2d(second_pass_material),
        Transform {
            scale: Vec3::new(window_size.0, window_size.1, 1.0),
            translation: Vec3::new(0.0, 0.0, 0.1),
            ..Default::default()
        },
        FullScreenRect,
    ));

    commands.spawn((
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::linear_rgba(1.0, 1.0, 1.0, 0.0)),
            ..default()
        },
        Camera2d,
        RenderTarget::Image(ImageRenderTarget {
            handle: image_handle,
            scale_factor: 1.0,
        }),
        // This filters out only entities which also share this layer.
        RenderLayers::layer(1),
        FirstPassCamera,
    ));

    commands.spawn((
        Camera {
            order: 1,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Camera2d,
        SecondPassCamera,
    ));
}

fn resize_render_target_on_window_resize(
    windows: Query<&Window, With<PrimaryWindow>>,
    monitor: Query<&Monitor, With<PrimaryMonitor>>,
    mut resized_events: MessageReader<WindowResized>,
    mut scale_factor_events: MessageReader<WindowScaleFactorChanged>,
    mut image_assets: ResMut<Assets<Image>>,
    mut second_pass_material: ResMut<Assets<SecondPassMaterial>>,
    mut fullscreen_mesh_query: Query<
        (&mut Transform, &mut MeshMaterial2d<SecondPassMaterial>),
        With<FullScreenRect>,
    >,
    mut first_pass_render_target: Query<&mut RenderTarget, With<FirstPassCamera>>,
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

    let Ok((mut transform, mut mesh_material)) = fullscreen_mesh_query.single_mut() else {
        eprintln!("No fullscreen mesh transform found, cannot resize fullscreen mesh.");
        return;
    };

    let Ok(mut render_target) = first_pass_render_target.single_mut() else {
        eprintln!("No first pass render target found, cannot resize render target.");
        return;
    };

    let window_size = get_window_size(window, monitor.single().ok());

    let image = create_render_target_image(window_size);
    let image_handle = image_assets.add(image);

    let material = second_pass_material.add(SecondPassMaterial::new(image_handle.clone()));
    *mesh_material = MeshMaterial2d(material);

    *render_target = RenderTarget::Image(ImageRenderTarget {
        handle: image_handle,
        scale_factor: 1.0,
    });

    transform.scale = Vec3::new(window_size.0, window_size.1, 1.0);
}

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MeshHandle(Handle::default()))
            .add_plugins(Material2dPlugin::<ColouringMaterial>::default())
            .add_plugins(Material2dPlugin::<ExplosionMaterial>::default())
            .add_plugins(Material2dPlugin::<SecondPassMaterial>::default())
            .add_systems(PreStartup, setup_camera)
            .add_systems(Update, resize_render_target_on_window_resize);
    }
}
