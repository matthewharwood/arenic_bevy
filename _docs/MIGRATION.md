# Comprehensive Bevy Migration Guide: 0.13 → 0.16

This guide covers all breaking changes when migrating from Bevy 0.13 to 0.16, with emphasis on major architectural
shifts that fundamentally change how Bevy applications are structured.

## 🚨 Major Architectural Changes Overview

Bevy 0.14-0.16 introduces several paradigm shifts that affect the entire framework design philosophy:

1. **Bundle Elimination**: Bundles are being phased out in favor of required components and component constructors
2. **Required Components Pattern**: Components can now automatically include other components, eliminating manual bundle
   management
3. **Typed Asset Components**: Generic `Handle<T>` components replaced with specific typed components
4. **Error-Safe ECS**: Query and world access methods now return `Result` types instead of panicking
5. **Graph-Based Animation**: Complete replacement of clip-based animation with animation graphs
6. **Color Space Specificity**: Single `Color` enum replaced with specific color space structs

---

## Bundle System Elimination (MAJOR ARCHITECTURAL SHIFT)

### ⚠️ DEPRECATED: All Bundle Patterns

**The bundle system is being completely phased out.** Bundles were the primary method for spawning entities with
multiple related components, but Bevy now uses required components and component constructors instead.

### Camera Bundles → Camera Components

```rust
// 0.13/0.14/0.15 - DEPRECATED
commands.spawn(Camera2dBundle::default ());
commands.spawn(Camera3dBundle::default ());
commands.spawn(PerspectiveCameraBundle {
camera: Camera::default (),
perspective_projection: PerspectiveProjection::default (),
transform: Transform::from_xyz(0.0, 0.0, 5.0),
..default ()
});

// 0.16 - NEW REQUIRED COMPONENT PATTERN
commands.spawn(Camera2d);
commands.spawn(Camera3d);
commands.spawn((
Camera3d,
Transform::from_xyz(0.0, 0.0, 5.0),
Projection::Perspective(PerspectiveProjection::default ()),
));
```

**Migration Strategy**: Replace all camera bundles with individual camera components. The camera components now
automatically include required components like `Camera`, `GlobalTransform`, `ViewVisibility`, etc.

### Sprite Bundles → Sprite Components

```rust
// 0.13/0.14/0.15 - DEPRECATED
commands.spawn(SpriteBundle {
texture: asset_server.load("sprite.png"),
transform: Transform::from_xyz(0.0, 0.0, 0.0),
sprite: Sprite {
color: Color::RED,
..default ()
},
..default ()
});

// 0.16 - NEW COMPONENT CONSTRUCTOR PATTERN
commands.spawn((
Sprite {
image: asset_server.load("sprite.png"),
color: Color::srgb(1.0, 0.0, 0.0),
..default ()
},
Transform::from_xyz(0.0, 0.0, 0.0),
));
```

**Migration Strategy**: Replace `SpriteBundle` with `Sprite` component. The image field moves from a separate component
to within the `Sprite` struct itself.

### Mesh Bundles → Mesh Components

```rust
// 0.13/0.14/0.15 - DEPRECATED
commands.spawn(MaterialMeshBundle {
mesh: meshes.add(Mesh::from(Cuboid::default ())),
material: materials.add(Color::RED),
transform: Transform::from_xyz(0.0, 0.0, 0.0),
..default ()
});

// 0.16 - NEW TYPED COMPONENT PATTERN
commands.spawn((
Mesh3d(meshes.add(Mesh::from(Cuboid::default ()))),
MeshMaterial3d(materials.add(StandardMaterial {
base_color: Color::srgb(1.0, 0.0, 0.0),
..default ()
})),
Transform::from_xyz(0.0, 0.0, 0.0),
));
```

**Migration Strategy**: Replace `MaterialMeshBundle` with `Mesh3d` and `MeshMaterial3d` components. This eliminates the
generic `Handle<T>` pattern in favor of specifically typed components.

### UI Bundles → UI Components

```rust
// 0.13/0.14/0.15 - DEPRECATED
commands.spawn(NodeBundle {
style: Style {
width: Val::Px(200.0),
height: Val::Px(100.0),
..default ()
},
background_color: Color::BLUE.into(),
..default ()
});

commands.spawn(TextBundle::from_section(
"Hello World",
TextStyle {
font: asset_server.load("fonts/FiraSans-Bold.ttf"),
font_size: 40.0,
color: Color::WHITE,
},
));

// 0.16 - NEW REQUIRED COMPONENT PATTERN
commands.spawn((
Node {
width: Val::Px(200.0),
height: Val::Px(100.0),
..default ()
},
BackgroundColor(Color::srgb(0.0, 0.0, 1.0)),
));

commands.spawn((
Text::new("Hello World"),
TextFont {
font: asset_server.load("fonts/FiraSans-Bold.ttf"),
font_size: 40.0,
..default ()
},
TextColor(Color::srgb(1.0, 1.0, 1.0)),
));
```

**Migration Strategy**: Replace all UI bundles with individual components. UI components now use required components to
automatically include necessary functionality like `GlobalTransform`, `ViewVisibility`, and `InheritedVisibility`.

### Audio Bundles → Audio Components

```rust
// 0.13/0.14/0.15 - DEPRECATED
commands.spawn(AudioSourceBundle {
source: asset_server.load("sounds/music.ogg"),
settings: PlaybackSettings::LOOP,
..default ()
});

// 0.16 - NEW AUDIO COMPONENT PATTERN
commands.spawn((
AudioPlayer(asset_server.load("sounds/music.ogg")),
PlaybackSettings::LOOP,
));
```

## Camera Bundles

- `Camera2dBundle` → `Camera2d`
- `Camera3dBundle` → `Camera3d`
- `PerspectiveCameraBundle` → `(Camera3d, Transform, Projection::Perspective(PerspectiveProjection))`
- `OrthographicCameraBundle` → `(Camera3d, Transform, Projection::Orthographic(OrthographicProjection))`

## Sprite Bundles

- `SpriteBundle` → `Sprite` (with `image` field integrated)
- `SpriteSheetBundle` → `(Sprite, TextureAtlas)` (with sprite sheet data integrated)

## Mesh/Material Bundles

- `MaterialMeshBundle` → `(Mesh3d(...), MeshMaterial3d(...))`
- `MaterialMesh2dBundle` → `(Mesh2d(...), MeshMaterial2d(...))`
- `PbrBundle` → `(Mesh3d(...), MeshMaterial3d<StandardMaterial>(...))`

## UI Bundles

- `NodeBundle` → `Node` (with style properties integrated)
- `TextBundle` → `(Text::new(...), TextFont, TextColor)`
- `ButtonBundle` → `(Button, Node)` (with style in Node)
- `ImageBundle` → `(UiImage, Node)`

## Audio Bundles

- `AudioSourceBundle` → `(AudioPlayer(...), PlaybackSettings)`
- `SpatialAudioSourceBundle` → `(AudioPlayer(...), SpatialSettings, PlaybackSettings)`

## Light Bundles

- `PointLightBundle` → `PointLight`
- `DirectionalLightBundle` → `DirectionalLight`
- `SpotLightBundle` → `SpotLight`

## Transform Bundles

- `TransformBundle` → `Transform` (GlobalTransform automatically included via required components)
- `SpatialBundle` → `(Transform, Visibility)` (other spatial components auto-included)

## Additional Bundle Patterns

- Generic asset bundles with `Handle<T>` → Typed components like `Mesh3d(Handle<Mesh>)`,
  `MeshMaterial3d(Handle<Material>)`

The key architectural change is that components now use the `#[require()]` attribute to automatically include their
dependencies, eliminating the need for bundles entirely. Required components like `GlobalTransform`, `ViewVisibility`,
`InheritedVisibility` are automatically added when spawning entities with components that require them.

---

## Required Components System (NEW ARCHITECTURAL PATTERN)

### Understanding Required Components

Required components automatically add dependent components when a component is inserted, eliminating the need for
bundles.

```rust
// 0.16 - Component with requirements
#[derive(Component)]
#[require(Transform, Visibility)]
struct Player;

// When spawning, only specify the primary component
commands.spawn(Player);
// Transform and Visibility are automatically added
```

### Custom Required Components

```rust
// 0.16 - Define custom requirements
#[derive(Component)]
#[require(
    Transform,
    GlobalTransform,
    Visibility,
    InheritedVisibility,
    ViewVisibility,
    Health(|| Health(100))  // With default value
)]
struct Enemy;

fn spawn_enemy(mut commands: Commands) {
    commands.spawn((
        Enemy,
        // Only specify non-default values
        Transform::from_xyz(10.0, 0.0, 0.0),
    ));
}
```

**Migration Strategy**: When creating custom components that previously required bundles, use the `#[require()]`
attribute to specify dependent components automatically.

---

## Asset Handle System Overhaul (MAJOR ARCHITECTURAL SHIFT)

### ⚠️ DEPRECATED: Generic Handle<T> Components

**Generic asset handles as components are being eliminated** in favor of specifically typed asset components.

### Mesh Asset Handles

```rust
// 0.13/0.14/0.15 - DEPRECATED
commands.spawn((
Handle::<Mesh>::default (),
Handle::<StandardMaterial>::default (),
));

// 0.16 - NEW TYPED COMPONENT SYSTEM
commands.spawn((
Mesh3d(meshes.add(Mesh::from(Sphere::default ()))),
MeshMaterial3d(materials.add(StandardMaterial::default ())),
));
```

### Image Asset Handles

```rust
// 0.13/0.14/0.15 - DEPRECATED
commands.spawn((
Handle::<Image>::default (),
Sprite::default (),
));

// 0.16 - NEW INTEGRATED COMPONENT SYSTEM
commands.spawn(Sprite {
image: asset_server.load("texture.png"),
..default ()
});
```

### Audio Asset Handles

```rust
// 0.13/0.14/0.15 - DEPRECATED
commands.spawn(Handle::<AudioSource>::default ());

// 0.16 - NEW AUDIO PLAYER SYSTEM
commands.spawn(AudioPlayer::<AudioSource>(asset_server.load("sound.ogg")));
```

**Migration Strategy**: Replace all generic `Handle<T>` components with the new typed components. This provides better
type safety and eliminates the need for separate handle and configuration components.

---

## Animation System Complete Overhaul (MAJOR ARCHITECTURAL SHIFT)

### ⚠️ DEPRECATED: Clip-Based Animation

**The entire clip-based animation system has been replaced** with a graph-based animation system that uses UUIDs instead
of hierarchical paths.

### Animation Player Setup

```rust
// 0.13/0.14/0.15 - DEPRECATED CLIP SYSTEM
fn setup_animation(
    mut commands: Commands,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    let mut animation = AnimationClip::default();
    // Add curves directly to clip
    animation.add_curve_to_target(
        AnimationTargetId::from_name(&Name::new("Bone")),
        VariableCurve {
            keyframe_timestamps: vec![0.0, 1.0],
            keyframes: Keyframes::Rotation(vec![Quat::IDENTITY, Quat::from_rotation_y(PI)]),
            interpolation: Interpolation::Linear,
        },
    );

    let mut player = AnimationPlayer::default();
    player.play(animations.add(animation));
    commands.spawn(player);
}

// 0.16 - NEW GRAPH-BASED SYSTEM
fn setup_animation(
    mut commands: Commands,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut animation = AnimationClip::default();
    // Use new curve constructors
    animation.add_curve_to_target(
        AnimationTargetId::from_name(&Name::new("Bone")),
        AnimatableKeyframeCurve::new([0.0, 1.0].into_iter().zip([
            Quat::IDENTITY,
            Quat::from_rotation_y(PI),
        ]))
            .map(RotationCurve)
            .expect("Failed to build rotation curve"),
    );

    // Create animation graph
    let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));
    let mut player = AnimationPlayer::default();
    player.play(animation_index);

    commands.spawn((
        player,
        AnimationGraphHandle(graphs.add(graph)),
    ));
}
```

### Animation Transitions and Blending

```rust
// 0.16 - NEW GRAPH SYSTEM SUPPORTS COMPLEX BLENDING
fn setup_complex_animation(
    mut commands: Commands,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut graph = AnimationGraph::new();
    let blend_node = graph.add_blend();
    let clip1 = graph.add_clip(clip1_handle, 1.0, blend_node);
    let clip2 = graph.add_clip(clip2_handle, 1.0, blend_node);

    commands.spawn((
        AnimationPlayer::default(),
        AnimationGraphHandle(graphs.add(graph)),
    ));
}
```

**Migration Strategy**: Completely rewrite animation code to use the new graph-based system. This requires restructuring
how animations are created, managed, and played. The new system provides much more powerful blending and transition
capabilities but requires understanding the graph architecture.

---

## Color System Complete Redesign (MAJOR ARCHITECTURAL SHIFT)

### ⚠️ DEPRECATED: Single Color Enum

**The monolithic Color enum has been completely replaced** with specific color space structs for better type safety and
color accuracy.

### Color Creation and Usage

```rust
// 0.13/0.14/0.15 - DEPRECATED COLOR SYSTEM
let red = Color::rgb(1.0, 0.0, 0.0);
let blue = Color::BLUE;
let hex_color = Color::hex("#FF0000").unwrap();
let hsla = red.as_hsla();
let linear = Color::rgb_linear(1.0, 0.0, 0.0);

// Modify color channels
let mut color = Color::WHITE;
color.set_a(0.5);
let alpha = color.a();

// 0.16 - NEW COLOR SPACE SYSTEM
use bevy::color::palettes::css::{RED, BLUE};
let red = Color::srgb(1.0, 0.0, 0.0);
let blue = BLUE;
let hex_color = Color::from(Srgba::hex("#FF0000").unwrap());
let hsla: Hsla = Srgba::srgb(1.0, 0.0, 0.0).into();
let linear = Color::linear_rgb(1.0, 0.0, 0.0);

// Modify color channels with specific color space
let srgba = Srgba::WHITE.with_alpha(0.5);
let color = Color::from(srgba);
let alpha = srgba.alpha;
```

### Color Space Conversions

```rust
// 0.16 - EXPLICIT COLOR SPACE HANDLING
// Convert between color spaces
let srgb = Srgba::srgb(1.0, 0.0, 0.0);
let linear: LinearRgba = srgb.into();
let hsla: Hsla = srgb.into();
let oklcha: Oklcha = srgb.into();

// Work with specific color spaces for better accuracy
let warm_color = Oklcha {
lightness: 0.7,
chroma: 0.15,
hue: 50.0,
alpha: 1.0,
};
```

**Migration Strategy**: Replace all `Color::rgb()` calls with `Color::srgb()` and import specific color constants from
`bevy::color::palettes`. Use appropriate color space structs when you need to manipulate color channels or perform color
operations.

---

## Error-Safe ECS Operations (MAJOR ARCHITECTURAL SHIFT)

### ⚠️ DEPRECATED: Panicking Query Methods

**Query and world access methods that previously panicked now return Result types** for safer error handling.

### Query Access Patterns

```rust
// 0.13/0.14/0.15 - DEPRECATED PANICKING METHODS
fn system(
    player_query: Query<&Transform, With<Player>>,
    entities: Query<&Transform>,
    players: Res<Players>,
) {
    // These methods panic if entity doesn't exist or constraints aren't met
    let player_transform = player_query.single();
    let [transform1, transform2] = entities.many([players.player1, players.player2]);
    let transform = entities.get(players.player1).unwrap();
}

// 0.16 - NEW RESULT-BASED ERROR HANDLING
fn system(
    player_query: Query<&Transform, With<Player>>,
    entities: Query<&Transform>,
    players: Res<Players>,
) -> Result<(), Box<dyn std::error::Error>> {
    // All methods return Results for safe error handling
    let player_transform = player_query.single()?;
    let [transform1, transform2] = entities.get_many([players.player1, players.player2])?;
    let transform = entities.get(players.player1)?;
    Ok(())
}
```

### World Entity Access

```rust
// 0.13/0.14/0.15 - DEPRECATED PANICKING ACCESS
fn update_entity(world: &mut World, entity: Entity) {
    let mut entity_mut = world.get_entity_mut(entity).unwrap();
    entity_mut.insert(Health(100));
}

// 0.16 - NEW SAFE ACCESS PATTERNS
fn update_entity(world: &mut World, entity: Entity) -> Result<(), String> {
    let mut entity_mut = world.get_entity_mut(entity)
        .ok_or("Entity does not exist")?;
    entity_mut.insert(Health(100));
    Ok(())
}
```

**Migration Strategy**: Update all query and world access code to handle Result types. This prevents runtime panics and
makes error conditions explicit. Consider using the `?` operator for clean error propagation.

---

## Event System API Modernization

### ⚠️ DEPRECATED: EventWriter.send()

**Event writing methods have been renamed** for consistency with other Rust APIs.

```rust
// 0.13/0.14/0.15 - DEPRECATED SEND METHODS
fn fire_events(mut event_writer: EventWriter<PlayerDied>) {
    event_writer.send(PlayerDied { player_id: 1 });
    event_writer.send_batch(vec![
        PlayerDied { player_id: 2 },
        PlayerDied { player_id: 3 },
    ]);
    event_writer.send_default();
}

// 0.16 - NEW WRITE METHODS
fn fire_events(mut event_writer: EventWriter<PlayerDied>) {
    event_writer.write(PlayerDied { player_id: 1 });
    event_writer.write_batch(vec![
        PlayerDied { player_id: 2 },
        PlayerDied { player_id: 3 },
    ]);
    event_writer.write_default();
}
```

**Migration Strategy**: Replace all `send` method calls with `write` method calls. This aligns with Rust's standard
library naming conventions for write operations.

---

## Application Lifecycle Changes

### App::run() Return Type

```rust
// 0.13/0.14/0.15 - VOID RETURN
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}

// 0.16 - APPEXXIT RETURN TYPE
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .run()
}
```

### AppExit Event Structure

```rust
// 0.13/0.14/0.15 - UNIT STRUCT
fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.write(AppExit);
}

// 0.16 - RICH EXIT CODES
fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.write(AppExit::Success);
    // Or for error conditions:
    exit.write(AppExit::Error(NonZeroU8::new(1).unwrap()));
}
```

**Migration Strategy**: Update main function signatures to return `AppExit` and use the new structured exit codes for
better application lifecycle management.

---

## Asset System Modernization

### LoadContext API Restructuring

```rust
// 0.13/0.14/0.15 - DEPRECATED DIRECT METHODS
impl AssetLoader for MyLoader {
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let dependency = load_context.load_direct("dependency.asset");
            let untyped = load_context.load_untyped("other.asset");
            // Process asset...
            Ok(my_asset)
        })
    }
}

// 0.16 - NEW BUILDER PATTERN
impl AssetLoader for MyLoader {
    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let dependency = load_context.loader()
            .immediate()
            .load("dependency.asset");
        let untyped = load_context.loader()
            .with_unknown_type()
            .load("other.asset");
        // Process asset...
        Ok(my_asset)
    }
}
```

**Migration Strategy**: Replace direct LoadContext methods with the new builder pattern. This provides more explicit
control over loading behavior and asset type handling.

---

## Platform and Module Reorganization

### ⚠️ DEPRECATED: bevy_core Crate

**The entire bevy_core crate has been removed** and its functionality distributed to other crates.

```rust
// 0.13/0.14/0.15 - DEPRECATED IMPORTS
use bevy_core::{FrameCount, Name, TypeRegistrationPlugin};
use bevy_utils::{HashMap, HashSet, Instant, Duration};

// 0.16 - NEW IMPORT LOCATIONS
use bevy_diagnostic::FrameCount;
use bevy_ecs::name::Name;
use bevy_platform::collections::{HashMap, HashSet};
use bevy_platform::time::Instant;
use core::time::Duration;

// Type registration is now handled differently
app.register_type::<MyComponent>();
```

### Utility Crate Reorganization

```rust
// 0.15 - OLD LOCATIONS
use bevy_utils::{StableHashMap, StableHashSet, all_tuples, assert_object_safe};

// 0.16 - REPLACEMENT PATTERNS
// StableHashMap/StableHashSet: implement manually or use external crate
// all_tuples: use variadics_please crate
// assert_object_safe: inline the assertion
fn _assert_object_safe<T: ?Sized>() {
    const _: fn() = || {
        fn assert_object_safe<T: ?Sized>() {}
        assert_object_safe::<dyn T>();
    };
}
```

**Migration Strategy**: Update all imports to use the new crate locations. Remove dependencies on removed utilities and
implement alternatives where necessary.

---

## Immediate Action Items for Migration

1. **Replace All Bundles**: Systematically replace every bundle spawn with individual components
2. **Update Asset Handles**: Convert all `Handle<T>` components to typed asset components
3. **Implement Error Handling**: Add Result return types to systems using query methods
4. **Modernize Color Usage**: Replace Color enum usage with specific color space structs
5. **Restructure Animation**: Completely rewrite animation code for the graph-based system
6. **Update Event Writing**: Change all EventWriter.send() calls to write()
7. **Fix Import Statements**: Update all import paths for reorganized modules
8. **Handle App Lifecycle**: Update main function to return AppExit

This migration represents a fundamental shift in how Bevy applications are structured. The new patterns provide better
type safety, performance, and developer experience, but require comprehensive code updates to adopt the new
architectural approaches.