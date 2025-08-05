mod arena;
mod arena_camera;
mod battleground;

// Uncomment these modules to debug pink material issues
mod class_type;
mod selectors;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use crate::arena::{get_local_tile_space, Arena};
use crate::selectors::{Active};

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
        .add_systems(Startup, setup_scene)
        .add_systems(Startup, parent_pending_arena_children.after(setup_scene))
        .add_systems(Update, spawn_sphere)
        .run();
}

#[derive(Component, Debug)]
pub struct Debug;

/// Marker component for spheres that need to be parented to an arena
#[derive(Component)]
struct PendingArenaChild {
    arena_id: arena::ArenaId,
    local_position: Vec3,
}

fn setup_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
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


    // Add simple lighting positioned at the camera's target
    setup_lighting(&mut commands, default_arena);
}

/// System to parent pending arena children to their respective arenas
fn parent_pending_arena_children(
    mut commands: Commands,
    arena_query: Query<(Entity, &arena::ArenaId), With<Arena>>,
    pending_query: Query<(Entity, &PendingArenaChild)>,
) {
    for (child_entity, pending) in pending_query.iter() {
        // Find the arena entity with matching ArenaId
        if let Some(arena_entity) = arena_query
            .iter()
            .find(|(_, id)| **id == pending.arena_id)
            .map(|(entity, _)| entity)
        {
            // Parent the child to the arena and update its transform to be relative
            commands.entity(arena_entity).add_child(child_entity);
            commands.entity(child_entity)
                .remove::<PendingArenaChild>()
                .insert(Transform::from_translation(pending.local_position));
            
            info!("Parented sphere to arena {:?} at local position {:?}", pending.arena_id, pending.local_position);
        }
    }
}
fn spawn_sphere(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Single<(Entity, &arena::ArenaId), (With<Arena>, With<Active>)>,
) {
    let (arena_entity, arena_id) = query.into_inner();
    println!("Spawn sphere in arena {:?}", arena_id);
    let blue_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.153, 0.431, 0.945), // #276EF1
        metallic: 0.0, // Non-metallic
        perceptual_roughness: 1.0, // Maximum roughness
        ..default()
    });
    let sphere_radius = 8.0; // Slightly smaller than half tile size (9.5) for visual spacing
    let sphere_mesh = meshes.add(Sphere::new(sphere_radius));
    let local_position = get_local_tile_space(32, 15);
    commands.entity(arena_entity).with_child((
        Mesh3d(sphere_mesh),
        MeshMaterial3d(blue_material),
        Transform::from_translation(local_position),
    ));
}

/// Simple lighting setup positioned at camera target
fn setup_lighting(commands: &mut Commands, arena_id: arena::ArenaId) {
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

    // Calculate the camera's target position (center of the arena)
    let camera_target = arena_camera::calculate_camera_position(arena_id);
    
    // Point light positioned at the camera's target position
    // This ensures the light illuminates where the camera is looking
    commands.spawn((
        PointLight {
            intensity: 500000.0, // Strong intensity for good tile illumination
            range: 800.0, // Range to cover a good portion of the arena
            radius: 0.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        // Position at the camera's target with some elevation
        Transform::from_translation(camera_target + Vec3::new(0.0, 0.0, 300.0)),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.8, 0.8, 0.8),
        brightness: 0.5,
        affects_lightmapped_meshes: false,
    });
}
