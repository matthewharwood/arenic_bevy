# Camera System Documentation

## Overview

The camera system provides orthographic projection for viewing the 3x3 grid of arenas. It supports centering on specific arenas and maintains consistent scaling across different window sizes.

## 3D Coordinate System and Orientation

### World Space Layout

```
                            Y-axis (Up)
                            ^
                            |
                            |
    (-640, 360, 0) +--------+--------+--------+ (640, 360, 0)
                   |        |        |        |
                   |Arena 0 |Arena 1 |Arena 2 |
                   |        |        |        |
                   +--------+--------+--------+
                   |        |        |        |
                   |Arena 3 |Arena 4 |Arena 5 |
                   |        |        |        |
                   +--------+--------+--------+
                   |        |        |        |
                   |Arena 6 |Arena 7 |Arena 8 |
                   |        |        |        |
    (-640,-360, 0) +--------+--------+--------+ (640,-360, 0) --> X-axis (Right)
                                               /
                                              /
                                             v
                                          Z-axis (Out of screen)
```

### 3D View - Side Perspective (Looking along X-axis)

```
    Y (Up)
    ^
    |     Camera (0, 0, 1000)
    |         👁️ 
    |         /|
    |        / |
    |       /  | 1000 units
    |      /   |
    |     /    |
    |    /     |
    |   /      v
    +--⬛⬛⬛⬛⬛⬛⬛⬛⬛-- Z=0 (Arena Grid)
    |  └─ 9 Arenas
    |
    v
   -Y (Down)
```

### 3D View - Isometric Perspective

```
                      Camera
                        👁️ (x, y, z+1000)
                       ╱ |
                     ╱   |
                   ╱     | Looking at (x, y, z)
                 ╱       | with Y as "up"
               ╱         |
             ╱           v
           ╱    ┌─────────────────┐
         ╱      │ Arena Grid (3x3) │
       ╱        │   Z = 0 plane    │
     Y↗         │ ┌───┬───┬───┐   │
     │        ╱ │ │ 0 │ 1 │ 2 │   │
     │      ╱   │ ├───┼───┼───┤   │
     │    ╱     │ │ 3 │ 4 │ 5 │   │
     │  ╱       │ ├───┼───┼───┤   │
     │╱         │ │ 6 │ 7 │ 8 │   │
     └──────X   │ └───┴───┴───┘   │
    ╱           └─────────────────┘
  ╱
Z (Out)
```

## Key Concepts

### Arena Layout
- **9 Arenas** arranged in a 3x3 grid
- Each arena is 66×31 tiles (1254×589 world units)
- Arenas are numbered 0-8, left to right, top to bottom
- The entire grid fits within a 1280×720 window

### Coordinate System Details
- **Origin (0,0,0)**: Center of the window
- **X-axis**: Positive right, negative left
- **Y-axis**: Positive up, negative down (Bevy's default)
- **Z-axis**: Positive out of screen (towards camera)

### Camera Properties
- **Type**: Orthographic projection (no perspective distortion)
- **Position**: 1000 units above the arena on Z-axis
- **Look Target**: Center of the selected arena
- **Up Vector**: +Y (Vec3::Y)
- **Scaling Mode**: WindowSize (1:1 pixel mapping)

### Arena Positioning
Each arena's top-left corner is calculated as:
```rust
x = -HALF_WINDOW_WIDTH + (col * ARENA_WIDTH) + HALF_TILE
y = HALF_WINDOW_HEIGHT - (row * ARENA_HEIGHT) - HALF_TILE
```

Where:
- `HALF_WINDOW_WIDTH` = 640.0
- `HALF_WINDOW_HEIGHT` = 360.0
- `ARENA_WIDTH` = 1254.0 (66 tiles × 19 units/tile)
- `ARENA_HEIGHT` = 589.0 (31 tiles × 19 units/tile)
- `HALF_TILE` = 9.5 (offset to align tile centers)

## Camera Functions

### `get_arena_position(arena_index: u32) -> Vec3`
Returns the world position of an arena's top-left corner.

### `calculate_camera_position(arena_index: u8) -> Vec3`
Calculates the center point of an arena for camera targeting.

### `setup_camera(commands: &mut Commands, arena_index: u8)`
Spawns a camera entity centered on the specified arena.

## Usage Example

```rust
// Center camera on arena 4 (middle arena)
arena_camera::setup_camera(&mut commands, 4);
```

## Blender to Bevy Asset Pipeline

### Coordinate System Transformation

Blender and Bevy have different default orientations for objects:

#### Blender Default Orientation
```
    Y (Up - Top face points here)
    ^
    |  ┌─────┐
    | /     /|
    |/     / |
    +─────+  | --> X
   /|     |  |
  / |     | /
 Z  |     |/
    +─────+
```

#### Required Orientation for Bevy Camera
```
    Y (Up - Used as camera "up" vector)
    ^
    |     Z (Top face must point here)
    |    ^
    |   /  ┌─────┐
    |  /  /     /|
    | /  /     / |
    |/  +─────+  | --> X
    +  |     |  |
      |     | /
      |     |/
      +─────+
```

### Why +90° X Rotation is Required

Since the camera looks down from +Z towards -Z, the top face of tiles must point towards +Z to be visible. In Blender:
1. Default cube has top face pointing to +Y
2. Rotating +90° on X axis makes top face point to +Z
3. This aligns with Bevy's camera view direction

### Blender Export Steps for Proper Camera Alignment

1. **Model your tile** with dimensions 19×19×2 units (matching `TILE_SIZE` and `TILE_HEIGHT`)

2. **Apply the critical rotation**:
   ```
   - Select your tile in Object Mode
   - Press N to open properties panel
   - Rotate +90° on X axis
   - Apply rotation (Ctrl+A → Rotation)
   ```

3. **Verify orientation**:
   - Enable Face Orientation overlay
   - The top face (with any special features like insets) should show blue
   - Blue faces should point towards +Z

4. **Export settings**:
   - File → Export → glTF 2.0 (.glb)
   - Enable: Apply Modifiers, Selected Objects, Apply Transform
   - Ensure +Y Up orientation is selected
   - Save as `assets/tile.glb`

5. **Test in Bevy**:
   ```rust
   commands.spawn(SceneRoot(asset_server.load("tile.glb#Scene0")));
   ```

### Visual Guide: Tile Orientation

```
WRONG (Top face not visible):          CORRECT (After +90° X rotation):
                                       
    Camera 👁️                              Camera 👁️
       |                                      |
       v                                      v
    ───────  (Side view visible)           ┌─────┐ (Top view visible)
                                          │░░░░░│ (Inset/details visible)
                                          │░░░░░│
                                          └─────┘
```

## Important Notes

1. **Orthographic Projection**: All tiles appear the same size regardless of distance
2. **1:1 Pixel Mapping**: Each world unit maps to exactly one pixel
3. **Z-ordering**: Entities with higher Z values appear in front
4. **Arena Centers**: The camera focuses on the geometric center of each arena
5. **No Rotation**: The camera always looks straight down (no tilt or rotation)
6. **Blender Export**: Always rotate tiles +90° on X axis before exporting to ensure proper camera visibility