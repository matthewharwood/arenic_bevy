use super::GameState;
use crate::arena::{
    Arena, ArenaTransform, Bastion, Casino, Crucible, CurrentArena, Gala, GuildHouse, Labyrinth,
    Mountain, Pawnshop, Sanctum,
};
use crate::battleground::Battleground;
use crate::boss::guild_master::GuildMaster;
use crate::boss::{Boss, BossAnimationConfig};
use crate::character::CharacterType;
use crate::config::arena::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::config::display::TILE_SIZE;
use crate::game_state::character_create::CharacterCreateState;
use crate::trait_utils::ComponentDisplay;
use crate::ui::styles_config::Colors;
use bevy::prelude::*;

/// Plugin for the Intro/main game state
pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Intro),
            (setup_intro, setup_arena_tiles, spawn_guild_master).chain(),
        )
        .add_systems(
            OnEnter(GameState::Intro),
            (spawn_circle_shape, debug_circle_character_type)
                .chain()
                .after(spawn_guild_master),
        )
        .add_systems(
            Update,
            animate_guild_master.run_if(in_state(GameState::Intro)),
        )
        .add_systems(OnExit(GameState::Intro), cleanup_intro);
    }
}

/// Marker component for intro state entities
#[derive(Component)]
struct IntroScreen;

/// Marker component for the circle shape
#[derive(Component)]
struct CircleShape;

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

fn spawn_guild_master(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<Entity, With<GuildHouse>>,
) {
    let Some(arena_entity) = query.iter().next() else {
        warn!("No GuildHouse entity found to spawn GuildMaster in!");
        return;
    };

    info!("Spawning GuildMaster in arena entity: {:?}", arena_entity);

    let texture = asset_server.load(GuildMaster::TEXTURE_PATH);
    let layout = layouts.add(GuildMaster::create_atlas_layout());

    commands.entity(arena_entity).with_children(|parent| {
        let guild_master_entity = parent
            .spawn((
                GuildMaster,
                Sprite {
                    image: texture,
                    texture_atlas: Some(TextureAtlas { layout, index: 0 }),
                    ..default()
                },
                Transform::from_xyz(ARENA_WIDTH / 2.0, -ARENA_HEIGHT / 2.0, 1.0),
                GuildMaster::animation_config(),
            ))
            .id();

        info!("GuildMaster spawned with entity: {:?}", guild_master_entity);
    });
}
fn animate_guild_master(
    time: Res<Time>,
    guild_master: Single<(&mut Sprite, &mut BossAnimationConfig), With<GuildMaster>>,
) {
    let (mut sprite, mut config) = guild_master.into_inner();
    if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
        config.timer.tick(time.delta());

        if config.timer.just_finished() {
            texture_atlas.index = if texture_atlas.index >= config.last_frame {
                config.first_frame
            } else {
                texture_atlas.index + 1
            };
        }
    }
}

fn spawn_circle_shape(
    mut commands: Commands,
    guild_house_query: Single<Entity, With<GuildHouse>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    character_state: Res<CharacterCreateState>,
) {
    let guild_house_entity = guild_house_query.into_inner();

    let tile_x = 40.0;
    let tile_y = -15.0;

    // Convert tile coordinates to world position within the arena
    let world_x = tile_x * TILE_SIZE;
    let world_y = tile_y * TILE_SIZE;

    commands.entity(guild_house_entity).with_children(|parent| {
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(TILE_SIZE / 2.0))),
            MeshMaterial2d(materials.add(Colors::BLACK)),
            Transform::from_xyz(world_x, world_y, 1.0),
            CircleShape,
            character_state.selected_character,
        ));
    });
}

/// Debug system that prints the CharacterType of entities with CircleShape
fn debug_circle_character_type(
    circle_query: Query<(Entity, &CharacterType), With<CircleShape>>,
) {
    for (entity, character_type) in &circle_query {
        info!(
            "CircleShape entity {:?} has CharacterType: {:?}", 
            entity, character_type
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    #[test]
    fn test_animation_frame_progression() {
        let mut sprite = Sprite {
            texture_atlas: Some(TextureAtlas {
                layout: Handle::default(),
                index: 0,
            }),
            ..default()
        };
        let mut config = GuildMaster::animation_config();
        config.timer.tick(Duration::from_secs_f32(0.1));

        if config.timer.just_finished() {
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = if atlas.index >= config.last_frame {
                    config.first_frame
                } else {
                    atlas.index + 1
                };
            }
        }
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, 1);
    }
    fn test_animation_wrap_around() {
        let mut sprite = Sprite {
            texture_atlas: Some(TextureAtlas {
                layout: Handle::default(),
                index: 13,
            }),
            ..default()
        };
        let mut config = GuildMaster::animation_config();
        config.timer.tick(Duration::from_secs_f32(0.1));
        if config.timer.just_finished() {
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = if atlas.index >= config.last_frame {
                    config.first_frame
                } else {
                    atlas.index + 1
                };
            }
        }
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, 0);
    }
}
