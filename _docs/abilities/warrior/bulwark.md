# Bulwark

A complete implementation guide for the Warrior's frontal barrier and area denial defensive ability.

## Overview

The **Bulwark** ability represents the Warrior's mastery over battlefield fortification through instant frontal barrier creation. When activated with double-tap, the ability immediately erects a temporary defensive wall in front of the Warrior that absorbs all projectiles and cone attacks from the frontal direction. This area denial ability provides reliable protection against telegraphed attacks while maintaining enemy movement schedules and creating tactical positioning opportunities.

## Game Design Philosophy

This ability demonstrates area control design through directional damage absorption mechanics:

**Frontal Focus**: The forward-facing barrier creates clear tactical decisions about positioning and facing direction for optimal protection coverage.

**Predictable Protection**: The instant activation and reliable absorption provide dependable defense against telegraphed enemy attacks and projectile barrages.

**Temporary Fortification**: The brief duration creates windows of safety without permanently altering battlefield dynamics or enemy behavior patterns.

## Implementation Architecture

### Component-Based Design

```rust
Bulwark {
    barrier_width: 3.0,                 // 3-tile width frontal barrier
    barrier_height: 2.0,                // 2-tile height coverage
    duration: 4.0,                      // 4 second barrier persistence
    absorption_unlimited: true,         // Absorbs all projectiles during duration
    cast_time: 0.0,                     // Instant activation
    cooldown: 15.0,                     // 15 second ability cooldown
    facing_dependent: true,             // Barrier faces Warrior's current direction
}

FrontalBarrier {
    center_position: Vec2,
    facing_direction: Vec2,
    width: f32,
    height: f32,
    duration_remaining: f32,
    projectiles_absorbed: u32,
    visual_effect: Entity,
    collision_boundary: Entity,
}

ProjectileAbsorption {
    barrier_entity: Entity,
    absorbed_projectile: Entity,
    absorption_position: Vec2,
    visual_effect: Entity,
    audio_source: Entity,
}
```

### Event-Driven Systems

The ability operates through five barrier management systems:
1. **Instant Deployment** - Handles immediate barrier creation in Warrior's facing direction
2. **Projectile Absorption** - Intercepts and nullifies all frontal projectiles and attacks
3. **Area Denial** - Creates temporary safe zone behind barrier for team positioning
4. **Duration Management** - Tracks 4-second barrier lifetime and automatic removal
5. **Visual Coordination** - Manages barrier appearance and absorption effect feedback

## Step-by-Step Gameplay

### Phase 1: Instant Barrier Deployment (Double-Tap Activation)
- **Input Method**: Double-tap to instantly create frontal barrier
- **Direction Lock**: Barrier faces Warrior's current facing direction at activation moment
- **Immediate Effect**: 3x2 tile barrier appears instantly in front of Warrior
- **Area Coverage**: Barrier covers 3-tile width and 2-tile height frontal area

### Phase 2: Active Protection (4 Second Duration)
- **Projectile Interception**: All projectiles hitting barrier front are absorbed completely
- **Cone Attack Blocking**: Area attacks and cone effects stopped by barrier presence
- **Safe Zone Creation**: Area behind barrier provides protected positioning for allies
- **Unlimited Absorption**: No limit on number of projectiles barrier can absorb

### Phase 3: Tactical Positioning (Team Coordination)
- **Formation Protection**: Team positions behind barrier for projectile immunity
- **Offensive Opportunities**: Allies use protection to position for counterattacks
- **Movement Coverage**: Barrier provides mobile cover for tactical repositioning
- **Strategic Timing**: Team coordinates abilities during barrier protection window

### Phase 4: Barrier Dissolution (Automatic Expiration)
- **Duration Completion**: Barrier automatically dissolves after 4 seconds
- **Visual Fade**: Gradual barrier dissipation over final 0.5 seconds
- **Protection Loss**: Frontal projectile protection ends with barrier dissolution
- **Cooldown Start**: 15-second cooldown begins for next bulwark deployment

## Barrier Mechanics and Coverage

### Absorption System
```rust
fn check_projectile_absorption(barrier: &FrontalBarrier, projectile: &Projectile) -> bool {
    let barrier_bounds = calculate_barrier_boundaries(barrier);
    let projectile_trajectory = calculate_projectile_path(projectile);
    
    // Check if projectile intersects with barrier from front
    if trajectory_intersects_barrier(projectile_trajectory, barrier_bounds) {
        let impact_angle = calculate_impact_angle(projectile.direction, barrier.facing_direction);
        
        // Only absorb projectiles coming from frontal direction (within 180 degree arc)
        if impact_angle.abs() <= 90.0_f32.to_radians() {
            absorb_projectile(barrier, projectile);
            return true;
        }
    }
    
    false
}

fn calculate_barrier_boundaries(barrier: &FrontalBarrier) -> BoundingBox {
    let left_edge = barrier.center_position + barrier.facing_direction.perpendicular() * (barrier.width / 2.0);
    let right_edge = barrier.center_position - barrier.facing_direction.perpendicular() * (barrier.width / 2.0);
    let front_edge = barrier.center_position + barrier.facing_direction * 0.5;
    let back_edge = barrier.center_position - barrier.facing_direction * 0.5;
    
    BoundingBox::from_points(&[left_edge, right_edge, front_edge, back_edge])
}
```

### Coverage Area
- **Width**: 3-tile frontal coverage provides substantial team protection
- **Height**: 2-tile vertical coverage blocks most projectile trajectories
- **Direction**: Barrier orientation locked to Warrior's facing at activation
- **Penetration**: Projectiles from behind or sides pass through unaffected

## Strategic Applications

### Formation Defense
- **Projectile Phases**: Deploy during heavy enemy projectile attack phases
- **Team Coverage**: Position barrier to protect maximum number of allies
- **Chokepoint Control**: Block projectile corridors and attack lanes
- **Emergency Protection**: React to sudden projectile threats with instant coverage

### Tactical Positioning
- **Advance Cover**: Use barrier to enable aggressive team positioning
- **Retreat Protection**: Cover tactical withdrawals with barrier coverage
- **Formation Transitions**: Protect team during formation changes and movement
- **Objective Control**: Secure important positions with temporary fortification

## Timing and Positioning Strategy

### Optimal Activation Timing
- **Telegraph Recognition**: Deploy barrier when enemies telegraph major projectile attacks
- **Formation Preparation**: Use before team executes aggressive positioning maneuvers
- **Emergency Response**: React to overwhelming projectile pressure with instant protection
- **Cooldown Management**: Balance immediate needs with future protection requirements

### Facing Direction Optimization
- **Threat Assessment**: Face toward primary projectile sources before activation
- **Formation Consideration**: Position to protect maximum allies with barrier coverage
- **Battlefield Awareness**: Consider multiple threat angles when choosing facing direction
- **Movement Planning**: Coordinate facing direction with team tactical plans

## Upgrade Paths

### Tier 1: Reinforced Bulwark
- **Duration Extension**: 4 seconds → 6 seconds barrier persistence
- **Size Increase**: 3x2 tiles → 4x3 tiles coverage area
- **Cooldown Reduction**: 15 seconds → 12 seconds between deployments
- **Strategic Value**: Longer protection with larger coverage and more frequent access

### Tier 2: Aegis Protocol
- **Damage Absorption**: Barrier can absorb limited direct damage before breaking
- **Reflection Component**: 25% of absorbed projectiles reflect back toward enemies
- **Mobility Enhancement**: Warrior can move barrier by changing facing direction
- **Tactical Evolution**: Adds durability and offensive capabilities to defensive barrier

### Tier 3: Fortress Deployment
- **Omnidirectional Protection**: Barrier provides 360-degree projectile absorption
- **Damage Amplification**: Allies behind barrier deal 25% increased damage
- **Persistent Effect**: Barrier duration extended to 12 seconds
- **Ultimate Defense**: Creates mobile fortress providing comprehensive team protection

## Team Coordination and Communication

### Formation Integration
- **Protection Priority**: Team positions behind barrier for maximum projectile immunity
- **Offensive Coordination**: Allies use barrier protection to execute aggressive abilities
- **Movement Synchronization**: Team coordinates movement with barrier placement and timing
- **Role Support**: Barrier enables support characters to position more aggressively

### Communication Strategy
- **Barrier Callouts**: Clear communication about barrier timing and facing direction
- **Threat Assessment**: Team identifies projectile threats requiring barrier protection
- **Formation Planning**: Coordinate team positioning with barrier deployment strategy
- **Emergency Protocols**: Establish signals for emergency barrier usage during crisis

## Visual & Audio Design

### Instant Deployment
- **Visual**: Massive defensive wall materializes instantly from ground in front of Warrior
- **Animation**: Dramatic barrier emergence with earth and energy effects
- **Audio**: Powerful fortification sound with magical barrier creation tones
- **Scale**: Imposing 3x2 barrier clearly shows protection coverage area

### Active Protection
- **Visual**: Solid, imposing barrier with clear frontal protection indication
- **Animation**: Subtle energy flows showing active defensive capabilities
- **Audio**: Continuous barrier hum indicating active protection status
- **Boundary**: Clear visual boundaries show protection coverage and safe zones

### Projectile Absorption
- **Visual**: Dramatic absorption effects when projectiles impact barrier
- **Animation**: Projectiles disappear with energy dissipation effects
- **Audio**: Satisfying absorption sounds with barrier impact feedback
- **Feedback**: Clear indication that barrier successfully stopped incoming threats

### Barrier Dissolution
- **Visual**: Gradual barrier fade with dissipating energy effects
- **Animation**: Fortification dissolves back into ground over 0.5 seconds
- **Audio**: Barrier deactivation sound indicating end of protection
- **Transition**: Clear indication that frontal protection has ended