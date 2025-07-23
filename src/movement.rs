//! Character movement plugin.
//! 
//! This plugin handles all character movement functionality using an event-driven
//! architecture that separates input handling from movement processing.

use bevy::prelude::*;
use crate::components::{Character, CharacterSelected};
use crate::config::display::TILE_SIZE;
use crate::utils::clamp_to_grid_boundaries;

/// Movement direction for character movement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Event triggered when a character should move
#[derive(Event, Debug, Clone)]
pub struct CharacterMoveEvent {
    /// The entity that should move
    pub entity: Entity,
    /// The direction to move
    pub direction: MovementDirection,
}

impl CharacterMoveEvent {
    /// Create a new movement event
    pub fn new(entity: Entity, direction: MovementDirection) -> Self {
        Self {
            entity,
            direction,
        }
    }
}

/// Plugin that handles character movement using events
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CharacterMoveEvent>()
            .add_systems(Update, (
                handle_movement_input,
                process_movement_events,
            ).chain());
    }
}

/// Maps keyboard input to movement direction
fn get_movement_direction_from_input(input: &ButtonInput<KeyCode>) -> Option<MovementDirection> {
    if input.just_pressed(KeyCode::KeyW) {
        Some(MovementDirection::Up)
    } else if input.just_pressed(KeyCode::KeyS) {
        Some(MovementDirection::Down)
    } else if input.just_pressed(KeyCode::KeyA) {
        Some(MovementDirection::Left)
    } else if input.just_pressed(KeyCode::KeyD) {
        Some(MovementDirection::Right)
    } else {
        None
    }
}

/// System that handles input and emits movement events
pub fn handle_movement_input(
    selected_query: Query<Entity, With<CharacterSelected>>,
    input: Res<ButtonInput<KeyCode>>,
    mut move_events: EventWriter<CharacterMoveEvent>,
) {
    if let Ok(selected_entity) = selected_query.single() {
        if let Some(direction) = get_movement_direction_from_input(&input) {
            move_events.write(CharacterMoveEvent::new(selected_entity, direction));
        }
    }
}

/// Calculate new position based on movement direction
fn calculate_new_position(current_x: f32, current_y: f32, direction: MovementDirection) -> (f32, f32) {
    match direction {
        MovementDirection::Left => (current_x - TILE_SIZE, current_y),
        MovementDirection::Down => (current_x, current_y - TILE_SIZE),
        MovementDirection::Right => (current_x + TILE_SIZE, current_y),
        MovementDirection::Up => (current_x, current_y + TILE_SIZE),
    }
}

/// System that processes movement events and updates character positions
fn process_movement_events(
    mut move_events: EventReader<CharacterMoveEvent>,
    mut character_query: Query<&mut Transform, With<Character>>,
    #[cfg(debug_assertions)]
    character_info_query: Query<&Character>,
) {
    for event in move_events.read() {
        if let Ok(mut transform) = character_query.get_mut(event.entity) {
            let (old_x, old_y) = (transform.translation.x, transform.translation.y);
            let (new_x, new_y) = calculate_new_position(old_x, old_y, event.direction);
            
            // Clamp position to stay within grid boundaries
            let (clamped_x, clamped_y) = clamp_to_grid_boundaries(new_x, new_y);
            
            // Apply the new position
            transform.translation.x = clamped_x;
            transform.translation.y = clamped_y;
            
            // Debug output (only in debug builds)
            #[cfg(debug_assertions)]
            if let Ok(character) = character_info_query.get(event.entity) {
                println!(
                    "Movement processed for {}: {:?} from ({:.0}, {:.0}) to ({:.0}, {:.0})", 
                    character.name, event.direction, old_x, old_y, clamped_x, clamped_y
                );
            }
        }
    }
}