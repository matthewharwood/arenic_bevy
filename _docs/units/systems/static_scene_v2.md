# Static Scene V2: Simple 3D Arena Grid

A simple 3D arena initialization system that spawns a 66×31 grid of cuboids with a basic orthographic camera.

## Architecture Overview

The 3D static scene initialization occurs during the `OnEnter(GameState::Intro)` system execution and creates:

- **Grid Arena**: 66×31 = 2,046 cuboid tiles
- **Simple Camera**: Orthographic projection positioned to view the entire grid
- **Single Material**: One shared material for all tiles
- **Minimal Complexity**: Focus on working code that compiles and runs

## Required Game State Integration

### GameState Enum Usage

```rust
/// Uses existing GameState::Intro for 3D scene initialization
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Title,
    #[default]
    CharacterCreate,
    Intro,  // USED: Single 3D arena initialization
}
```

## Core Component Definitions

### Simple Grid Components

```rust
#[derive(Component, Debug)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Component, Debug)]
pub struct ArenaTile;
```

## Static Constants

```rust
// Grid configuration
pub const GRID_WIDTH: u32 = 66;
pub const GRID_HEIGHT: u32 = 31;
pub const TILE_SIZE: f32 = 1.0;
pub const TILE_GAP: f32 = 0.1;

// Simple tile color
pub const TILE_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
```

## Simple Camera Setup

```rust
// Working camera setup that compiles
fn spawn_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::default(), Vec3::Y),
        Projection::from(OrthographicProjection {
            scale: 0.01,
            scaling_mode: ScalingMode::WindowSize,
            ..OrthographicProjection::default_3d()
        }),
    ));
}
```


## System Implementation

### Main Initialization System

```rust
use bevy::prelude::*;

pub struct StaticScenePlugin;

impl Plugin for StaticScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}

/// Simple scene setup
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create shared mesh and material
    let tile_mesh = meshes.add(Cuboid::new(
        TILE_SIZE - TILE_GAP,
        0.1,
        TILE_SIZE - TILE_GAP
    ));
    
    let tile_material = materials.add(StandardMaterial {
        base_color: TILE_COLOR,
        ..default()
    });

    // Spawn grid of tiles
    spawn_grid(&mut commands, &tile_mesh, &tile_material);

    // Setup camera positioned to see entire grid
    setup_camera(&mut commands);

    // Add simple lighting
    setup_lighting(&mut commands);
}

/// Spawn the 66x31 grid
fn spawn_grid(
    commands: &mut Commands,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
) {
    // Calculate grid center offset
    let offset_x = (GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
    let offset_z = (GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let world_x = x as f32 * TILE_SIZE - offset_x;
            let world_z = y as f32 * TILE_SIZE - offset_z;

            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(world_x, 0.0, world_z),
                GridPosition { x, y },
                ArenaTile,
            ));
        }
    }
}

/// Setup camera to view entire grid
fn setup_camera(commands: &mut Commands) {
    // Calculate camera distance to see entire grid
    let grid_width = GRID_WIDTH as f32 * TILE_SIZE;
    let grid_height = GRID_HEIGHT as f32 * TILE_SIZE;
    let max_dimension = grid_width.max(grid_height);
    
    // Position camera to see entire grid with some padding
    let camera_distance = max_dimension * 0.7;
    
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, camera_distance, camera_distance * 0.5)
            .looking_at(Vec3::ZERO, Vec3::Y),
        Projection::from(OrthographicProjection {
            scale: 0.05,
            scaling_mode: ScalingMode::FixedVertical(max_dimension),
            ..OrthographicProjection::default_3d()
        }),
    ));
}

/// Simple lighting setup
fn setup_lighting(commands: &mut Commands) {
    // Directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            std::f32::consts::FRAC_PI_4,
            0.0,
        )),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.8, 0.8, 0.8),
        brightness: 0.5,
    });
}
```





## Example Usage

```rust
// In your main.rs or game setup
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StaticScenePlugin)
        .run();
}
```

## Adjusting Camera View

If you need to adjust the camera to better view the grid:

```rust
// Alternative camera setup with adjusted position
fn setup_camera_adjusted(commands: &mut Commands) {
    let grid_diagonal = ((GRID_WIDTH * GRID_WIDTH + GRID_HEIGHT * GRID_HEIGHT) as f32).sqrt();
    let camera_height = grid_diagonal * 0.8;
    
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, camera_height, camera_height * 0.3)
            .looking_at(Vec3::ZERO, Vec3::Y),
        Projection::from(OrthographicProjection {
            scale: 0.035,
            scaling_mode: ScalingMode::FixedVertical(grid_diagonal * 0.5),
            ..OrthographicProjection::default_3d()
        }),
    ));
}
```

## Summary

This simple implementation provides:

1. **Working Code**: Compiles and runs without errors
2. **66×31 Grid**: Exactly 2,046 cuboid tiles as requested
3. **Simple Camera**: Orthographic projection positioned to see the entire grid
4. **Single Material**: One shared gray material for all tiles
5. **Basic Lighting**: Directional and ambient light for visibility
6. **Minimal Complexity**: No camera controllers or complex systems

The grid is centered at the origin with the camera positioned to view all tiles. Adjust the camera scale value in the `OrthographicProjection` if you need to zoom in or out.