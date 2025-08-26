// Standard library and external crates
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{
    ChildOf, Children, Commands, Component, Entity, EventWriter, KeyCode, Query, Res, ResMut,
    Single, Transform, With,
};

// Local crate modules
use crate::arena::{
    Arena, ArenaEntities, ArenaName, CharacterMoved, CurrentArena, CurrentArenaEntity, GRID_HEIGHT,
    GRID_WIDTH, LastActiveHero, TILE_SIZE,
};
use crate::materials::Materials;
use crate::selectors::Active;
use crate::timeline::{DraftTimeline, GlobalTimelinePause, TimelineClock};

/// Marker component for character entities.
#[derive(Component, Debug)]
pub struct Character;

#[derive(Component, Debug)]
pub struct Boss;

#[derive(Component, Debug)]
pub struct Ghost;

pub fn toggle_active_character(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current: CurrentArenaEntity,
    arena_q: Query<(&Arena, &Children), With<Arena>>,
    characters_q: Query<(Entity, Option<&Active>), With<Character>>,
    mats: Res<Materials>,
) {
    if !keyboard_input.just_pressed(KeyCode::Tab) {
        return;
    }

    // O(1) lookup for current arena entity
    let current_arena_entity = current.get();

    // Direct query for the current arena - no iteration needed
    if let Ok((arena, children)) = arena_q.get(current_arena_entity) {
        // Get all characters in this arena
        let characters_data: Vec<(Entity, Option<&Active>)> =
            characters_q.iter_many(children).collect();

        // Need at least 2 characters to cycle
        if characters_data.len() < 2 {
            println!("Not enough characters to cycle in {}", arena);
            return;
        }

        // Find the currently active character's index
        let active_index = characters_data
            .iter()
            .position(|(_, active)| active.is_some());

        if let Some(current_index) = active_index {
            // Calculate next index (cyclical)
            let next_index = (current_index + 1) % characters_data.len();

            // Remove Active from the current character
            let current_entity = characters_data[current_index].0;
            commands
                .entity(current_entity)
                .insert(MeshMaterial3d(mats.gray.clone()))
                .remove::<Active>();

            // Add Active to the next character
            let next_entity = characters_data[next_index].0;
            commands
                .entity(next_entity)
                .insert(MeshMaterial3d(mats.blue.clone()))
                .insert(Active);

            // Update LastActiveHero
            commands
                .entity(current_arena_entity)
                .insert(LastActiveHero(Some(next_entity)));

            println!(
                "Cycled from character {:?} to {:?} in {}",
                current_entity, next_entity, arena
            );
        }
    }
}

pub fn move_active_character(
    mut commands: Commands,
    keycode: Res<ButtonInput<KeyCode>>,
    mut current_arena: ResMut<CurrentArena>,
    active_character_q: Single<(Entity, &mut Transform), (With<Character>, With<Active>)>,
    arena_entities: Res<ArenaEntities>,
    mut character_moved_event: EventWriter<CharacterMoved>,
    mut draft_timeline: ResMut<DraftTimeline>,
    arena_q: Query<(&Arena, &TimelineClock)>,
    global_pause: Res<GlobalTimelinePause>,
) {
    if global_pause.is_paused {
        return;
    }

    // Calculate grid direction directly from key presses
    let grid_direction = if keycode.just_pressed(KeyCode::KeyW) {
        Vec3::new(0.0, 1.0, 0.0) // Up
    } else if keycode.just_pressed(KeyCode::KeyS) {
        Vec3::new(0.0, -1.0, 0.0) // Down
    } else if keycode.just_pressed(KeyCode::KeyA) {
        Vec3::new(-1.0, 0.0, 0.0) // Left
    } else if keycode.just_pressed(KeyCode::KeyD) {
        Vec3::new(1.0, 0.0, 0.0) // Right
    } else {
        return;
    };

    let (character_entity, mut character_transform) = active_character_q.into_inner();

    // Calculate a new position (scale grid direction by TILE_SIZE)
    let new_position = character_transform.translation + grid_direction * TILE_SIZE;

    // Arena boundaries (in local space)
    let min_x = 0.0;
    let max_x = (GRID_WIDTH - 1) as f32 * TILE_SIZE;
    let min_y = 0.0;
    let max_y = (GRID_HEIGHT - 1) as f32 * TILE_SIZE;

    // Arena grid layout (3x3):
    // 0 1 2
    // 3 4 5
    // 6 7 8

    let current_arena_index = current_arena.0.as_u8();
    let col = current_arena_index % 3;
    let row = current_arena_index / 3;

    // Check boundaries and handle transitions
    if new_position.x < min_x {
        // Moving left out of bounds
        if col > 0 {
            // Can move to an arena on the left
            let from_arena = current_arena.0;
            let new_arena_index = current_arena_index - 1;
            let new_arena_name = ArenaName::from_index_safe(new_arena_index);

            // Teleport character to the right side of a new arena
            character_transform.translation.x = max_x;

            // Update CurrentArena after character movement
            current_arena.0 = new_arena_name;

            // Reparent character to new arena - O(1) lookup
            let new_arena_entity = arena_entities.get(new_arena_name);
            commands
                .entity(character_entity)
                .insert(ChildOf(new_arena_entity));
            println!("Moved to {} (left)", new_arena_name);
            // Send character moved event
            character_moved_event.write(CharacterMoved {
                character_entity,
                from_arena,
                to_arena: new_arena_name,
            });
        } else {
            println!("Cannot move left - at battleground boundary");
            return; // Prevent movement
        }
    } else if new_position.x > max_x {
        // Moving right out of bounds
        if col < 2 {
            // Can move to an arena on the right
            let from_arena = current_arena.0;
            let new_arena_index = current_arena_index + 1;
            let new_arena_name = ArenaName::from_index_safe(new_arena_index);

            // Teleport character to the left side of a new arena
            character_transform.translation.x = min_x;

            // Update CurrentArena after character movement
            current_arena.0 = new_arena_name;

            // Reparent character to new arena - O(1) lookup
            let new_arena_entity = arena_entities.get(new_arena_name);
            commands
                .entity(character_entity)
                .insert(ChildOf(new_arena_entity));
            println!("Moved to {} (right)", new_arena_name);
            // Send character moved event
            character_moved_event.write(CharacterMoved {
                character_entity,
                from_arena,
                to_arena: new_arena_name,
            });
        } else {
            println!("Cannot move right - at battleground boundary");
            return; // Prevent movement
        }
    } else if new_position.y < min_y {
        // Moving down out of bounds
        if row < 2 {
            // Can move to an arena below
            let from_arena = current_arena.0;
            let new_arena_index = current_arena_index + 3;
            let new_arena_name = ArenaName::from_index_safe(new_arena_index);

            // Teleport character to the top side of the new arena
            character_transform.translation.y = max_y;

            // Update CurrentArena after character movement
            current_arena.0 = new_arena_name;

            // Reparent character to new arena - O(1) lookup
            let new_arena_entity = arena_entities.get(new_arena_name);
            commands
                .entity(character_entity)
                .insert(ChildOf(new_arena_entity));
            println!("Moved to {} (down)", new_arena_name);
            // Send character moved event
            character_moved_event.write(CharacterMoved {
                character_entity,
                from_arena,
                to_arena: new_arena_name,
            });
        } else {
            println!("Cannot move down - at battleground boundary");
            return; // Prevent movement
        }
    } else if new_position.y > max_y {
        // Moving up out of bounds
        if row > 0 {
            // Can move to the arena above
            let from_arena = current_arena.0;
            let new_arena_index = current_arena_index - 3;
            let new_arena_name = ArenaName::from_index_safe(new_arena_index);

            // Teleport character to the bottom side of the new arena
            character_transform.translation.y = min_y;

            // Update CurrentArena after character movement
            current_arena.0 = new_arena_name;

            // Reparent character to new arena - O(1) lookup
            let new_arena_entity = arena_entities.get(new_arena_name);
            commands
                .entity(character_entity)
                .insert(ChildOf(new_arena_entity));
            println!("Moved to {} (up)", new_arena_name);
            // Send character moved event
            character_moved_event.write(CharacterMoved {
                character_entity,
                from_arena,
                to_arena: new_arena_name,
            });
        } else {
            println!("Cannot move up - at battleground boundary");
            return; // Prevent movement
        }
    } else {
        // Normal movement within arena bounds
        character_transform.translation = new_position;
    }

    println!(
        "Character at: {:?} in {}",
        character_transform.translation, current_arena.0
    );
}

#[cfg(test)]
mod tests;
