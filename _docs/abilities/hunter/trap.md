# Trap

A complete implementation guide for the Hunter's explosive area denial utility ability using single-component architecture.

## Overview

The **Trap** ability demonstrates the Hunter's mastery over battlefield control through strategic explosive placement. When activated, the Hunter instantly places a concealed trap on their current grid position with a 3-second cooldown. The trap triggers when any enemy steps on it, creating a 2x2 area explosion that deals significant damage. This ability excels at area denial, enemy routing, and strategic positioning control.

## Game Design Philosophy

This ability showcases tactical preparation design through predictive placement mechanics:

**Predictive Skill Expression**: Success requires understanding enemy movement patterns and anticipating future positions rather than reactive combat responses.

**Risk-Free Placement**: The instant cast and short cooldown encourage experimentation and tactical creativity without punishing failed predictions.

**Area Control**: The 2x2 explosion creates meaningful space control that affects enemy routing and formation decisions.

## Component Architecture

### Entity Composition Pattern

Following the single-component architecture, the Trap ability uses simple, single-value components:

```rust
// Trap Placement Action
commands.spawn((
    // Marker Components
    TrapPlaceAction,
    InstantCast,
    
    // Ability Components
    Cooldown(3.0),
    TargetPosition(hunter_grid_pos),
    
    // Recording
    RecordableAction,
    Timestamp(recording.current_time),
));

// Placed Trap Entity
commands.spawn((
    // Marker Components
    ExplosiveTrap,
    Concealed,
    AllyVisible,
    
    // Position Components
    GridPosition(grid_pos),
    Transform::from_translation(world_pos),
    
    // Effect Components
    TriggerRadius(0.5),
    ExplosionRadius(2.0),
    Damage(150.0),
    
    // Time Components
    Duration(60.0),
    ElapsedTime(0.0),
    
    // Owner
    Owner(hunter_entity),
));

// Trap Explosion Entity (on trigger)
commands.spawn((
    // Marker Components
    TrapExplosion,
    AreaEffect,
    
    // Position Components
    Origin(trap_pos),
    Radius(2.0),
    
    // Effect Components
    Damage(150.0),
    
    // Time Components
    Duration(1.0),
    ElapsedTime(0.0),
    
    // Visual Components
    StartRadius(0.5),
    EndRadius(4.0),
    EmissiveIntensity(3.0),
    EmissiveColor(Color::srgb(1.0, 0.3, 0.0)),
    ParticleCount(100),
    ScreenShakeIntensity(0.8),
));
```

### Core Components Used

All components are single-value/single-purpose:

```rust
// Position Components
pub struct GridPosition(pub IVec2);
pub struct Origin(pub Vec3);
pub struct TargetPosition(pub Vec3);

// Effect Components
pub struct TriggerRadius(pub f32);
pub struct ExplosionRadius(pub f32);
pub struct Radius(pub f32);
pub struct Damage(pub f32);
pub struct Cooldown(pub f32);

// Time Components
pub struct Duration(pub f32);
pub struct ElapsedTime(pub f32);
pub struct Timestamp(pub f32);

// Visual Components
pub struct StartRadius(pub f32);
pub struct EndRadius(pub f32);
pub struct EmissiveIntensity(pub f32);
pub struct EmissiveColor(pub Color);
pub struct ParticleCount(pub u32);
pub struct ScreenShakeIntensity(pub f32);

// Ownership
pub struct Owner(pub Entity);

// Marker Components (zero-sized)
pub struct ExplosiveTrap;
pub struct TrapExplosion;
pub struct Concealed;
pub struct AllyVisible;
pub struct AreaEffect;
pub struct InstantCast;
pub struct TrapPlaceAction;
pub struct RecordableAction;
```

### System Implementation

Systems query only the components they need:

```rust
// Trap placement system
fn trap_placement_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    hunters: Query<(Entity, &Transform, &Cooldown), With<TrapAbility>>,
) {
    if input.just_pressed(KeyCode::Digit4) {
        for (entity, transform, cooldown) in hunters.iter() {
            if cooldown.0 <= 0.0 {
                let grid_pos = world_to_grid(transform.translation.truncate());
                
                commands.spawn((
                    ExplosiveTrap,
                    Concealed,
                    AllyVisible,
                    GridPosition(grid_pos),
                    Transform::from_translation(transform.translation),
                    TriggerRadius(0.5),
                    ExplosionRadius(2.0),
                    Damage(150.0),
                    Duration(60.0),
                    ElapsedTime(0.0),
                    Owner(entity),
                ));
            }
        }
    }
}

// Enemy proximity detection system
fn trap_trigger_system(
    mut commands: Commands,
    traps: Query<(Entity, &Transform, &TriggerRadius, &ExplosionRadius, &Damage), With<ExplosiveTrap>>,
    enemies: Query<&Transform, With<Enemy>>,
) {
    for (trap_entity, trap_transform, trigger_radius, explosion_radius, damage) in traps.iter() {
        for enemy_transform in enemies.iter() {
            let distance = trap_transform.translation.distance(enemy_transform.translation);
            
            if distance < trigger_radius.0 * TILE_SIZE {
                // Spawn explosion entity
                commands.spawn((
                    TrapExplosion,
                    AreaEffect,
                    Origin(trap_transform.translation),
                    Radius(explosion_radius.0),
                    Damage(damage.0),
                    Duration(1.0),
                    ElapsedTime(0.0),
                    StartRadius(0.5),
                    EndRadius(explosion_radius.0 * 2.0),
                    EmissiveIntensity(3.0),
                    EmissiveColor(Color::srgb(1.0, 0.3, 0.0)),
                    ParticleCount(100),
                    ScreenShakeIntensity(0.8),
                ));
                
                // Despawn trap
                commands.entity(trap_entity).despawn();
                break;
            }
        }
    }
}

// Explosion damage system
fn explosion_damage_system(
    explosions: Query<(&Origin, &Radius, &Damage), (With<TrapExplosion>, Added<TrapExplosion>)>,
    mut enemies: Query<(&Transform, &mut Health), With<Enemy>>,
) {
    for (origin, radius, damage) in explosions.iter() {
        for (enemy_transform, mut health) in enemies.iter_mut() {
            let distance = origin.0.distance(enemy_transform.translation);
            
            if distance <= radius.0 * TILE_SIZE {
                health.0 -= damage.0;
            }
        }
    }
}

// Trap lifetime system
fn trap_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut traps: Query<(Entity, &mut ElapsedTime, &Duration), With<ExplosiveTrap>>,
) {
    for (entity, mut elapsed, duration) in traps.iter_mut() {
        elapsed.0 += time.delta_secs();
        
        if elapsed.0 >= duration.0 {
            commands.entity(entity).despawn();
        }
    }
}

// Visual expansion system
fn explosion_visual_system(
    mut commands: Commands,
    time: Res<Time>,
    mut explosions: Query<(
        Entity,
        &mut Transform,
        &StartRadius,
        &EndRadius,
        &mut ElapsedTime,
        &Duration
    ), With<TrapExplosion>>,
) {
    for (entity, mut transform, start, end, mut elapsed, duration) in explosions.iter_mut() {
        elapsed.0 += time.delta_secs();
        
        if elapsed.0 >= duration.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        
        let progress = elapsed.0 / duration.0;
        let current_scale = start.0 + (end.0 - start.0) * progress;
        transform.scale = Vec3::splat(current_scale);
    }
}

// Ally visibility system
fn trap_visibility_system(
    traps: Query<(Entity, &Transform), (With<ExplosiveTrap>, With<AllyVisible>)>,
    allies: Query<Entity, With<Ally>>,
    mut visibility: Query<&mut Visibility>,
) {
    for (trap_entity, _) in traps.iter() {
        if let Ok(mut vis) = visibility.get_mut(trap_entity) {
            // Make visible to allies, hidden from enemies
            // This would integrate with your visibility system
        }
    }
}
```

## Upgrade Paths

### Tier 1: Enhanced Explosives
Adds enhanced damage and area components:
```rust
// Additional components
DamageBonus(50.0)  // +50 damage
RadiusBonus(1.0)   // +1 tile radius
MaxCharges(2)      // Can place 2 traps
```

### Tier 2: Advanced Triggers
Adds proximity and chain components:
```rust
// Additional components
ProximityTrigger
TriggerRadiusBonus(0.5)
ChainExplosion
ChainRange(3.0)
```

### Tier 3: Master Trapper
Adds automation and debuff components:
```rust
// Additional components
AutoPlace
PlaceInterval(5.0)
RespawnOnDestroy
RespawnDelay(10.0)
DamageAmplification(0.5)
AmplificationDuration(8.0)
```

## Visual & Audio Design

Visual effects use single-value components:

```rust
// Trap placement
commands.spawn((
    TrapPlaceEffect,
    Duration(0.5),
    ElapsedTime(0.0),
    ParticleCount(20),
    EmissiveIntensity(0.5),
    AudioVolume(0.3),
));

// Armed trap (ally vision)
commands.spawn((
    TrapIndicator,
    AllyOnly,
    EmissiveColor(Color::srgba(1.0, 0.0, 0.0, 0.3)),
    EmissiveIntensity(0.2),
    PulseRate(2.0),
));

// Explosion effect
commands.spawn((
    ExplosionVfx,
    Duration(1.0),
    ElapsedTime(0.0),
    StartRadius(0.5),
    EndRadius(4.0),
    ParticleCount(100),
    FlashIntensity(2.0),
    ScreenShakeIntensity(0.8),
    AudioVolume(1.0),
));
```

## Strategic Applications

The component system enables flexible trap strategies:

```rust
// Chokepoint control
commands.spawn((
    ExplosiveTrap,
    GridPosition(chokepoint),
    TriggerRadius(1.0),  // Larger trigger for corridors
));

// Retreat coverage
commands.spawn((
    ExplosiveTrap,
    GridPosition(retreat_path),
    DelayedArm(2.0),  // Arms after delay
));

// Layered defense
for position in defensive_line {
    commands.spawn((
        ExplosiveTrap,
        GridPosition(position),
        ChainTrigger,  // Triggers nearby traps
    ));
}
```

## Recording Integration

All components are deterministic and recordable:

```rust
commands.spawn((
    RecordableAction,
    ActionType::PlaceTrap,
    Timestamp(recording.current_time),
    GridPosition(trap_pos),
    Owner(hunter_entity),
));
```

This single-component architecture ensures:
- **Simple queries** with minimal component access
- **Efficient caching** with single-value components  
- **Easy testing** of individual systems
- **Flexible composition** for trap variations
- **Deterministic replay** for recording system