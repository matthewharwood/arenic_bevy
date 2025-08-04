// use bevy::ecs::query::QuerySingleError;
// use bevy::prelude::*;
//
// use super::{ActiveArena, Arena, ArenaId, Character, InArena};
//
// /// Give me the active arena: returns the ArenaId if exactly one exists.
// pub fn give_me_active_arena_id(
//     arenas: Query<&ArenaId, (With<Arena>, With<ActiveArena>)>,
// ) -> Result<ArenaId, QuerySingleError> {
//     arenas.single().copied()
// }
//
// /// Give me the active arena?: safely returns the active arena if it exists.
// pub fn give_me_active_arena_id_optional(
//     arenas: Query<&ArenaId, (With<Arena>, With<ActiveArena>)>,
// ) -> Option<ArenaId> {
//     arenas.single().ok().copied()
// }
//
// /// Give me the active arena entity: returns the entity and ArenaId for the active arena.
// pub fn give_me_active_arena(
//     arenas: Query<(Entity, &ArenaId), (With<Arena>, With<ActiveArena>)>,
// ) -> Option<(Entity, ArenaId)> {
//     arenas.single().ok().map(|(e, id)| (e, *id))
// }
//
// /// Give me all arenas: collects all arena entities and their IDs.
// pub fn give_me_all_arenas(
//     arenas: Query<(Entity, &ArenaId), With<Arena>>,
// ) -> Vec<(Entity, ArenaId)> {
//     arenas.iter().map(|(e, id)| (e, *id)).collect()
// }
//
// /// Give me all arena IDs: collects just the arena IDs.
// pub fn give_me_all_arena_ids(arenas: Query<&ArenaId, With<Arena>>) -> Vec<ArenaId> {
//     arenas.iter().copied().collect()
// }
//
// /// Give me arena by ID: finds a specific arena by its ArenaId.
// pub fn give_me_arena_by_id(
//     arenas: Query<(Entity, &ArenaId), With<Arena>>,
//     target_id: ArenaId,
// ) -> Option<Entity> {
//     arenas
//         .iter()
//         .find(|(_, id)| **id == target_id)
//         .map(|(entity, _)| entity)
// }
//
// /// Give me all characters in arena X: returns all character entities in a specific arena.
// pub fn give_me_characters_in_arena(
//     characters: Query<(Entity, &InArena), With<Character>>,
//     arena_id: ArenaId,
// ) -> Vec<Entity> {
//     characters
//         .iter()
//         .filter_map(|(entity, in_arena)| {
//             if in_arena.arena_id == arena_id {
//                 Some(entity)
//             } else {
//                 None
//             }
//         })
//         .collect()
// }
//
// /// Give me all characters in the active arena: returns all character entities in the currently active arena.
// pub fn give_me_characters_in_active_arena(
//     characters: Query<(Entity, &InArena), With<Character>>,
//     active_arenas: Query<&ArenaId, (With<Arena>, With<ActiveArena>)>,
// ) -> Vec<Entity> {
//     if let Ok(active_id) = active_arenas.single() {
//         give_me_characters_in_arena(characters, *active_id)
//     } else {
//         Vec::new()
//     }
// }
//
// /// Give me character count in arena: returns the number of characters in a specific arena.
// pub fn give_me_character_count_in_arena(
//     characters: Query<&InArena, With<Character>>,
//     arena_id: ArenaId,
// ) -> usize {
//     characters
//         .iter()
//         .filter(|in_arena| in_arena.arena_id == arena_id)
//         .count()
// }
//
// /// Give me arena for character: returns the arena ID that a character belongs to.
// pub fn give_me_arena_for_character(
//     character_entity: Entity,
//     characters: Query<&InArena, With<Character>>,
// ) -> Option<ArenaId> {
//     characters.get(character_entity).ok().map(|in_arena| in_arena.arena_id)
// }
//
// /// Give me arena entity count: returns the total number of arena entities.
// pub fn give_me_arena_count(arenas: Query<&Arena>) -> usize {
//     arenas.iter().count()
// }
//
// /// System helper: Set active arena by ArenaId.
// pub fn set_active_arena(
//     mut commands: Commands,
//     arenas: Query<(Entity, &ArenaId), With<Arena>>,
//     target_id: ArenaId,
// ) -> Result<(), &'static str> {
//     // Remove ActiveArena from all arenas
//     for (entity, _) in arenas.iter() {
//         commands.entity(entity).remove::<ActiveArena>();
//     }
//
//     // Add ActiveArena to the target arena
//     if let Some(target_entity) = give_me_arena_by_id(arenas, target_id) {
//         commands.entity(target_entity).insert(ActiveArena);
//         Ok(())
//     } else {
//         Err("Arena with specified ID not found")
//     }
// }