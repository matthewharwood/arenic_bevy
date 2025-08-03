# Boulder

A complete implementation guide for the Forager's rolling stone offensive ability.

## Overview

The **Boulder** ability unleashes the Forager's mastery over earth magic through a devastating 2x2 rolling stone that travels across the entire screen until obstructed. This resource-intensive ability requires 2 rocks from digging operations and deals significant damage while collecting resources from destroyed objects. The boulder's unlimited range and resource collection mechanics make it excellent for both combat and environmental interaction.

## Game Design Philosophy

This ability demonstrates high-impact resource conversion design with environmental interaction:

**Resource Investment Scaling**: The 2-rock cost creates meaningful resource management decisions, as players must weigh the substantial cost against the potential for high damage and resource collection.

**Environmental Synergy**: The ability to collect resources from destroyed objects creates interesting risk-reward decisions about boulder trajectory through destructible terrain.

**Piercing Momentum**: The unlimited range until obstruction rewards strategic positioning and enemy formation prediction for maximum damage potential.

## Implementation Architecture

### Component-Based Design

```rust
Boulder {
    size: GridArea::new(2, 2),          // 2x2 boulder dimensions
    base_damage: 200.0,                 // 200 damage per enemy hit
    movement_speed: 4.0,                // 4 tiles per second travel speed
    resource_cost: 2,                   // Requires 2 rocks from inventory
    max_range: f32::INFINITY,           // Unlimited range until obstruction
    resource_collection: true,          // Collects items from destroyed objects
}

RollingBoulder {
    position: Vec2,
    direction: Vec2,
    velocity: Vec2,
    size: Vec2,
    damage_dealt: HashMap<Entity, bool>, // Track hit enemies to prevent double-damage
    collected_resources: Vec<ItemDrop>,
    visual_effect: Entity,
    audio_source: Entity,
}

BoulderCollision {
    boulder_entity: Entity,
    collision_target: Entity,
    collision_type: CollisionType,      // Enemy, Destructible, Obstacle
    resource_drops: Vec<ItemType>,
}
```

### Event-Driven Systems

The ability coordinates through six dynamic systems:
1. **Launch Mechanics** - Handles boulder creation and initial trajectory calculation
2. **Movement Physics** - Manages rolling animation and collision detection
3. **Damage Application** - Applies damage to enemies while preventing double-hits
4. **Resource Collection** - Gathers items from destroyed environmental objects
5. **Collision Resolution** - Determines boulder stopping conditions and final position
6. **Visual Orchestration** - Manages rolling stone effects and destruction feedback

## Step-by-Step Gameplay

### Phase 1: Launch Preparation (Tap Activation)
- **Resource Validation**: Verify 2 rocks available in Forager's inventory
- **Direction Selection**: Boulder launches in Forager's current facing direction
- **Trajectory Preview**: Brief visual indicator shows initial boulder path
- **Resource Commitment**: 2 rocks automatically consumed upon activation

### Phase 2: Boulder Creation (Instant Spawn)
- **Size Manifestation**: 2x2 boulder materializes in front of Forager
- **Physics Activation**: Boulder immediately begins rolling in target direction
- **Visual Effects**: Stone particles and dust clouds mark boulder creation
- **Audio Initiation**: Deep rumbling sound begins boulder's journey

### Phase 3: Rolling Destruction (Until Obstruction)
- **Continuous Movement**: Boulder travels at 4 tiles per second in straight line
- **Enemy Collision**: Each enemy hit takes 200 damage (once per boulder)
- **Resource Collection**: Destructible objects yield materials when crushed
- **Momentum Preservation**: Boulder continues rolling until hitting solid obstruction

### Phase 4: Impact Resolution (Collision Stop)
- **Obstruction Contact**: Boulder stops upon hitting walls, large obstacles, or terrain
- **Final Damage**: Any enemies at stopping point receive collision damage
- **Resource Distribution**: Collected items scatter around boulder's final position
- **Boulder Dissolution**: Stone crumbles and disappears after stopping

## Resource Economics

### Cost-Benefit Analysis
```rust
fn calculate_boulder_value(potential_targets: &[Entity], destructibles: &[Entity]) -> f32 {
    let damage_value = potential_targets.len() as f32 * 200.0;
    let resource_value = destructibles.iter()
        .map(|obj| estimate_resource_value(*obj))
        .sum::<f32>();
    
    let total_value = damage_value + resource_value;
    let rock_cost = 2.0 * ROCK_VALUE;
    
    total_value - rock_cost // Net value calculation
}

fn estimate_resource_value(destructible: Entity) -> f32 {
    match get_destructible_type(destructible) {
        DestructibleType::ResourceCache => 150.0,  // High value resource deposits
        DestructibleType::Environment => 50.0,     // Trees, rocks, etc.
        DestructibleType::Container => 100.0,      // Chests, barrels, etc.
        DestructibleType::Structure => 75.0,       // Walls, barriers, etc.
    }
}
```

### Resource Collection Mechanics
- **Destructible Priority**: Boulder preferentially targets valuable resource objects
- **Collection Radius**: Items within 1 tile of boulder path automatically collected
- **Inventory Integration**: Collected resources added to Forager's inventory
- **Economic Scaling**: Higher-tier destructibles yield more valuable materials

## Trajectory and Targeting

### Path Calculation
- **Straight Line**: Boulder travels in perfectly straight line from launch point
- **Facing Direction**: Trajectory determined by Forager's orientation at cast time
- **Obstacle Prediction**: Advanced players can predict boulder stopping points
- **Enemy Formation**: Optimal usage requires enemy lineup prediction

### Strategic Positioning
- **Launch Angle**: Position Forager to maximize enemies in boulder path
- **Resource Corridors**: Aim through destructible object clusters for resource gain
- **Escape Routes**: Consider boulder path as temporary area denial after stopping
- **Team Coordination**: Communicate boulder trajectory to prevent ally interference

## Upgrade Paths

### Tier 1: Reinforced Stone
- **Damage Increase**: 200 → 280 damage per enemy collision
- **Size Enhancement**: Visual boulder becomes more imposing and intimidating
- **Penetration Power**: Can break through light obstacles that previously stopped it
- **Strategic Value**: Higher damage output and improved tactical flexibility

### Tier 2: Magnetic Collection
- **Collection Range**: Resource gathering extends to 2-tile radius around boulder
- **Selective Targeting**: Boulder slightly curves toward valuable destructible objects
- **Resource Bonus**: 25% chance for collected items to duplicate
- **Economic Evolution**: Transforms into powerful resource generation tool

### Tier 3: Earthen Avalanche
- **Multi-Boulder**: Spawns 3 boulders in spread pattern (center + 30° angles)
- **Chain Reaction**: Boulder destruction creates 4 smaller 1x1 boulders
- **Persistent Hazard**: Stopped boulders remain as destructible obstacles for 30 seconds
- **Master Destruction**: Area-wide devastation with lasting battlefield impact

## Environmental Interactions

### Destructible Object Types
- **Resource Nodes**: Stone deposits, crystal formations, mineral veins
- **Vegetation**: Trees, large bushes, thorn barriers
- **Structures**: Wooden walls, stone barriers, enemy fortifications
- **Containers**: Treasure chests, resource caches, storage barrels

### Collection Rewards
- **Common Materials**: Wood, stone fragments, basic crafting components
- **Uncommon Resources**: Refined metals, crystal shards, rare herbs
- **Special Items**: Occasionally rare crafting materials or equipment pieces
- **Currency**: Gold and other valuable trading commodities

## Visual & Audio Design

### Launch Sequence
- **Visual**: Ground cracks and bulges as massive boulder forms from earth
- **Animation**: Forager performs earth-shaping gestures with dramatic stance
- **Audio**: Deep rumbling earth sounds mixed with magical stone formation
- **Particle**: Dust clouds and stone fragments mark boulder materialization

### Rolling Motion
- **Visual**: Realistic rolling animation with rotating texture and debris trail
- **Environment**: Dust clouds kick up behind boulder, marking its passage
- **Audio**: Continuous rumbling that intensifies with speed and impacts
- **Screen Shake**: Subtle camera shake follows boulder movement for impact

### Collision Effects
- **Enemy Impact**: Dramatic impact effects with stone fragments flying
- **Destructible Breaking**: Object-specific destruction effects (wood splinters, stone chunks)
- **Audio**: Layered impact sounds - stone collision plus object-specific destruction
- **Particle**: Resource collection shows items flying toward boulder with collection sparkles

### Final Resolution
- **Visual**: Boulder crumbles into manageable stone pieces over 2 seconds
- **Resource Display**: Collected items scatter visibly around final position
- **Audio**: Final earth rumble as boulder settles and dissolves
- **Feedback**: UI notification shows total damage dealt and resources collected