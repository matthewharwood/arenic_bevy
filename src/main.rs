#![feature(gen_blocks)]
#![feature(yield_expr)]
#![feature(associated_type_defaults)]

use bevy::prelude::*;
use bevy::window::WindowResolution;

// Module declarations
mod animation;
mod bundles;
mod components;
mod config;
mod generators;
mod input;
mod plugins;
mod ui;
mod utils;

// Re-exports for convenience
use bundles::{CharacterBundle, SelectedCharacterBundle};
use components::*;
use config::{arena::*, assets::*, camera::*, display::*};
use generators::{arena_positions, arena_tile_positions};
use plugins::{ArenaPlugin, CharacterPlugin};
use ui::UIPlugin;
use utils::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Arenic".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    ..default()
                }),
                ..default()
            }),
            ArenaPlugin,
            CharacterPlugin,
            UIPlugin,
        ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(CurrentArena(1));
    let (camera_x, camera_y) = calculate_camera_position(1);
    commands
        .spawn(Camera2d)
        .insert(Transform::from_xyz(camera_x, camera_y, 0.0))
        .insert(Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scale: SCALE_NORMAL,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
            scaling_mode: Default::default(),
        }));

    // Use Rust 2024 generator for cleaner arena setup
    for arena_pos in arena_positions() {
        let mut arena = commands.spawn(Transform::from_xyz(
            arena_pos.world_x,
            arena_pos.world_y,
            0.0,
        ));
        
        let image_path = format!("Grid_{}.png", arena_pos.index);
        
        // Use generator for tile positions too
        for tile_pos in arena_tile_positions() {
            arena
                .insert(InheritedVisibility::default())
                .with_children(|parent| {
                    parent
                        .spawn(Sprite {
                            image: asset_server.load(image_path.clone()),
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        })
                        .insert(Transform::from_xyz(
                            tile_pos.world_x,
                            tile_pos.world_y,
                            0.0,
                        ));
                });
        }
    }
}

fn handle_arena_navigation_keys(
    mut arena_query: Query<&mut CurrentArena>,
    camera_query: Query<&Projection, With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // Check if camera is at scale 3.0
    let _is_zoomed_out = camera_query.iter().any(|projection| {
        matches!(projection, Projection::Orthographic(ortho) if ortho.scale == SCALE_ZOOMED_OUT)
    });

    if input.just_pressed(KeyCode::BracketRight) {
        for mut arena in &mut arena_query {
            arena.0 = CurrentArena::increment(arena.0);
        }
    }
    if input.just_pressed(KeyCode::BracketLeft) {
        for mut arena in &mut arena_query {
            arena.0 = CurrentArena::decrement(arena.0);
        }
    }
}


fn draw_arena_gizmo(
    mut gizmos: Gizmos,
    arena_query: Query<&CurrentArena>,
    camera_query: Query<&Projection, With<Camera>>,
) {
    for projection in &camera_query {
        if let Projection::Orthographic(ortho) = projection {
            if ortho.scale == SCALE_ZOOMED_OUT {
                // Only draw gizmo when zoomed out
                for arena in &arena_query {
                    let arena_col = arena.0 % 3;
                    let arena_row = arena.0 / 3;

                    // Calculate the center of the current arena in world coordinates
                    let arena_center_x =
                        -HALF_WINDOW_WIDTH + (arena_col as f32 * ARENA_WIDTH) + ARENA_WIDTH / 2.0;
                    let arena_center_y =
                        HALF_WINDOW_HEIGHT - (arena_row as f32 * ARENA_HEIGHT) - ARENA_HEIGHT / 2.0;
                    let arena_center = Vec2::new(arena_center_x, arena_center_y);

                    let border_thickness = 10.0; // Desired total border thickness
                    let border_color = Color::BLACK; // Your desired border color

                    // Draw the border using a loop, building inwardly
                    for i in 0..border_thickness as u32 {
                        let current_thickness_offset = i as f32;
                        gizmos.rect_2d(
                            arena_center,
                            Vec2::new(
                                ARENA_WIDTH - current_thickness_offset * 2.0,
                                ARENA_HEIGHT - current_thickness_offset * 2.0,
                            ),
                            border_color,
                        );
                    }
                }
            }
        }
    }
}

fn spawn_player_selected(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn first character at tile position (33, 15) in arena 1 (center of the arena)
    let (char1_x, char1_y) = calculate_character_position(1, 33, 15);
    commands.spawn(SelectedCharacterBundle::new(
        &asset_server,
        char1_x,
        char1_y,
    ));

    // Spawn second character at tile position (30, 15) in arena 1 (3 tiles to the left)
    let (char2_x, char2_y) = calculate_character_position(1, 30, 15);
    commands.spawn(CharacterBundle::new(&asset_server, char2_x, char2_y, false));
}

fn move_selected_player(
    mut player_query: Query<&mut Transform, With<CharacterSelected>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in &mut player_query {
        let mut new_x = transform.translation.x;
        let mut new_y = transform.translation.y;

        if input.just_pressed(KeyCode::KeyA) {
            // Move left
            new_x -= TILE_SIZE;
        }
        if input.just_pressed(KeyCode::KeyS) {
            // Move down
            new_y -= TILE_SIZE;
        }
        if input.just_pressed(KeyCode::KeyD) {
            // Move right
            new_x += TILE_SIZE;
        }
        if input.just_pressed(KeyCode::KeyW) {
            // Move up
            new_y += TILE_SIZE;
        }

        // Clamp position to stay within the 3x3 grid boundaries
        let (clamped_x, clamped_y) = clamp_to_grid_boundaries(new_x, new_y);

        // Apply the clamped position
        transform.translation.x = clamped_x;
        transform.translation.y = clamped_y;
    }
}

fn cycle_selected_character(
    mut commands: Commands,
    characters_query: Query<Entity, With<Character>>,
    selected_query: Query<Entity, With<CharacterSelected>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Tab) {
        // Get all character entities as a Vec
        let characters: Vec<Entity> = characters_query.iter().collect();

        if characters.len() <= 1 {
            return; // No cycling needed with 0 or 1 characters
        }

        // Find current selected character index using cleaner Rust 2024 patterns
        let next_index = selected_query
            .single()
            .ok()
            .and_then(|selected_entity| characters.iter().position(|&e| e == selected_entity))
            .map(|current_index| (current_index + 1) % characters.len())
            .unwrap_or(0);

        // Remove CharacterSelected from all characters
        for entity in &characters {
            commands.entity(*entity).remove::<CharacterSelected>();
        }

        // Add CharacterSelected to next character
        commands
            .entity(characters[next_index])
            .insert(CharacterSelected);
    }
}

fn update_character_sprites(
    mut character_query: Query<(Entity, &mut Sprite), With<Character>>,
    selected_query: Query<Entity, With<CharacterSelected>>,
    asset_server: Res<AssetServer>,
) {
    let selected_entity = selected_query.single().ok();

    for (entity, mut sprite) in &mut character_query {
        if Some(entity) == selected_entity {
            sprite.image = asset_server.load(PLAYER_SELECTED);
        } else {
            sprite.image = asset_server.load(PLAYER_UNSELECTED);
        }
    }
}

fn update_character_arena_markers(
    mut commands: Commands,
    mut character_query: Query<(Entity, &Transform, Option<&ArenaName>), With<Character>>,
) {
    for (entity, transform, current_arena_name) in &mut character_query {
        let x = transform.translation.x;
        let y = transform.translation.y;

        // Calculate which arena this character is in based on position
        let arena_col = ((x + HALF_WINDOW_WIDTH) / ARENA_WIDTH).floor() as i32;
        let arena_row = ((HALF_WINDOW_HEIGHT - y) / ARENA_HEIGHT).floor() as i32;

        // Clamp to valid arena bounds (0-2 for both col and row)
        let arena_col = arena_col.clamp(0, 2) as u8;
        let arena_row = arena_row.clamp(0, 2) as u8;

        let arena_index = arena_row * 3 + arena_col;
        let new_arena_name = ArenaName::from_index(arena_index);

        // Only update if the arena has changed
        if current_arena_name != Some(&new_arena_name) {
            commands.entity(entity).insert(new_arena_name);
        }
    }
}

fn sync_current_arena_with_selected_character(
    mut arena_query: Query<&mut CurrentArena>,
    selected_character_query: Query<&ArenaName, (With<CharacterSelected>, Changed<ArenaName>)>,
) {
    if let Ok(arena_name) = selected_character_query.single() {
        for mut current_arena in &mut arena_query {
            current_arena.0 = arena_name.to_index();
            println!(
                "CurrentArena updated to: {} (index: {})",
                arena_name.name(),
                arena_name.to_index()
            );
        }
    }
}

fn debug_character_arena_changes(
    query: Query<&ArenaName, (With<CharacterSelected>, Changed<ArenaName>)>,
) {
    if let Ok(arena_name) = query.single() {
        println!("CharacterSelected entered arena: {}", arena_name.name());
    }
}

fn ensure_character_selected_in_current_arena(
    mut commands: Commands,
    current_arena_query: Query<&CurrentArena, Changed<CurrentArena>>,
    selected_character_query: Query<&ArenaName, With<CharacterSelected>>,
    all_characters_query: Query<(Entity, &ArenaName), With<Character>>,
) {
    if let Ok(current_arena) = current_arena_query.single() {
        let target_arena = ArenaName::from_index(current_arena.0);

        // Check if there's already a selected character in the current arena
        let has_selected_in_arena = selected_character_query
            .single()
            .map(|arena_name| *arena_name == target_arena)
            .unwrap_or(false);

        if !has_selected_in_arena {
            // Find the first character in the target arena
            let first_character_in_arena = all_characters_query
                .iter()
                .find(|(_, arena_name)| **arena_name == target_arena)
                .map(|(entity, _)| entity);

            if let Some(character_entity) = first_character_in_arena {
                // Remove CharacterSelected from all characters first
                for (entity, _) in all_characters_query.iter() {
                    commands.entity(entity).remove::<CharacterSelected>();
                }

                // Add CharacterSelected to the found character
                commands.entity(character_entity).insert(CharacterSelected);

                println!("Auto-selected character in arena: {}", target_arena.name());
            }
        }
    }
}

