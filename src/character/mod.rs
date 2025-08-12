use crate::arena::{Arena, CurrentArena, LastActiveHero};
use crate::materials::Materials;
use crate::selectors::Active;
use bevy::input::ButtonInput;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{Children, Commands, Component, Entity, KeyCode, Query, Res, Single, With};

/// Marker component for character entities.
#[derive(Component, Debug)]
pub struct Character;

#[derive(Component, Debug)]
pub struct Boss;

pub fn toggle_active_character(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_arena_q: Single<&CurrentArena>,
    arena_q: Query<(Entity, &Arena, &Children), With<Arena>>,
    characters_q: Query<(Entity, Option<&Active>), With<Character>>,
    mats: Res<Materials>,
) {
    if !keyboard_input.just_pressed(KeyCode::Tab) {
        return;
    }

    let current_arena = current_arena_q.into_inner();
    let current_arena_index = current_arena.0;

    for (arena_entity, arena, children) in arena_q.iter() {
        if arena.0 == current_arena_index {
            // Get all characters in this arena
            let characters_data: Vec<(Entity, Option<&Active>)> =
                characters_q.iter_many(children).collect();

            // Need at least 2 characters to cycle
            if characters_data.len() < 2 {
                println!("Not enough characters to cycle in arena {}", arena.0);
                return;
            }

            // Find the currently active character's index
            let active_index = characters_data
                .iter()
                .position(|(_, active)| active.is_some());

            if let Some(current_index) = active_index {
                // Calculate next index (cyclical)
                let next_index = (current_index + 1) % characters_data.len();

                // Remove Active from current character
                let current_entity = characters_data[current_index].0;
                commands
                    .entity(current_entity)
                    .insert(MeshMaterial3d(mats.gray.clone()))
                    .remove::<Active>();

                // Add Active to next character
                let next_entity = characters_data[next_index].0;
                commands
                    .entity(next_entity)
                    .insert(MeshMaterial3d(mats.blue.clone()))
                    .insert(Active);

                // Update LastActiveHero
                commands
                    .entity(arena_entity)
                    .insert(LastActiveHero(Some(next_entity)));

                println!(
                    "Cycled from character {:?} to {:?} in arena {}",
                    current_entity, next_entity, arena.0
                );
            }
        }
    }
}
