//! Character movement plugin.
//! 
//! This plugin handles all character movement functionality using an event-driven
//! architecture that separates input handling from movement processing.

use bevy::prelude::*;
use crate::components::CharacterSelected;
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
    /// Optional timestamp for replay systems
    pub timestamp: Option<f64>,
}

impl CharacterMoveEvent {
    /// Create a new movement event for the current time
    pub fn new(entity: Entity, direction: MovementDirection) -> Self {
        Self {
            entity,
            direction,
            timestamp: None,
        }
    }

    /// Create a new movement event with a specific timestamp (for replay)
    pub fn with_timestamp(entity: Entity, direction: MovementDirection, timestamp: f64) -> Self {
        Self {
            entity,
            direction,
            timestamp: Some(timestamp),
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

/// System that handles input and emits movement events
fn handle_movement_input(
    selected_query: Query<Entity, With<CharacterSelected>>,
    input: Res<ButtonInput<KeyCode>>,
    mut move_events: EventWriter<CharacterMoveEvent>,
) {
    if let Ok(selected_entity) = selected_query.single() {
        if input.just_pressed(KeyCode::KeyA) {
            move_events.write(CharacterMoveEvent::new(selected_entity, MovementDirection::Left));
        }
        if input.just_pressed(KeyCode::KeyS) {
            move_events.write(CharacterMoveEvent::new(selected_entity, MovementDirection::Down));
        }
        if input.just_pressed(KeyCode::KeyD) {
            move_events.write(CharacterMoveEvent::new(selected_entity, MovementDirection::Right));
        }
        if input.just_pressed(KeyCode::KeyW) {
            move_events.write(CharacterMoveEvent::new(selected_entity, MovementDirection::Up));
        }
    }
}

/// System that processes movement events and updates character positions
fn process_movement_events(
    mut move_events: EventReader<CharacterMoveEvent>,
    mut character_query: Query<&mut Transform>,
) {
    for event in move_events.read() {
        if let Ok(mut transform) = character_query.get_mut(event.entity) {
            let mut new_x = transform.translation.x;
            let mut new_y = transform.translation.y;

            match event.direction {
                MovementDirection::Left => new_x -= TILE_SIZE,
                MovementDirection::Down => new_y -= TILE_SIZE,
                MovementDirection::Right => new_x += TILE_SIZE,
                MovementDirection::Up => new_y += TILE_SIZE,
            }

            // Clamp position to stay within the 3x3 grid boundaries
            let (clamped_x, clamped_y) = clamp_to_grid_boundaries(new_x, new_y);

            // Apply the clamped position
            transform.translation.x = clamped_x;
            transform.translation.y = clamped_y;
        }
    }
}