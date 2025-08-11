use crate::arena::{Arena, LastActiveHero};
use crate::materials::Materials;
use crate::selectors::Active;
use bevy::input::ButtonInput;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{
    Children, Commands, Component, Entity, KeyCode, Query, Res, Single, With,
};

/// Marker component for character entities.
#[derive(Component, Debug)]
pub struct Character;

#[derive(Component, Debug)]
pub struct Boss;

pub fn active_character_selection(
    mut commands: Commands,
    active_character_q: Option<Single<Entity, (With<Character>, With<Active>)>>,
    active_arena_query: Single<(Entity, &LastActiveHero, &Children), (With<Arena>, With<Active>)>,
    character_query: Query<Entity, With<Character>>,
    mats: Res<Materials>,
) {
    // Destructure the arena query once to avoid borrowing issues
    let (active_arena_entity, last_active_hero, arena_children) = active_arena_query.into_inner();

    // There may not be an active hero in the arena so we check the Option first
    if let Some(active_character) = active_character_q {
        // There is an active character
        let active_character_entity = active_character.into_inner();
        // When Active set the LastActiveHero for that Arena.
        commands
            .entity(active_arena_entity)
            .insert(LastActiveHero(Some(active_character_entity)));
    } else {
        if let Some(hero_entity) = last_active_hero.0 {
            commands
                .entity(hero_entity)
                .insert(Active)
                .insert(MeshMaterial3d(mats.blue.clone()));
        } else {
            let character_entities: Vec<Entity> =
                character_query.iter_many(arena_children).collect();
            if character_entities.is_empty() {
                println!("No characters in active arena!");
                return;
            }
            let first_character = character_entities[0];
            commands
                .entity(first_character)
                .insert(Active)
                .insert(MeshMaterial3d(mats.blue.clone()));
            commands
                .entity(active_arena_entity)
                .insert(LastActiveHero(Some(first_character)));
        }
    }
}
pub fn toggle_active_character(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    active_arena_query: Single<(Entity, &Children), (With<Active>, With<Arena>)>,
    active_character: Single<Entity, (With<Character>, With<Active>)>,
    character_query: Query<Entity, With<Character>>,
    mats: Res<Materials>,
) {
    if !keyboard_input.just_pressed(KeyCode::Tab) {
        return;
    }

    let (active_arena_entity, arena_children) = active_arena_query.into_inner();
    let current_active_entity = active_character.into_inner(); // TODO this will prob error out
    let character_entities: Vec<Entity> = character_query.iter_many(arena_children).collect();

    if character_entities.is_empty() {
        println!("No characters in active arena!");
        return;
    }

    let current_index = character_entities
        .iter()
        .position(|&e| e == current_active_entity)
        .expect("Active character must be in active arena");

    let next_index = (current_index + 1) % character_entities.len();
    let next_active_entity = character_entities[next_index];

    commands
        .entity(current_active_entity)
        .remove::<Active>()
        .insert(MeshMaterial3d(mats.gray.clone()));
    commands
        .entity(next_active_entity)
        .insert(Active)
        .insert(MeshMaterial3d(mats.blue.clone()));

    commands
        .entity(active_arena_entity)
        .insert(LastActiveHero(Some(next_active_entity)));
}
