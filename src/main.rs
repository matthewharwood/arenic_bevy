use bevy::prelude::*;
use bevy::window::WindowResolution;

// Module declarations
mod arena;
mod battleground;

mod recording;

mod boss;
mod camera;
mod character;
mod config;
mod psuedo_states;
mod relationships;
mod tile;
mod trait_utils;
mod ui;

use crate::arena::{
    Arena, ArenaTransform, Bastion, Casino, Crucible, CurrentArena, Gala, GuildHouse, Labyrinth,
    Mountain, Pawnshop, Sanctum,
};
use crate::battleground::Battleground;
use crate::camera::CameraPlugin;
use crate::config::display::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::psuedo_states::Checked;
use crate::trait_utils::ComponentDisplay;
use crate::ui::UiPlugin;
use recording::RecordingPlugin;

const GAME_NAME: &str = "Arenic";
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: GAME_NAME.to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (startup_system, setup_arena_tiles).chain())
        .add_plugins(CameraPlugin)
        .add_plugins(RecordingPlugin)
        .add_plugins(UiPlugin)
        .run();
}

fn startup_system(mut commands: Commands) {
    // Spawn the CurrentArena as a resource or component
    commands.spawn(CurrentArena(1));

    commands
        .spawn((
            Battleground,
            Name::new(GAME_NAME),
            Transform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
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
