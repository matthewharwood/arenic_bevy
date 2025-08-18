use crate::arena::{CameraUpdate, LastActiveHero};
use crate::arena_camera::{position_camera_for_arena, ZoomOut, ZOOM};
use crate::character::Character;
use crate::materials::Materials;
use crate::selectors::Active;
use bevy::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Newtype for arena indices (0-8)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Arena(pub u8);

impl Arena {
    const MAX_ARENAS: u8 = 9;
    
    /// Creates new Arena if value is valid (0-8)
    #[must_use]
    pub fn new(idx: u8) -> Option<Self> {
        (idx < Self::MAX_ARENAS).then(|| Self(idx))
    }

    #[must_use]
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Arena {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or("Arena out of bounds")
    }
}

impl Display for Arena {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Arena {}", self.0)
    }
}

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
    mats: Res<Materials>,
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
                commands
                    .entity(entity)
                    .insert(MeshMaterial3d(mats.gray.clone()))
                    .remove::<Active>();
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
                            commands
                                .entity(entity)
                                .insert(MeshMaterial3d(mats.gray.clone()))
                                .remove::<Active>();
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
                                    commands
                                        .entity(entity)
                                        .insert(MeshMaterial3d(mats.gray.clone()))
                                        .remove::<Active>();
                                }
                            }
                            // Now set the new active character
                            commands
                                .entity(hero_entity)
                                .insert(MeshMaterial3d(mats.blue.clone()))
                                .insert(Active);
                        } else {
                            println!("Using first character");
                            let first_character = characters_data[0].0;
                            // Remove Active from all currently active characters across all arenas
                            for (entity, active) in characters_q.iter() {
                                if active.is_some() {
                                    commands
                                        .entity(entity)
                                        .insert(MeshMaterial3d(mats.gray.clone()))
                                        .remove::<Active>();
                                }
                            }
                            // Now set the new active character
                            commands
                                .entity(first_character)
                                .insert(MeshMaterial3d(mats.blue.clone()))
                                .insert(Active);
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
