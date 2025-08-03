# Arenic Marker Components System

This document provides a comprehensive guide to all marker components in Arenic, designed for performance and maintainability across 8 arenas with up to 320 characters. All markers follow Bevy 0.16 patterns with zero-sized structs for optimal memory usage and query performance.

## Core Design Principles

1. **Zero-Sized Components**: All markers are unit structs for optimal memory efficiency
2. **Query Optimization**: Designed for `With<T>` and `Without<T>` filtering patterns
3. **Recording Integration**: All markers support the 2-minute recording/replay system
4. **Performance First**: Cache-friendly patterns for 320+ character simulations
5. **Composability**: Markers combine to create complex entity archetypes

---

## 1. Character Class Markers

### Hero Classes
Eight core character classes, each with unique abilities and arena associations.

```rust
/// Warrior class marker - Tank/Melee specialist
/// Associated with: Crucible Arena
/// Role: Front-line tank with high health and defensive abilities
#[derive(Component, Debug)]
pub struct Warrior;

/// Hunter class marker - Ranged DPS specialist  
/// Associated with: Mountain Arena
/// Role: Long-range damage dealer with projectile abilities
#[derive(Component, Debug)]
pub struct Hunter;

/// Thief class marker - Stealth/Mobility specialist
/// Associated with: Pawnshop Arena
/// Role: High mobility with stealth and positioning abilities
#[derive(Component, Debug)]
pub struct Thief;

/// Alchemist class marker - DOT/Debuff specialist
/// Associated with: Labyrinth Arena
/// Role: Area denial and damage-over-time effects
#[derive(Component, Debug)]
pub struct Alchemist;

/// Bard class marker - Support/Buffer specialist
/// Associated with: Gala Arena
/// Role: Party buffs and crowd control abilities
#[derive(Component, Debug)]
pub struct Bard;

/// Cardinal class marker - Healer/Cleric specialist
/// Associated with: Sanctum Arena
/// Role: Primary healing and resurrection abilities
#[derive(Component, Debug)]
pub struct Cardinal;

/// Forager class marker - Environmental specialist
/// Associated with: Bastion Arena
/// Role: Terrain manipulation and resource abilities
#[derive(Component, Debug)]
pub struct Forager;

/// Merchant class marker - Economic/Utility specialist
/// Associated with: Casino Arena
/// Role: Currency abilities and random effects
#[derive(Component, Debug)]
pub struct Merchant;
```

**Usage Example:**
```rust
// Spawn a warrior character
commands.spawn((
    Warrior,
    Player,
    Health(100.0),
    Position(Vec3::ZERO),
    CharacterType::Warrior,
));

// Query all warriors in the system
fn warrior_system(warriors: Query<&Position, With<Warrior>>) {
    for position in &warriors {
        // Process warrior-specific logic
    }
}
```

---

## 2. Boss and Enemy Markers

### Boss Types
Each arena contains unique bosses with specific mechanics and timelines.

```rust
/// Primary boss marker - Entity is a major boss
/// Performance: Requires special rendering and ability processing
#[derive(Component, Debug)]
pub struct Boss;

/// Guild Master boss - Tutorial/Training arena boss
/// Associated with: Guild House Arena
/// Mechanics: Teaching boss with reduced difficulty
#[derive(Component, Debug)]
pub struct GuildMaster;

/// Elite enemy marker - Mid-tier enemy with enhanced abilities
/// Performance: Requires enhanced AI processing but not boss-level
#[derive(Component, Debug)]
pub struct Elite;

/// Minion marker - Basic enemy spawned by bosses or abilities
/// Performance: Simplified AI and abilities for large numbers
#[derive(Component, Debug)]
pub struct Minion;

/// Environmental enemy - Part of arena hazards rather than active AI
/// Performance: Minimal processing, primarily reactive behavior
#[derive(Component, Debug)]
pub struct Environmental;
```

**Recording Integration:**
```rust
// Boss abilities are recorded in the same 2-minute cycles
commands.spawn((
    Boss,
    GuildMaster,
    Health(1000.0),
    ArenaTimer::for_arena(8).0, // Guild House is arena index 8
    BossAbilitySet(vec![
        "slam".to_string(),
        "charge".to_string(),
        "summon_minions".to_string(),
    ]),
));
```

---

## 3. Arena Markers

### Arena Location Markers
Each of the 8 arenas plus the Guild House has specific characteristics.

```rust
/// Crucible Arena - Warrior-themed arena with melee combat focus
/// Grid: 320x180, Boss: Crucible Champion
/// Mechanics: Close-quarters combat, destructible terrain
#[derive(Component, Debug)]
pub struct CrucibleArena;

/// Mountain Arena - Hunter-themed arena with ranged combat focus
/// Grid: 320x180, Boss: Mountain Beast
/// Mechanics: Long sight lines, elevation changes
#[derive(Component, Debug)]
pub struct MountainArena;

/// Pawnshop Arena - Thief-themed arena with stealth mechanics
/// Grid: 320x180, Boss: Shadow Broker
/// Mechanics: Hidden passages, stealth bonuses
#[derive(Component, Debug)]
pub struct PawnshopArena;

/// Labyrinth Arena - Alchemist-themed arena with environmental hazards
/// Grid: 320x180, Boss: Toxic Overseer
/// Mechanics: Acid pools, poison clouds, environmental damage
#[derive(Component, Debug)]
pub struct LabyrinthArena;

/// Gala Arena - Bard-themed arena with musical mechanics
/// Grid: 320x180, Boss: Maestro of Mayhem
/// Mechanics: Rhythm-based abilities, sound amplification
#[derive(Component, Debug)]
pub struct GalaArena;

/// Sanctum Arena - Cardinal-themed arena with holy mechanics
/// Grid: 320x180, Boss: Corrupted Saint
/// Mechanics: Consecrated ground, light/dark mechanics
#[derive(Component, Debug)]
pub struct SanctumArena;

/// Bastion Arena - Forager-themed arena with nature mechanics
/// Grid: 320x180, Boss: Earth Warden
/// Mechanics: Growing terrain, natural hazards
#[derive(Component, Debug)]
pub struct BastionArena;

/// Casino Arena - Merchant-themed arena with random mechanics
/// Grid: 320x180, Boss: Lucky Strike
/// Mechanics: Random events, currency mechanics
#[derive(Component, Debug)]
pub struct CasinoArena;

/// Guild House - Central hub and tutorial area
/// Grid: 320x180, Boss: Guild Master (tutorial)
/// Mechanics: Safe zone, recruitment, management
#[derive(Component, Debug)]
pub struct GuildHouse;
```

---

## 4. Grid and Terrain Markers

### Tile Types
Grid-based positioning system with 320x180 resolution per arena.

```rust
/// Standard walkable terrain tile
/// Performance: Minimal processing, default movement cost
#[derive(Component, Debug)]
pub struct WalkableTile;

/// Impassable terrain (walls, obstacles)
/// Performance: Excluded from pathfinding calculations
#[derive(Component, Debug)]
pub struct BlockedTile;

/// Hazardous terrain that deals damage over time
/// Performance: Requires periodic damage processing
#[derive(Component, Debug)]
pub struct HazardTile;

/// Beneficial terrain that provides buffs or healing
/// Performance: Requires periodic buff application
#[derive(Component, Debug)]
pub struct BuffTile;

/// Teleporter tile - transports to another location
/// Performance: Requires position validation and teleport logic
#[derive(Component, Debug)]
pub struct TeleportTile;

/// Elevated terrain - provides sight/range bonuses
/// Performance: Affects line-of-sight calculations
#[derive(Component, Debug)]
pub struct ElevatedTile;

/// Concealment terrain - provides stealth bonuses
/// Performance: Affects visibility calculations
#[derive(Component, Debug)]
pub struct ConcealmentTile;

/// Resource node - can be harvested for materials
/// Performance: Requires interaction processing
#[derive(Component, Debug)]
pub struct ResourceTile;
```

**Grid Integration:**
```rust
// Tile positioning uses arena-relative coordinates
#[derive(Component, Debug)]
pub struct GridPosition {
    pub x: u16, // 0-319 (arena width)
    pub y: u16, // 0-179 (arena height)
    pub arena: u8, // 0-8 (arena index)
}

// Efficient tile queries
fn hazard_damage_system(
    mut characters: Query<&mut Health, With<GridPosition>>,
    hazard_tiles: Query<&GridPosition, With<HazardTile>>,
) {
    // Process damage for characters on hazard tiles
}
```

---

## 5. Object and Item Markers

### Interactive Objects
Items and objects that characters can interact with during recordings.

```rust
/// Loot chest - contains rewards when opened
/// Performance: Requires interaction detection and inventory management
#[derive(Component, Debug)]
pub struct LootChest;

/// Healing fountain - provides regeneration when nearby
/// Performance: Requires proximity detection and healing application
#[derive(Component, Debug)]
pub struct HealingFountain;

/// Weapon rack - allows equipment swapping
/// Performance: Requires equipment validation and stat recalculation
#[derive(Component, Debug)]
pub struct WeaponRack;

/// Trap - activates when stepped on
/// Performance: Requires trigger detection and effect application
#[derive(Component, Debug)]
pub struct Trap;

/// Portal - connects different arena locations
/// Performance: Requires teleportation logic and position validation
#[derive(Component, Debug)]
pub struct Portal;

/// Shrine - provides temporary buffs when activated
/// Performance: Requires buff application and duration tracking
#[derive(Component, Debug)]
pub struct Shrine;

/// Destructible object - can be broken for resources
/// Performance: Requires health tracking and destruction logic
#[derive(Component, Debug)]
pub struct Destructible;

/// Pickupable item - can be collected into inventory
/// Performance: Requires inventory space validation
#[derive(Component, Debug)]
pub struct Pickupable;
```

---

## 6. State Markers

### Entity States
Current state information for recording and replay systems.

```rust
/// Player-controlled entity during recording phase
/// Performance: Requires input processing and command validation
#[derive(Component, Debug)]
pub struct Player;

/// Ghost entity - replaying a previously recorded timeline
/// Performance: Follows predetermined actions, minimal input processing
#[derive(Component, Debug)]
pub struct Ghost;

/// Currently recording actions for future replay
/// Performance: Requires action logging and timeline building
#[derive(Component, Debug)]
pub struct Recording;

/// Entity is idle - not currently recording or being controlled
/// Performance: Minimal processing, background simulation only
#[derive(Component, Debug)]
pub struct Idle;

/// Entity has died and is awaiting revival
/// Performance: Excluded from most processing until revived
#[derive(Component, Debug)]
pub struct Dead;

/// Entity is temporarily invulnerable
/// Performance: Requires damage immunity processing
#[derive(Component, Debug)]
pub struct Invulnerable;

/// Entity is currently channeling an ability
/// Performance: Requires cast time tracking and interruption detection
#[derive(Component, Debug)]
pub struct Channeling;

/// Entity is currently moving between positions
/// Performance: Requires movement interpolation and collision detection
#[derive(Component, Debug)]
pub struct Moving;

/// Entity is selected by the player
/// Performance: Requires UI highlighting and input routing
#[derive(Component, Debug)]
pub struct Selected;

/// Entity is stunned and cannot act
/// Performance: Disables ability processing and movement
#[derive(Component, Debug)]
pub struct Stunned;
```

---

## 7. Recording and Replay Markers

### Timeline Integration
Markers specifically for the 2-minute recording/replay system.

```rust
/// Entity can have its actions recorded
/// Performance: Requires action capture and timeline storage
#[derive(Component, Debug)]
pub struct Recordable;

/// Action was replayed from a previous recording
/// Performance: Marks actions for debugging and validation
#[derive(Component, Debug)]
pub struct Replayed;

/// Action requires synchronization with other entities
/// Performance: Requires coordination timing and event ordering
#[derive(Component, Debug)]
pub struct Synchronized;

/// Action is deterministic and will replay exactly
/// Performance: Predictable behavior for timeline validation
#[derive(Component, Debug)]
pub struct Deterministic;

/// Action has conditional behavior based on game state
/// Performance: Requires state evaluation during replay
#[derive(Component, Debug)]
pub struct Conditional;

/// High-priority action that affects timeline ordering
/// Performance: Processed before other actions in same frame
#[derive(Component, Debug)]
pub struct Priority;

/// Action creates a timeline checkpoint for debugging
/// Performance: Requires state snapshot storage
#[derive(Component, Debug)]
pub struct Checkpoint;

/// Marks the start of a new recording cycle
/// Performance: Resets entity state and begins action capture
#[derive(Component, Debug)]
pub struct RecordingStart;

/// Marks the end of a recording cycle
/// Performance: Finalizes timeline and switches to replay mode
#[derive(Component, Debug)]
pub struct RecordingEnd;
```

---

## 8. Performance Bundle Markers

### Optimization Categories
Markers that help optimize processing for 320+ character simulation.

```rust
/// High-performance entity - requires optimized processing
/// Performance: Uses specialized systems for maximum efficiency
#[derive(Component, Debug)]
pub struct HighPerformance;

/// Background entity - minimal processing requirements
/// Performance: Batch processed with simplified logic
#[derive(Component, Debug)]
pub struct Background;

/// Visible entity - requires rendering and visual updates
/// Performance: Included in rendering passes and visual effects
#[derive(Component, Debug)]
pub struct Visible;

/// Off-screen entity - excluded from rendering
/// Performance: Simulation only, no visual processing
#[derive(Component, Debug)]
pub struct OffScreen;

/// Static entity - position and state rarely change
/// Performance: Cached for spatial queries and collision detection
#[derive(Component, Debug)]
pub struct Static;

/// Dynamic entity - frequently changing position/state
/// Performance: Requires frequent updates and spatial reindexing
#[derive(Component, Debug)]
pub struct Dynamic;

/// Batch-processable entity - can be processed in groups
/// Performance: Optimized for SIMD operations and data parallelism
#[derive(Component, Debug)]
pub struct Batchable;

/// Cache-friendly entity - data arranged for optimal memory access
/// Performance: Components arranged to minimize cache misses
#[derive(Component, Debug)]
pub struct CacheFriendly;
```

---

## 9. Common Marker Combinations

### Archetype Patterns
Common combinations of markers that create specific entity types.

```rust
/// Active Player Character Bundle
/// A character currently being controlled by the player
pub type ActivePlayerBundle = (
    Player,
    Selected,
    Recording,
    Visible,
    Dynamic,
    HighPerformance,
    // Class marker (Warrior, Hunter, etc.)
    // Arena marker
);

/// Ghost Character Bundle  
/// A character replaying a previous recording
pub type GhostBundle = (
    Ghost,
    Replayed,
    Visible,
    Dynamic,
    Batchable,
    // Class marker
    // Arena marker
);

/// Idle Character Bundle
/// A character in background simulation
pub type IdleCharacterBundle = (
    Idle,
    Background,
    OffScreen,
    Static,
    CacheFriendly,
    // Class marker
    // Arena marker
);

/// Boss Entity Bundle
/// A major boss with complex mechanics
pub type BossBundle = (
    Boss,
    Visible,
    Dynamic,
    HighPerformance,
    Recordable,
    Deterministic,
    // Specific boss marker (GuildMaster, etc.)
    // Arena marker
);

/// Environment Object Bundle
/// Static objects in the arena
pub type EnvironmentBundle = (
    Static,
    Visible,
    CacheFriendly,
    Background,
    // Object type marker (LootChest, Portal, etc.)
    // Arena marker
);
```

---

## 10. System Integration Examples

### Query Patterns
Efficient queries using marker combinations for high-performance systems.

```rust
/// Process only active player characters
fn player_input_system(
    mut players: Query<
        (&mut Position, &mut Velocity),
        (With<Player>, With<Recording>, Without<Stunned>)
    >,
    input: Res<Input<KeyCode>>,
) {
    for (mut pos, mut vel) in &mut players {
        // Process player input
    }
}

/// Batch process all ghost characters for replay
fn ghost_replay_system(
    mut ghosts: Query<
        (&mut Position, &Timeline),
        (With<Ghost>, With<Batchable>, Without<Dead>)
    >,
    time: Res<Time>,
) {
    // Batch process ghost movements for efficiency
    ghosts.par_iter_mut().for_each(|(mut pos, timeline)| {
        // SIMD-optimized replay processing
    });
}

/// Update visibility for rendering optimization
fn visibility_culling_system(
    mut entities: Query<
        (&Position, &mut Visible),
        (With<Dynamic>, Changed<Position>)
    >,
    camera: Query<&Transform, With<Camera>>,
) {
    let camera_pos = camera.single().translation;
    for (pos, mut visible) in &mut entities {
        // Cull off-screen entities for performance
    }
}

/// Arena-specific processing
fn arena_hazard_system(
    mut characters: Query<
        &mut Health,
        (With<CrucibleArena>, With<Visible>, Without<Invulnerable>)
    >,
    hazards: Query<&Position, With<HazardTile>>,
) {
    // Process arena-specific hazards
}
```

---

## 11. Recording Integration

### Timeline Markers
How markers integrate with the 2-minute recording cycles.

```rust
/// Timeline Event Structure
#[derive(Debug, Clone)]
pub struct TimelineEvent {
    pub timestamp: f32,        // 0.0 to 120.0 seconds
    pub entity: Entity,
    pub action_type: String,
    pub position: Vec3,
    pub markers: Vec<String>,  // Active markers at time of recording
}

/// Recording Integration Example
fn record_action_system(
    mut timeline: ResMut<Timeline>,
    entities: Query<
        (Entity, &Position, &ActionComponent),
        (With<Recording>, With<Player>, Changed<ActionComponent>)
    >,
    time: Res<ArenaTimer>,
) {
    for (entity, pos, action) in &entities {
        let event = TimelineEvent {
            timestamp: time.current_seconds(),
            entity,
            action_type: action.name.clone(),
            position: pos.0,
            markers: vec![
                "Player".to_string(),
                "Recording".to_string(),
                "Warrior".to_string(), // Class marker
                "CrucibleArena".to_string(), // Arena marker
            ],
        };
        timeline.add_event(event);
    }
}
```

---

## 12. Performance Considerations

### Memory Layout
Optimized component storage for 320+ character simulation.

```rust
/// Memory-efficient marker storage
/// Zero-sized markers consume no memory per entity
/// Bevy's sparse storage optimizes for marker presence/absence queries

/// Cache-friendly query patterns
fn optimized_batch_system(
    // Group related components together
    characters: Query<
        (&Position, &Velocity, &Health),
        (With<Warrior>, With<Visible>, Without<Dead>)
    >,
) {
    // Process in cache-friendly order
    characters.iter().for_each(|(pos, vel, health)| {
        // Optimized processing
    });
}

/// Arena partitioning for scalability
fn arena_partitioned_system(
    crucible_chars: Query<&Position, (With<CrucibleArena>, With<Visible>)>,
    mountain_chars: Query<&Position, (With<MountainArena>, With<Visible>)>,
    // ... other arenas
) {
    // Process each arena independently for parallelization
}
```

### Frame Time Optimization
```rust
/// Performance budgeting system
const MAX_CHARACTERS_PER_FRAME: usize = 40; // Process 40 chars per frame max
const TARGET_FRAME_TIME_MS: f32 = 16.67; // 60 FPS target

fn frame_budgeted_system(
    mut query: Query<&mut Health, With<Dynamic>>,
    time: Res<Time>,
) {
    let start_time = time.elapsed_seconds();
    let mut processed = 0;
    
    for mut health in &mut query {
        if processed >= MAX_CHARACTERS_PER_FRAME {
            break; // Maintain frame rate
        }
        
        // Process character
        processed += 1;
        
        if (time.elapsed_seconds() - start_time) * 1000.0 > TARGET_FRAME_TIME_MS {
            break; // Prevent frame drops
        }
    }
}
```

---

This marker system provides a robust foundation for Arenic's 320-character simulation across 8 arenas while maintaining 60 FPS performance. The zero-sized markers enable efficient queries, and the clear categorization supports both rapid prototyping and production optimization.

The system integrates seamlessly with the 2-minute recording/replay cycles, ensuring deterministic behavior while providing the flexibility needed for complex gameplay mechanics across all character classes and arena types.