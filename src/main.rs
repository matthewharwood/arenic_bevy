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
pub const ARENA_WIDTH_HALF: f32 = ARENA_WIDTH / 2.0;
pub const ARENA_HEIGHT_HALF: f32 = ARENA_HEIGHT / 2.0;

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
    Color::srgb(1.0, 0.329, 0.0),     // #ff5400
    Color::srgb(1.0, 0.557, 0.0),     // #ff8e00
    Color::srgb(1.0, 0.824, 0.0),     // #ffd200
    Color::srgb(0.506, 0.902, 0.314), // #81e650
    Color::srgb(0.0, 0.824, 0.404),   // #00d267
    Color::srgb(0.0, 0.753, 1.0),     // #00c0ff
    Color::srgb(0.545, 0.282, 0.996), // #8b48fe
    Color::srgb(0.792, 0.255, 0.988), // #ca41fc
    Color::srgb(1.0, 0.275, 0.984),   // #ff46fb
];

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create shared mesh
    // Cuboid dimensions: width (X), height (Y), depth (Z)
    let tile_mesh = meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE));

    // Add Debug component to enable debug visualization
    commands.spawn(Debug);

    // Create 3x3 grid of arenas (9 arenas total)
    setup_arena_grid(&mut commands, &tile_mesh, &mut materials);

    // Setup camera positioned to see entire grid
    setup_camera(&mut commands, 1);

    // Add simple lighting
    setup_lighting(&mut commands);
}

/// Spawns the battleground parent entity and returns its Entity ID
fn spawn_battleground(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Transform::from_xyz(-ARENA_WIDTH_HALF, ARENA_HEIGHT_HALF, 0.0),
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

/// Helper function to get arena position from index
fn get_arena_position(arena_index: u32) -> Vec3 {
    let col = arena_index % ARENAS_PER_ROW;
    let row = arena_index / ARENAS_PER_ROW;
    
    // Start from window top-left corner and offset by arena size
    let x = -HALF_WINDOW_WIDTH + (col as f32 * ARENA_WIDTH) + HALF_TILE;
    let y = HALF_WINDOW_HEIGHT - (row as f32 * ARENA_HEIGHT) - HALF_TILE;
    
    Vec3::new(x, y, 0.0)
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
    let arena_materials: Vec<Handle<StandardMaterial>> = DEBUG_COLORS
        .iter()
        .map(|&color| {
            materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })
        })
        .collect();

    // Set up 3x3 grid of arenas (9 arenas total)
    for arena_index in 0..TOTAL_ARENAS {
        let position = get_arena_position(arena_index);

        // Create battleground entity for this arena
        let battleground_entity = commands
            .spawn((
                Transform::from_translation(position),
                InheritedVisibility::default(),
                BattleGround,
            ))
            .id();

        // Spawn the arena grid tiles as children
        spawn_arena(
            commands,
            battleground_entity,
            tile_mesh,
            &arena_materials[arena_index as usize],
        );
    }
}

/// Calculate camera position to center on a specific arena
pub fn calculate_camera_position(arena_index: u8) -> Vec3 {
    let position = get_arena_position(arena_index as u32);
    // Add half arena dimensions to get center
    position + Vec3::new(ARENA_WIDTH_HALF - HALF_TILE, -ARENA_HEIGHT_HALF + HALF_TILE, 0.0)
}

/// Setup camera to center on a specific arena (0-8) or view entire grid if None
fn setup_camera(commands: &mut Commands, arena_index: u8) {
    let center = calculate_camera_position(arena_index);
    let camera_pos = center + Vec3::new(0.0, 0.0, 1000.0); // Add height
    
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(camera_pos).looking_at(center, Vec3::Y),
        Projection::from(OrthographicProjection {
            scale: 1.0,
            scaling_mode: ScalingMode::WindowSize,
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
