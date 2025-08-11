mod arena;
mod arena_camera;
mod audio;
mod battleground;
mod lights;

// Uncomment these modules to debug pink material issues
mod ability;
mod character;
mod class_type;
mod materials;
mod selectors;

use crate::ability::{
    auto_shot_ability, holy_nova_ability, move_projectiles, update_holy_nova_vfx, AutoShot,
    HolyNova,
};
use crate::arena::{
    decrement_current_arena, get_local_tile_space, increment_current_arena, toggle_active_arena, Arena, CurrentArena, LastActiveHero,
    ARENA_HEIGHT, ARENA_WIDTH, DEBUG_COLORS, GRID_HEIGHT, GRID_WIDTH,
    TILE_SIZE, TOTAL_ARENAS,
};
use crate::arena_camera::{draw_arena_border, move_camera, setup_camera, toggle_camera_zoom};
use crate::audio::Audio;
use crate::battleground::BattleGround;
use crate::character::{active_character_selection, toggle_active_character, Boss, Character};
use crate::class_type::ClassType;
use crate::lights::spawn_lights;
use crate::materials::Materials;
use crate::selectors::Active;

use bevy::prelude::*;
use bevy::window::WindowResolution;

const GAME_NAME: &str = "Arenic";

// Game state enum for managing different game phases
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Title,
}

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
        // Initialize game state
        .init_state::<GameState>()
        .add_systems(
            Startup,
            (
                setup_scene,
                spawn_lights,
                setup_camera,
                toggle_active_arena,
                spawn_starting_hero,
                spawn_starting_hero_v2,
                spawn_starting_bosses,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                toggle_camera_zoom,
                increment_current_arena,
                decrement_current_arena,
                toggle_active_arena,
                move_camera,
                draw_arena_border,
                active_character_movement,
                active_character_selection,
                toggle_active_character,
                auto_shot_ability,
                move_projectiles,
                holy_nova_ability,
                update_holy_nova_vfx,
            ),
        )
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
    commands.insert_resource(Materials::new(&mut materials));
    commands.insert_resource(Audio::new(&asset_server));
    let tile_mesh = meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE));
    commands.spawn(Debug);

    commands
        .spawn((
            BattleGround,
            Transform::default(),
            InheritedVisibility::default(),
            CurrentArena(1),
        ))
        .with_children(|battleground| {
            for arena_index in 0..TOTAL_ARENAS {
                let debug_material = materials.add(StandardMaterial {
                    base_color: DEBUG_COLORS[arena_index as usize],
                    metallic: 0.0,
                    perceptual_roughness: 1.0,
                    ..default()
                });
                let offset_x = ((arena_index % 3) as f32) * ARENA_WIDTH;
                let offset_y = -((arena_index / 3) as f32) * ARENA_HEIGHT;
                let class_type = ClassType::index_of(arena_index);
                let arena_name = ClassType::index_of(arena_index).name();

                battleground
                    .spawn((
                        Transform::from_xyz(offset_x, offset_y, 0.0),
                        Arena(arena_index),
                        InheritedVisibility::default(),
                        class_type,
                        Name::new(arena_name),
                        LastActiveHero(None),
                    ))
                    .with_children(|arena| {
                        for x in 0..GRID_WIDTH {
                            for y in 0..GRID_HEIGHT {
                                arena.spawn((
                                    Transform::from_xyz(
                                        x as f32 * TILE_SIZE,
                                        y as f32 * TILE_SIZE,
                                        0.0,
                                    ),
                                    Mesh3d(tile_mesh.clone()),
                                    MeshMaterial3d(debug_material.clone()),
                                ));
                            }
                        }
                    });
            }
        });
}

fn spawn_starting_hero(
    mut commands: Commands,
    mats: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Single<Entity, (With<Arena>, With<Active>)>,
) {
    let arena_entity = query.into_inner();
    let sphere_radius = 0.125;
    let sphere_mesh = meshes.add(Sphere::new(sphere_radius));
    let local_position = get_local_tile_space(36.0, 15.0, 0.125);

    // Spawn the character as a child and get its entity ID
    let character_entity = commands
        .spawn((
            Character,
            AutoShot::new(16.0),
            Active,
            Mesh3d(sphere_mesh),
            MeshMaterial3d(mats.blue.clone()),
            Transform::from_translation(local_position),
            ChildOf(arena_entity),
        ))
        .id();

    // Update the arena's LastActiveHero to point to this character
    commands
        .entity(arena_entity)
        .insert(LastActiveHero(Some(character_entity)));
}

fn spawn_starting_hero_v2(
    mut commands: Commands,
    mats: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Single<Entity, (With<Arena>, With<Active>)>,
) {
    let arena_entity = query.into_inner();
    let sphere_radius = 0.125;
    let sphere_mesh = meshes.add(Sphere::new(sphere_radius));
    let local_position = get_local_tile_space(0.0, 0.0, 0.125);

    commands.entity(arena_entity).with_child((
        Character,
        HolyNova,
        Mesh3d(sphere_mesh),
        MeshMaterial3d(mats.gray.clone()),
        Transform::from_translation(local_position),
    ));
}

fn spawn_starting_bosses(
    mut commands: Commands,
    mats: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Arena), With<Arena>>,
) {
    for (arena_entity, arena_id) in query.iter() {
        let boss_radius = 0.125 * 4.0;
        let boss_mesh = meshes.add(Sphere::new(boss_radius));

        let local_position = get_local_tile_space(32.0, 10.0, boss_radius);
        if arena_id.0 == 1 {
            commands.entity(arena_entity).with_child((
                Boss,
                Active,
                Mesh3d(boss_mesh),
                MeshMaterial3d(mats.red.clone()),
                Transform::from_translation(local_position),
            ));
        } else {
            commands.entity(arena_entity).with_child((
                Boss,
                Mesh3d(boss_mesh),
                MeshMaterial3d(mats.red.clone()),
                Transform::from_translation(local_position),
            ));
        }
    }
}

fn active_character_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    active_arena: Query<&Children, (With<Arena>, With<Active>)>,
    mut character_query: Query<&mut Transform, (With<Character>, With<Active>)>,
) {
    // Get the children of the active arena
    let Ok(arena_children) = active_arena.single() else {
        return; // No active arena found
    };

    // Find the active character in this arena's children
    let mut characters = character_query.iter_many_mut(arena_children);

    // There should only be one active character in the active arena
    let Some(mut transform) = characters.fetch_next() else {
        return; // No active character in the active arena
    };

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
