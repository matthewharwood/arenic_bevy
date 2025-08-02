## Instruction to setup prototyping

0. ReadFile MOST IMPORTANTLY YOU MUST FOLLOW CODE FROM THIS FILE `_docs/MIGRATION.md`
1. ReadFile .claude/agents/jon-game-engineer.md
2. You are Jon going forward.
3. Make a 2d Camera
4. Make a 2dMesh Circle Shape that is black and 19x19 pixels.

```rust
const TILE_SIZE: f32 = 19.0;
fn spawn_hero_shape(
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
            Hero,
            // character_state.selected_character,
        ));
    });
}
```

4. Make another character

```rust
const TILE_SIZE: f32 = 19.0;
fn spawn_boss_shape(
    mut commands: Commands,
    guild_house_query: Single<Entity, With<GuildHouse>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let guild_house_entity = guild_house_query.into_inner();

    let tile_x = 30.0;
    let tile_y = -15.0;

    // Convert tile coordinates to world position within the arena
    let world_x = tile_x * TILE_SIZE;
    let world_y = tile_y * TILE_SIZE;

    commands.entity(guild_house_entity).with_children(|parent| {
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(TILE_SIZE / 2.0))),
            MeshMaterial2d(materials.add(Colors::PRIMARY)),
            Transform::from_xyz(world_x, world_y, 1.0),
            Boss,
        ));
    });
}
```

8. ReadFile `_docs/abilities/potential_ability_components.md`
9. Pick a set of random components. Make a detailed plan on implementing this ability between Hero & Boss.
10. As you play the Hero in this game. Your goal is to attack the boss with the ability that you created.
11. Make sure plan has code snippets that follow the spec outlined here: `.claude/agents/jon-game-engineer.md`
12. WriteFile write code that compiles with tests that ensure the code works as intended.
13. REMEMBER ANY COMPILER ERRORS OR WARNINGS, ReadFile `_docs/MIGRATIONS.md` and revise the code to follow this guide
14. WriteFile write a README.md file for that ability in the `_docs/prototyping/` dir give that markdown file a creative
    name of the ability that you created that serves as a tutorial.
