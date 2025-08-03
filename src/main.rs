mod battleground;

use crate::battleground::BattleGround;
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

#[derive(Component, Debug)]
pub struct CurrentArena(pub u32);

#[derive(Component, Debug)]
pub struct Debug;

pub const GRID_WIDTH: u32 = 66;
pub const GRID_HEIGHT: u32 = 31;
pub const ARENA_WIDTH: f32 = GRID_WIDTH as f32 * TILE_SIZE;
pub const ARENA_HEIGHT: f32 = GRID_HEIGHT as f32 * TILE_SIZE;

// 3x3 arena grid constants
pub const ARENAS_PER_ROW: u32 = 3;
pub const TOTAL_ARENAS: u32 = 9;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const HALF_WINDOW_WIDTH: f32 = WINDOW_WIDTH / 2.0;
pub const HALF_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;
// Tile dimensions
pub const TILE_SIZE: f32 = 19.0; // Each tile is 19 world units (will map to 19 pixels)
pub const HALF_TILE: f32 = TILE_SIZE / 2.0;
pub const TILE_GAP: f32 = 0.0; // No gap between tiles
pub const TILE_HEIGHT: f32 = 2.0; // Height of the tile mesh

// Simple tile color
pub const TILE_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);

// Debug arena colors (hex colors converted to sRGB)
pub const DEBUG_COLORS: [Color; 9] = [
    Color::srgb(1.0, 0.329, 0.0),      // #ff5400
    Color::srgb(1.0, 0.557, 0.0),      // #ff8e00
    Color::srgb(1.0, 0.824, 0.0),      // #ffd200
    Color::srgb(0.506, 0.902, 0.314),  // #81e650
    Color::srgb(0.0, 0.824, 0.404),    // #00d267
    Color::srgb(0.0, 0.753, 1.0),      // #00c0ff
    Color::srgb(0.545, 0.282, 0.996),  // #8b48fe
    Color::srgb(0.792, 0.255, 0.988),  // #ca41fc
    Color::srgb(1.0, 0.275, 0.984),    // #ff46fb
];

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create shared mesh
    // Cuboid dimensions: width (X), height (Y), depth (Z)
    let tile_mesh = meshes.add(Cuboid::new(
        TILE_SIZE - TILE_GAP,
        TILE_SIZE - TILE_GAP,
        TILE_HEIGHT,
    ));

    // Add Debug component to enable debug visualization
    commands.spawn(Debug);

    // Create 3x3 grid of arenas (9 arenas total)
    setup_arena_grid(&mut commands, &tile_mesh, &mut materials);

    // Setup camera positioned to see entire grid
    setup_camera(&mut commands);

    // Add simple lighting
    setup_lighting(&mut commands);
}

/// Spawns the battleground parent entity and returns its Entity ID
fn spawn_battleground(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Transform::from_xyz(-ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0),
            InheritedVisibility::default(),
            BattleGround,
        ))
        .id()
}

/// Spawns the arena grid tiles as children of the given parent entity
fn spawn_arena(
    commands: &mut Commands,
    parent_entity: Entity,
    tile_mesh: &Handle<Mesh>,
    tile_material: &Handle<StandardMaterial>,
) {
    commands.entity(parent_entity).with_children(|parent| {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let world_x = x as f32 * TILE_SIZE;
                let world_y = -(y as f32 * TILE_SIZE);

                // Spawn each tile as a child
                parent.spawn((
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(tile_material.clone()),
                    Transform::from_xyz(world_x, world_y, 0.0),
                    GridPosition { x, y },
                    ArenaTile,
                ));
            }
        }
    });
}

/// Sets up a 3x3 grid of arenas (9 arenas total) adapted from 2D implementation
fn setup_arena_grid(
    commands: &mut Commands,
    tile_mesh: &Handle<Mesh>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Spawn current arena marker
    commands.spawn(CurrentArena(1));

    // Create materials for each arena (using debug colors since Debug component is spawned)
    let arena_materials: Vec<Handle<StandardMaterial>> = (0..TOTAL_ARENAS)
        .map(|i| {
            materials.add(StandardMaterial {
                base_color: DEBUG_COLORS[i as usize],
                ..default()
            })
        })
        .collect();

    // Set up 3x3 grid of arenas (9 arenas total)
    for arena_index in 0..TOTAL_ARENAS {
        let arena_col = arena_index % ARENAS_PER_ROW;
        let arena_row = arena_index / ARENAS_PER_ROW;

        // Calculate 3D positioning (adapted from your 2D version)
        let x_offset = arena_col as f32 * ARENA_WIDTH;
        let y_offset = arena_row as f32 * ARENA_HEIGHT;

        // Position arenas in 3D space (XY plane for top-down view)
        let arena_x = -HALF_WINDOW_WIDTH + HALF_TILE + x_offset;
        let arena_y = HALF_WINDOW_HEIGHT - HALF_TILE - y_offset;

        // Create battleground entity for this arena
        let battleground_entity = commands
            .spawn((
                Transform::from_xyz(arena_x, arena_y, 0.0),
                InheritedVisibility::default(),
                BattleGround,
            ))
            .id();

        // Spawn the arena grid tiles as children
        spawn_arena(commands, battleground_entity, tile_mesh, &arena_materials[arena_index as usize]);
    }
}

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
