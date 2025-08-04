mod arena;
mod arena_camera;
mod battleground;

// Uncomment these modules to debug pink material issues
mod material_debugger;
// mod material_test_scene;
// mod material_inspector;

use bevy::prelude::*;
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
        // Uncomment these plugins to debug pink material issues
        .add_plugins(material_debugger::MaterialDebuggerPlugin)
        // .add_plugins(material_test_scene::MaterialTestScenePlugin)
        // .add_plugins(material_inspector::MaterialInspectorPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

#[derive(Component, Debug)]
pub struct Debug;

fn setup_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    // Load the tile model
    let tile_scene = asset_server.load("tile.glb#Scene0");

    // Add Debug component to enable debug visualization
    commands.spawn(Debug);

    // Create 3x3 grid of arenas (9 arenas total)
    arena::setup_arena_grid(&mut commands, tile_scene, &mut materials);

    // Setup camera positioned to see entire grid
    let default_arena = arena::ArenaId::new(1).expect("Arena 1 should be valid");
    arena_camera::setup_camera(&mut commands, default_arena);

    // Spawn blue sphere in arena 1 at column 32, row 15
    spawn_sphere(&mut commands, &mut meshes, &mut materials, default_arena, 32, 15);

    // Add simple lighting
    setup_lighting(&mut commands);
}

/// Spawn a blue sphere at the specified grid position within an arena
fn spawn_sphere(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    arena_id: arena::ArenaId,
    column: u32,
    row: u32,
) {
    // Create blue material close to #276EF1
    let blue_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.153, 0.431, 0.945), // #276EF1
        ..default()
    });

    // Create sphere mesh that fits within a 19x19 tile
    let sphere_radius = 8.0; // Slightly smaller than half tile size (9.5) for visual spacing
    let sphere_mesh = meshes.add(Sphere::new(sphere_radius));

    // Calculate the arena's world position
    let arena_position = arena::get_arena_position(arena_id);
    
    // Calculate local position within the arena based on grid coordinates
    // Each tile is 19 units, positioned from top-left corner of arena
    let local_x = column as f32 * arena::TILE_SIZE + arena::HALF_TILE;
    let local_y = -(row as f32 * arena::TILE_SIZE) - arena::HALF_TILE;
    
    // Combine arena position with local position, add slight elevation
    let world_position = arena_position + Vec3::new(local_x, local_y, sphere_radius);

    // Spawn the sphere entity with appropriate components
    commands.spawn((
        Mesh3d(sphere_mesh),
        MeshMaterial3d(blue_material),
        Transform::from_translation(world_position),
        arena::InArena::new(arena_id),
        arena::GridPosition::new(column, row),
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
