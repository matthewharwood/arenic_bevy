use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResolution;

const GAME_NAME: &str = "Arenic";
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: GAME_NAME.to_string(),
                resolution: WindowResolution::new(1280.0, 720.0),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .run();
}

#[derive(Component, Debug)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Component, Debug)]
pub struct ArenaTile;

pub const GRID_WIDTH: u32 = 66;
pub const GRID_HEIGHT: u32 = 31;

// Tile dimensions
pub const TILE_SIZE: f32 = 19.0; // Each tile is 19 world units (will map to 19 pixels)
pub const HALF_TILE: f32 = TILE_SIZE / 2.0;
pub const TILE_GAP: f32 = 0.0; // No gap between tiles
pub const TILE_HEIGHT: f32 = 2.0; // Height of the tile mesh

// Simple tile color
pub const TILE_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create shared mesh and material
    // Cuboid dimensions: width (X), height (Y), depth (Z)
    let tile_mesh = meshes.add(Cuboid::new(
        TILE_SIZE - TILE_GAP,
        TILE_SIZE - TILE_GAP,
        TILE_HEIGHT,
    ));

    let tile_material = materials.add(StandardMaterial {
        base_color: TILE_COLOR,
        ..default()
    });

    // Spawn a few test tiles to see the coordinate system
    // For a top-down 2D-style game, we place tiles on the XZ plane:
    // - X axis: Left (-X) to Right (+X) on screen
    // - Z axis: Up (-Z) to Down (+Z) on screen (inverted!)
    // - Y axis: Height above the ground (all tiles at Y=0)

    // Center tile (red)
    commands.spawn((
        Mesh3d(tile_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0), // Red
            ..default()
        })),
        Transform::from_xyz(-1280.0 / 2.0, 720.0 / 2.0, 0.0),
        ArenaTile,
    ));

    // Setup camera positioned to see entire grid
    setup_camera(&mut commands);

    // Add simple lighting
    setup_lighting(&mut commands);
}
// fn spawn_grid(commands: &mut Commands, mesh: &Handle<Mesh>, material: &Handle<StandardMaterial>) {
//     // Calculate grid center offset
//     let offset_x = (GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
//     let offset_z = (GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;
//
//     for x in 0..GRID_WIDTH {
//         for y in 0..GRID_HEIGHT {
//             let world_x = x as f32 * TILE_SIZE - offset_x;
//             let world_z = y as f32 * TILE_SIZE - offset_z;
//
//
//         }
//     }
// }

/// Setup camera to view entire grid
fn setup_camera(commands: &mut Commands) {
    // CAMERA POSITIONING
    // Place camera high above the grid for birds eye view
    let camera_height = 1000.0;

    // MAKING TILES APPEAR AS 19x19 PIXELS:
    // With orthographic projection, the 'scale' determines the world-to-pixel mapping
    // The formula is: scale = (world_units_visible / window_pixels)
    //
    // Our grid dimensions:
    // - Width: 66 tiles × 19 units = 1254 world units
    // - Height: 31 tiles × 19 units = 589 world units
    // - Window: 1280×720 pixels
    //
    // To fit the grid with minimal margins:
    let grid_width = GRID_WIDTH as f32 * TILE_SIZE; // 1254 world units
    let grid_height = GRID_HEIGHT as f32 * TILE_SIZE; // 589 world units

    // Add 5% margin to ensure everything fits
    let margin = 1.05;

    // Calculate scale to fit both dimensions
    // We need the larger scale to ensure both width and height fit
    let scale_for_width = (grid_width * margin) / 1280.0; // World units per pixel horizontally
    let scale_for_height = (grid_height * margin) / 720.0; // World units per pixel vertically
    let scale = scale_for_width.max(scale_for_height); // Use larger to ensure fit

    // With this scale, 1 world unit ≈ 1 pixel (approximately)
    // So our 19 world unit tiles will appear as ~19 pixels on screen

    // Position camera to make Y-axis point up on screen
    // This makes the coordinate system more intuitive for 2D-style games
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, camera_height) // Position along +Z axis
            .looking_at(Vec3::ZERO, Vec3::Y), // Look at origin, Y is "up"
        Projection::from(OrthographicProjection {
            scale,
            scaling_mode: ScalingMode::Fixed {
                width: 1280.0,
                height: 720.0,
            },
            ..OrthographicProjection::default_3d()
        }),
    ));
}

/// Simple lighting setup
fn setup_lighting(commands: &mut Commands) {
    // Directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            std::f32::consts::FRAC_PI_4,
            0.0,
        )),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.8, 0.8, 0.8),
        brightness: 0.5,
        affects_lightmapped_meshes: false,
    });
}
