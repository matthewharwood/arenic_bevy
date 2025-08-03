# Poison Shot

A complete implementation guide for the Hunter's toxic projectile with knockback ability.

## Overview

The **Poison Shot** ability demonstrates the Hunter's mastery over chemical warfare through a specialized projectile that combines immediate knockback effects with sustained damage over time. When fired, the shot pushes the Hunter back one tile while delivering a toxic payload that inflicts 20 seconds of poison damage with a 12-second cooldown. This ability excels at both positioning control and sustained damage application.

## Game Design Philosophy

This ability showcases dual-purpose design through combined immediate and persistent effects:

**Newton's Third Law Design**: The Hunter's knockback creates interesting risk-reward positioning decisions, as the tactical advantage of distance comes with potential repositioning challenges.

**Sustained Pressure**: The 20-second poison duration creates ongoing battlefield pressure that rewards timing and target prioritization over raw damage output.

**Cooldown Efficiency**: The 12-second cooldown allows for strategic timing while preventing poison from being the Hunter's only combat option.

## Implementation Architecture

### Component-Based Design

```rust
PoisonShot {
    projectile_speed: 8.0,              // 8 tiles per second travel speed
    knockback_distance: 1.0,            // Hunter moves back 1 tile
    poison_damage_per_tick: 15.0,       // 15 damage every 2 seconds
    poison_duration: 20.0,              // 20 second total poison effect
    tick_interval: 2.0,                 // Poison damage every 2 seconds
    cooldown: 12.0,                     // 12 second ability cooldown
    projectile_range: 10.0,             // Maximum projectile travel distance
}

PoisonProjectile {
    position: Vec2,
    velocity: Vec2,
    visual_effect: Entity,
    poison_payload: PoisonEffect,
    hunter_origin: Entity,
}

PoisonEffect {
    damage_per_tick: f32,
    tick_interval: f32,
    duration_remaining: f32,
    last_tick_time: f32,
    visual_indicator: Entity,
}
```

### Event-Driven Systems

The ability operates through six coordinated systems:
1. **Knockback Physics** - Handles Hunter repositioning upon shot activation
2. **Projectile Tracking** - Manages poison shot flight and collision detection
3. **Poison Application** - Applies DOT effect upon successful target hit
4. **Damage Tick Management** - Handles periodic poison damage application
5. **Visual Coordination** - Manages toxic projectile and poison effect visualization
6. **Cooldown Management** - Tracks ability availability and recharge timing

### Required Components

```rust
// Core Components
ProjectileSpeed(8.0)
Knockback { force: 1.0, direction: Vec2::new(0.0, 0.0) }  // Direction set at cast time
DamageOverTime { 
    damage_per_tick: 15.0, 
    duration: Timer::from_seconds(20.0, TimerMode::Once), 
    tick_interval: Timer::from_seconds(2.0, TimerMode::Repeating) 
}
Cooldown(Timer::from_seconds(12.0, TimerMode::Once))
AttackRange(10.0)
InstantCast

// Projectile Components
ProjectileTarget(target_position)
ProjectileLifetime(Timer::from_seconds(2.0, TimerMode::Once))
EnemyOnly
RequiresLOS

// Visual Components
VisualEffect { effect_type: "poison_projectile".to_string(), scale: 1.0, color: Color::srgb(0.2, 0.8, 0.2), duration: Timer::from_seconds(2.0, TimerMode::Once) }
AudioEffect { sound_file: "poison_shot.ogg".to_string(), volume: 0.8, pitch: 1.0 }

// Upgrade Components
Upgrade 1:
- DamageOverTime { damage_per_tick: 20.0, duration: Timer::from_seconds(25.0, TimerMode::Once), tick_interval: Timer::from_seconds(2.0, TimerMode::Repeating) }

Upgrade 2:
- AreaOfEffect(2.0)
- Knockback { force: 1.5, direction: Vec2::new(0.0, 0.0) }
- PierceCount(1)

Upgrade 3:
- ConditionalReplay { condition_type: "on_target_death".to_string(), condition_value: 1.0, fallback_action: Some("spread_poison".to_string()) }
- DamageOverTime { damage_per_tick: 20.0, duration: Timer::from_seconds(25.0, TimerMode::Once), tick_interval: Timer::from_seconds(1.0, TimerMode::Repeating) }
- IgnoresCover
```

### High-Level Implementation Plan

1. **Ability Activation System**
   ```rust
   fn poison_shot_cast_system(
       mut commands: Commands,
       input: Res<ButtonInput<KeyCode>>,
       mut hunters: Query<(Entity, &Transform, &mut Cooldown), With<PoisonShotAbility>>,
       cursor: Res<CursorGridPosition>,
       recording: Res<RecordingState>,
   ) {
       if input.just_pressed(KeyCode::Digit2) {
           for (entity, transform, mut cooldown) in hunters.iter_mut() {
               if cooldown.0.finished() {
                   let direction = (cursor.world_pos - transform.translation.truncate()).normalize();
                   
                   // Apply knockback to hunter
                   commands.entity(entity).insert(Knockback {
                       force: 1.0,
                       direction: -direction,
                   });
                   
                   // Spawn poison projectile
                   commands.spawn((
                       PoisonShotProjectile,
                       ProjectileSpeed(8.0),
                       ProjectileTarget(cursor.world_pos.extend(0.0)),
                       ProjectileLifetime(Timer::from_seconds(2.0, TimerMode::Once)),
                       Transform::from_translation(transform.translation),
                       RecordableAction {
                           action_type: "poison_shot".to_string(),
                           timestamp: recording.current_time,
                           position: transform.translation,
                           parameters: HashMap::from([
                               ("direction_x".to_string(), direction.x),
                               ("direction_y".to_string(), direction.y),
                           ]),
                       },
                   ));
                   
                   cooldown.0.reset();
               }
           }
       }
   }
   ```

2. **Knockback Application System**
   ```rust
   fn knockback_system(
       mut commands: Commands,
       mut entities: Query<(Entity, &mut Transform, &Knockback)>,
       grid: Res<GridMap>,
   ) {
       for (entity, mut transform, knockback) in entities.iter_mut() {
           let new_pos = transform.translation.truncate() + knockback.direction * knockback.force * TILE_SIZE;
           
           // Check if new position is valid on grid
           if grid.is_walkable(new_pos) {
               transform.translation = new_pos.extend(transform.translation.z);
           }
           
           // Remove knockback component after application
           commands.entity(entity).remove::<Knockback>();
       }
   }
   ```

3. **Poison Application System**
   ```rust
   fn poison_impact_system(
       mut commands: Commands,
       projectiles: Query<(Entity, &Transform), With<PoisonShotProjectile>>,
       enemies: Query<(Entity, &Transform), With<Enemy>>,
       mut poison_events: EventWriter<ApplyPoisonEvent>,
   ) {
       for (proj_entity, proj_transform) in projectiles.iter() {
           for (enemy_entity, enemy_transform) in enemies.iter() {
               if is_collision(proj_transform, enemy_transform) {
                   poison_events.write(ApplyPoisonEvent {
                       target: enemy_entity,
                       dot_component: DamageOverTime {
                           damage_per_tick: 15.0,
                           duration: Timer::from_seconds(20.0, TimerMode::Once),
                           tick_interval: Timer::from_seconds(2.0, TimerMode::Repeating),
                       },
                   });
                   
                   commands.entity(proj_entity).despawn_recursive();
                   break;
               }
           }
       }
   }
   ```

4. **Damage Over Time System**
   ```rust
   fn poison_damage_system(
       mut commands: Commands,
       time: Res<Time>,
       mut poisoned: Query<(Entity, &mut Health, &mut DamageOverTime)>,
       mut damage_events: EventWriter<DamageEvent>,
   ) {
       for (entity, mut health, mut dot) in poisoned.iter_mut() {
           dot.duration.tick(time.delta());
           dot.tick_interval.tick(time.delta());
           
           if dot.tick_interval.just_finished() {
               damage_events.write(DamageEvent {
                   target: entity,
                   amount: dot.damage_per_tick,
                   damage_type: DamageType::Poison,
               });
           }
           
           if dot.duration.finished() {
               commands.entity(entity).remove::<DamageOverTime>();
           }
       }
   }
   ```

5. **Visual Effects System**
   ```rust
   fn poison_visual_system(
       mut commands: Commands,
       poisoned: Query<(Entity, &Transform), Added<DamageOverTime>>,
       mut removed: RemovedComponents<DamageOverTime>,
   ) {
       // Add poison visual when DOT is applied
       for (entity, transform) in poisoned.iter() {
           commands.spawn((
               PoisonVisualEffect,
               Transform::from_translation(transform.translation),
               Parent(entity),
           ));
       }
       
       // Remove visual when poison expires
       for entity in removed.read() {
           // Despawn associated visual effects
       }
   }
   ```

6. **Recording Integration**
   - Knockback direction and projectile spawn are deterministic
   - Poison application timing recorded with frame-perfect accuracy
   - DOT ticks synchronized with recording timeline
   - All visual effects tied to deterministic game state

## Step-by-Step Gameplay

### Phase 1: Shot Preparation and Firing (Tap Activation)
- **Input Method**: Tap to fire poison shot in Hunter's facing direction
- **Immediate Knockback**: Hunter automatically moves 1 tile backward from firing direction
- **Projectile Launch**: Toxic projectile begins traveling toward target at 8 tiles/second
- **Visual Feedback**: Green toxic trail follows projectile with distinctive poison effects

### Phase 2: Projectile Flight (Until Impact or Range Limit)
- **Travel Path**: Projectile flies in straight line for up to 10 tiles maximum range
- **Collision Detection**: Hit detection against enemies, obstacles, and environmental objects
- **Visual Trail**: Distinctive green particle trail marks projectile path
- **Audio Tracking**: Toxic hissing sound follows projectile movement

### Phase 3: Impact and Poison Application (On Hit)
- **Target Contact**: Projectile hits first enemy or obstacle in flight path
- **Poison Infection**: Target receives poison effect lasting 20 seconds
- **Visual Application**: Green poison aura appears around infected target
- **Damage Initiation**: First poison damage tick occurs immediately upon infection

### Phase 4: Sustained Poison Damage (20 Second Duration)
- **Periodic Damage**: Target takes 15 damage every 2 seconds for 20 seconds total
- **Visual Persistence**: Green poison effects continue throughout full duration
- **Damage Stacking**: Multiple poison shots can stack for increased damage
- **Effect Monitoring**: UI shows poison duration and remaining damage on target

## Knockback Positioning Mechanics

### Hunter Repositioning
```rust
fn apply_hunter_knockback(hunter: Entity, shot_direction: Vec2) {
    let knockback_direction = -shot_direction.normalize();
    let knockback_position = get_position(hunter) + knockback_direction;
    
    // Validate new position is legal and safe
    if is_valid_position(knockback_position) && !is_occupied(knockback_position) {
        set_position(hunter, knockback_position);
    } else {
        // Handle collision with walls or other characters
        handle_knockback_collision(hunter, knockback_direction);
    }
}

fn handle_knockback_collision(hunter: Entity, desired_direction: Vec2) {
    // Try alternative positions if direct knockback is blocked
    let alternative_positions = generate_safe_positions(hunter, desired_direction);
    
    if let Some(safe_pos) = alternative_positions.first() {
        set_position(hunter, *safe_pos);
    }
    // If no safe position exists, Hunter remains in place
}
```

### Strategic Positioning Considerations
- **Escape Routes**: Use knockback to create distance from approaching enemies
- **Obstacle Awareness**: Avoid firing when knockback would push into walls or hazards
- **Team Coordination**: Consider knockback effect on formation and ally positioning
- **Tactical Spacing**: Use knockback to maintain optimal engagement distance

## Poison Damage Over Time System

### DOT Calculation and Application
- **Total Damage**: 150 damage over 20 seconds (15 damage × 10 ticks)
- **Tick Frequency**: Damage applies every 2 seconds consistently
- **Stacking Mechanics**: Multiple poison shots increase tick damage additively
- **Resistance Factors**: Some enemies may have poison resistance reducing effectiveness

### Visual and Audio Feedback
- **Poison Aura**: Continuous green particle effects around poisoned targets
- **Damage Numbers**: Green damage numbers appear every 2 seconds during ticks
- **Audio Cues**: Subtle acid sizzling sounds accompany each damage tick
- **Status Indicators**: UI shows poison icon with remaining duration timer

## Upgrade Paths

### Tier 1: Concentrated Toxin
- **Damage Increase**: 15 → 20 damage per tick (200 total damage over 20 seconds)
- **Duration Extension**: 20 → 25 seconds total poison effect duration
- **Visual Enhancement**: More intense green poison effects with darker coloration
- **Strategic Value**: Higher damage output makes poison shots more threatening

### Tier 2: Explosive Payload
- **Area Infection**: Poison spreads to enemies within 2 tiles of impact point
- **Knockback Enhancement**: Hunter knockback distance increases to 1.5 tiles
- **Penetration**: Poison shot pierces through first target to hit secondary targets
- **Tactical Evolution**: Transforms single-target ability into area denial tool

### Tier 3: Virulent Strain
- **Poison Spreading**: Poisoned enemies spread infection to nearby allies when they die
- **Damage Acceleration**: Poison ticks become more frequent (every 1 second instead of 2)
- **Immunity Bypass**: Poison ignores enemy resistances and immunities
- **Ultimate Toxicity**: Creates cascading poison effects with battlefield-wide potential

## Projectile Physics and Targeting

### Ballistic Calculations
- **Straight Line**: Projectile travels in perfectly straight trajectory
- **Consistent Speed**: 8 tiles/second velocity throughout flight
- **Range Limitation**: Maximum 10-tile travel distance before dissipation
- **Collision Priority**: Hits first valid target in flight path

### Target Acquisition Strategy
- **Enemy Prioritization**: Aim for high-value targets that benefit from sustained damage
- **Formation Targeting**: With upgrades, target enemy clusters for maximum poison spread
- **Movement Prediction**: Lead moving targets to ensure poison shot connects
- **Positioning Optimization**: Use knockback strategically for better future shot angles

## Visual & Audio Design

### Shot Preparation and Firing
- **Visual**: Hunter draws back with specialized toxic-tipped ammunition
- **Animation**: Dramatic firing pose with emphasis on poison projectile launch
- **Audio**: Sharp bow/crossbow release sound mixed with toxic hiss
- **Knockback**: Hunter slides backward with realistic physics and dust effects

### Projectile Flight
- **Visual**: Green projectile with trailing toxic particle effects
- **Trail**: Distinctive poison vapor trail marking flight path
- **Audio**: Continuous toxic hissing sound throughout flight duration
- **Environment**: Slight green tint to areas projectile passes through

### Impact and Infection
- **Visual**: Toxic explosion effect with green poison clouds on impact
- **Animation**: Target recoils with green poison effects beginning to swirl around them
- **Audio**: Sharp impact sound followed by ongoing poison sizzling
- **Status**: Clear poison icon appears in target's status effects display

### Poison Duration
- **Visual**: Persistent green aura around poisoned target with periodic intensity pulses
- **Damage Ticks**: Green damage numbers with poison-themed styling every 2 seconds
- **Audio**: Continuous subtle acid damage sounds with periodic tick emphasis
- **Expiration**: Gradual fade of poison effects as duration completes