# Poison Shot

A complete implementation guide for the Hunter's toxic projectile with knockback ability using single-component architecture.

## Overview

The **Poison Shot** ability demonstrates the Hunter's mastery over chemical warfare through a specialized projectile that combines immediate knockback effects with sustained damage over time. When fired, the shot pushes the Hunter back one tile while delivering a toxic payload that inflicts 20 seconds of poison damage with a 12-second cooldown. This ability excels at both positioning control and sustained damage application.

## Game Design Philosophy

This ability showcases dual-purpose design through combined immediate and persistent effects:

**Newton's Third Law Design**: The Hunter's knockback creates interesting risk-reward positioning decisions, as the tactical advantage of distance comes with potential repositioning challenges.

**Sustained Pressure**: The 20-second poison duration creates ongoing battlefield pressure that rewards timing and target prioritization over raw damage output.

**Cooldown Efficiency**: The 12-second cooldown allows for strategic timing while preventing poison from being the Hunter's only combat option.

## Component Architecture

### Entity Composition Pattern

Following the single-component architecture, the Poison Shot ability is composed of multiple single-purpose components:

```rust
// Poison Shot Projectile Entity
commands.spawn((
    // Marker Components
    PoisonShotProjectile,
    Projectile,
    
    // Movement Components
    Origin(hunter_pos),
    Target(cursor_pos),
    Speed(8.0),
    ElapsedTime(0.0),
    Duration(2.0),
    
    // Effect Components
    PoisonPayload,
    Damage(15.0),
    TickInterval(2.0),
    TicksRemaining(10),
    
    // Visual Components
    Transform::from_translation(hunter_pos),
    Mesh3d(projectile_mesh),
    MeshMaterial3d(materials.poison_green.clone()),
    TrailLength(1.5),
    TrailColor(Color::srgb(0.2, 0.8, 0.2)),
));

// Hunter Knockback Entity
commands.entity(hunter_entity).with_child((
    // Marker
    KnockbackEffect,
    
    // Movement
    Origin(hunter_pos),
    Target(knockback_pos),
    Distance(1.0),
    Duration(0.2),
    ElapsedTime(0.0),
));

// Poison Effect Entity (on hit)
commands.entity(target_entity).with_child((
    // Marker Components
    PoisonDebuff,
    DamageOverTime,
    
    // Effect Components
    Damage(15.0),
    Duration(20.0),
    ElapsedTime(0.0),
    TickInterval(2.0),
    LastTickTime(0.0),
    
    // Visual Components
    EmissiveColor(Color::srgb(0.2, 0.8, 0.2)),
    EmissiveIntensity(0.5),
    ParticleCount(20),
));
```

### Core Components Used

All components are single-value/single-purpose:

```rust
// Movement Components
pub struct Origin(pub Vec3);
pub struct Target(pub Vec3);
pub struct Speed(pub f32);
pub struct Distance(pub f32);

// Time Components
pub struct Duration(pub f32);
pub struct ElapsedTime(pub f32);
pub struct TickInterval(pub f32);
pub struct LastTickTime(pub f32);
pub struct TicksRemaining(pub u32);

// Effect Components
pub struct Damage(pub f32);
pub struct Cooldown(pub f32);

// Visual Components
pub struct TrailLength(pub f32);
pub struct TrailColor(pub Color);
pub struct EmissiveColor(pub Color);
pub struct EmissiveIntensity(pub f32);
pub struct ParticleCount(pub u32);

// Marker Components (zero-sized)
pub struct PoisonShotProjectile;
pub struct PoisonPayload;
pub struct PoisonDebuff;
pub struct DamageOverTime;
pub struct KnockbackEffect;
pub struct Projectile;
```

### System Implementation

Systems query only the components they need:

```rust
// Projectile movement system
fn projectile_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(
        Entity,
        &Origin,
        &Target,
        &Speed,
        &mut ElapsedTime,
        &Duration,
        &mut Transform
    ), With<Projectile>>,
) {
    for (entity, origin, target, speed, mut elapsed, duration, mut transform) in projectiles.iter_mut() {
        elapsed.0 += time.delta_secs();
        
        if elapsed.0 >= duration.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        
        let progress = elapsed.0 / duration.0;
        transform.translation = origin.0.lerp(target.0, progress);
    }
}

// Poison damage tick system
fn poison_tick_system(
    mut commands: Commands,
    time: Res<Time>,
    mut poisoned: Query<(
        Entity,
        &Damage,
        &mut ElapsedTime,
        &mut LastTickTime,
        &TickInterval,
        &Duration,
        &Parent
    ), With<PoisonDebuff>>,
    mut health_query: Query<&mut Health>,
) {
    for (entity, damage, mut elapsed, mut last_tick, interval, duration, parent) in poisoned.iter_mut() {
        elapsed.0 += time.delta_secs();
        
        // Check if effect expired
        if elapsed.0 >= duration.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        
        // Check if it's time for next tick
        if elapsed.0 - last_tick.0 >= interval.0 {
            if let Ok(mut health) = health_query.get_mut(parent.get()) {
                health.0 -= damage.0;
            }
            last_tick.0 = elapsed.0;
        }
    }
}

// Collision detection system
fn poison_collision_system(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &PoisonPayload, &Damage, &TickInterval), With<PoisonShotProjectile>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (proj_entity, proj_transform, _, damage, tick_interval) in projectiles.iter() {
        for (enemy_entity, enemy_transform) in enemies.iter() {
            let distance = proj_transform.translation.distance(enemy_transform.translation);
            
            if distance < COLLISION_RADIUS {
                // Apply poison effect as child entity
                commands.entity(enemy_entity).with_child((
                    PoisonDebuff,
                    DamageOverTime,
                    Damage(damage.0),
                    Duration(20.0),
                    ElapsedTime(0.0),
                    TickInterval(tick_interval.0),
                    LastTickTime(0.0),
                    EmissiveColor(Color::srgb(0.2, 0.8, 0.2)),
                    EmissiveIntensity(0.5),
                ));
                
                // Despawn projectile
                commands.entity(proj_entity).despawn_recursive();
                break;
            }
        }
    }
}
```

## Upgrade Paths

### Tier 1: Concentrated Toxin
Adds single-value components to enhance poison:
```rust
// Additional components
DamageBonus(5.0)  // +5 damage per tick
DurationBonus(5.0)  // +5 seconds duration
```

### Tier 2: Explosive Payload
Adds area and pierce components:
```rust
// Additional components
SplashRadius(2.0)
PierceCount(1)
KnockbackForce(1.5)
```

### Tier 3: Virulent Strain
Adds spreading and acceleration:
```rust
// Additional components
SpreadOnDeath
TickAcceleration(0.5)  // Ticks get faster
IgnoresResistance
```

## Visual & Audio Design

Visual effects use single-value components:

```rust
// Shot preparation
commands.spawn((
    PoisonShotCharge,
    Duration(0.5),
    ElapsedTime(0.0),
    EmissiveIntensity(2.0),
    EmissiveColor(Color::srgb(0.2, 0.8, 0.2)),
));

// Projectile trail
commands.spawn((
    ProjectileTrail,
    TrailLength(1.5),
    TrailColor(Color::srgb(0.2, 0.8, 0.2)),
    ParticleCount(30),
    Duration(2.0),
));

// Impact explosion
commands.spawn((
    PoisonExplosion,
    Duration(1.0),
    ElapsedTime(0.0),
    StartRadius(0.5),
    EndRadius(3.0),
    ParticleCount(50),
    ScreenShakeIntensity(0.3),
));
```

## Recording Integration

All components are deterministic and recordable:

```rust
commands.spawn((
    RecordableAction,
    ActionType::PoisonShot,
    Timestamp(recording.current_time),
    OriginPosition(hunter_pos),
    TargetPosition(cursor_pos),
));
```

This single-component architecture ensures:
- **Cache-friendly** component access
- **Parallelizable** systems
- **Deterministic** behavior
- **Easy composition** of new abilities
- **Frame-perfect** recording/replay