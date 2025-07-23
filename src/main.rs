use bevy::prelude::*;
use bevy::window::WindowResolution;

#[derive(Component)]
pub struct CurrentArena(pub u8);

#[derive(Component)]
pub struct ArenaGizmo;

#[derive(Component)]
pub struct CharacterSelected;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct TopNavBar;

#[derive(Component)]
pub struct SideNavBar;

#[derive(Component)]
pub struct BottomNavBar;

// Arena name enum and component
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaName {
    Labyrinth = 0,
    GuildHouse = 1,
    Sanctum = 2,
    Mountain = 3,
    Bastion = 4,
    Pawnshop = 5,
    Crucible = 6,
    Casino = 7,
    Gala = 8,
}

impl ArenaName {
    pub fn from_index(index: u8) -> ArenaName {
        match index {
            0 => ArenaName::Labyrinth,
            1 => ArenaName::GuildHouse,
            2 => ArenaName::Sanctum,
            3 => ArenaName::Mountain,
            4 => ArenaName::Bastion,
            5 => ArenaName::Pawnshop,
            6 => ArenaName::Crucible,
            7 => ArenaName::Casino,
            8 => ArenaName::Gala,
            _ => panic!("Invalid arena index: {}", index),
        }
    }

    pub fn to_index(&self) -> u8 {
        *self as u8
    }

    pub fn name(&self) -> &'static str {
        match self {
            ArenaName::Labyrinth => "Labyrinth",
            ArenaName::GuildHouse => "Guild House",
            ArenaName::Sanctum => "Sanctum",
            ArenaName::Mountain => "Mountain",
            ArenaName::Bastion => "Bastion",
            ArenaName::Pawnshop => "Pawnshop",
            ArenaName::Crucible => "Crucible",
            ArenaName::Casino => "Casino",
            ArenaName::Gala => "Gala",
        }
    }
}

impl CurrentArena {
    pub fn inc(value: u8) -> u8 {
        (value + 1) % 9
    }

    pub fn dec(value: u8) -> u8 {
        if value == 0 { 8 } else { value - 1 }
    }
}

pub const TILE_SIZE: f32 = 19.0;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const HALF_WINDOW_WIDTH: f32 = WINDOW_WIDTH / 2.0;
pub const HALF_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;
pub const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;
pub const GRID_WIDTH: usize = 66;
pub const GRID_HEIGHT: usize = 31;
pub const ARENA_WIDTH: f32 = GRID_WIDTH as f32 * TILE_SIZE;
pub const ARENA_HEIGHT: f32 = GRID_HEIGHT as f32 * TILE_SIZE;

pub const CAMERA_PADDING_X: f32 = -22.0;
pub const CAMERA_PADDING_Y: f32 = 36.0;
pub const SIDEBAR_WIDTH: f32 = 13.5;

// Helper function to calculate camera position to center on arena
fn calculate_camera_position(arena_index: u8) -> (f32, f32) {
    let arena_col = arena_index % 3;
    let arena_row = arena_index / 3;

    // Calculate arena top-left corner (matching setup positioning)
    let arena_x = -SIDEBAR_WIDTH + (arena_col as f32 * ARENA_WIDTH);
    let arena_y = CAMERA_PADDING_Y - (arena_row as f32 * ARENA_HEIGHT);

    // Calculate arena center by adding half arena dimensions
    let center_x = arena_x;
    let center_y = arena_y;

    // Shift right by one tile size to correct alignment
    (center_x, center_y)
}

// Helper function to calculate character position within an arena
fn calculate_character_position(arena_index: u8, tile_x: usize, tile_y: usize) -> (f32, f32) {
    let arena_col = arena_index % 3;
    let arena_row = arena_index / 3;

    // Calculate arena top-left corner (matching setup positioning)
    let arena_x = -HALF_WINDOW_WIDTH + HALF_TILE_SIZE + (arena_col as f32 * ARENA_WIDTH);
    let arena_y = HALF_WINDOW_HEIGHT - HALF_TILE_SIZE - (arena_row as f32 * ARENA_HEIGHT);

    // Calculate character position within the arena
    let char_x = arena_x + (tile_x as f32 * TILE_SIZE);
    let char_y = arena_y - (tile_y as f32 * TILE_SIZE);

    (char_x, char_y)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Arenic".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (
                setup,
                spawn_player_selected,
                spawn_top_nav_bar,
                spawn_side_nav_bars,
                spawn_bottom_nav_bar,
            ),
        )
        .add_systems(
            Update,
            (
                handle_arena_navigation_keys,
                update_camera_on_arena_change,
                handle_zoom_toggle,
                draw_arena_gizmo,
                move_selected_player,
                cycle_selected_character,
                update_character_sprites,
                update_character_arena_markers,
                sync_current_arena_with_selected_character,
                ensure_character_selected_in_current_arena,
                debug_character_arena_changes,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let arena_col_x = 0 % 3;
    let arena_row_x = 2 / 3;
    commands.spawn(CurrentArena(1));
    let (camera_x, camera_y) = calculate_camera_position(1);
    commands
        .spawn(Camera2d)
        .insert(Transform::from_xyz(camera_x, camera_y, 0.0))
        .insert(Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scale: 1.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
            scaling_mode: Default::default(),
        }));

    for arena_index in 0..9 {
        let arenas_per_row = 3;
        let arena_col = arena_index % arenas_per_row;
        let arena_row = arena_index / arenas_per_row;

        let x_offset = arena_col as f32 * ARENA_WIDTH;
        let y_offset = arena_row as f32 * ARENA_HEIGHT;

        let mut arena = commands.spawn(Transform::from_xyz(
            -HALF_WINDOW_WIDTH + HALF_TILE_SIZE + x_offset,
            HALF_WINDOW_HEIGHT - HALF_TILE_SIZE - y_offset,
            0.0,
        ));
        let image_path = format!("Grid_{}.png", arena_index);
        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
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
                                col as f32 * TILE_SIZE,
                                -(row as f32 * TILE_SIZE),
                                0.0,
                            ));
                    });
            }
        }
    }
}

fn handle_arena_navigation_keys(
    mut arena_query: Query<&mut CurrentArena>,
    camera_query: Query<&Projection, With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // Check if camera is at scale 3.0
    let is_zoomed_out = camera_query.iter().any(|projection| {
        if let Projection::Orthographic(ortho) = projection {
            ortho.scale == 3.0
        } else {
            false
        }
    });

    if input.just_pressed(KeyCode::BracketRight) {
        for mut arena in &mut arena_query {
            arena.0 = CurrentArena::inc(arena.0);
        }
    }
    if input.just_pressed(KeyCode::BracketLeft) {
        for mut arena in &mut arena_query {
            arena.0 = CurrentArena::dec(arena.0);
        }
    }
}

fn update_camera_on_arena_change(
    arena_query: Query<&CurrentArena, Changed<CurrentArena>>,
    mut camera_query: Query<(&mut Transform, &Projection), With<Camera>>,
) {
    if let Ok(current_arena) = arena_query.single() {
        let (camera_x, camera_y) = calculate_camera_position(current_arena.0);

        for (mut transform, projection) in &mut camera_query {
            // Only move camera if not zoomed out (scale 1.0)
            if let Projection::Orthographic(ortho) = projection {
                if ortho.scale == 1.0 {
                    transform.translation.x = camera_x;
                    transform.translation.y = camera_y;
                }
            }
        }
    }
}

fn handle_zoom_toggle(
    arena_query: Query<&CurrentArena>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        for (mut transform, mut projection) in &mut camera_query {
            if let Projection::Orthographic(ortho) = &mut *projection {
                if ortho.scale == 1.0 {
                    ortho.scale = 3.0;
                    // Center on arena index 4 for zoom out and move down by TILE_SIZE * 3
                    let (camera_x, camera_y) = calculate_camera_position(4);
                    transform.translation.x = camera_x;
                    transform.translation.y = camera_y - (TILE_SIZE * 3.0);
                } else {
                    ortho.scale = 1.0;
                    // Return to current arena position (without Y offset)
                    for arena in &arena_query {
                        let (camera_x, camera_y) = calculate_camera_position(arena.0);
                        transform.translation.x = camera_x;
                        transform.translation.y = camera_y;
                    }
                }
            }
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
            if ortho.scale == 3.0 {
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
    commands
        .spawn(Sprite {
            image: asset_server.load("player_selected.png"),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        })
        .insert(Transform::from_xyz(char1_x, char1_y, 1.0))
        .insert(Character)
        .insert(CharacterSelected);

    // Spawn second character at tile position (30, 15) in arena 1 (3 tiles to the left)
    let (char2_x, char2_y) = calculate_character_position(1, 30, 15);
    commands
        .spawn(Sprite {
            image: asset_server.load("player_unselected.png"),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        })
        .insert(Transform::from_xyz(char2_x, char2_y, 1.0))
        .insert(Character);
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
        
        // Calculate boundaries of the entire 3x3 arena grid
        let grid_left = -HALF_WINDOW_WIDTH + HALF_TILE_SIZE;
        let grid_right = grid_left + (3.0 * ARENA_WIDTH) - TILE_SIZE;
        let grid_top = HALF_WINDOW_HEIGHT - HALF_TILE_SIZE;
        let grid_bottom = grid_top - (3.0 * ARENA_HEIGHT) + TILE_SIZE;
        
        // Clamp position to stay within the 3x3 grid boundaries
        new_x = new_x.clamp(grid_left, grid_right);
        new_y = new_y.clamp(grid_bottom, grid_top);
        
        // Apply the clamped position
        transform.translation.x = new_x;
        transform.translation.y = new_y;
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

        // Find current selected character index
        let current_selected = selected_query.single();

        let next_index = match current_selected {
            Ok(selected_entity) => {
                // Find the current index and get next index cyclically
                if let Some(current_index) = characters.iter().position(|&e| e == selected_entity) {
                    (current_index + 1) % characters.len()
                } else {
                    0 // Default to first if not found
                }
            }
            Err(_) => 0, // No current selection, start with first
        };

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
            sprite.image = asset_server.load("player_selected.png");
        } else {
            sprite.image = asset_server.load("player_unselected.png");
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

fn spawn_top_nav_bar(mut commands: Commands) {
    // Calculate the navigation bar height based on CAMERA_PADDING_Y + 1 pixel
    let nav_bar_height = CAMERA_PADDING_Y.abs() + 1.0;

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(nav_bar_height),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        })
        .insert(BackgroundColor(Color::WHITE))
        .insert(TopNavBar);
}

fn spawn_side_nav_bars(mut commands: Commands) {
    // Calculate the sidebar width based on CAMERA_PADDING_Y + 1 pixel
    let sidebar_width = SIDEBAR_WIDTH;

    // Spawn left sidebar
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Px(sidebar_width),
            height: Val::Percent(100.0),
            ..default()
        })
        .insert(BackgroundColor(Color::WHITE.with_alpha(0.5)))
        .insert(SideNavBar);

    // Spawn right sidebar
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            width: Val::Px(sidebar_width),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        })
        .insert(BackgroundColor(Color::WHITE.with_alpha(0.5)))
        .insert(SideNavBar);
}

fn spawn_bottom_nav_bar(mut commands: Commands) {
    // Calculate the bottom navigation bar height based on TILE_SIZE * 5
    let nav_bar_height = TILE_SIZE * 5.0;

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(nav_bar_height),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        })
        .insert(BackgroundColor(Color::WHITE.with_alpha(0.5)))
        .insert(BottomNavBar);
}
