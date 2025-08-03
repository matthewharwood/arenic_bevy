# Block

A complete implementation guide for the Warrior's directional projectile defense ability.

## Overview

The **Block** ability represents the Warrior's mastery over directional defense through tactical shield positioning. When activated, the Warrior raises a shield in the north direction initially, which can be rotated clockwise through subsequent taps to face any cardinal direction. This active defense ability deflects all projectiles coming from the shielded direction while maintaining enemy attack schedules and movement patterns for deterministic gameplay.

## Game Design Philosophy

This ability demonstrates active defense design through directional positioning mechanics:

**Directional Skill Expression**: Success requires understanding projectile sources and timing shield orientation to match incoming threat directions.

**Active Management**: The multi-tap rotation system creates ongoing tactical decisions about shield facing versus reaction time for threat changes.

**Predictable Integration**: Enemy projectiles and attacks continue on schedule, allowing players to learn patterns and optimize shield positioning.

## Implementation Architecture

### Component-Based Design

```rust
Block {
    initial_direction: Direction::North,  // Always starts facing north
    rotation_sequence: [North, East, South, West], // Clockwise rotation
    shield_coverage: 90.0,              // 90-degree coverage arc
    projectile_deflection: true,        // Deflects all projectiles from direction
    rotation_cooldown: 0.2,             // 0.2 seconds between rotations
    duration: f32::INFINITY,            // Active until manually deactivated
    movement_penalty: 0.5,              // 50% movement speed while blocking
}

ShieldState {
    current_direction: Direction,
    coverage_arc: f32,
    active: bool,
    last_rotation_time: f32,
    deflected_projectiles: u32,
    visual_effect: Entity,
}

ProjectileDeflection {
    shield_entity: Entity,
    projectile_entity: Entity,
    deflection_angle: f32,
    impact_position: Vec2,
    visual_effect: Entity,
}
```

### Required Components

```rust
// Core Components
InstantCast
DirectionalInput { required_direction: Vec2::new(0.0, 1.0), tolerance: 0.1 }  // Initially north
ConicalArea { angle: PI/2.0, range: 2.0, direction: Vec2::new(0.0, 1.0) }
MovementSpeed(0.5)  // 50% movement penalty
MultiTap { required_taps: 1, tap_window: 0.2, current_taps: 0, last_tap_time: 0.0 }

// Shield State Components
Duration(Timer::from_seconds(f32::INFINITY, TimerMode::Once))  // Active until cancelled
DamageReflection(1.0)  // 100% reflection for projectiles from shield direction
IsShielded

// Visual Components
VisualEffect { 
    effect_type: "shield_glow".to_string(), 
    scale: 1.0, 
    color: Color::srgb(0.2, 0.4, 1.0), 
    duration: Timer::from_seconds(f32::INFINITY, TimerMode::Once) 
}
AudioEffect { sound_file: "shield_raise.ogg".to_string(), volume: 0.8, pitch: 1.0 }

// Upgrade Components
Upgrade 1:
- ConicalArea { angle: PI * 0.75, range: 2.5, direction: Vec2::new(0.0, 1.0) }  // 135-degree coverage
- MovementSpeed(0.7)  // Reduced movement penalty

Upgrade 2:
- AreaOfEffect(1.5)  // Projectile deflection affects nearby area
- DamageReflection(1.5)  // 150% reflection damage
- MultiTap { required_taps: 1, tap_window: 0.1, current_taps: 0, last_tap_time: 0.0 }  // Faster rotation

Upgrade 3:
- AbilityReflection { reflect_chance: 0.5, duration: Timer::from_seconds(f32::INFINITY, TimerMode::Once) }
- DamageReduction(0.25)  // 25% damage reduction from all sources
- ResourceGeneration { mana_per_second: 2.0, health_per_second: 1.0 }  // Passive regen while blocking
```

### High-Level Implementation Plan

1. **Block Activation System**
   ```rust
   fn block_activation_system(
       mut commands: Commands,
       input: Res<ButtonInput<KeyCode>>,
       mut warriors: Query<(Entity, &mut Transform), With<BlockAbility>>,
       time: Res<Time>,
       recording: Res<RecordingState>,
   ) {
       if input.just_pressed(KeyCode::Digit2) {
           for (entity, mut transform) in warriors.iter_mut() {
               // Check if already blocking
               if let Ok(shield_state) = warriors.get::<ShieldState>(entity) {
                   // Rotate shield clockwise
                   let new_direction = rotate_direction_clockwise(shield_state.current_direction);
                   commands.entity(entity).insert(ShieldState {
                       current_direction: new_direction,
                       coverage_arc: PI/2.0,
                       active: true,
                       last_rotation_time: time.elapsed_seconds(),
                       deflected_projectiles: shield_state.deflected_projectiles,
                       visual_effect: shield_state.visual_effect,
                   });
               } else {
                   // Start blocking facing north
                   commands.entity(entity).insert((
                       ShieldState {
                           current_direction: Direction::North,
                           coverage_arc: PI/2.0,
                           active: true,
                           last_rotation_time: time.elapsed_seconds(),
                           deflected_projectiles: 0,
                           visual_effect: Entity::PLACEHOLDER,
                       },
                       MovementSpeed(0.5),
                       IsShielded,
                       RecordableAction {
                           action_type: "block_start".to_string(),
                           timestamp: recording.current_time,
                           position: transform.translation,
                           parameters: HashMap::new(),
                       },
                   ));
               }
           }
       }
   }
   ```

2. **Projectile Deflection System**
   ```rust
   fn projectile_deflection_system(
       mut commands: Commands,
       warriors: Query<(&Transform, &ShieldState), With<BlockAbility>>,
       projectiles: Query<(Entity, &Transform, &ProjectileSpeed), With<Projectile>>,
       mut deflection_events: EventWriter<ProjectileDeflectionEvent>,
   ) {
       for (warrior_transform, shield_state) in warriors.iter() {
           if !shield_state.active { continue; }
           
           for (proj_entity, proj_transform, proj_speed) in projectiles.iter() {
               let to_projectile = (proj_transform.translation - warrior_transform.translation).normalize();
               let shield_direction = direction_to_vector(shield_state.current_direction);
               
               // Check if projectile is coming from shielded direction
               let angle = shield_direction.angle_between(to_projectile);
               if angle.abs() <= shield_state.coverage_arc / 2.0 {
                   // Deflect projectile
                   deflection_events.write(ProjectileDeflectionEvent {
                       projectile: proj_entity,
                       shield_entity: warrior_entity,
                       deflection_angle: angle + PI,  // Reflect back
                   });
               }
           }
       }
   }
   ```

3. **Shield Direction Management**
   ```rust
   fn shield_direction_system(
       mut warriors: Query<(&mut Transform, &mut ShieldState), With<BlockAbility>>,
   ) {
       for (mut transform, mut shield_state) in warriors.iter_mut() {
           if shield_state.active {
               // Update visual shield direction
               let shield_direction = direction_to_vector(shield_state.current_direction);
               let shield_rotation = Quat::from_rotation_z(shield_direction.y.atan2(shield_direction.x));
               
               // Apply shield visual rotation without affecting character facing
               // This would be handled by the visual effects system
           }
       }
   }
   ```

4. **Movement Penalty System**
   ```rust
   fn block_movement_penalty_system(
       mut warriors: Query<(&mut MovementSpeed, &ShieldState), With<BlockAbility>>,
   ) {
       for (mut movement_speed, shield_state) in warriors.iter_mut() {
           if shield_state.active {
               movement_speed.0 = movement_speed.0.min(0.5);  // Cap at 50% while blocking
           }
       }
   }
   ```

5. **Block Deactivation System**
   ```rust
   fn block_deactivation_system(
       mut commands: Commands,
       input: Res<ButtonInput<KeyCode>>,
       warriors: Query<Entity, (With<BlockAbility>, With<ShieldState>)>,
   ) {
       // Allow canceling block with same key or different ability
       if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::Digit1) {
           for entity in warriors.iter() {
               commands.entity(entity).remove::<(ShieldState, IsShielded)>();
               commands.entity(entity).insert(MovementSpeed(1.0));  // Restore full movement
           }
       }
   }
   ```

6. **Recording Integration**
   - Shield rotation timing is deterministic based on input timing
   - Projectile deflection calculations are frame-perfect
   - Direction changes are recorded with exact timestamps
   - All visual effects synced to deterministic shield state

### Event-Driven Systems

The ability operates through five directional defense systems:
1. **Shield Management** - Handles activation, rotation, and deactivation
2. **Direction Control** - Manages clockwise rotation through cardinal directions
3. **Projectile Interception** - Detects and deflects incoming projectiles from shielded direction
4. **Coverage Calculation** - Determines which projectiles are within shield arc
5. **Visual Coordination** - Shows shield orientation and deflection effects

## Step-by-Step Gameplay

### Phase 1: Shield Activation (Initial Tap)
- **Input Method**: First tap activates shield in north direction
- **Visual Manifestation**: Shield appears in Warrior's hands facing north
- **Movement Penalty**: Warrior movement speed reduced to 50% while shield is active
- **Projectile Coverage**: 90-degree arc facing north provides protection from northern projectiles

### Phase 2: Directional Rotation (Subsequent Taps)
- **Rotation Input**: Each additional tap rotates shield clockwise to next cardinal direction
- **Sequence Order**: North → East → South → West → North (continuous cycle)
- **Rotation Timing**: 0.2-second cooldown between rotations prevents input spam
- **Coverage Update**: Shield protection immediately updates to new facing direction

### Phase 3: Active Deflection (Continuous Protection)
- **Projectile Detection**: System monitors all incoming projectiles for shield intersection
- **Direction Validation**: Only projectiles from shielded direction are deflected
- **Deflection Physics**: Projectiles bounce at realistic angles based on shield angle
- **Damage Negation**: Warrior takes no damage from successfully deflected projectiles

### Phase 4: Shield Deactivation (Manual Control)
- **Deactivation Method**: Hold tap for extended duration or use separate input to lower shield
- **Movement Recovery**: Warrior movement speed returns to normal when shield is lowered
- **Tactical Flexibility**: Can deactivate shield for movement and reactivate as needed
- **Resource Independence**: No mana or cooldown costs for activation/deactivation cycles

## Direction Management and Rotation

### Cardinal Direction System
```rust
#[derive(Copy, Clone, PartialEq)]
enum Direction {
    North = 0,
    East = 90,
    South = 180,
    West = 270,
}

impl Direction {
    fn next_clockwise(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    
    fn to_vector(self) -> Vec2 {
        match self {
            Direction::North => Vec2::new(0.0, 1.0),
            Direction::East => Vec2::new(1.0, 0.0),
            Direction::South => Vec2::new(0.0, -1.0),
            Direction::West => Vec2::new(-1.0, 0.0),
        }
    }
}

fn is_projectile_in_coverage(projectile_direction: Vec2, shield_direction: Direction) -> bool {
    let shield_vector = shield_direction.to_vector();
    let angle_difference = projectile_direction.angle_between(shield_vector).abs();
    
    angle_difference <= 45.0_f32.to_radians() // 90-degree total coverage (±45 degrees)
}
```

### Rotation Strategy
- **Predictive Positioning**: Rotate shield to face anticipated projectile sources
- **Threat Assessment**: Prioritize coverage based on most dangerous projectile directions
- **Reaction Timing**: Balance quick rotation against movement penalty duration
- **Pattern Recognition**: Learn enemy projectile patterns for optimal shield positioning

## Projectile Deflection Mechanics

### Coverage Calculation
- **Arc Size**: 90-degree coverage provides substantial protection without being overpowered
- **Direction Precision**: Must face correct cardinal direction for effective deflection
- **Angle Tolerance**: 45-degree tolerance on each side of shield direction
- **Projectile Types**: All projectile types affected equally by shield deflection

### Deflection Physics
- **Realistic Angles**: Projectiles deflect at angles consistent with shield surface impact
- **Velocity Preservation**: Deflected projectiles maintain original speed
- **Redirection Potential**: Deflected projectiles can potentially hit enemies
- **Visual Feedback**: Clear particle effects show successful deflection events

## Strategic Applications

### Defensive Positioning
- **Formation Protection**: Position to shield allies behind Warrior from projectile threats
- **Chokepoint Control**: Use shield to block projectile corridors and passages
- **Boss Encounters**: Face shield toward boss position during major projectile phases
- **Environmental Cover**: Combine shield with terrain features for comprehensive protection

### Rotation Timing
- **Threat Tracking**: Monitor multiple projectile sources and rotate as needed
- **Priority Management**: Focus shield toward most dangerous or frequent projectile sources
- **Formation Awareness**: Consider team positioning when choosing shield orientation
- **Movement Planning**: Plan shield rotations around movement needs and positioning

## Upgrade Paths

### Tier 1: Enhanced Coverage
- **Arc Expansion**: 90-degree → 120-degree coverage per direction
- **Rotation Speed**: 0.2 → 0.1 seconds between rotations
- **Movement Penalty**: 50% → 25% movement speed reduction while blocking
- **Strategic Value**: Wider protection with faster response time and improved mobility

### Tier 2: Active Defense
- **Deflection Damage**: Deflected projectiles gain 25% damage boost against enemies
- **Multi-Direction**: Can maintain shields in 2 directions simultaneously
- **Impact Counter**: Successful deflections reduce other ability cooldowns by 0.5 seconds
- **Tactical Evolution**: Transforms from pure defense to active battlefield control

### Tier 3: Fortress Protocol
- **Omnidirectional Shield**: Single activation creates 360-degree projectile protection
- **Damage Absorption**: Shield can absorb limited direct damage before breaking
- **Ally Protection**: Shield effects extend to allies within 1 tile of Warrior
- **Ultimate Defense**: Provides comprehensive protection for entire team formation

## Team Coordination and Formation

### Formation Leadership
- **Tank Positioning**: Warrior positions to intercept projectiles threatening allies
- **Direction Communication**: Clear shield orientation visible to team for positioning
- **Ally Coverage**: Team positions behind active shield direction for protection
- **Formation Flexibility**: Shield rotation enables dynamic team positioning changes

### Coordination Strategy
- **Threat Callouts**: Team communicates projectile threats requiring shield repositioning
- **Movement Synchronization**: Team movement coordinates with Warrior shield rotations
- **Role Integration**: Shield protection enables more aggressive positioning for damage dealers
- **Support Protection**: Prioritize shield coverage for vulnerable support characters

## Visual & Audio Design

### Shield Activation
- **Visual**: Warrior draws large defensive shield with authoritative stance
- **Animation**: Confident defensive posture with shield prominently displayed
- **Audio**: Metallic shield preparation sound with determination undertones
- **Direction**: Clear visual indicator shows shield facing direction (north initially)

### Rotation Mechanics
- **Visual**: Smooth shield rotation animation showing directional change
- **Animation**: Practiced defensive movement with shield repositioning
- **Audio**: Metallic rotation sound with tactical precision audio cues
- **Feedback**: Directional indicator updates to show new shield facing

### Projectile Deflection
- **Visual**: Dramatic spark effects at impact point with deflected projectile trail
- **Animation**: Shield recoils slightly with successful deflection showing impact
- **Audio**: Satisfying metallic clang with deflection confirmation sound
- **Trajectory**: Clear visual showing deflected projectile's new path

### Active Defense Status
- **Visual**: Shield glows with defensive energy indicating active protection
- **Animation**: Subtle shield movement showing readiness and alertness
- **Audio**: Quiet background metallic humming indicating active defense
- **UI**: Clear directional indicator shows current shield facing for team awareness