use crate::arena::{CameraUpdate, LastActiveHero, GRID_HEIGHT, GRID_WIDTH, TILE_SIZE};
use crate::arena_camera::{position_camera_for_arena, ZoomOut, ZOOM};
use crate::character::Character;
use crate::selectors::Active;
use bevy::prelude::*;

/// Marker component identifying an arena entity in the world.
/// Each arena entity should have this component along with ArenaId.
#[derive(Component, Debug)]
pub struct Arena(pub u8);

/// Marker component for arena tile entities.
#[derive(Component, Debug)]
pub struct ArenaTile;

#[derive(Component, Debug, Clone)]
pub struct CurrentArena(pub u8);

impl CurrentArena {
    /// Increment arena index cyclically (0-8)
    pub fn increment(value: u8) -> u8 {
        (value + 1) % 9
    }

    /// Decrement arena index cyclically (0-8)
    pub fn decrement(value: u8) -> u8 {
        if value == 0 { 8 } else { value - 1 }
    }

    /// Go to specific arena index (0-8), clamps invalid values
    pub fn go_to(value: u8) -> u8 {
        if value > 8 { 8 } else { value }
    }
}

pub fn decrement_current_arena(
    keycode: Res<ButtonInput<KeyCode>>,
    current_arena_q: Single<&mut CurrentArena>,
    mut arena_refresh_event: EventWriter<CameraUpdate>,
) {
    if keycode.just_pressed(KeyCode::BracketLeft) {
        let mut current_arena = current_arena_q.into_inner();
        current_arena.0 = CurrentArena::decrement(current_arena.0);

        // Send event
        arena_refresh_event.write(CameraUpdate);
    }
}

pub fn increment_current_arena(
    keycode: Res<ButtonInput<KeyCode>>,
    current_arena_q: Single<&mut CurrentArena>,
    mut arena_refresh_event: EventWriter<CameraUpdate>,
) {
    if keycode.just_pressed(KeyCode::BracketRight) {
        let mut current_arena = current_arena_q.into_inner();
        current_arena.0 = CurrentArena::increment(current_arena.0);

        // Send event
        arena_refresh_event.write(CameraUpdate);
    }
}

pub fn arena_update(
    mut commands: Commands,
    mut arena_refresh_events: EventReader<CameraUpdate>,
    current_arena_q: Single<&CurrentArena>,
    camera: Single<(Entity, &mut Transform, Option<&ZoomOut>), With<Camera3d>>,
    arena_q: Query<(Entity, &Arena, &Children, Option<&LastActiveHero>), With<Arena>>,
    characters_q: Query<(Entity, Option<&Active>), With<Character>>,
) {
    // Only run when CameraUpdate event is triggered
    if arena_refresh_events.is_empty() {
        return;
    }
    arena_refresh_events.clear();
    let current_arena = current_arena_q.into_inner();
    let (camera_entity, mut camera_transform, zoom) = camera.into_inner();
    if zoom.is_some() {
        for (entity, active) in characters_q.iter() {
            if active.is_some() {
                commands.entity(entity).remove::<Active>();
            }
        }
    } else {
        let current_arena_index = current_arena.0;
        position_camera_for_arena(&mut camera_transform, current_arena_index, ZOOM.0);
        commands.entity(camera_entity).remove::<ZoomOut>();

        for (arena_entity, arena, children, last_active_hero) in arena_q.iter() {
            if arena.0 == current_arena_index {
                // Check if arena has characters
                let characters_data: Vec<(Entity, Option<&Active>)> =
                    characters_q.iter_many(children).collect();

                if characters_data.is_empty() {
                    println!("No characters in arena {}", arena.0);
                    for (entity, active) in characters_q.iter() {
                        if active.is_some() {
                            commands.entity(entity).remove::<Active>();
                        }
                    }
                    return;
                } else {
                    println!(
                        "Found {} characters in arena {}",
                        characters_data.len(),
                        arena.0
                    );

                    // Find the active character (if any)
                    let active_character = characters_data
                        .iter()
                        .find(|(_, active)| active.is_some())
                        .map(|(entity, _)| *entity);

                    if let Some(active_entity) = active_character {
                        println!(
                            "Found active character in arena {}: {:?}",
                            arena.0, active_entity
                        );
                        // This character is already active - handle this case
                        return;
                    } else {
                        println!("No active character in arena {}", arena.0);

                        // Check if LastActiveHero has a character present and still exists
                        let use_last_active_hero =
                            last_active_hero
                                .and_then(|hero| hero.0)
                                .filter(|&hero_entity| {
                                    characters_data
                                        .iter()
                                        .any(|(entity, _)| *entity == hero_entity)
                                });

                        if let Some(hero_entity) = use_last_active_hero {
                            println!("Using LastActiveHero: {:?}", hero_entity);
                            // Remove Active from all currently active characters across all arenas
                            for (entity, active) in characters_q.iter() {
                                if active.is_some() {
                                    commands.entity(entity).remove::<Active>();
                                }
                            }
                            // Now set the new active character
                            commands.entity(hero_entity).insert(Active);
                        } else {
                            println!("Using first character");
                            let first_character = characters_data[0].0;
                            // Remove Active from all currently active characters across all arenas
                            for (entity, active) in characters_q.iter() {
                                if active.is_some() {
                                    commands.entity(entity).remove::<Active>();
                                }
                            }
                            // Now set the new active character
                            commands.entity(first_character).insert(Active);
                            commands
                                .entity(arena_entity)
                                .insert(LastActiveHero(Some(first_character)));
                        }
                    }
                }
            }
        }
    }
}

pub fn move_active_character(
    mut commands: Commands,
    keycode: Res<ButtonInput<KeyCode>>,
    mut current_arena_q: Single<&mut CurrentArena>,
    mut active_character_q: Single<(Entity, &mut Transform), (With<Character>, With<Active>)>,
    arena_q: Query<(Entity, &Arena), With<Arena>>,
    mut arena_refresh_event: EventWriter<CameraUpdate>,
) {
    let mut movement = Vec3::ZERO;
    if keycode.just_pressed(KeyCode::KeyW) {
        movement.y += TILE_SIZE;
    }
    if keycode.just_pressed(KeyCode::KeyS) {
        movement.y -= TILE_SIZE;
    }
    if keycode.just_pressed(KeyCode::KeyA) {
        movement.x -= TILE_SIZE;
    }
    if keycode.just_pressed(KeyCode::KeyD) {
        movement.x += TILE_SIZE;
    }
    if movement == Vec3::ZERO {
        return;
    }

    let mut current_arena = current_arena_q.into_inner();
    let (character_entity, mut character_transform) = active_character_q.into_inner();

    // Calculate new position
    let new_position = character_transform.translation + movement;

    // Arena boundaries (in local space)
    let min_x = 0.0;
    let max_x = (GRID_WIDTH - 1) as f32 * TILE_SIZE;
    let min_y = 0.0;
    let max_y = (GRID_HEIGHT - 1) as f32 * TILE_SIZE;

    // Arena grid layout (3x3):
    // 0 1 2
    // 3 4 5
    // 6 7 8

    let current_arena_index = current_arena.0;
    let col = current_arena_index % 3;
    let row = current_arena_index / 3;

    // Check boundaries and handle transitions
    if new_position.x < min_x {
        // Moving left out of bounds
        if col > 0 {
            // Can move to arena on the left
            let new_arena_index = current_arena_index - 1;
            current_arena.0 = new_arena_index;

            // Teleport character to right side of new arena
            character_transform.translation.x = max_x;

            // Reparent character to new arena
            if let Some((new_arena_entity, _)) =
                arena_q.iter().find(|(_, arena)| arena.0 == new_arena_index)
            {
                commands
                    .entity(character_entity)
                    .insert(ChildOf(new_arena_entity));
            }
            println!("Moved to arena {} (left)", new_arena_index);
            // Send arena refresh event
            arena_refresh_event.write(CameraUpdate);
        } else {
            println!("Cannot move left - at battleground boundary");
            return; // Prevent movement
        }
    } else if new_position.x > max_x {
        // Moving right out of bounds
        if col < 2 {
            // Can move to arena on the right
            let new_arena_index = current_arena_index + 1;
            current_arena.0 = new_arena_index;

            // Teleport character to left side of new arena
            character_transform.translation.x = min_x;

            // Reparent character to new arena
            if let Some((new_arena_entity, _)) =
                arena_q.iter().find(|(_, arena)| arena.0 == new_arena_index)
            {
                commands
                    .entity(character_entity)
                    .insert(ChildOf(new_arena_entity));
            }
            println!("Moved to arena {} (right)", new_arena_index);
            // Send arena refresh event
            arena_refresh_event.write(CameraUpdate);
        } else {
            println!("Cannot move right - at battleground boundary");
            return; // Prevent movement
        }
    } else if new_position.y < min_y {
        // Moving down out of bounds
        if row < 2 {
            // Can move to arena below
            let new_arena_index = current_arena_index + 3;
            current_arena.0 = new_arena_index;

            // Teleport character to top side of new arena
            character_transform.translation.y = max_y;

            // Reparent character to new arena
            if let Some((new_arena_entity, _)) =
                arena_q.iter().find(|(_, arena)| arena.0 == new_arena_index)
            {
                commands
                    .entity(character_entity)
                    .insert(ChildOf(new_arena_entity));
            }
            println!("Moved to arena {} (down)", new_arena_index);
            // Send arena refresh event
            arena_refresh_event.write(CameraUpdate);
        } else {
            println!("Cannot move down - at battleground boundary");
            return; // Prevent movement
        }
    } else if new_position.y > max_y {
        // Moving up out of bounds
        if row > 0 {
            // Can move to arena above
            let new_arena_index = current_arena_index - 3;
            current_arena.0 = new_arena_index;

            // Teleport character to bottom side of new arena
            character_transform.translation.y = min_y;

            // Reparent character to new arena
            if let Some((new_arena_entity, _)) =
                arena_q.iter().find(|(_, arena)| arena.0 == new_arena_index)
            {
                commands
                    .entity(character_entity)
                    .insert(ChildOf(new_arena_entity));
            }
            println!("Moved to arena {} (up)", new_arena_index);
            // Send arena refresh event
            arena_refresh_event.write(CameraUpdate);
        } else {
            println!("Cannot move up - at battleground boundary");
            return; // Prevent movement
        }
    } else {
        // Normal movement within arena bounds
        character_transform.translation = new_position;
    }

    println!(
        "Character at: {:?} in arena {}",
        character_transform.translation, current_arena.0
    );
}
