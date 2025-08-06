mod arena;
mod arena_camera;
mod battleground;

// Uncomment these modules to debug pink material issues
mod class_type;
mod selectors;
mod character;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use crate::arena::{get_local_tile_space, Arena, TILE_SIZE};
use crate::character::{AutoShot, Boss, Character};
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
        .add_systems(Startup, (parent_pending_arena_children, spawn_starting_hero, spawn_starting_bosses).after(setup_scene))
        .add_systems(Update, (active_character_movement, autoshot_ability, move_projectiles))
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

/// Component for projectiles with target and movement data
#[derive(Component)]
struct Projectile {
    start_position: Vec3,
    destination: Vec3,
    elapsed_time: f32,
    total_time: f32,  // Total time for the journey
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
fn spawn_starting_hero(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Single<Entity, (With<Arena>, With<Active>)>,
) {
    let arena_entity = query.into_inner();
    let blue_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.153, 0.431, 0.945), // #276EF1
        metallic: 0.0, // Non-metallic
        perceptual_roughness: 1.0, // Maximum roughness
        ..default()
    });
    let sphere_radius = 8.0; // Slightly smaller than half tile size (9.5) for visual spacing
    let sphere_mesh = meshes.add(Sphere::new(sphere_radius));
    let local_position = get_local_tile_space(35, 15);
    commands.entity(arena_entity).with_child((
        Character,
        AutoShot,
        Active,
        Mesh3d(sphere_mesh),
        MeshMaterial3d(blue_material),
        Transform::from_translation(local_position),
    ));
}

fn spawn_starting_bosses(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &arena::ArenaId), With<Arena>>,
) {
    for (arena_entity, arena_id) in query.iter() {
        // Spawn a boss in each arena
        println!("Spawning boss in arena {:?}", arena_id);

        // Example: spawn a red sphere boss in each arena
        let red_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.945, 0.153, 0.153), // Red color
            metallic: 0.0,
            perceptual_roughness: 1.0,
            ..default()
        });

        let boss_radius = 32.0; // Slightly larger than hero
        let boss_mesh = meshes.add(Sphere::new(boss_radius));

        // Place boss at a different position in each arena
        let local_position = get_local_tile_space(32, 10);
        if(arena_id.0 == 1){
            commands.entity(arena_entity).with_child((
                Boss,
                Active,
                Mesh3d(boss_mesh),
                MeshMaterial3d(red_material),
                Transform::from_translation(local_position),
            ));
        } else {
            commands.entity(arena_entity).with_child((
                Boss,

                Mesh3d(boss_mesh),
                MeshMaterial3d(red_material),
                Transform::from_translation(local_position),
            ));
        }

    }
}

fn active_character_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Single<&mut Transform, (With<Active>, With<Character>)>,
) {
    let mut transform = query.into_inner();

    let mut movement = Vec3::ZERO;

    // WASD movement - one tile at a time
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        movement.y += TILE_SIZE; // Move up (positive Y)
    }
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        movement.y -= TILE_SIZE; // Move down (negative Y)
    }
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        movement.x -= TILE_SIZE; // Move left (negative X)
    }
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        movement.x += TILE_SIZE; // Move right (positive X)
    }

    // Apply movement
    if movement != Vec3::ZERO {
        transform.translation += movement;
        println!("Character moved to: {:?}", transform.translation);
    }
}

/// System to move projectiles using lerp
fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Transform, &mut Projectile)>,
) {
    for (entity, mut transform, mut projectile) in projectiles.iter_mut() {
        // Update elapsed time
        projectile.elapsed_time += time.delta_secs();
        
        // Calculate lerp progress (0.0 to 1.0)
        let progress = (projectile.elapsed_time / projectile.total_time).clamp(0.0, 1.0);
        
        // Lerp between start and destination
        transform.translation = projectile.start_position.lerp(projectile.destination, progress);
        
        // Despawn when reached destination
        if progress >= 1.0 {
            commands.entity(entity).despawn();
            println!("Projectile reached destination and despawned");
        }
    }
}

fn autoshot_ability(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    mut timer: Local<Timer>,
    arena: Single<Entity, (With<Arena>, With<Active>)>,
    character: Single<&Transform, (With<Character>, With<AutoShot>, With<Active>)>,
    boss: Single<&Transform, (With<Boss>, With<Active>)>,
) {
    if timer.duration().as_secs_f32() == 0.0 {
        *timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    }

    let char_pos  = character.into_inner();
    let a  = arena.into_inner();
    let b  = boss.into_inner();
    let black_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.1),
        metallic: 0.0,
        perceptual_roughness: 1.0,
        ..default()
    });
    timer.tick(time.delta());

    // Only fire if timer is finished
    if timer.just_finished() {
        let projectile_radius = 2.5;
        let projectile_mesh = meshes.add(Sphere::new(projectile_radius));;

        
        // Calculate distance and time (1 TILE_SIZE per second)
        let distance = char_pos.translation.distance(b.translation);
        let travel_time = distance / TILE_SIZE; // seconds
        
        commands.entity(a).with_child((

            Mesh3d(projectile_mesh),
            MeshMaterial3d(black_material),
            Projectile {
                start_position: char_pos.translation,
                destination: b.translation,
                elapsed_time: 0.0,
                total_time: travel_time,
            },
        ));
        println!("Fired projectile from {:?} to {:?}, travel time: {:.2}s", char_pos.translation, b.translation, travel_time);
    }
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
