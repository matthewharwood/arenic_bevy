# Border

A complete implementation guide for the Forager's defensive projectile barrier ability.

## Overview

The **Border** ability demonstrates the Forager's mastery over defensive earth magic, creating a 1x1 protective barrier that deflects all projectiles for one minute. This defensive ability requires resource management through rock consumption and strategic placement consideration, as the barrier can only be placed on previously dug ground. The ability excels at creating safe zones and controlling projectile-heavy encounters through strategic positioning.

## Game Design Philosophy

This ability showcases resource-dependent defensive design with environmental requirements:

**Resource Scarcity Management**: The rock consumption requirement creates meaningful resource decisions, as players must balance offensive boulder usage with defensive border needs.

**Environmental Prerequisite**: The requirement for dug ground creates interesting spatial planning, as players must prepare defensive positions through prior digging efforts.

**Persistent Area Control**: The 1-minute duration provides long-term battlefield value, rewarding strategic foresight over reactive placement.

## Implementation Architecture

### Component-Based Design

```rust
Border {
    barrier_size: GridArea::new(1, 1),  // 1x1 tile coverage
    duration: 60.0,                     // 60 second (1 minute) persistence
    cast_time: 1.5,                     // 1.5 second creation time
    resource_cost: 1,                   // Requires 1 rock from inventory
    placement_requirement: DugGround,    // Must be placed on dug terrain
    projectile_deflection: true,        // Deflects all projectile types
}

BorderBarrier {
    position: GridPos,
    duration_remaining: f32,
    visual_effect: Entity,
    deflection_count: u32,
    health: f32,                        // Can be destroyed by melee attacks
}

ProjectileDeflection {
    barrier_entity: Entity,
    deflected_projectile: Entity,
    deflection_angle: f32,
    visual_effect: Entity,
}
```

### Event-Driven Systems

The ability operates through five coordinated systems:
1. **Placement Validation** - Verifies dug ground requirement and rock availability
2. **Barrier Creation** - Spawns physical barrier with projectile deflection properties
3. **Projectile Interaction** - Handles deflection calculations and visual effects
4. **Duration Management** - Tracks barrier lifetime and automatic removal
5. **Visual Coordination** - Manages growing vines animation and deflection effects

## Step-by-Step Gameplay

### Phase 1: Placement Preparation (Pre-Activation)
- **Resource Check**: Verify at least 1 rock available in Forager's inventory
- **Ground Validation**: Target tile must have been previously dug by Forager
- **Strategic Assessment**: Evaluate optimal placement for projectile coverage
- **Tactical Timing**: Consider enemy projectile patterns and team positioning needs

### Phase 2: Barrier Creation (1.5 Second Cast)
- **Input Method**: Tap on valid dug ground tile to begin border creation
- **Resource Consumption**: 1 rock automatically removed from inventory
- **Visual Growth**: Thorny vines begin growing from the ground upward
- **Audio Feedback**: Earthy rumbling sound as natural barrier takes shape

### Phase 3: Active Deflection (60 Second Duration)
- **Projectile Detection**: Any projectile hitting the barrier automatically deflects
- **Deflection Mechanics**: Projectiles bounce at realistic angles based on impact
- **Visual Effects**: Sparks and natural energy show successful deflections
- **Persistent Protection**: Barrier remains active for full 60-second duration

### Phase 4: Barrier Expiration (Natural Decay)
- **Duration Completion**: Barrier automatically dissolves after 60 seconds
- **Visual Dissolution**: Vines gradually wither and sink back into ground
- **Ground Restoration**: Tile returns to normal dug ground state
- **Resource Availability**: Area becomes available for new border placement

## Resource Management System

### Rock Inventory Requirements
```rust
fn can_place_border(forager: Entity, target_pos: GridPos) -> bool {
    let rock_count = get_inventory_count(forager, ItemType::Rock);
    let ground_state = get_ground_state(target_pos);
    
    rock_count >= 1 && ground_state == GroundState::Dug
}

fn place_border(forager: Entity, target_pos: GridPos) -> Result<Entity, PlacementError> {
    if !can_place_border(forager, target_pos) {
        return Err(PlacementError::InvalidPlacement);
    }
    
    consume_item(forager, ItemType::Rock, 1);
    let barrier = spawn_border_barrier(target_pos);
    
    Ok(barrier)
}
```

### Rock Acquisition Strategy
- **Dig Ability Synergy**: Each dig operation grants +1 rock for future border usage
- **Resource Planning**: Balance digging for resources versus immediate tactical needs
- **Inventory Management**: Track rock count for strategic border placement timing
- **Economic Efficiency**: Consider rock opportunity cost against boulder offensive potential

## Projectile Deflection Mechanics

### Deflection Physics
- **Angle Calculation**: Projectiles reflect based on realistic impact angles
- **Velocity Preservation**: Deflected projectiles maintain original speed
- **Target Redirection**: Deflected projectiles can potentially hit enemies
- **Multi-Deflection**: Single barrier can deflect unlimited projectiles during duration

### Projectile Types Affected
- **Enemy Projectiles**: All hostile projectiles deflect away from team
- **Ally Projectiles**: Friendly projectiles also deflect (strategic consideration)
- **Environmental**: Falling debris and environmental projectiles affected
- **Special Effects**: Magical projectiles show unique deflection visual effects

## Upgrade Paths

### Tier 1: Reinforced Barriers
- **Duration Extension**: 60 seconds → 90 seconds barrier lifetime
- **Health Addition**: Barriers gain 100 HP, can absorb melee damage before breaking
- **Visual Enhancement**: Thicker, more imposing vine barriers with metallic thorns
- **Strategic Value**: Longer protection duration and resistance to direct assault

### Tier 2: Expanding Growth
- **Size Increase**: 1x1 → 2x2 tile coverage area per barrier
- **Resource Efficiency**: Still costs only 1 rock for larger protection zone
- **Multiple Projectiles**: Can deflect multiple simultaneous projectiles
- **Tactical Evolution**: Creates larger safe zones and choke point control

### Tier 3: Living Fortress
- **Damage Reflection**: Deflected projectiles gain 25% damage boost against enemies
- **Auto-Placement**: Barriers automatically spawn at Forager's location when taking damage
- **Resource Generation**: Barriers slowly generate 1 rock per 30 seconds while active
- **Master Defense**: Self-sustaining protective system with offensive benefits

## Tactical Applications

### Defensive Positioning
- **Choke Points**: Place barriers to funnel enemy projectiles away from team
- **Cover Creation**: Establish safe zones during heavy projectile phases
- **Healing Sanctuaries**: Create protected areas for team recovery and regrouping
- **Escape Routes**: Block projectile corridors to enable safe retreats

### Team Coordination
- **Tank Support**: Barriers protect main tank from ranged damage
- **Healer Protection**: Shield support characters from sniper-style attacks
- **Formation Control**: Channel team movement through protected corridors
- **Siege Breaking**: Counter enemy ranged advantage through strategic barrier placement

## Visual & Audio Design

### Placement Phase
- **Visual**: Targeting reticle shows valid dug ground with green confirmation
- **Ground Check**: Invalid placements show red overlay with rock requirement indicator
- **Audio**: Soft earth rumbling when targeting valid placement locations
- **UI**: Rock count display and placement validation feedback

### Barrier Growth
- **Visual**: Thorny vines rapidly spiral upward from dug earth
- **Animation**: Natural growth with realistic vine movement and leaf unfurling
- **Audio**: Deep earthy rumble mixed with organic growth sounds
- **Particle**: Soil particles and natural energy emanate during creation

### Active Deflection
- **Visual**: Bright spark effects at impact points with deflection trails
- **Projectile**: Deflected projectiles show altered trajectory with energy trails
- **Audio**: Sharp metallic ping sounds for successful deflections
- **Feedback**: Impact counter shows number of projectiles deflected

### Barrier Persistence
- **Visual**: Subtle swaying animation keeps barriers feeling alive
- **Duration**: Gradual color shift from vibrant green to brown as expiration approaches
- **Audio**: Quiet natural ambient sounds (wind through leaves, etc.)
- **Status**: UI indicator shows remaining barrier duration and health if applicable