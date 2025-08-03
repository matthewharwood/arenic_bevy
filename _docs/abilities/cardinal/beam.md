# Beam

A complete implementation guide for the Cardinal's piercing divine offense ability.

## Overview

The **Beam** ability channels the Cardinal's divine energy into a devastating straight-line attack that pierces through multiple enemies. This channeled ability requires the Cardinal to remain stationary during a brief casting period, creating a tactical tradeoff between positioning vulnerability and significant damage output. The beam's piercing nature makes it particularly effective against grouped enemies and creates opportunities for strategic positioning rewards.

## Game Design Philosophy

This ability demonstrates risk-reward combat design through channeled casting mechanics:

**Vulnerability Trading**: The immobilization requirement creates tension between safety and damage output, rewarding players who can identify safe casting windows.

**Piercing Line Strategy**: The straight-line effect encourages tactical positioning and enemy grouping prediction, creating skill expression through battlefield awareness.

**Dual Utility Design**: The healing effect on allies adds strategic depth by allowing the Cardinal to position the beam to both damage enemies and heal allies simultaneously.

## Implementation Architecture

### Component-Based Design

```rust
Beam {
    channel_time: 1.0,              // 1 second channel duration
    beam_length: 8.0,               // 8 tile maximum range
    beam_width: 1.0,                // 1 tile width (straight line)
    damage_per_enemy: 120.0,        // Base damage to each enemy hit
    ally_heal_amount: 60.0,         // Healing to each ally hit
    cooldown: 8.0,                  // 8 second ability cooldown
    movement_restriction: true,      // Cannot move while channeling
}

BeamChannel {
    direction: Vec2,
    channel_progress: f32,
    affected_entities: Vec<Entity>,
    visual_buildup: Entity,
    damage_queue: Vec<(Entity, f32)>,
    heal_queue: Vec<(Entity, f32)>,
}

BeamEffect {
    start_position: Vec2,
    end_position: Vec2,
    width: f32,
    duration: 1.5,                  // Visual effect duration
}
```

### Event-Driven Systems

The ability coordinates through six systems:
1. **Channel Initiation** - Handles targeting and channel startup
2. **Immobilization Control** - Enforces movement restriction during channel
3. **Line Calculation** - Determines beam path and affected entities
4. **Damage Application** - Applies damage to enemies in beam path
5. **Healing Application** - Heals allies caught in beam line
6. **Visual Orchestration** - Manages buildup, beam, and aftermath effects

## Step-by-Step Gameplay

### Phase 1: Targeting and Initiation (Tap and Hold)
- **Input Method**: Tap and hold to begin channel, hold direction determines beam orientation
- **Directional Aiming**: Cardinal faces chosen direction, beam will fire in facing direction
- **Tactical Assessment**: Evaluate potential enemy and ally positions in beam path
- **Commitment Point**: Once channeling begins, direction cannot be changed

### Phase 2: Channel Buildup (1 Second Duration)
- **Movement Lock**: Cardinal cannot move or use other abilities during channel
- **Visual Buildup**: Divine energy accumulates around Cardinal with growing intensity
- **Audio Preparation**: Rising harmonic tone indicates building divine power
- **Strategic Vulnerability**: Cardinal exposed to enemy attacks during channel time

### Phase 3: Beam Projection (Instant Release)
- **Beam Fire**: 8-tile straight line beam projects from Cardinal's position
- **Simultaneous Effects**: All entities in beam path affected instantly
- **Enemy Damage**: Each enemy in path takes 120 damage
- **Ally Healing**: Each ally in path receives 60 healing

### Phase 4: Effect Resolution (1.5 Second Visual)
- **Damage Application**: Enemy health reduces immediately upon beam contact
- **Healing Application**: Ally health increases immediately upon beam contact
- **Visual Duration**: Beam effect persists for 1.5 seconds for clear feedback
- **Audio Feedback**: Divine impact sounds for damage, healing chimes for allies

## Tactical Positioning Strategies

### Optimal Beam Placement
- **Enemy Grouping**: Position to hit multiple enemies in straight line formation
- **Ally Inclusion**: Angle beam to heal injured allies while damaging enemies
- **Cover Utilization**: Use obstacles to protect Cardinal during channel vulnerability
- **Escape Routes**: Ensure safe positioning for post-beam movement

### Channel Timing
- **Enemy Prediction**: Channel when enemies will be in predictable formations
- **Safety Windows**: Use during enemy cooldowns or after enemy abilities
- **Team Coordination**: Coordinate with allies to create optimal beam opportunities
- **Interruption Avoidance**: Monitor enemy threats and cancel channel if necessary

## Line of Effect Calculations

### Beam Pathing
```rust
fn calculate_beam_targets(start: Vec2, direction: Vec2, length: f32) -> Vec<Entity> {
    let mut targets = Vec::new();
    let step_size = 0.5; // Half-tile precision
    
    for i in 0..((length / step_size) as usize) {
        let check_position = start + direction * (i as f32 * step_size);
        
        // Check for entities at this position
        if let Some(entity) = get_entity_at_position(check_position) {
            if !targets.contains(&entity) {
                targets.push(entity);
            }
        }
        
        // Check for line-of-sight blocking
        if is_blocked_position(check_position) {
            break; // Beam stops at obstacles
        }
    }
    
    targets
}
```

### Piercing Mechanics
- **Entity Penetration**: Beam passes through all characters (enemies and allies)
- **Obstacle Blocking**: Solid terrain and structures stop beam progression
- **Maximum Range**: Beam extends 8 tiles regardless of targets hit
- **Width Precision**: 1-tile width requires precise alignment for maximum effect

## Upgrade Paths

### Tier 1: Empowered Radiance
- **Damage Increase**: 120 → 160 damage per enemy hit
- **Healing Boost**: 60 → 80 healing per ally hit
- **Channel Reduction**: 1.0 → 0.75 second channel time
- **Strategic Value**: Higher damage output with reduced vulnerability window

### Tier 2: Expanding Light
- **Width Increase**: 1 tile → 2 tile width beam coverage
- **Range Extension**: 8 tiles → 10 tiles maximum beam length
- **Multi-Hit**: Enemies can be hit multiple times if they occupy multiple tiles in path
- **Tactical Evolution**: Easier to hit multiple targets with wider beam coverage

### Tier 3: Divine Convergence
- **Chain Effect**: Beam splits into 3 parallel lines after hitting first enemy
- **Healing Amplification**: Healed allies gain 25% damage boost for 8 seconds
- **Instant Channel**: No channel time required, beam fires immediately
- **Master Capability**: Eliminates vulnerability while adding explosive area coverage

## Visual & Audio Design

### Channel Buildup
- **Visual**: Brilliant white-gold energy accumulates around Cardinal's raised hands
- **Animation**: Cardinal assumes casting stance with robes billowing from divine wind
- **Audio**: Rising harmonic tone building to crescendo over 1 second
- **Particle**: Divine symbols and light motes swirl around Cardinal during channel

### Beam Projection
- **Visual**: Blazing column of white-gold divine light extending in chosen direction
- **Animation**: Cardinal's hands thrust forward directing the energy beam
- **Audio**: Powerful divine impact sound like thunder mixed with angelic chorus
- **Effect**: Beam creates bright line across battlefield with particle trail

### Target Impact
- **Enemy Effects**: Dark entities recoil with divine damage effects and sound
- **Ally Effects**: Golden healing light envelops allies with restoration sounds
- **Environmental**: Beam briefly illuminates entire path with divine radiance
- **Feedback**: Damage and healing numbers appear in contrasting colors

### Resolution Phase
- **Visual**: Beam gradually fades over 1.5 seconds leaving sparkling light trail
- **Audio**: Echoing divine resonance slowly diminishes to silence
- **Particle**: Lingering divine sparkles mark beam path for several seconds
- **Cooldown**: Ability icon shows 8-second recharge with divine light theme