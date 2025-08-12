use crate::arena::LastActiveHero;
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
) {
    if keycode.just_pressed(KeyCode::BracketLeft) {
        let mut current_arena = current_arena_q.into_inner();
        current_arena.0 = CurrentArena::decrement(current_arena.0);
    }
}

pub fn increment_current_arena(
    keycode: Res<ButtonInput<KeyCode>>,
    current_arena_q: Single<&mut CurrentArena>,
) {
    if keycode.just_pressed(KeyCode::BracketRight) {
        let mut current_arena = current_arena_q.into_inner();
        current_arena.0 = CurrentArena::increment(current_arena.0);
    }
}

pub fn current_arena_change(
    mut commands: Commands,
    current_arena_q: Single<&CurrentArena, Changed<CurrentArena>>,
    camera: Single<(Entity, &mut Transform, Option<&ZoomOut>), With<Camera3d>>,
    arena_q: Query<(Entity, &Arena, &Children, Option<&LastActiveHero>), With<Arena>>,
    characters_q: Query<(Entity, Option<&Active>), With<Character>>,
) {
    let current_arena = current_arena_q.into_inner();
    let (camera_entity, mut camera_transform, zoom) = camera.into_inner();
    if zoom.is_some() {

        // When zoomed out, don't move camera (it stays centered on all arenas)
        // The gizmo drawing is handled by draw_arena_border system
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
                    // No characters in this arena - handle this case
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
