use super::GameState;
use crate::arena::{
    Arena, ArenaTransform, Bastion, Casino, Crucible, CurrentArena, Gala, GuildHouse, Labyrinth,
    Mountain, Pawnshop, Sanctum,
};
use crate::battleground::Battleground;
use crate::pseudo_states::Checked;
use crate::trait_utils::ComponentDisplay;
use bevy::prelude::*;

/// Plugin for the Intro/main game state
pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Intro),
            (setup_intro, setup_arena_tiles).chain(),
        )
        .add_systems(OnExit(GameState::Intro), cleanup_intro);
    }
}

/// Marker component for intro state entities
#[derive(Component)]
struct IntroScreen;

const GAME_NAME: &str = "Arenic";

fn setup_intro(mut commands: Commands) {
    // Spawn the CurrentArena as a resource or component
    commands.spawn(CurrentArena(1));

    commands
        .spawn((
            Battleground,
            Name::new(GAME_NAME),
            Transform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            IntroScreen, // Add marker for cleanup
        ))
        .with_children(|parent| {
            // Spawn Labyrinth
            parent.spawn((
                Arena,
                Name::new(Labyrinth::NAME),
                Labyrinth,
                Labyrinth::transform(),
                Visibility::default(),
                InheritedVisibility::default(),
            ));

            // Spawn GuildHouse with Checked state
            parent.spawn((
                Arena,
                Name::new(GuildHouse::NAME),
                GuildHouse,
                GuildHouse::transform(),
                Checked,
                Visibility::default(),
                InheritedVisibility::default(),
            ));

            // Spawn remaining arenas
            parent.spawn((
                Arena,
                Name::new(Sanctum::NAME),
                Sanctum,
                Sanctum::transform(),
                Visibility::default(),
                InheritedVisibility::default(),
            ));

            parent.spawn((
                Arena,
                Name::new(Mountain::NAME),
                Mountain,
                Mountain::transform(),
                Visibility::default(),
                InheritedVisibility::default(),
            ));

            parent.spawn((
                Arena,
                Name::new(Bastion::NAME),
                Bastion,
                Bastion::transform(),
                Visibility::default(),
                InheritedVisibility::default(),
            ));

            parent.spawn((
                Arena,
                Name::new(Pawnshop::NAME),
                Pawnshop,
                Pawnshop::transform(),
                Visibility::default(),
                InheritedVisibility::default(),
            ));

            parent.spawn((
                Arena,
                Name::new(Crucible::NAME),
                Crucible,
                Crucible::transform(),
                Visibility::default(),
                InheritedVisibility::default(),
            ));

            parent.spawn((
                Arena,
                Name::new(Casino::NAME),
                Casino,
                Casino::transform(),
                Visibility::default(),
                InheritedVisibility::default(),
            ));

            parent.spawn((
                Arena,
                Name::new(Gala::NAME),
                Gala,
                Gala::transform(),
                Visibility::default(),
                InheritedVisibility::default(),
            ));
        });
}

fn setup_arena_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arena_query: Query<
        (
            Entity,
            &Arena,
            Option<&Labyrinth>,
            Option<&GuildHouse>,
            Option<&Sanctum>,
            Option<&Mountain>,
            Option<&Bastion>,
            Option<&Pawnshop>,
            Option<&Crucible>,
            Option<&Casino>,
            Option<&Gala>,
        ),
        Added<Arena>,
    >,
) {
    for (
        arena_entity,
        arena,
        labyrinth,
        guildhouse,
        sanctum,
        mountain,
        bastion,
        pawnshop,
        crucible,
        casino,
        gala,
    ) in &arena_query
    {
        let arena_index = if labyrinth.is_some() {
            Labyrinth::INDEX
        } else if guildhouse.is_some() {
            GuildHouse::INDEX
        } else if sanctum.is_some() {
            Sanctum::INDEX
        } else if mountain.is_some() {
            Mountain::INDEX
        } else if bastion.is_some() {
            Bastion::INDEX
        } else if pawnshop.is_some() {
            Pawnshop::INDEX
        } else if crucible.is_some() {
            Crucible::INDEX
        } else if casino.is_some() {
            Casino::INDEX
        } else if gala.is_some() {
            Gala::INDEX
        } else {
            0 // default fallback
        };

        arena.spawn_tile_grid(&mut commands, arena_entity, &asset_server, arena_index);
    }
}

fn cleanup_intro(
    mut commands: Commands,
    query: Query<Entity, With<IntroScreen>>,
    current_arena_query: Query<Entity, With<CurrentArena>>,
) {
    // Clean up intro screen entities
    for entity in &query {
        commands.entity(entity).despawn();
    }

    // Clean up CurrentArena entity
    for entity in &current_arena_query {
        commands.entity(entity).despawn();
    }
}
