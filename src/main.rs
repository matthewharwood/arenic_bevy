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
// mod recording;
mod recording;
mod selectors;
mod timeline;
mod ui;

// Standard library and external crates
use bevy::prelude::*;
use bevy::window::WindowResolution;

// Local crate modules - abilities
use crate::ability::{
    AcidFlask, AutoShot, Bash, Block, Dance, HolyNova, Ironskin, Mimic, Siphon, Transmute,
    auto_shot_ability, holy_nova_ability, move_projectiles, update_holy_nova_vfx,
};

// Local crate modules - arena system
use crate::arena::{
    ARENA_HEIGHT, ARENA_WIDTH, Arena, ArenaEntities, ArenaName, CameraUpdate, CharacterMoved,
    CurrentArena, CurrentArenaEntity, DEBUG_COLORS, GRID_HEIGHT, GRID_WIDTH, LastActiveHero,
    TILE_SIZE, TOTAL_ARENAS, arena_update, decrement_current_arena, get_local_tile_space,
    handle_character_moved, increment_current_arena,
};
use crate::arena_camera::{draw_arena_border, setup_camera, toggle_camera_zoom};

// Local crate modules - core systems
use crate::audio::Audio;
use crate::battleground::BattleGround;
use crate::character::{Boss, Character, move_active_character, toggle_active_character};
use crate::class_type::ClassType;
use crate::lights::spawn_lights;
use crate::materials::Materials;
use crate::selectors::Active;
use crate::timeline::{TimelineClock, TimelineManager, TimelinePlugin};

// Fix for web audio and asset loading
#[cfg(target_arch = "wasm32")]
use bevy::asset::{AssetMetaCheck, AssetPlugin};

const GAME_NAME: &str = "Arenic";

// Game state enum for managing different game phases
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Title,
}

fn main() {
    // Configure plugins differently for web vs. native
    #[cfg(target_arch = "wasm32")]
    let default_plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: GAME_NAME.to_string(),
                resolution: WindowResolution::new(1280.0, 720.0),
                ..default()
            }),
            ..default()
        })
        .set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        });

    #[cfg(not(target_arch = "wasm32"))]
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: GAME_NAME.to_string(),
            resolution: WindowResolution::new(1280.0, 720.0),
            ..default()
        }),
        ..default()
    });

    App::new()
        .add_plugins(default_plugins)
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
                spawn_labyrinth_characters,
                spawn_bastion_characters,
                mark_timeline_ghosts,
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
        .add_plugins(recording::RecordingPlugin)
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
    commands.insert_resource(CurrentArena(ArenaName::GuildHouse)); // Arena index 1
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
                        Arena(ArenaName::from_index_safe(arena_index)),
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
    current: CurrentArenaEntity,
) {
    // O(1) lookup for current arena entity
    let arena_entity = current.get();

    let sphere_radius = 0.125;
    let sphere_mesh = meshes.add(Sphere::new(sphere_radius));
    let local_position = get_local_tile_space(36.0, 15.0, 0.125);

    // Spawn the character as a child and get its entity ID
    let character_entity = commands
        .spawn((
            Character,
            ClassType::Hunter,
            AutoShot::new(16.0),
            Active,
            Mesh3d(sphere_mesh),
            MeshMaterial3d(mats.blue.clone()),
            Transform::from_translation(local_position),
            ChildOf(arena_entity),
            TimelineManager::new(),
            Name::new("Dean"),
        ))
        .id();
    let sphere_radius_v2 = 0.125;
    let sphere_mesh_v2 = meshes.add(Sphere::new(sphere_radius_v2));
    let local_position_v2 = get_local_tile_space(0.0, 0.0, 0.125);
    commands.spawn((
        Character,
        ClassType::Cardinal,
        HolyNova::new(),
        Mesh3d(sphere_mesh_v2),
        MeshMaterial3d(mats.gray.clone()),
        Transform::from_translation(local_position_v2),
        ChildOf(arena_entity),
        TimelineManager::new(),
        Name::new("Matthew"),
    ));
    println!("Character entity ID: {}", character_entity);
    // Update the arena's LastActiveHero to point to this character
    commands
        .entity(arena_entity)
        .insert(LastActiveHero(Some(character_entity)));
}

// TODO DELETE LATER
/// Spawn a Warrior and Bard in the Labyrinth arena on startup
fn spawn_labyrinth_characters(
    mut commands: Commands,
    mats: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    arena_entities: Res<ArenaEntities>,
) {
    use crate::ability::{AbilityType, BardAbility, WarriorAbility};
    use crate::timeline::{
        DraftTimeline, EventType, PublishTimeline, TimeStamp, TimelineEvent, TimelineManager,
    };

    // Get the Labyrinth arena entity (index 0)
    let labyrinth_entity = arena_entities.get(ArenaName::Labyrinth);

    let character_radius = 0.125;
    let character_mesh = meshes.add(Sphere::new(character_radius));

    // Create a purple material for the Warrior
    let purple_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.0, 0.8), // Purple
        metallic: 0.2,
        perceptual_roughness: 0.5,
        ..default()
    });

    // Create Warrior timeline with movements and abilities
    let mut warrior_draft = DraftTimeline::new();
    // Movement events - warrior patrols and attacks
    warrior_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(5.0),
            event_type: EventType::Movement(Vec3::new(1.0, 0.0, 0.0)), // Move right
        })
        .ok();
    warrior_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Ability(AbilityType::Warrior(WarriorAbility::Bash), None), // Warrior uses Bash
        })
        .ok();
    warrior_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(15.0),
            event_type: EventType::Movement(Vec3::new(0.0, 1.0, 0.0)), // Move up
        })
        .ok();
    warrior_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(25.0),
            event_type: EventType::Ability(AbilityType::Warrior(WarriorAbility::Block), None), // Warrior uses Block
        })
        .ok();
    warrior_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(35.0),
            event_type: EventType::Movement(Vec3::new(-1.0, 0.0, 0.0)), // Move left
        })
        .ok();
    warrior_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(45.0),
            event_type: EventType::Ability(AbilityType::Warrior(WarriorAbility::Bash), None), // Another Bash
        })
        .ok();
    warrior_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(60.0),
            event_type: EventType::Movement(Vec3::new(0.0, -1.0, 0.0)), // Move down
        })
        .ok();
    warrior_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(75.0),
            event_type: EventType::Ability(AbilityType::Warrior(WarriorAbility::Block), None), // Block again
        })
        .ok();
    warrior_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(90.0),
            event_type: EventType::Movement(Vec3::new(1.0, 1.0, 0.0)), // Move diagonally
        })
        .ok();
    warrior_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(100.0),
            event_type: EventType::Ability(AbilityType::Warrior(WarriorAbility::Bash), None), // Final Bash
        })
        .ok();
    warrior_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(110.0),
            event_type: EventType::Movement(Vec3::new(-1.0, -1.0, 0.0)), // Return movement
        })
        .ok();

    let warrior_timeline = PublishTimeline::from_draft(warrior_draft);
    let mut warrior_timeline_manager = TimelineManager::new();
    warrior_timeline_manager.set_timeline(ArenaName::Labyrinth, warrior_timeline);

    // Spawn Warrior at position (20, 15)
    let warrior_position = get_local_tile_space(20.0, 15.0, character_radius);
    commands.entity(labyrinth_entity).with_child((
        Character,
        ClassType::Warrior,
        // Warrior abilities - instantiate the components
        Bash::new(),
        Block::new(),
        warrior_timeline_manager,
        Mesh3d(character_mesh.clone()),
        MeshMaterial3d(purple_material),
        Transform::from_translation(warrior_position),
        Name::new("Warrior"),
    ));

    // Create Bard timeline with movements and abilities
    let mut bard_draft = DraftTimeline::new();
    // Movement events - bard dances around and uses abilities
    bard_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(3.0),
            event_type: EventType::Movement(Vec3::new(0.0, 1.0, 0.0)), // Move up
        })
        .ok();
    bard_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(8.0),
            event_type: EventType::Ability(AbilityType::Bard(BardAbility::Dance), None), // Bard uses Dance
        })
        .ok();
    bard_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(12.0),
            event_type: EventType::Movement(Vec3::new(1.0, 0.0, 0.0)), // Move right
        })
        .ok();
    bard_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(20.0),
            event_type: EventType::Ability(AbilityType::Bard(BardAbility::Mimic), None), // Bard uses Mimic
        })
        .ok();
    bard_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(30.0),
            event_type: EventType::Movement(Vec3::new(-1.0, 1.0, 0.0)), // Move diagonally
        })
        .ok();
    bard_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(40.0),
            event_type: EventType::Ability(AbilityType::Bard(BardAbility::Dance), None), // Dance again
        })
        .ok();
    bard_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(50.0),
            event_type: EventType::Movement(Vec3::new(0.0, -1.0, 0.0)), // Move down
        })
        .ok();
    bard_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(65.0),
            event_type: EventType::Ability(AbilityType::Bard(BardAbility::Mimic), None), // Mimic again
        })
        .ok();
    bard_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(80.0),
            event_type: EventType::Movement(Vec3::new(1.0, -1.0, 0.0)), // Move diagonally
        })
        .ok();
    bard_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(95.0),
            event_type: EventType::Ability(AbilityType::Bard(BardAbility::Dance), None), // Final Dance
        })
        .ok();
    bard_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(105.0),
            event_type: EventType::Movement(Vec3::new(-1.0, 0.0, 0.0)), // Move left
        })
        .ok();
    bard_draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(115.0),
            event_type: EventType::Ability(AbilityType::Bard(BardAbility::Mimic), None), // Final Mimic
        })
        .ok();

    let bard_timeline = PublishTimeline::from_draft(bard_draft);
    let mut bard_timeline_manager = TimelineManager::new();
    bard_timeline_manager.set_timeline(ArenaName::Labyrinth, bard_timeline);

    // Spawn Bard at position (40, 15)
    let bard_position = get_local_tile_space(40.0, 15.0, character_radius);
    commands.entity(labyrinth_entity).with_child((
        Character,
        ClassType::Bard,
        // Bard abilities - instantiate the components
        Dance::new(),
        Mimic::new(),
        bard_timeline_manager,
        Mesh3d(character_mesh.clone()),
        MeshMaterial3d(mats.yellow.clone()), // Yellow for Bard
        Transform::from_translation(bard_position),
        Name::new("Bard"),
    ));

    info!("Spawned Warrior and Bard characters with timelines in Labyrinth arena");
}

/// Spawn an Alchemist in the Bastion arena on startup
fn spawn_bastion_characters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    arena_entities: Res<ArenaEntities>,
) {
    use crate::timeline::{DraftTimeline, PublishTimeline, TimelineManager};

    // Get the Bastion arena entity
    let bastion_entity = arena_entities.get(ArenaName::Bastion);

    let character_radius = 0.125;
    let character_mesh = meshes.add(Sphere::new(character_radius));

    // Create a green material for the Alchemist
    let green_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.8, 0.3), // Green
        metallic: 0.3,
        perceptual_roughness: 0.5,
        ..default()
    });

    // Create empty timeline for the Alchemist
    let alchemist_draft = DraftTimeline::new();
    let alchemist_timeline = PublishTimeline::from_draft(alchemist_draft);
    let mut alchemist_timeline_manager = TimelineManager::new();
    alchemist_timeline_manager.set_timeline(ArenaName::Bastion, alchemist_timeline);

    // Spawn Alchemist at position (30, 20) in the Bastion
    let alchemist_position = get_local_tile_space(30.0, 20.0, character_radius);
    commands.entity(bastion_entity).with_child((
        Character,
        ClassType::Alchemist,
        // All four Alchemist abilities
        AcidFlask::new(),
        Ironskin::new(),
        Siphon::new(),
        Transmute::new(),
        alchemist_timeline_manager,
        Mesh3d(character_mesh),
        MeshMaterial3d(green_material),
        Transform::from_translation(alchemist_position),
        Name::new("Zephyr"), // Random name for the Alchemist
    ));

    info!("Spawned Alchemist character with empty timeline in Bastion arena");
}

/// Mark characters that have published timelines as ghosts
fn mark_timeline_ghosts(
    mut commands: Commands,
    arena_q: Query<(Entity, &Children, &Arena), With<Arena>>,
    character_q: Query<(Entity, &TimelineManager), (With<Character>, Without<character::Ghost>)>,
) {
    use crate::character::Ghost;

    // Go through each arena
    for (_arena_entity, children, arena) in arena_q.iter() {
        let arena_name = arena.0;

        // Check each child character in this arena
        for child in children.iter() {
            if let Ok((character_entity, timeline_manager)) = character_q.get(child) {
                // Check if this character has a timeline for their parent arena
                if timeline_manager.has_recording_for(arena_name) {
                    // This character has a timeline for their arena, mark as ghost
                    commands.entity(character_entity).insert(Ghost);
                    info!("Marked character as Ghost in arena {:?}", arena_name);
                }
            }
        }
    }
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
