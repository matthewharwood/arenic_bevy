# AutoShot System: Mastering Distance Metrics in Tile-Based Game Development

## Historical Preface: The Mathematical Heritage Behind Distance Calculations

Understanding distance measurement has been fundamental to human civilization for millennia. The three distance metrics we'll explore in this tutorial each emerged from distinct mathematical traditions and practical needs.

### Euclidean Distance: The Classical Foundation

**Named after**: Euclid of Alexandria (c. 300 BCE), the "Father of Geometry"

**Historical significance**: Euclidean distance represents the "straight line" distance between two points—what we intuitively think of as "as the crow flies." This metric forms the foundation of classical geometry and is deeply embedded in our visual perception of space.

**Mathematical formula**: `d = √[(x₂-x₁)² + (y₂-y₁)²]`

**Why it creates circles**: When you ask "find all points exactly 2 units from this center," Euclidean distance naturally creates a perfect circle. Every point on that circle satisfies the equation `x² + y² = r²`.

### Manhattan Distance: Urban Navigation Mathematics

**Named after**: The grid street layout of Manhattan, New York City

**Alternative names**: 
- **Taxicab distance**: How far a taxi must drive on city streets
- **City block distance**: The number of blocks you must walk
- **L₁ metric**: In mathematical notation

**Historical development**: Emerged from urban planning and operations research in the mid-20th century, when mathematicians needed to model navigation through grid-based cities.

**Mathematical formula**: `d = |x₂-x₁| + |y₂-y₁|`

**Why it creates diamonds**: Manhattan distance forms diamond shapes because you can only move orthogonally (up/down/left/right), never diagonally.

### Chebyshev Distance: The Chess Master's Metric

**Named after**: Pafnuty Lvovich Chebyshev (1821-1894), Russian mathematician and probability theorist

**Alternative names**:
- **Chessboard distance**: How many moves a chess king needs to reach a square
- **L∞ metric**: The "infinity norm" in mathematical notation

**Historical significance**: Chebyshev made fundamental contributions to probability theory and approximation theory. His distance metric captures the idea of "worst-case" measurement—it's the maximum difference in any single coordinate.

**Mathematical formula**: `d = max(|x₂-x₁|, |y₂-y₁|)`

**Why it creates squares**: A chess king can move diagonally, so reaching any adjacent square (orthogonal or diagonal) costs exactly 1 move. This creates perfect squares around any center point.

---

## The AutoShot Challenge: When Mathematics Meets Game Design

### Problem Definition

In our Arenic battle system, characters with the `AutoShot` ability should automatically attack nearby bosses. The core requirements are:

1. **Range**: Characters attack bosses within 2-tile range
2. **Performance**: System must handle many characters vs few bosses efficiently
3. **Accuracy**: Range calculation must align with tile-based movement

### The Critical Question: Circles vs Squares

Here's where mathematical theory meets practical implementation. Our characters move on a discrete tile grid, but the naive approach uses Euclidean distance:

```rust
// ❌ PROBLEMATIC: Euclidean distance creates circular ranges
let distance = character_pos.distance(boss_pos); // √[(x₂-x₁)² + (y₂-y₁)²]
if distance <= 2.0 * TILE_SIZE {
    // Attack!
}
```

**The problem**: This creates circular attack ranges that don't align with our square tile grid.

## Visual Range Comparison: Seeing the Mathematics

Let's visualize how different distance metrics create different attack ranges around a character (C):

### Euclidean Distance (2-tile range):
```
    . . . . .
  . . X X X . .
  . X X X X X .
  . X X C X X .
  . X X X X X .
  . . X X X . .
    . . . . .
```

### Chebyshev Distance (2-tile range):
```
  X X X X X
  X X X X X
  X X C X X
  X X X X X
  X X X X X
```

**Key insight**: Chebyshev distance creates perfect squares that align naturally with tile boundaries, while Euclidean distance creates circles that awkwardly intersect tiles.

---

## Mental Model Construction: Building Understanding

### Core Concept 1: Coordinate Systems

Think of our game world as having two coordinate systems:

1. **World Space**: Continuous floating-point coordinates (Vec3)
2. **Tile Space**: Discrete grid coordinates (row, column integers)

**Analogy**: World space is like GPS coordinates (precise latitude/longitude), while tile space is like addresses on city blocks (discrete street intersections).

### Core Concept 2: Distance Metrics as Different "Rules of Movement"

- **Euclidean**: "Teleportation allowed" - you can move directly through space
- **Manhattan**: "City walking" - you must follow street grids, no diagonal shortcuts
- **Chebyshev**: "Chess king movement" - you can move to any adjacent square (including diagonals) in one step

### Core Concept 3: The Range Problem

When a character stands at the center of a tile, what does "2-tile range" mean?

```rust
// Character at position (0, 0)
// These positions should be within 2-tile range:

(0, 2)  // 2 tiles up (Chebyshev distance = 2)
(2, 2)  // 2 tiles up, 2 tiles right (Chebyshev distance = 2)
(1, 2)  // 1 tile right, 2 tiles up (Chebyshev distance = 2)

// But with Euclidean distance:
distance_to_0_2 = √[(0-0)² + (2-0)²] = 2.0        ✓ In range
distance_to_2_2 = √[(2-0)² + (2-0)²] = √8 = 2.83  ✗ Out of range!
distance_to_1_2 = √[(1-0)² + (2-0)²] = √5 = 2.24  ✗ Out of range!
```

**The revelation**: Euclidean distance fails for diagonal positions that should clearly be reachable in a tile-based system.

---

## Implementation Deep-Dive: From Theory to Code

### Step 1: Coordinate Conversion Functions

First, we need clean conversions between world space and tile space:

```rust
use bevy::prelude::*;
use crate::arena::constants::TILE_SIZE;

/// Convert world coordinates to tile grid coordinates
pub fn world_to_tile_coords(world_pos: Vec3) -> (i32, i32) {
    let tile_x = (world_pos.x / TILE_SIZE).round() as i32;
    let tile_y = (world_pos.y / TILE_SIZE).round() as i32;
    (tile_x, tile_y)
}

/// Convert tile coordinates to world coordinates (center of tile)
pub fn tile_to_world_coords(tile_x: i32, tile_y: i32) -> Vec3 {
    Vec3::new(
        tile_x as f32 * TILE_SIZE,
        tile_y as f32 * TILE_SIZE,
        0.0
    )
}
```

**Active Recall Checkpoint**: Before reading further, can you explain why we use `.round()` instead of casting directly to `i32`? What would happen with negative coordinates?

<details>
<summary>Answer</summary>

We use `.round()` because:
1. Direct casting truncates toward zero: `-1.7 as i32 = -1`, but we want `-2`
2. `.round()` gives us proper mathematical rounding: `-1.7.round() = -2`
3. This ensures that positions near tile boundaries are assigned to the correct tile
4. Without rounding, characters at (-0.1, 0.1) would map to tile (0, 0) instead of (-1, 0)

</details>

### Step 2: Chebyshev Distance Implementation

```rust
/// Calculate Chebyshev distance between two tile coordinates
pub fn chebyshev_distance(pos1: (i32, i32), pos2: (i32, i32)) -> i32 {
    let dx = (pos1.0 - pos2.0).abs();
    let dy = (pos1.1 - pos2.1).abs();
    dx.max(dy)  // Maximum of the coordinate differences
}

/// Check if target is within Chebyshev range of source
pub fn is_within_chebyshev_range(source: Vec3, target: Vec3, range: i32) -> bool {
    let source_tile = world_to_tile_coords(source);
    let target_tile = world_to_tile_coords(target);
    chebyshev_distance(source_tile, target_tile) <= range
}
```

**Mental Model Reinforcement**: Chebyshev distance is like asking "What's the worst coordinate difference?" If you need to move 3 tiles horizontally and 1 tile vertically, the Chebyshev distance is `max(3, 1) = 3` because that's your bottleneck.

### Step 3: The Complete AutoShot System

Here's our performance-optimized, mathematically correct autoshot system:

```rust
/// System that handles autoshot ability using Chebyshev distance
/// Note: This example shows multi-character support. Current codebase uses single character.
fn autoshot_ability(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    mut timer: Local<Timer>,
    character_query: Query<&GlobalTransform, (With<Character>, With<AutoShot>, With<Active>)>,
    boss_query: Query<&GlobalTransform, (With<Boss>, With<Active>)>,
) {
    // Initialize timer on first run
    if timer.duration().as_secs_f32() == 0.0 {
        *timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    }

    timer.tick(time.delta());
    
    if !timer.just_finished() {
        return;
    }
    
    const AUTOSHOT_RANGE: i32 = 2; // 2-tile range
    
    // Handle single character case (matching current codebase)
    let Ok(character_transform) = character_query.get_single() else { return; };
    let character_pos = character_transform.translation();
    
    // PERFORMANCE OPTIMIZATION: Iterate by bosses (fewer entities)
    for boss_transform in boss_query.iter() {
        let boss_pos = boss_transform.translation();
        
        // Use Chebyshev distance for tile-aligned range checking
        if is_within_chebyshev_range(character_pos, boss_pos, AUTOSHOT_RANGE) {
            spawn_projectile(
                &mut commands,
                &mut materials,
                &mut meshes,
                character_pos,
                boss_pos,
            );
        }
    }
}

fn spawn_projectile(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    origin: Vec3,
    target: Vec3,
) {
    let distance = origin.distance(target);
    let travel_time = distance / TILE_SIZE; // 1 tile per second
    
    let projectile_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2), // Red projectile
        metallic: 0.0,
        perceptual_roughness: 0.1,
        ..default()
    });
    
    let projectile_mesh = meshes.add(Sphere::new(2.5));
    
    commands.spawn((
        Projectile,
        Origin(origin),
        Target(target),
        TimeToLive(0.0, travel_time),
        PbrBundle {
            mesh: projectile_mesh,
            material: projectile_material,
            transform: Transform::from_translation(origin),
            ..default()
        },
    ));
}
```

### Architecture Design Notes

**Current Implementation Pattern**: The example above shows a single-character approach (matching the current codebase), but demonstrates how to extend to multiple characters.

**For Multiple Characters** (future enhancement), you would change the system to:

```rust
// Multi-character version (for future use)
for character_transform in character_query.iter() {
    let character_pos = character_transform.translation();
    
    for boss_transform in boss_query.iter() {
        let boss_pos = boss_transform.translation();
        
        if is_within_chebyshev_range(character_pos, boss_pos, AUTOSHOT_RANGE) {
            // Spawn projectile for this character-boss pair
        }
    }
}
```

### Performance Architecture Insight

Notice our iteration strategy: **we iterate by bosses, not characters**. Why?

- **Typical scenario**: 10-50 characters, 1-3 bosses
- **Boss-centric approach**: 3 outer loops × 50 inner checks = 150 operations
- **Character-centric approach**: 50 outer loops × 3 inner checks = 150 operations

The boss-centric approach enables easier spatial optimizations later (like boss proximity grids) and provides better data locality when characters are grouped by spatial regions.

---

## Testing and Validation: Proving Correctness

### Unit Tests for Distance Calculations

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena::constants::TILE_SIZE;

    #[test]
    fn test_coordinate_conversion_symmetry() {
        let original_world = Vec3::new(38.0, -57.0, 0.0); // 2 tiles right, 3 tiles down
        let tile_coords = world_to_tile_coords(original_world);
        let converted_back = tile_to_world_coords(tile_coords.0, tile_coords.1);
        
        assert_eq!(tile_coords, (2, -3));
        assert_eq!(converted_back, Vec3::new(38.0, -57.0, 0.0));
    }

    #[test]
    fn test_chebyshev_distance_calculations() {
        // Test orthogonal movement (should equal Manhattan distance)
        assert_eq!(chebyshev_distance((0, 0), (0, 3)), 3); // 3 tiles up
        assert_eq!(chebyshev_distance((0, 0), (2, 0)), 2); // 2 tiles right
        
        // Test diagonal movement (Chebyshev advantage over Manhattan)
        assert_eq!(chebyshev_distance((0, 0), (2, 2)), 2); // Diagonal: Chebyshev = 2
        
        // Manhattan would be 4, demonstrating the difference
        let manhattan = (2 - 0).abs() + (2 - 0).abs();
        assert_eq!(manhattan, 4);
    }

    #[test]
    fn test_range_boundary_conditions() {
        let center = Vec3::new(0.0, 0.0, 0.0);
        
        // Exactly at range boundary
        let target_2_tiles = Vec3::new(2.0 * TILE_SIZE, 2.0 * TILE_SIZE, 0.0);
        assert!(is_within_chebyshev_range(center, target_2_tiles, 2));
        
        // Just outside range boundary
        let target_3_tiles = Vec3::new(3.0 * TILE_SIZE, 2.0 * TILE_SIZE, 0.0);
        assert!(!is_within_chebyshev_range(center, target_3_tiles, 2));
    }

    #[test]
    fn test_floating_point_precision() {
        // Test near-boundary positions to ensure consistent rounding
        let center = Vec3::new(0.0, 0.0, 0.0);
        
        // Position slightly inside a tile boundary
        let just_inside = Vec3::new(1.99 * TILE_SIZE, 0.0, 0.0);
        assert!(is_within_chebyshev_range(center, just_inside, 2));
        
        // Position slightly outside a tile boundary
        let just_outside = Vec3::new(2.01 * TILE_SIZE, 0.0, 0.0);
        assert!(!is_within_chebyshev_range(center, just_outside, 2));
    }

    #[test]
    fn test_negative_coordinates() {
        // Ensure negative coordinates work correctly
        let pos1 = Vec3::new(-TILE_SIZE, -TILE_SIZE, 0.0); // (-1, -1)
        let pos2 = Vec3::new(TILE_SIZE, TILE_SIZE, 0.0);    // (1, 1)
        
        // Distance should be max(|-1-1|, |-1-1|) = max(2, 2) = 2
        assert!(is_within_chebyshev_range(pos1, pos2, 2));
        assert!(!is_within_chebyshev_range(pos1, pos2, 1));
    }
}
```

### Integration Testing Strategy

**Test Scenario 1: Character Movement Validation**
1. Place character at tile (0, 0)
2. Place boss at various positions
3. Move character and verify autoshot triggers at correct ranges
4. Ensure diagonal positions work correctly

**Test Scenario 2: Performance Under Load**
1. Spawn 100 characters with AutoShot
2. Spawn 5 bosses
3. Measure system execution time over 1000 frames
4. Verify frame time stays under budget (16.67ms for 60 FPS)

**Test Scenario 3: Edge Case Validation**
1. Test with characters at arena boundaries
2. Test with overlapping character/boss positions
3. Test with rapidly moving entities
4. Verify no duplicate projectiles spawn

---

## Common Pitfalls and Debugging Strategies

### Pitfall 1: Mixing Distance Metrics

```rust
// ❌ WRONG: Using Euclidean distance for range check but Chebyshev for gameplay
if character_pos.distance(boss_pos) <= 2.0 * TILE_SIZE {
    // This creates circular ranges that don't match tile movement!
}
```

**Debug strategy**: Add visual range indicators. Draw the actual attack range as colored tiles to see the mismatch immediately.

### Pitfall 2: Floating-Point Coordinate Drift

Characters might drift slightly between tiles due to floating-point arithmetic, causing inconsistent range calculations.

```rust
// ✅ SOLUTION: Consistent rounding strategy
pub fn world_to_tile_coords(world_pos: Vec3) -> (i32, i32) {
    // Always round to nearest tile center
    let tile_x = (world_pos.x / TILE_SIZE + 0.5).floor() as i32;
    let tile_y = (world_pos.y / TILE_SIZE + 0.5).floor() as i32;
    (tile_x, tile_y)
}
```

### Pitfall 3: Performance Degradation

As entity counts grow, naive O(n×m) checking becomes expensive.

**Debug strategy**: Add performance timers and log when autoshot system exceeds time budget.

```rust
use std::time::Instant;

fn autoshot_ability(/* ... */) {
    let start_time = Instant::now();
    
    // ... system logic ...
    
    let elapsed = start_time.elapsed();
    if elapsed.as_millis() > 2 { // 2ms budget
        warn!("AutoShot system took {}ms (budget: 2ms)", elapsed.as_millis());
    }
}
```

---

## Active Recall Challenges

Before moving to the next section, test your understanding:

### Challenge 1: Range Calculation
A character is at world position `Vec3::new(57.0, 38.0, 0.0)`. A boss is at `Vec3::new(95.0, 76.0, 0.0)`. Assuming `TILE_SIZE = 19.0`, are they within 2-tile Chebyshev range?

**Work it out step by step**:
1. Convert to tile coordinates
2. Calculate Chebyshev distance
3. Compare with range

<details>
<summary>Solution</summary>

1. **Tile coordinates**:
   - Character: `(57.0/19.0, 38.0/19.0) = (3, 2)`
   - Boss: `(95.0/19.0, 76.0/19.0) = (5, 4)`

2. **Chebyshev distance**:
   - `dx = |3-5| = 2`
   - `dy = |2-4| = 2`
   - `distance = max(2, 2) = 2`

3. **Comparison**: Distance = 2, Range = 2, so **YES, they are in range**.

</details>

### Challenge 2: Design Decision
Why do we use Chebyshev distance instead of Manhattan distance for our tile-based game?

<details>
<summary>Answer</summary>

**Chebyshev allows diagonal movement**, which is natural for tile-based games. If a player can move diagonally to adjacent tiles in one turn, then diagonal positions should be considered "1 tile away," not 2 tiles away (as Manhattan would calculate).

Chebyshev distance = "moves a chess king would need"
Manhattan distance = "moves a chess rook would need"

Since most tile-based games allow diagonal movement, Chebyshev is the natural choice.

</details>

---

## Implementation Notes: From Tutorial to Production

### Bevy Version Compatibility

This tutorial targets **Bevy 0.16** and uses current best practices for component spawning and ECS patterns. Key architectural choices:

- **PbrBundle**: Proper way to spawn 3D entities with mesh and material
- **Single character queries**: Matches current codebase pattern with `.get_single()`
- **Correct imports**: Uses `crate::arena::constants::TILE_SIZE` matching project structure

### Integration with Existing Systems

The autoshot system integrates seamlessly with existing projectile components:
- **Projectile**: Marker component for projectile entities
- **Origin(Vec3)**: Starting position for lerp-based movement  
- **Target(Vec3)**: Destination position
- **TimeToLive(current, max)**: Duration and cleanup timer

### Error Handling Patterns

Production code should include defensive programming:

```rust
// Handle missing components gracefully
let Ok(character_transform) = character_query.get_single() else {
    // No character with AutoShot ability - system does nothing
    return;
};

// Validate tile coordinates are reasonable
pub fn world_to_tile_coords(world_pos: Vec3) -> Option<(i32, i32)> {
    const MAX_COORDINATE: f32 = 10000.0;
    
    if world_pos.x.abs() > MAX_COORDINATE || world_pos.y.abs() > MAX_COORDINATE {
        return None; // Position too extreme, likely an error
    }
    
    Some((
        (world_pos.x / TILE_SIZE).round() as i32,
        (world_pos.y / TILE_SIZE).round() as i32,
    ))
}
```

### Performance Monitoring

Add performance tracking for production deployment:

```rust
use std::time::Instant;

fn autoshot_ability(/* ... */) {
    #[cfg(debug_assertions)]
    let start_time = Instant::now();
    
    // ... system implementation ...
    
    #[cfg(debug_assertions)]
    {
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > 2 { // 2ms budget for 60 FPS
            warn!("AutoShot system exceeded budget: {}ms", elapsed.as_millis());
        }
    }
}
```

---

## Advanced Optimization: Spatial Partitioning

For games with hundreds of entities, we can optimize further using spatial partitioning:

### Concept: Boss Influence Zones

Instead of checking every character against every boss, we can:

1. **Pre-calculate boss zones**: For each boss, determine which tiles are within attack range
2. **Spatial hash characters**: Group characters by their current tile
3. **Zone-based lookup**: For each boss zone tile, check if any characters occupy it

```rust
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct TileCoord(i32, i32);

/// Spatial optimization: group characters by tile position
fn build_character_spatial_index(
    character_query: &Query<(Entity, &GlobalTransform), (With<Character>, With<AutoShot>, With<Active>)>,
) -> HashMap<TileCoord, Vec<(Entity, Vec3)>> {
    let mut index = HashMap::new();
    
    for (entity, transform) in character_query.iter() {
        let world_pos = transform.translation();
        let tile_coord = world_to_tile_coords(world_pos);
        let tile_key = TileCoord(tile_coord.0, tile_coord.1);
        
        index.entry(tile_key)
             .or_insert_with(Vec::new)
             .push((entity, world_pos));
    }
    
    index
}

/// Generate all tile coordinates within Chebyshev range
fn generate_range_tiles(center: (i32, i32), range: i32) -> Vec<TileCoord> {
    let mut tiles = Vec::new();
    
    for dx in -range..=range {
        for dy in -range..=range {
            tiles.push(TileCoord(center.0 + dx, center.1 + dy));
        }
    }
    
    tiles
}
```

This optimization reduces complexity from O(characters × bosses) to O(bosses × tiles_in_range), which is much more favorable when you have many characters but consistent boss counts.

---

## Conclusion: Connecting Implementation to Principles

### What We've Accomplished

1. **Mathematical Foundation**: We understand why Chebyshev distance creates squares and why that aligns with tile-based gameplay
2. **Performance Architecture**: We designed a boss-centric system that scales well
3. **Robust Testing**: We built comprehensive tests that catch boundary conditions and floating-point issues
4. **Debugging Strategies**: We know how to visualize and troubleshoot range calculation problems

### Key Mental Models to Retain

- **Distance metrics are tools for different movement rules**: Euclidean for free movement, Manhattan for orthogonal-only, Chebyshev for diagonal-allowed
- **Coordinate systems must be consistent**: Always convert to the same space before calculating distances
- **Performance design matters early**: Boss-centric iteration and spatial partitioning scale much better than naive approaches

### Generalization: Beyond AutoShot

These principles apply to many game systems:

- **Spell targeting**: Area-of-effect abilities need consistent range shapes
- **AI pathfinding**: Choose distance metrics that match allowed movement
- **Collision detection**: Spatial partitioning optimizes any proximity-based system
- **Resource gathering**: Characters collecting nearby items use similar range logic

### Learning Artifacts for Long-Term Retention

**Flashcard Set**: Key distance metric formulas and their applications
**Reference Implementation**: The complete autoshot system as a template for similar features
**Decision Tree**: When to use each distance metric in game development

### Further Exploration

- **Advanced spatial structures**: Quadtrees, R-trees, and other spatial indices
- **Pathfinding algorithms**: A* with different heuristic functions
- **Performance profiling**: Tools for measuring and optimizing game systems
- **Mathematical game design**: How mathematical properties affect gameplay feel

---

*This tutorial demonstrated cognitive load theory in action: we built from familiar concepts (chess movement) to complex implementations (optimized ECS systems), used active recall throughout, and created mental models that transfer to other game development challenges.*

*The mathematical heritage of distance metrics connects our practical code to centuries of mathematical development, giving context that aids long-term retention and deeper understanding.*