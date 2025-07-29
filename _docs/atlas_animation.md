# How Do I Make My Bevy Boss Come Alive with Animation?

**The Problem**: Your Guild Master boss sits motionless in the arena—a static image that breaks immersion and makes
combat feel lifeless.

**The Solution**: Transform that static sprite into a living, breathing character using Bevy's Entity Component System (
ECS) and sprite atlas animation.

**What You'll Build**: A fully animated Guild Master that cycles through 15 frames of smooth animation, demonstrating
the fundamental patterns you'll use for all animated characters in your game.

---

## Mental Model: Animation as a Database Query

Before diving into code, let's establish the core mental model that will anchor everything you learn:

**Think of Bevy's ECS like a database where animation happens through queries:**

- **Components are columns**: `Sprite`, `TextureAtlas`, `AnimationConfig` store your data
- **Entities are row IDs**: Each Guild Master is a unique row in this database
- **Systems are queries**: They find all entities with animation components and update them

This mental model will help you understand why we structure the code the way we do.

---

## Prerequisites & Time Investment

**Before Starting, You Should Know**:

- How to spawn entities with components in Bevy 0.16
- What the Entity Component System (ECS) pattern is
- Basic Rust syntax (structs, impl blocks, functions)

**What You'll Need**:

- The `guild_master.png` sprite atlas (15 frames, 115x115 pixels each)
- About 90 minutes total (broken into 6 focused sessions)

**Important**: This guide uses Bevy 0.16's Required Components system—no more bundles!

## Learning Path (6 Progressive Steps)

**Each step builds on the previous one and includes immediate verification:**

1. **[Understand Your Animation Components](#step-1-animation-components)** (15 min)
    - What data structures store animation state?
    - Test: Can you identify the three core components?

2. **[Master the Sprite Sheet Layout](#step-2-sprite-sheet-layout)** (15 min)
    - How does Bevy slice your atlas into frames?
    - Test: Can you predict frame boundaries?

3. **[Build the Spawn System](#step-3-spawn-system)** (15 min)
    - How do you create an animated entity?
    - Test: Does your Guild Master appear in the arena?

4. **[Implement Frame Animation](#step-4-frame-animation)** (15 min)
    - How do you cycle through frames over time?
    - Test: Does your sprite animate smoothly?

5. **[Connect to Game States](#step-5-game-states)** (15 min)
    - How do you control when animation starts/stops?
    - Test: Does animation respect game state changes?

6. **[Optimize for Multiple Characters](#step-6-optimization)** (15 min)
    - How do you handle many animated entities efficiently?
    - Test: Can you spawn 50+ animated bosses without lag?

---

## Step 1: Animation Components

*Time: 15 minutes*

### The Question: What Data Do We Need to Store?

Animation requires tracking three pieces of information. Before looking at the answer, try to guess what they are:

**Think**: What information does the computer need to know to show frame 7 of your sprite at the right time?

<details>
<summary>Click to reveal the three essential pieces</summary>

1. **Which frame to show right now** (current index)
2. **How much time between frame changes** (timing)
3. **Which frames belong to this animation** (range)

</details>

### The Animation Data Structure

Here's how Bevy stores this information in your existing codebase:

**File: `src/boss/mod.rs:14-19`**

```rust
#[derive(Component)]
pub struct BossAnimationConfig {
    pub first_frame: usize,    // Animation range start
    pub last_frame: usize,     // Animation range end  
    pub timer: Timer,          // Time tracking
}
```

**Active Recall Challenge**: Without looking back, what are the three fields in `BossAnimationConfig` and what does each
one do?

<details>
<summary>Check your answer</summary>

- `first_frame`: Which frame starts the animation (usually 0)
- `last_frame`: Which frame ends the animation (14 for Guild Master)
- `timer`: Tracks time to know when to advance to the next frame

</details>

### The Complete Component Picture

Your Guild Master entity needs these three components working together:

```rust
// 1. Identity: What type of boss is this?
GuildMaster,

// 2. Visual: How does it look and which frame is showing?
Sprite {
image: texture_handle,
texture_atlas: Some(TextureAtlas {
layout: atlas_layout_handle,
index: 0,  // Current frame (starts at 0)
}),
..default ()
},

// 3. Animation: How does it change over time?
BossAnimationConfig {
first_frame: 0,
last_frame: 14,
timer: Timer::from_seconds(0.1, TimerMode::Repeating),
}
```

### Test Your Understanding

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guild_master_animation_config() {
        let config = GuildMaster::animation_config();

        // What frame does the animation start on?
        assert_eq!(config.first_frame, 0);

        // What frame does the animation end on?
        assert_eq!(config.last_frame, 14);

        // How long between frame changes? (in seconds)
        assert_eq!(config.timer.duration().as_secs_f32(), 0.1);

        // Is the timer set to repeat?
        assert!(matches!(config.timer.mode(), TimerMode::Repeating));
    }
}
```

**Verify Your Learning**: Run this test. If it passes, you understand how animation data is structured. If it fails,
review the component fields above.

### Key Insight

The `TextureAtlas.index` field is what actually controls which frame displays. The animation system's job is to
increment this number over time, wrapping back to `first_frame` when it exceeds `last_frame`.

---

## Step 2: Sprite Sheet Layout

*Time: 15 minutes*

### The Question: How Does Bevy Know Where Each Frame Is?

Your `guild_master.png` file is 1725×115 pixels containing 15 frames. But how does Bevy know where frame 7 starts and
ends?

**Prediction Challenge**: If each frame is 115×115 pixels, what are the pixel coordinates for frame 7?

<details>
<summary>Click to reveal the answer</summary>

Frame 7 starts at pixel (805, 0) and ends at (920, 115).
Math: 7 frames × 115 pixels = 805 pixels from left edge.

</details>

### Visual Layout Understanding

```
guild_master.png (1725×115 pixels total):
┌──────┬──────┬──────┬──────┬──────┬──────┬──────┬──────┬──────┬───────┬───────┬───────┬───────┬───────┬───────┐
│  0   │  1   │  2   │  3   │  4   │  5   │  6   │  7   │  8   │   9   │  10   │  11   │  12   │  13   │  14   │
│ 0px  │115px │230px │345px │460px │575px │690px │805px │920px │1035px │1150px │1265px │1380px │1495px │1610px │
└──────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴───────┴───────┴───────┴───────┴───────┴───────┘
```

### The Atlas Layout Creator

**File: `src/boss/mod.rs:37-45`**

```rust
fn create_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(
        UVec2::new(115, 115),  // Each frame is 115×115 pixels
        15,                    // 15 frames horizontally  
        1,                     // 1 row vertically
        None,                  // No padding between frames
        None,                  // No offset from top-left
    )
}
```

**Active Recall**: What would you change in `from_grid()` if your sprite had 2 rows of 8 frames each?

<details>
<summary>Check your answer</summary>

```rust
TextureAtlasLayout::from_grid(
UVec2::new(115, 115),  // Frame size stays the same
8,                     // 8 frames per row (not 15)
2,                     // 2 rows (not 1)
None,                  // No padding
None,                  // No offset
)
```

</details>

### The Handle<T> Pattern

Bevy uses a handle system for managing assets efficiently:

```rust
// Pattern: Create the layout data structure
let layout_data = GuildMaster::create_atlas_layout();

// Pattern: Store it in Bevy's asset system and get a handle
let layout_handle: Handle<TextureAtlasLayout> = layouts.add(layout_data);

// Pattern: Use the handle to reference the layout
TextureAtlas {
layout: layout_handle,  // Points to the stored layout
index: 0,               // Which frame to show
}
```

**Why Handles?** Multiple entities can share the same layout without duplicating memory. 100 Guild Masters = 1 layout in
memory + 100 cheap handles.

**💡 Deep Dive**: Want to understand exactly how Bevy's Handle and Asset system works under the hood? Check out our
comprehensive guide: [Understanding Handle<T> and Assets in Bevy](understanding_handle_and_assets.md) - it covers memory
management, reference counting, performance optimization, and production patterns.

### Test Your Layout Understanding

```rust
#[test]
fn test_atlas_layout_math() {
    let layout = GuildMaster::create_atlas_layout();

    // How many total frames should we have?
    assert_eq!(layout.textures.len(), 15);

    // What should be the pixel boundaries of frame 0?
    let frame_0 = &layout.textures[0];
    assert_eq!(frame_0.min, UVec2::new(0, 0));      // Top-left
    assert_eq!(frame_0.max, UVec2::new(115, 115));  // Bottom-right

    // What should be the pixel boundaries of frame 7?
    let frame_7 = &layout.textures[7];
    assert_eq!(frame_7.min, UVec2::new(805, 0));    // 7 * 115 = 805
    assert_eq!(frame_7.max, UVec2::new(920, 115));  // 805 + 115 = 920

    // What should be the total atlas dimensions?
    assert_eq!(layout.size, UVec2::new(1725, 115)); // 15 * 115 = 1725 width
}
```

**Verify Your Learning**: Run this test. If it passes, you understand how Bevy calculates frame boundaries.

### What's the Output?

Given this layout creation:

```rust
let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 2, None, None);
```

How many total frames will `layout.textures.len()` return?

<details>
<summary>Click for answer</summary>

8 frames total (4 columns × 2 rows = 8)

</details>

---

## Step 3: Spawn System

*Time: 15 minutes*

### The Question: How Do You Create an Animated Entity?

Now we need to spawn a Guild Master entity with all the components we've learned about. But there's a specific order and
pattern to follow.

**Think First**: What three things do you need before you can spawn an animated sprite?

<details>
<summary>Click to reveal the three prerequisites</summary>

1. **Arena entity** to spawn into (parent-child relationship)
2. **Texture handle** from the asset server
3. **Atlas layout handle** from the layout assets

</details>

### The Complete Spawn System

**File: Location in your codebase**

```rust
fn spawn_guild_master(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<Entity, With<GuildHouse>>,
) {
    // Step 1: Find the arena to spawn into
    let Some(arena_entity) = query.iter().next() else {
        return; // No arena? Nothing to do
    };

    // Step 2: Load the texture file
    let texture = asset_server.load(GuildMaster::TEXTURE_PATH);

    // Step 3: Create and store the atlas layout
    let layout = layouts.add(GuildMaster::create_atlas_layout());

    // Step 4: Spawn as child of the arena
    commands.entity(arena_entity).with_children(|parent| {
        parent.spawn((
            GuildMaster,                                    // Identity

            Sprite {                                        // Visual representation
                image: texture,
                texture_atlas: Some(TextureAtlas {
                    layout,                                 // Which frames to use
                    index: 0,                              // Start at frame 0
                }),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1.0),           // Position in arena
            GuildMaster::animation_config(),                // Animation timing
        ));
    });
}
```

### Breaking Down the Bevy 0.16 Pattern

**Key Insight**: Notice how we embed `TextureAtlas` inside `Sprite` rather than as a separate component:

```rust
// ✅ Bevy 0.16 way (embedded)
Sprite {
image: texture,
texture_atlas: Some(TextureAtlas { layout, index: 0 }),
..default ()
}

// ❌ Old way (separate components)
// SpriteBundle { ... },
// TextureAtlas { layout, index: 0 },
```

**Why?** The new approach reduces the number of components and makes the relationship clearer.

### Active Recall Challenge

Without looking back, what are the four main components that get added to your Guild Master entity?

<details>
<summary>Check your answer</summary>

1. `GuildMaster` - Identity marker
2. `Sprite` - Visual representation with embedded TextureAtlas
3. `Transform` - Position, rotation, scale
4. `BossAnimationConfig` - Animation timing and range

Plus Bevy auto-adds: `GlobalTransform`, `Visibility`, `InheritedVisibility`

</details>

### Test Your Spawn System

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn test_spawn_system_create_entity() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .init_asset::<Image>()
            .init_resource::<Assets<TextureAtlasLayout>>();

        let arena_id = app.world_mut().spawn(GuildHouse).id();

        app.add_systems(Update, spawn_guildmaster);
        app.update();

        let children = app.world().get::<Children>(arena_id).unwrap();
        assert_eq!(children.len(), 1, "Arena should have exactly 1 child");

        let guild_master_id = children[0];

        assert!(
            app.world().get::<GuildMaster>(guild_master_id).is_some(),
            "Guild master should be spawned"
        );
        assert!(
            app.world().get::<Sprite>(guild_master_id).is_some(),
            "Guild master shall have a Sprite"
        );
        assert!(
            app.world().get::<Transform>(guild_master_id).is_some(),
            "Guild master shall have a Transform"
        );
        assert!(
            app.world()
                .get::<BossAnimationConfig>(guild_master_id)
                .is_some(),
            "Guild master shall have a BossAnimationConfig"
        );
    }
    #[test]
    fn test_spawn_guildmaster_with_arena() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .init_asset::<Image>()
            .init_resource::<Assets<TextureAtlasLayout>>();
        app.add_systems(Update, spawn_guildmaster);
        app.update();

        let world = app.world_mut();
        let guild_master: Vec<_> = world
            .query::<&GuildMaster>()
            .iter(world)
            .collect();
        assert_eq!(guild_master.len(), 0, "Should not spawn without arena");
    }
}
```

**Verify Your Learning**: Run these tests. If they pass, you understand the spawn system mechanics.

### What's the Output?

If you run this spawn system and then immediately query for all entities with `GuildMaster`, how many will you find?

<details>
<summary>Click for answer</summary>

Exactly 1, assuming there's one arena entity. The system only spawns one Guild Master per arena.

</details>

### Common Pitfall

**Problem**: Your Guild Master spawns but doesn't appear on screen.

**Diagnosis Questions**:

1. Is the `Transform` z-value positive? (Should be 1.0 for foreground)
2. Is the arena `Transform` positioning the child off-screen?
3. Is the texture path correct? (`"bosses/guild_master.png"`)

### Key Insight

The parent-child relationship (`with_children`) means the Guild Master's position is relative to the arena. If the arena
is at (100, 100) and the Guild Master is at (0, 0), the Guild Master appears at (100, 100) in world space.

---

## Step 4: The Animation System

*Time: 15 minutes*

### The Question: How Do You Make Frames Change Over Time?

You have a static Guild Master showing frame 0. Now you need to make it cycle through all 15 frames smoothly.

**Think First**: What needs to happen every frame to create animation?

<details>
<summary>Click to reveal the animation loop logic</summary>

1. **Tick the timer** forward by the time that passed
2. **Check if timer finished** (reached 0.1 seconds)
3. **Increment the frame index** (0→1→2→...→14)
4. **Wrap back to start** when reaching the last frame (14→0)

</details>

### The Modern Animation System

Here's the updated animation system using Bevy's `Single` query pattern:

```rust
fn animate_guild_master(
    time: Res<Time>,
    mut guild_master: Single<(&mut Sprite, &mut BossAnimationConfig), With<GuildMaster>>,
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
```

### Understanding Single Queries

**Mental Model**: Think of `Single` as Bevy's way to say "I know exactly one entity matches this criteria."

The `Single<(&mut Sprite, &mut BossAnimationConfig), With<GuildMaster>>` query means:

- **Find the one entity** that has all three components: `Sprite`, `BossAnimationConfig`, and `GuildMaster`
- **Give me mutable access** to `Sprite` and `BossAnimationConfig` so I can change them
- **Panic if there's 0 or more than 1** matching entity (this is intentional - it catches bugs early!)

**Why Use Single Instead of Query?**

```rust
// ❌ Old way: Loop through potentially many entities
fn animate_with_query(
    mut query: Query<(&mut Sprite, &mut BossAnimationConfig), With<GuildMaster>>,
) {
    for (mut sprite, mut config) in &mut query {
        // Animation code here...
    }
}

// ✅ New way: Directly access the one entity we know exists
fn animate_with_single(
    mut guild_master: Single<(&mut Sprite, &mut BossAnimationConfig), With<GuildMaster>>,
) {
    let (mut sprite, mut config) = guild_master.into_inner();
    // Animation code here...
}
```

**Key Benefits of Single**:
1. **Performance**: No loop overhead for single entities
2. **Safety**: Compile-time guarantee there's exactly one match
3. **Clarity**: Code clearly communicates intent ("this is unique")
4. **Bug Detection**: Panics if assumptions are wrong (e.g., multiple Guild Masters spawned accidentally)

**Active Recall**: What happens if you accidentally spawn 2 Guild Masters with the `Single` query?

<details>
<summary>Check your answer</summary>

The system will panic at runtime with a clear error message, helping you catch the bug immediately rather than having subtle animation issues.

</details>

### Understanding the .into_inner() API

The `.into_inner()` method extracts the actual component references from the `Single` wrapper:

```rust
// This gives you a Single wrapper around your components
let mut guild_master: Single<(&mut Sprite, &mut BossAnimationConfig), With<GuildMaster>> = // ...

// This extracts the actual component references you can work with
let (mut sprite, mut config) = guild_master.into_inner();
//      ^^^^^^^^^^^^^^^^^^^^^ These are now direct mutable references
```

**Why .into_inner()?** Bevy's `Single` is a smart wrapper that provides safety guarantees. When you call `.into_inner()`, you're saying "I've verified this is safe, give me the raw components." It's similar to how `Option::unwrap()` extracts the value from `Some(value)`.

**Alternative Pattern** (less common):
```rust
// You could also access components through the wrapper
let sprite_ref = guild_master.get_mut(); // Returns (&mut Sprite, &mut BossAnimationConfig)
```

### The Timer Tick System Explained

The most complex part of animation is understanding how `config.timer.tick(time.delta())` works with Bevy's global time system:

```
Frame N:     Frame N+1:    Frame N+2:
┌─────────┐  ┌─────────┐   ┌─────────┐
│ time.   │  │ time.   │   │ time.   │
│ delta() │  │ delta() │   │ delta() │
│ = 16ms  │  │ = 17ms  │   │ = 16ms  │
└─────────┘  └─────────┘   └─────────┘
     │            │             │
     ▼            ▼             ▼
┌─────────────────────────────────────┐
│        Timer Internal State        │
│                                     │
│ Start: 0ms    →  16ms   →   33ms    │
│ Target: 100ms →  100ms  →   100ms   │
│ just_finished()? No     →   No      │
└─────────────────────────────────────┘

Frame N+6:    (After ~100ms total)
┌─────────┐
│ time.   │
│ delta() │   
│ = 16ms  │
└─────────┘
     │
     ▼
┌─────────────────────────────────────┐
│ Timer: 84ms + 16ms = 100ms ✅      │
│ just_finished()? YES (one frame!)   │
│ Resets to: 0ms (TimerMode::Repeat)  │
└─────────────────────────────────────┘
```

**The Time System Relationship**:

1. **`Res<Time>`** is Bevy's global time resource, updated once per frame by the engine
2. **`time.delta()`** returns the duration since the last frame (usually 16ms at 60 FPS)
3. **`timer.tick(duration)`** adds that duration to the timer's internal elapsed time
4. **Timer checks**: "Have I reached my target duration?" (100ms in our case)

**Key Insight**: The timer doesn't care about frame rates! Whether you run at 30 FPS or 144 FPS, the animation will always take exactly 0.1 seconds per frame because it's based on real time, not frame count.

### Understanding just_finished()

The `just_finished()` method is crucial for preventing animation bugs:

```rust
if config.timer.just_finished() {
    // This code runs EXACTLY ONCE when timer completes
    texture_atlas.index += 1;
}
```

**How just_finished() Works Internally**:

```rust
// Simplified Timer implementation
impl Timer {
    fn just_finished(&self) -> bool {
        // Only true for ONE frame when timer completes
        self.finished && !self.finished_last_frame
    }
    
    fn tick(&mut self, delta: Duration) {
        self.finished_last_frame = self.finished;
        self.elapsed += delta;
        self.finished = self.elapsed >= self.duration;
        
        if self.finished && self.mode == TimerMode::Repeating {
            self.elapsed = Duration::ZERO; // Reset for next cycle
        }
    }
}
```

**Why This Matters**: Without `just_finished()`, you might use `finished()` which would be true for EVERY frame after the timer completes, causing your animation to advance multiple frames instantly!

**Alternative Timer APIs** for different use cases:
- **`timer.finished()`** - True from completion until reset
- **`timer.percent()`** - Returns 0.0 to 1.0 progress (great for smooth interpolation)
- **`timer.remaining()`** - Time left until completion
- **`timer.elapsed()`** - Time passed since last reset

**🔗 Deep Dive Reference**: Want to see the actual implementation? Check Bevy's Timer source: [bevy/crates/bevy_time/src/timer.rs](https://github.com/bevyengine/bevy/blob/main/crates/bevy_time/src/timer.rs)

### Test Your Animation System

Here are the updated tests with better failure messages:

```rust
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
        
        assert_eq!(
            sprite.texture_atlas.as_ref().unwrap().index, 
            1,
            "Animation should advance from frame 0 to frame 1 after 0.1 seconds. \
             This failure suggests either: (1) Timer duration is not 0.1 seconds, \
             (2) Timer mode is not Repeating, or (3) just_finished() logic is broken"
        );
    }

    #[test]
    fn test_animation_wrap_around() {
        let mut sprite = Sprite {
            texture_atlas: Some(TextureAtlas {
                layout: Handle::default(),
                index: 14,
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
        
        assert_eq!(
            sprite.texture_atlas.as_ref().unwrap().index, 
            0,
            "Animation should wrap from frame 14 (last_frame) back to frame 0 (first_frame). \
             This failure suggests the wrap-around logic is incorrect or last_frame/first_frame \
             values are wrong in animation_config()"
        );
    }
}
```

**Verify Your Learning**: Run these tests. The enhanced failure messages will help you debug any issues with timer configuration or animation logic.

### System Integration

Add your animation system to your app:

```rust
impl Plugin for GuildMasterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                spawn_guild_master,
                animate_guild_master,
            ).chain()); // chain() ensures spawn happens before animation
    }
}
```

**Why `.chain()`?** Without it, the animation system might run before the spawn system in the same frame, trying to
animate entities that don't exist yet.

### Common Debugging Issues

**Problem**: Animation is too fast or too slow.
**Solution**: Adjust the timer in `animation_config()`:

```rust
Timer::from_seconds(0.05, TimerMode::Repeating)  // Faster (20 FPS)
Timer::from_seconds(0.2, TimerMode::Repeating)   // Slower (5 FPS)
```

**Problem**: Animation stops after one frame.
**Solution**: Ensure timer mode is `TimerMode::Repeating`, not `TimerMode::Once`.

### Key Insight

The combination of `time.delta()`, `Timer::tick()`, and `just_finished()` creates frame-rate independent animation.
Whether you run at 30 FPS or 144 FPS, your Guild Master will always animate at exactly 10 frames per second.

---

## Step 5: Game States

*Time: 15 minutes*

### The Question: When Should Animation Start and Stop?

Your Guild Master shouldn't animate during the title screen or character creation. It should only animate when the
player is actually in the arena fighting.

**Think First**: What game states do you have, and which one represents "in combat"?

<details>
<summary>Click to reveal your current states</summary>

Looking at your codebase:

- `GameState::Title` - Main menu
- `GameState::CharacterCreate` - Building your character
- `GameState::Intro` - Story/cutscene
- Need to add: `GameState::Arena` - Combat arena

</details>

### Adding the Arena State

First, extend your `GameState` enum:

**File: `src/game_state/mod.rs`**

```rust
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Title,
    CharacterCreate,
    Intro,
    Arena,  // Add this for boss battles
}
```

### The Plugin Architecture

Now create a plugin that manages the entire Guild Master lifecycle:

```rust
pub struct GuildMasterPlugin;

impl Plugin for GuildMasterPlugin {
    fn build(&self, app: &mut App) {
        app
            // Spawn when entering arena
            .add_systems(OnEnter(GameState::Arena), spawn_guild_master)

            // Animate while in arena
            .add_systems(
                Update,
                animate_guild_master.run_if(in_state(GameState::Arena))
            )

            // Clean up when leaving arena
            .add_systems(OnExit(GameState::Arena), despawn_guild_master);
    }
}

fn despawn_guild_master(
    mut commands: Commands,
    query: Query<Entity, With<GuildMaster>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
```

### Understanding the System Schedule

**Key Insight**: Bevy provides special schedules for state transitions:

- **`OnEnter(state)`**: Runs exactly once when entering a state
- **`Update`**: Runs every frame (but we add `.run_if()` to control when)
- **`OnExit(state)`**: Runs exactly once when leaving a state

**Active Recall**: If you transition from `Arena` to `Title`, in what order do these systems run?

<details>
<summary>Check your answer</summary>

1. `OnExit(GameState::Arena)` - `despawn_guild_master` runs
2. State changes to `Title`
3. `OnEnter(GameState::Title)` - Any title screen setup runs
4. `Update` systems with `run_if(in_state(GameState::Title))` run each frame

</details>

### Test Your State Management

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_state_spawning() {
        let mut app = App::new();
        app
            .add_plugins(MinimalPlugins)
            .init_state::<GameState>()
            .add_plugins(GuildMasterPlugin);

        // Create arena first
        app.world.spawn(GuildHouse);

        // Start in Title state - should have no Guild Masters
        let count = app.world.query::<&GuildMaster>().iter(&app.world).count();
        assert_eq!(count, 0, "Should not spawn Guild Master in Title state");

        // Transition to Arena state
        app.world.resource_mut::<NextState<GameState>>().set(GameState::Arena);
        app.update(); // Process the state change

        // Should now have exactly 1 Guild Master
        let count = app.world.query::<&GuildMaster>().iter(&app.world).count();
        assert_eq!(count, 1, "Should spawn Guild Master when entering Arena");

        // Transition back to Title
        app.world.resource_mut::<NextState<GameState>>().set(GameState::Title);
        app.update(); // Process the state change

        // Should despawn the Guild Master
        let count = app.world.query::<&GuildMaster>().iter(&app.world).count();
        assert_eq!(count, 0, "Should despawn Guild Master when leaving Arena");
    }

    #[test]
    fn test_animation_only_in_arena() {
        let mut app = App::new();
        app
            .add_plugins(MinimalPlugins)
            .init_state::<GameState>()
            .add_plugins(GuildMasterPlugin);

        // Create and spawn a guild master manually
        let entity = app.world.spawn((
            GuildMaster,
            Sprite::default(),
            GuildMaster::animation_config(),
        )).id();

        // In Title state, animation system shouldn't run
        app.update();
        let config = app.world.get::<BossAnimationConfig>(entity).unwrap();
        let initial_elapsed = config.timer.elapsed();

        // Switch to Arena state
        app.world.resource_mut::<NextState<GameState>>().set(GameState::Arena);
        app.update();

        // Now animation system should run
        let config = app.world.get::<BossAnimationConfig>(entity).unwrap();
        let after_elapsed = config.timer.elapsed();
        assert!(after_elapsed > initial_elapsed, "Timer should advance in Arena state");
    }
}
```

**Verify Your Learning**: Run these tests. They verify that spawning and animation respect game state boundaries.

### Integration with Your Main App

**File: `src/main.rs`**

```rust
use crate::boss::GuildMasterPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameStatePlugin)        // Your existing state management
        .add_plugins(GuildMasterPlugin)      // Add this line
        .run();
}
```

### State Transition Triggers

How do you actually get to the Arena state? You'll need to trigger it somewhere:

```rust
// Example: After character creation is complete
fn transition_to_arena(
    mut next_state: ResMut<NextState<GameState>>,
    // Add whatever condition determines when to enter arena
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Arena);
    }
}
```

### What's the Output?

If you start in `Title` state and transition to `Arena`, how many times will `spawn_guild_master` run?

<details>
<summary>Click for answer</summary>

Exactly once, when the `OnEnter(GameState::Arena)` schedule runs. Even if you stay in Arena state for 10 minutes, it
won't run again.

</details>

### Key Insight

State-based system scheduling gives you precise control over when game features are active. This pattern scales
beautifully - you can add pause states, menu states, victory states, etc., each with their own system behaviors.

---

## Step 6: Optimization

*Time: 15 minutes*

### The Question: How Do You Handle Many Animated Characters?

Your game might eventually have dozens of animated bosses, enemies, and NPCs. How do you keep performance smooth?

**Think First**: What performance bottlenecks might occur with 50+ animated entities?

<details>
<summary>Click to reveal the main bottlenecks</summary>

1. **Asset Loading**: Loading duplicate textures and atlas layouts
2. **System Queries**: Inefficient queries that check every entity
3. **Memory Usage**: Storing redundant animation data
4. **CPU Overhead**: Unnecessary timer calculations

</details>

### Optimization 1: Shared Asset Resources

Instead of loading assets in the spawn system, pre-load them once:

```rust
#[derive(Resource)]
pub struct GuildMasterAssets {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

// Run once at startup
fn setup_guild_master_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(GuildMaster::TEXTURE_PATH);
    let atlas_layout = layouts.add(GuildMaster::create_atlas_layout());

    commands.insert_resource(GuildMasterAssets {
        texture,
        atlas_layout,
    });
}

// Now spawning is much faster
fn spawn_guild_master_optimized(
    mut commands: Commands,
    assets: Res<GuildMasterAssets>,
    arenas: Query<Entity, With<GuildHouse>>,
) {
    for arena_entity in &arenas {
        commands.entity(arena_entity).with_children(|parent| {
            parent.spawn((
                GuildMaster,
                Sprite {
                    image: assets.texture.clone(),      // Cheap handle clone
                    texture_atlas: Some(TextureAtlas {
                        layout: assets.atlas_layout.clone(), // Cheap handle clone
                        index: 0,
                    }),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 1.0),
                GuildMaster::animation_config(),
            ));
        });
    }
}
```

### Optimization 2: Parallel System Processing

Use Bevy's parallel query processing for multiple entities:

```rust
fn animate_all_bosses_parallel(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut BossAnimationConfig)>,
) {
    // Process all animated entities in parallel across CPU cores
    query.par_iter_mut().for_each(|(mut sprite, mut config)| {
        let Some(ref mut texture_atlas) = sprite.texture_atlas else {
            return;
        };

        config.timer.tick(time.delta());

        if config.timer.just_finished() {
            texture_atlas.index = if texture_atlas.index >= config.last_frame {
                config.first_frame
            } else {
                texture_atlas.index + 1
            };
        }
    });
}
```

### Optimization 3: Conditional System Running

Only run animation systems when there are entities to animate:

```rust
impl Plugin for GuildMasterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_guild_master_assets)
            .add_systems(OnEnter(GameState::Arena), spawn_guild_master_optimized)
            .add_systems(
                Update,
                animate_all_bosses_parallel
                    .run_if(any_with_component::<GuildMaster>)  // Only run if Guild Masters exist
                    .run_if(in_state(GameState::Arena))         // Only run in arena
            )
            .add_systems(OnExit(GameState::Arena), despawn_guild_master);
    }
}
```

### Performance Testing

Test your optimizations with many entities:

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_many_animated_entities() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn 100 animated entities
        for i in 0..100 {
            app.world.spawn((
                GuildMaster,
                Sprite {
                    texture_atlas: Some(TextureAtlas {
                        layout: Handle::default(),
                        index: 0,
                    }),
                    ..default()
                },
                GuildMaster::animation_config(),
            ));
        }

        // Measure animation system performance
        let start = Instant::now();

        // Run animation for 60 frames (1 second at 60 FPS)
        for _ in 0..60 {
            animate_all_bosses_parallel.run((), &mut app.world);
        }

        let elapsed = start.elapsed();
        println!("100 entities, 60 frames: {:?}", elapsed);

        // Should complete very quickly (under 10ms on modern hardware)
        assert!(elapsed.as_millis() < 100, "Animation should be fast even with many entities");
    }

    #[test]
    fn test_asset_sharing() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Setup shared assets
        let texture = Handle::<Image>::default();
        let layout = Handle::<TextureAtlasLayout>::default();
        app.world.insert_resource(GuildMasterAssets {
            texture: texture.clone(),
            atlas_layout: layout.clone(),
        });

        // Spawn multiple entities
        for _ in 0..10 {
            app.world.spawn((
                GuildMaster,
                Sprite {
                    image: texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: layout.clone(),
                        index: 0,
                    }),
                    ..default()
                },
                GuildMaster::animation_config(),
            ));
        }

        // All entities should share the same handles (cheap clones)
        let entities: Vec<_> = app.world.query::<&Sprite>().iter(&app.world).collect();
        assert_eq!(entities.len(), 10);

        // All sprites should reference the same texture handle
        for sprite in entities {
            assert_eq!(sprite.image, texture);
        }
    }
}
```

### What's the Output?

If you spawn 100 Guild Masters using the optimized approach vs. the basic approach, how many texture loads occur?

<details>
<summary>Click for answer</summary>

**Optimized**: 1 texture load (shared resource)
**Basic**: 100 texture loads (asset_server.load() called 100 times)

The optimized approach is dramatically faster and uses much less memory.

</details>

### Key Performance Insights

1. **Handle Cloning is Cheap**: `handle.clone()` just copies a lightweight reference, not the actual asset data
2. **Parallel Processing Helps**: `par_iter_mut()` can use multiple CPU cores for animation calculations
3. **Conditional Systems Save CPU**: `run_if()` prevents systems from running when they have no work to do
4. **Asset Sharing Reduces Memory**: One texture in memory can be displayed by hundreds of sprites

### Memory Usage Comparison

```rust
// ❌ Bad: Each entity loads its own copy
// Memory usage: 100 entities × texture size = huge
for _ in 0..100 {
let texture = asset_server.load("boss.png"); // Loads every time!
}

// ✅ Good: All entities share one copy  
// Memory usage: 1 × texture size = minimal
let shared_texture = asset_server.load("boss.png"); // Load once
for _ in 0..100 {
let texture = shared_texture.clone(); // Cheap reference copy
}
```

---

## Summary: What You've Built

**Congratulations!** You've just built a complete atlas animation system using modern Bevy patterns. Let's review what
you accomplished:

### 🎯 Core Achievement

Your Guild Master now smoothly cycles through 15 animation frames at 10 FPS, spawning in arena state and despawning when
leaving - exactly like a professional game.

### 📚 Learning Reinforcement

**Active Recall Challenge**: Without looking back, list the 6 main steps you learned:

<details>
<summary>Check your memory</summary>

1. **Animation Components** - Data structures for animation state
2. **Sprite Sheet Layout** - How Bevy slices atlases into frames
3. **Spawn System** - Creating animated entities with proper components
4. **Frame Animation** - Timer-based frame advancement logic
5. **Game States** - Controlling when animation starts/stops
6. **Optimization** - Handling many animated entities efficiently

</details>

### 🔑 Key Patterns You Mastered

**The ECS Database Mental Model**: You now understand how Components (columns), Entities (rows), and Systems (queries)
work together for animation.

**Modern Bevy 0.16 Patterns**:

- ✅ Individual components instead of bundles
- ✅ Embedded `TextureAtlas` in `Sprite` component
- ✅ Required Components automatically adding dependencies
- ✅ State-based system scheduling

**Performance-Conscious Design**:

- ✅ Shared asset resources (1 texture → 100 entities)
- ✅ Parallel processing with `par_iter_mut()`
- ✅ Conditional system execution with `run_if()`

### 🚀 What You Can Do Next

**Immediate Extensions** (15 minutes each):

1. **Animation States**: Split your 15 frames into Idle (0-4), Attack (5-9), Hurt (10-12), Death (13-14)
2. **Multiple Bosses**: Create additional boss types using the same animation patterns
3. **Sound Integration**: Trigger audio when specific frames display

**Advanced Projects** (1+ hours):

1. **Animation Blending**: Smooth transitions between animation states
2. **Event-Driven Animation**: Change animations based on combat events
3. **Visual Animation Editor**: Build an in-game tool for adjusting timing and ranges

### 🧠 Mental Models for Long-Term Retention

**Remember These Core Principles**:

1. **Animation = Data + Time**: `TextureAtlas.index` (data) + `Timer` (time) = smooth animation
2. **Handles Are Cheap**: Clone handles freely - you're copying references, not assets
3. **ECS Scales**: The same patterns work for 1 entity or 1000 entities
4. **State Management Matters**: Use Bevy's state system to control when features are active

### 🔄 Spaced Repetition Triggers

**Set these reminders in your calendar**:

- **1 week**: Re-implement Guild Master animation from memory
- **1 month**: Extend the system to handle multiple animation states
- **3 months**: Build a different animated character using the same patterns

This spaced repetition will cement these patterns in long-term memory.

### 📖 Reference Card

**Keep this cheat sheet handy**:

```rust
// Spawn animated entity
parent.spawn((
EntityMarker,
Sprite {
image: texture_handle,
texture_atlas: Some(TextureAtlas { layout, index: 0 }),
},
Transform,
AnimationConfig,
));

// Animate frames
for ( mut sprite, mut config) in & mut query {
config.timer.tick(time.delta());
if config.timer.just_finished() {
sprite.texture_atlas.as_mut().unwrap().index =
(index + 1) % frame_count;
}
}

// State-based plugin
app.add_systems(OnEnter(State), spawn_system)
.add_systems(Update, animate_system.run_if(in_state(State)))
.add_systems(OnExit(State), despawn_system);
```

### 🎯 Final Challenge

**Test Your Mastery**: Can you implement a simple animated enemy that uses the same patterns but different assets? If
yes, you've truly internalized these concepts.

---

## Debugging Reference

### Common Issues & Solutions

**Problem**: Animation not playing

- ✅ Check: Timer mode is `TimerMode::Repeating`
- ✅ Check: System is running in correct game state
- ✅ Check: Entity has all required components

**Problem**: Animation too fast/slow

- ✅ Solution: Adjust `Timer::from_seconds(duration, ...)`
- ✅ 20 FPS = 0.05 seconds, 5 FPS = 0.2 seconds

**Problem**: Sprite not visible

- ✅ Check: Transform z-value > 0 (foreground)
- ✅ Check: Texture path is correct
- ✅ Check: Parent positioning isn't moving child off-screen

**Problem**: Poor performance with many entities

- ✅ Use shared asset resources
- ✅ Enable parallel processing with `par_iter_mut()`
- ✅ Add conditional system execution

### 🎉 You Did It!

You've successfully mastered atlas animation in Bevy using a learning-optimized approach. These patterns will serve as
the foundation for all your future animated characters, from simple NPCs to complex boss battles.

The Guild Master awaits in the arena - ready to demonstrate the power of your new animation system!