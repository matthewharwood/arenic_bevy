mod arena;
mod arena_camera;
mod battleground;

// Uncomment these modules to debug pink material issues
// mod material_debugger;
// mod material_test_scene;
// mod material_inspector;

use bevy::color::Srgba;
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
        // .add_plugins(material_debugger::MaterialDebuggerPlugin)
        // .add_plugins(material_test_scene::MaterialTestScenePlugin)
        // .add_plugins(material_inspector::MaterialInspectorPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Startup, parent_pending_arena_children.after(setup_scene))
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
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Add Debug component to enable debug visualization
    commands.spawn(Debug);

    // Create tile meshes and materials
    let (base_mesh, inset_mesh) = arena::build_tile_meshes();
    let base_handle = meshes.add(base_mesh);
    let inset_handle = meshes.add(inset_mesh);

    let gray_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.8, 0.8),
        ..default()
    });

    let hot_pink = Srgba::hex("ff00ff").unwrap();
    let pink_material = materials.add(StandardMaterial {
        base_color: Color::from(hot_pink),
        emissive: hot_pink.into(),
        unlit: true,
        ..default()
    });

    // Create 3x3 grid of arenas (9 arenas total)
    arena::setup_arena_grid(
        &mut commands,
        base_handle,
        inset_handle,
        gray_material,
        pink_material,
    );

    // Setup camera positioned to see entire grid
    let default_arena = arena::ArenaId::new(1).expect("Arena 1 should be valid");
    arena_camera::setup_camera(&mut commands, default_arena);

    // Spawn blue sphere in arena 1 at column 32, row 15
    spawn_sphere(
        &mut commands,
        &mut meshes,
        &mut materials,
        default_arena,
        32,
        15,
    );

    // Add simple lighting positioned at the camera's target
    setup_lighting(&mut commands, default_arena);
}

/// System to parent pending arena children to their respective arenas
fn parent_pending_arena_children(
    mut commands: Commands,
    arena_query: Query<(Entity, &arena::ArenaId), With<arena::Arena>>,
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

    // Calculate local position within the arena based on grid coordinates
    // Each tile is 19 units, positioned from top-left corner of arena
    let local_x = column as f32 * arena::TILE_SIZE;
    let local_y = -(row as f32 * arena::TILE_SIZE);
    let local_position = Vec3::new(local_x, local_y, sphere_radius);

    // Calculate the arena's world position for temporary placement
    let arena_position = arena::get_arena_position(arena_id);
    let world_position = arena_position + local_position;

    // Spawn the sphere entity with a marker for deferred parenting
    commands.spawn((
        Mesh3d(sphere_mesh),
        MeshMaterial3d(blue_material),
        Transform::from_translation(world_position),
        arena::InArena::new(arena_id),
        arena::GridPosition::new(column, row),
        PendingArenaChild {
            arena_id,
            local_position,
        },
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
