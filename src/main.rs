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
mod recording;
mod selectors;
mod timeline;

use crate::ability::{AutoShot, HolyNova};
use crate::ability::{
    auto_shot_ability, holy_nova_ability, move_projectiles, update_holy_nova_vfx,
};
use crate::arena::{
    ARENA_HEIGHT, ARENA_WIDTH, Arena, ArenaEntities, ArenaId, ArenaName, CameraUpdate,
    CharacterMoved, CurrentArena, DEBUG_COLORS, GRID_HEIGHT, GRID_WIDTH, LastActiveHero, TILE_SIZE,
    TOTAL_ARENAS, arena_update, decrement_current_arena, get_local_tile_space,
    handle_character_moved, increment_current_arena,
};
use crate::arena_camera::{draw_arena_border, setup_camera, toggle_camera_zoom};
use crate::audio::Audio;
use crate::battleground::BattleGround;
use crate::character::{Boss, Character, move_active_character, toggle_active_character};
use crate::class_type::ClassType;
use crate::lights::spawn_lights;
use crate::materials::Materials;
use crate::recording::RecordingPlugin;
use crate::selectors::Active;

use crate::timeline::{TimelineClock, TimelinePlugin};
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
        // Register custom events
        .add_event::<CameraUpdate>()
        .add_event::<CharacterMoved>()
        .add_systems(
            Startup,
            (
                setup_scene,
                spawn_lights,
                setup_camera,
                spawn_starting_bosses,
                spawn_starting_hero,
                // spawn_starting_hero_v2,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                toggle_camera_zoom,
                toggle_active_character,
                increment_current_arena,
                decrement_current_arena,
                arena_update,
                handle_character_moved,
                move_active_character,
                draw_arena_border,
            ),
        )
        .add_systems(
            Update,
            (
                auto_shot_ability,
                move_projectiles,
                holy_nova_ability,
                update_holy_nova_vfx,
            ),
        )
        .add_plugins(TimelinePlugin)
        .add_plugins(RecordingPlugin)
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
    commands.insert_resource(CurrentArena(ArenaId::new(ArenaName::GuildHouse))); // Arena index 1
    let tile_mesh = meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE));
    commands.spawn(Debug);

    // Collect arena entities for O(1) lookup resource
    let mut arena_entity_pairs = Vec::new();

    commands
        .spawn((
            BattleGround,
            Transform::default(),
            InheritedVisibility::default(),
        ))
        .with_children(|battleground| {
            for arena_index in 0..TOTAL_ARENAS {
                let debug_material = materials.add(StandardMaterial {
                    base_color: DEBUG_COLORS[arena_index as usize].clone(),
                    metallic: 0.0,
                    perceptual_roughness: 1.0,
                    ..default()
                });
                let offset_x = ((arena_index % 3) as f32) * ARENA_WIDTH;
                let offset_y = -((arena_index / 3) as f32) * ARENA_HEIGHT;
                let class_type = ClassType::index_of(arena_index);
                let arena_name = ClassType::index_of(arena_index).name();
                let arena_name_enum = ArenaName::from_index_safe(arena_index);

                let arena_entity = battleground
                    .spawn((
                        Transform::from_xyz(offset_x, offset_y, 0.0),
                        Arena::from_index_safe(arena_index),
                        InheritedVisibility::default(),
                        TimelineClock::default(),
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
                    })
                    .id();

                arena_entity_pairs.push((arena_name_enum, arena_entity));
            }
        });

    // Convert Vec to array - compile-time guaranteed to have exactly 9 elements
    let arena_entities_array: [(ArenaName, Entity); 9] = arena_entity_pairs
        .try_into()
        .expect("Arena spawn must create exactly 9 arenas");

    // Insert O(1) arena lookup resource
    commands.insert_resource(ArenaEntities::new(arena_entities_array));
}

fn spawn_starting_hero(
    mut commands: Commands,
    mats: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    current_arena: Res<CurrentArena>,
    arena_entities: Res<ArenaEntities>,
) {
    // O(1) lookup for current arena entity
    let arena_entity = arena_entities.get(current_arena.name());

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
    let sphere_radius_v2 = 0.125;
    let sphere_mesh_v2 = meshes.add(Sphere::new(sphere_radius_v2));
    let local_position_v2 = get_local_tile_space(0.0, 0.0, 0.125);
    commands.entity(arena_entity).with_child((
        Character,
        HolyNova,
        Mesh3d(sphere_mesh_v2),
        MeshMaterial3d(mats.gray.clone()),
        Transform::from_translation(local_position_v2),
    ));
    println!("Character entity ID: {}", character_entity);
    // Update the arena's LastActiveHero to point to this character
    commands
        .entity(arena_entity)
        .insert(LastActiveHero(Some(character_entity)));
}

fn spawn_starting_bosses(
    mut commands: Commands,
    mats: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    arena_entities: Res<ArenaEntities>,
) {
    // Spawn boss in GuildHouse only - O(1) lookup
    let guildhouse_entity = arena_entities.get(ArenaName::GuildHouse);
    let boss_radius = 0.125 * 4.0;
    let boss_mesh = meshes.add(Sphere::new(boss_radius));
    let local_position = get_local_tile_space(32.0, 10.0, boss_radius);

    commands.entity(guildhouse_entity).with_child((
        Boss,
        Active,
        Mesh3d(boss_mesh.clone()),
        MeshMaterial3d(mats.red.clone()),
        Transform::from_translation(local_position),
    ));

    // Spawn regular (inactive) bosses in all other arenas
    for arena_name in ArenaName::ALL_ARENAS {
        if arena_name != ArenaName::GuildHouse {
            let arena_entity = arena_entities.get(arena_name);
            let boss_mesh = meshes.add(Sphere::new(boss_radius));
            commands.entity(arena_entity).with_child((
                Boss,
                Mesh3d(boss_mesh),
                MeshMaterial3d(mats.red.clone()),
                Transform::from_translation(local_position),
            ));
        }
    }
}
