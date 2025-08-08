# Ability Components Architecture

A comprehensive single-use component architecture for Arenic's deterministic ability system, following the patterns established in `holy_nova.rs` and `auto_shot.rs`.

## Architecture Overview

Every ability in Arenic follows a **composition-based pattern** where abilities are built from multiple single-purpose components:

1. **Unique Ability Markers** - Zero-sized components identifying ability types (e.g., `HolyNova`, `AutoShot`)
2. **Single-Value Components** - Each component holds exactly one value or purpose
3. **Ephemeral Effect Entities** - Short-lived entities spawned for projectiles, areas, and effects
4. **Deterministic Systems** - Small, focused systems that process specific component combinations

This design pattern, as demonstrated in the codebase:
- Uses components like `Duration(f32)`, `ElapsedTime(f32)`, `StartRadius(f32)`, `EndRadius(f32)`
- Spawns projectiles as independent entities with `Origin(Vec3)`, `Target(Vec3)` components
- Keeps systems under 50 lines with single responsibilities
- Ensures perfect determinism for the recording/replay system

## Core Design Principles (From Codebase Analysis)

### 1. Single-Value Components
- Each component holds exactly one value: `Duration(f32)`, not `Duration { value: f32, unit: TimeUnit }`
- Compose behavior from multiple components: `ElapsedTime` + `Duration` + `StartRadius` + `EndRadius`
- Zero-sized markers for categorization: `HolyNova`, `HolyNovaVfx`, `Projectile`

### 2. Ephemeral Entity Pattern
- Spawn short-lived entities for effects (projectiles, areas, pulses)
- Compose all needed components at spawn time
- Systems process and despawn when `ElapsedTime >= Duration`

### 3. Component Composition at Spawn
```rust
// From holy_nova.rs - VFX entity composition
commands.entity(character_entity).with_child((
    HolyNovaVfx::new(),
    ElapsedTime(0.0),
    Duration(0.225),
    StartRadius(4.0),
    EndRadius(32.0),
    Transform::from_scale(Vec3::splat(4.0)),
    Mesh3d(vfx_mesh),
    MeshMaterial3d(mats.yellow.clone()),
));

// From auto_shot.rs - Projectile composition
commands.spawn((
    Projectile,
    Transform::from_translation(character_pos),
    Origin(character_pos),
    Target(boss_pos),
    ElapsedTime(0.0),
    Duration(travel_time),
    Mesh3d(projectile_mesh),
    MeshMaterial3d(mats.black.clone()),
));
```

### 4. Focused Systems
- One system per behavior: `move_projectiles`, `update_holy_nova_vfx`
- Query only needed components: `Query<(Entity, &mut Transform, &mut ElapsedTime, &Duration, &Origin, &Target), With<Projectile>>`
- Despawn entities when their purpose is complete

## Shared Single-Purpose Components

All abilities compose their behavior from these single-value components:

### Time and Duration Components
```rust
#[derive(Component)]
pub struct ElapsedTime(pub f32);  // Seconds elapsed since spawn

#[derive(Component)]
pub struct Duration(pub f32);  // Total duration in seconds

#[derive(Component)]
pub struct Cooldown(pub f32);  // Cooldown remaining in seconds

#[derive(Component)]
pub struct CastTime(pub f32);  // Cast time required in seconds

#[derive(Component)]
pub struct ChannelTime(pub f32);  // Channel duration in seconds

#[derive(Component)]
pub struct TickInterval(pub f32);  // Seconds between periodic ticks

#[derive(Component)]
pub struct TicksRemaining(pub u32);  // Number of ticks left
```

### Spatial Components
```rust
#[derive(Component)]
pub struct Origin(pub Vec3);  // Starting position

#[derive(Component)]
pub struct Target(pub Vec3);  // Target position

#[derive(Component)]
pub struct Range(pub f32);  // Maximum range in world units

#[derive(Component)]
pub struct Radius(pub f32);  // Effect radius in world units

#[derive(Component)]
pub struct StartRadius(pub f32);  // Initial radius for expanding effects

#[derive(Component)]
pub struct EndRadius(pub f32);  // Final radius for expanding effects

#[derive(Component)]
pub struct Speed(pub f32);  // Movement speed in units/second

#[derive(Component)]
pub struct Distance(pub f32);  // Distance value in world units

#[derive(Component)]
pub struct Height(pub f32);  // Height/altitude value

#[derive(Component)]
pub struct Width(pub f32);  // Width value for areas

#[derive(Component)]
pub struct Angle(pub f32);  // Angle in radians for cones/arcs
```

### Effect Value Components
```rust
#[derive(Component)]
pub struct Damage(pub f32);  // Damage amount

#[derive(Component)]
pub struct Healing(pub f32);  // Healing amount

#[derive(Component)]
pub struct DamageReduction(pub f32);  // Percentage reduction (0.0-1.0)

#[derive(Component)]
pub struct HealingReduction(pub f32);  // Percentage reduction (0.0-1.0)

#[derive(Component)]
pub struct MovementReduction(pub f32);  // Percentage slow (0.0-1.0)

#[derive(Component)]
pub struct AttackSpeedReduction(pub f32);  // Percentage reduction (0.0-1.0)

#[derive(Component)]
pub struct CritChance(pub f32);  // Critical strike chance (0.0-1.0)

#[derive(Component)]
pub struct CritMultiplier(pub f32);  // Critical damage multiplier

#[derive(Component)]
pub struct BlockChance(pub f32);  // Chance to block (0.0-1.0)

#[derive(Component)]
pub struct DodgeChance(pub f32);  // Chance to dodge (0.0-1.0)

#[derive(Component)]
pub struct ArmorValue(pub f32);  // Armor amount

#[derive(Component)]
pub struct ResistanceValue(pub f32);  // Resistance amount

#[derive(Component)]
pub struct ShieldAmount(pub f32);  // Shield/barrier health
```

### Resource Components
```rust
#[derive(Component)]
pub struct ManaCost(pub f32);  // Mana required to cast

#[derive(Component)]
pub struct EnergyCost(pub f32);  // Energy required

#[derive(Component)]
pub struct HealthCost(pub f32);  // Health sacrificed

#[derive(Component)]
pub struct ManaGeneration(pub f32);  // Mana per second

#[derive(Component)]
pub struct EnergyGeneration(pub f32);  // Energy per second

#[derive(Component)]
pub struct ResourceDrain(pub f32);  // Resource drain per second
```

### Stacking and Charges
```rust
#[derive(Component)]
pub struct Stacks(pub u32);  // Current stack count

#[derive(Component)]
pub struct MaxStacks(pub u32);  // Maximum stacks allowed

#[derive(Component)]
pub struct Charges(pub u32);  // Ability charges available

#[derive(Component)]
pub struct MaxCharges(pub u32);  // Maximum charges

#[derive(Component)]
pub struct ChargeRegenTime(pub f32);  // Seconds to regenerate one charge

#[derive(Component)]
pub struct StackDecayTime(pub f32);  // Seconds before a stack decays
```

### Targeting Components
```rust
#[derive(Component)]
pub struct TargetEntity(pub Entity);  // Specific target entity

#[derive(Component)]
pub struct TargetPosition(pub Vec3);  // Target world position

#[derive(Component)]
pub struct TargetCount(pub u32);  // Number of targets to affect

#[derive(Component)]
pub struct PierceCount(pub u32);  // Enemies to pierce through

#[derive(Component)]
pub struct BounceCount(pub u32);  // Times to bounce

#[derive(Component)]
pub struct ChainCount(pub u32);  // Targets to chain to

#[derive(Component)]
pub struct ChainRange(pub f32);  // Max distance between chain targets

#[derive(Component)]
pub struct SplashRadius(pub f32);  // Splash damage radius

#[derive(Component)]
pub struct SplashDamage(pub f32);  // Splash damage amount
```

### Probability Components
```rust
#[derive(Component)]
pub struct ProcChance(pub f32);  // Chance to trigger effect (0.0-1.0)

#[derive(Component)]
pub struct MissChance(pub f32);  // Chance to miss (0.0-1.0)

#[derive(Component)]
pub struct FailChance(pub f32);  // Chance to fail (0.0-1.0)

#[derive(Component)]
pub struct SuccessRate(pub f32);  // Success rate (0.0-1.0)
```

## Marker Components for Abilities

These zero-sized components mark ability entities or their effects:

### Ability Type Markers
```rust
// Hunter Abilities
#[derive(Component)]
pub struct AutoShot;

#[derive(Component)]
pub struct PoisonShot;

#[derive(Component)]
pub struct TrapAbility;

#[derive(Component)]
pub struct SniperShot;

// Cardinal Abilities
#[derive(Component)]
pub struct HolyNova;

#[derive(Component)]
pub struct HealAbility;

#[derive(Component)]
pub struct BarrierAbility;

#[derive(Component)]
pub struct BeamAbility;

#[derive(Component)]
pub struct ResurrectAbility;

// Warrior Abilities
#[derive(Component)]
pub struct BashAbility;

#[derive(Component)]
pub struct BlockAbility;

#[derive(Component)]
pub struct TauntAbility;

#[derive(Component)]
pub struct BulwarkAbility;

// Thief Abilities
#[derive(Component)]
pub struct BackstabAbility;

#[derive(Component)]
pub struct ShadowStepAbility;

#[derive(Component)]
pub struct SmokeScreenAbility;

#[derive(Component)]
pub struct PickpocketAbility;

// Merchant Abilities
#[derive(Component)]
pub struct DiceAbility;

#[derive(Component)]
pub struct CoinTossAbility;

#[derive(Component)]
pub struct FortuneAbility;

#[derive(Component)]
pub struct VaultAbility;

// Alchemist Abilities
#[derive(Component)]
pub struct TransmuteAbility;

#[derive(Component)]
pub struct AcidFlaskAbility;

#[derive(Component)]
pub struct SiphonAbility;

#[derive(Component)]
pub struct IronskinAbility;

// Bard Abilities
#[derive(Component)]
pub struct DanceAbility;

#[derive(Component)]
pub struct CleanseAbility;

#[derive(Component)]
pub struct HelixAbility;

#[derive(Component)]
pub struct MimicAbility;

// Forager Abilities
#[derive(Component)]
pub struct DigAbility;

#[derive(Component)]
pub struct BorderAbility;

#[derive(Component)]
pub struct BoulderAbility;

#[derive(Component)]
pub struct MushroomAbility;
```

### Effect Markers
```rust
#[derive(Component)]
pub struct Projectile;  // Marks projectile entities

#[derive(Component)]
pub struct AreaEffect;  // Marks area effect entities

#[derive(Component)]
pub struct Buff;  // Marks buff entities

#[derive(Component)]
pub struct Debuff;  // Marks debuff entities

#[derive(Component)]
pub struct Shield;  // Marks shield entities

#[derive(Component)]
pub struct Barrier;  // Marks barrier entities

#[derive(Component)]
pub struct Trap;  // Marks trap entities

#[derive(Component)]
pub struct Totem;  // Marks totem entities

#[derive(Component)]
pub struct Pet;  // Marks pet/summon entities

#[derive(Component)]
pub struct Clone;  // Marks clone/illusion entities

#[derive(Component)]
pub struct Pulse;  // Marks pulse/nova entities

#[derive(Component)]
pub struct Beam;  // Marks beam entities

#[derive(Component)]
pub struct Chain;  // Marks chain effect entities

#[derive(Component)]
pub struct Pool;  // Marks persistent pool entities
```

### Visual Effect Markers
```rust
#[derive(Component)]
pub struct HolyNovaVfx;  // Holy nova visual effect

#[derive(Component)]
pub struct HealVfx;  // Healing visual effect

#[derive(Component)]
pub struct DamageVfx;  // Damage visual effect

#[derive(Component)]
pub struct ExplosionVfx;  // Explosion visual effect

#[derive(Component)]
pub struct TrailVfx;  // Trail visual effect

#[derive(Component)]
pub struct FlashVfx;  // Flash visual effect

#[derive(Component)]
pub struct ShieldVfx;  // Shield visual effect

#[derive(Component)]
pub struct BuffVfx;  // Buff visual effect

#[derive(Component)]
pub struct DebuffVfx;  // Debuff visual effect
```

### Behavior Markers
```rust
#[derive(Component)]
pub struct Homing;  // Projectile homes to target

#[derive(Component)]
pub struct Piercing;  // Projectile pierces targets

#[derive(Component)]
pub struct Bouncing;  // Projectile bounces between targets

#[derive(Component)]
pub struct Chaining;  // Effect chains to nearby targets

#[derive(Component)]
pub struct Explosive;  // Creates explosion on impact

#[derive(Component)]
pub struct Persistent;  // Effect persists after initial application

#[derive(Component)]
pub struct Spreading;  // Effect spreads to nearby entities

#[derive(Component)]
pub struct Stacking;  // Effect can stack multiple times

#[derive(Component)]
pub struct Channeled;  // Ability requires channeling

#[derive(Component)]
pub struct Instant;  // Ability has no cast time

#[derive(Component)]
pub struct Periodic;  // Effect ticks periodically

#[derive(Component)]
pub struct Delayed;  // Effect has a delay before activation
```

### Targeting Markers
```rust
#[derive(Component)]
pub struct TargetSelf;  // Targets the caster

#[derive(Component)]
pub struct TargetAlly;  // Can target allies

#[derive(Component)]
pub struct TargetEnemy;  // Can target enemies

#[derive(Component)]
pub struct TargetGround;  // Targets ground position

#[derive(Component)]
pub struct TargetNearest;  // Auto-targets nearest valid target

#[derive(Component)]
pub struct TargetLowest;  // Targets lowest health

#[derive(Component)]
pub struct TargetHighest;  // Targets highest threat/damage

#[derive(Component)]
pub struct RequiresLineOfSight;  // Needs clear path to target

#[derive(Component)]
pub struct IgnoresLineOfSight;  // Can target through obstacles

#[derive(Component)]
pub struct SmartTargeting;  // Uses intelligent target selection
```

## Visual and Audio Components

Single-purpose components for effects:

### Visual Components
```rust
#[derive(Component)]
pub struct EmissiveIntensity(pub f32);  // Glow intensity

#[derive(Component)]
pub struct EmissiveColor(pub Color);  // Glow color

#[derive(Component)]
pub struct TrailLength(pub f32);  // Trail effect length

#[derive(Component)]
pub struct TrailColor(pub Color);  // Trail color

#[derive(Component)]
pub struct ParticleCount(pub u32);  // Number of particles

#[derive(Component)]
pub struct ParticleVelocity(pub Vec3);  // Particle emission velocity

#[derive(Component)]
pub struct ParticleLifetime(pub f32);  // Particle duration

#[derive(Component)]
pub struct FlashIntensity(pub f32);  // Flash brightness

#[derive(Component)]
pub struct FlashDuration(pub f32);  // Flash duration

#[derive(Component)]
pub struct ScreenShakeIntensity(pub f32);  // Camera shake strength

#[derive(Component)]
pub struct ScreenShakeDuration(pub f32);  // Shake duration

#[derive(Component)]
pub struct LightIntensity(pub f32);  // Light brightness

#[derive(Component)]
pub struct LightRadius(pub f32);  // Light range

#[derive(Component)]
pub struct LightColor(pub Color);  // Light color
```

### Audio Components
```rust
#[derive(Component)]
pub struct SoundVolume(pub f32);  // Volume level (0.0-1.0)

#[derive(Component)]
pub struct SoundPitch(pub f32);  // Pitch modifier

#[derive(Component)]
pub struct SoundFalloff(pub f32);  // Distance falloff rate

#[derive(Component)]
pub struct SoundLoop(pub bool);  // Whether sound loops

#[derive(Component)]
pub struct SoundDelay(pub f32);  // Delay before playing

#[derive(Component)]
pub struct AudioHandle(pub Handle<AudioSource>);  // Audio asset handle
```

## Composition Examples from Codebase

### Holy Nova VFX Entity (from holy_nova.rs)
```rust
commands.entity(character_entity).with_child((
    HolyNovaVfx::new(),     // Marker
    ElapsedTime(0.0),       // Time tracking
    Duration(0.225),        // Total duration
    StartRadius(4.0),       // Initial size
    EndRadius(32.0),        // Final size
    Transform::from_scale(Vec3::splat(4.0)),
    Mesh3d(vfx_mesh),
    MeshMaterial3d(mats.yellow.clone()),
));
```

### Auto Shot Projectile (from auto_shot.rs)
```rust
commands.spawn((
    Projectile,             // Marker
    Transform::from_translation(character_pos),
    Origin(character_pos),  // Start position
    Target(boss_pos),       // End position
    ElapsedTime(0.0),       // Time tracking
    Duration(travel_time),  // Travel duration
    Mesh3d(projectile_mesh),
    MeshMaterial3d(mats.black.clone()),
));
```

## System Patterns

### Movement System (from auto_shot.rs)
```rust
pub fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut ElapsedTime,
        &Duration,
        &Origin,
        &Target
    ), With<Projectile>>,
) {
    for (entity, mut transform, mut elapsed, duration, origin, target) in query.iter_mut() {
        elapsed.0 += time.delta_secs();
        let progress = (elapsed.0 / duration.0).clamp(0.0, 1.0);
        transform.translation = origin.0.lerp(target.0, progress);
        
        if progress >= 1.0 {
            commands.entity(entity).despawn();
        }
    }
}
```

### Expansion System (from holy_nova.rs)
```rust
pub fn update_holy_nova_vfx(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut ElapsedTime,
        &Duration,
        &StartRadius,
        &EndRadius,
    ), With<HolyNovaVfx>>,
) {
    for (entity, mut transform, mut elapsed, duration, start_radius, end_radius) in query.iter_mut() {
        elapsed.0 += time.delta_secs();
        let t = (elapsed.0 / duration.0).clamp(0.0, 1.0);
        
        let easing_curve = EasingCurve::new(0.0, 1.0, EaseFunction::ExponentialOut);
        let eased = easing_curve.sample(t).unwrap_or(0.0);
        
        let radius = start_radius.0 + (end_radius.0 - start_radius.0) * eased;
        transform.scale = Vec3::splat(radius);
        
        if elapsed.0 >= duration.0 {
            commands.entity(entity).despawn();
        }
    }
}
```

## Implementation Guidelines

### 1. Ability Entity Composition Pattern
Every ability should spawn ephemeral entities with composed components:

```rust
// Example: Heal Pulse
commands.spawn((
    HealPulse,              // Marker
    Origin(caster_pos),     // Center position
    Radius(5.0 * TILE_SIZE), // Effect radius
    Healing(150.0),         // Heal amount
    ElapsedTime(0.0),       // Time tracking
    Duration(0.1),          // Brief pulse
    HealVfx,                // Visual marker
));

// Example: Poison Cloud
commands.spawn((
    PoisonCloud,            // Marker
    AreaEffect,             // Area marker
    Origin(target_pos),     // Center position
    Radius(3.0 * TILE_SIZE), // Cloud radius
    Damage(20.0),           // Damage per tick
    TickInterval(1.0),      // Tick every second
    TicksRemaining(10),     // 10 ticks total
    ElapsedTime(0.0),       // Time tracking
    Duration(10.0),         // Total duration
));
```

### 2. System Design Pattern
Keep systems small and focused on single responsibilities:

```rust
// Good: Single responsibility
fn apply_healing_system(
    mut commands: Commands,
    pulses: Query<(Entity, &Origin, &Radius, &Healing), With<HealPulse>>,
    mut targets: Query<(&Transform, &mut Health), With<Ally>>,
) {
    for (pulse_entity, origin, radius, healing) in pulses.iter() {
        for (transform, mut health) in targets.iter_mut() {
            if transform.translation.distance(origin.0) <= radius.0 {
                health.0 = (health.0 + healing.0).min(health.1); // health.1 is max
            }
        }
        commands.entity(pulse_entity).despawn();
    }
}

// Good: Timer management separate
fn pulse_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut pulses: Query<(Entity, &mut ElapsedTime, &Duration), With<HealPulse>>,
) {
    for (entity, mut elapsed, duration) in pulses.iter_mut() {
        elapsed.0 += time.delta_secs();
        if elapsed.0 >= duration.0 {
            commands.entity(entity).despawn();
        }
    }
}
```

### 3. Avoid Complex Components
Never create components with multiple fields when single values suffice:

```rust
// BAD: Complex component
#[derive(Component)]
pub struct ProjectileData {
    pub speed: f32,
    pub damage: f32,
    pub lifetime: f32,
    pub pierce_count: u32,
}

// GOOD: Single-value components
#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct Duration(pub f32);

#[derive(Component)]
pub struct PierceCount(pub u32);
```

### 4. Use Markers for Behavior
Combine markers to create complex behaviors:

```rust
// Homing explosive projectile
commands.spawn((
    Projectile,
    Homing,
    Explosive,
    Origin(start_pos),
    TargetEntity(enemy),
    Speed(10.0),
    Damage(100.0),
    SplashRadius(2.0),
    SplashDamage(50.0),
    // ... visual components
));

// System queries for specific combinations
fn homing_projectile_system(
    mut projectiles: Query<(&mut Transform, &TargetEntity, &Speed), (With<Projectile>, With<Homing>)>,
    targets: Query<&Transform, Without<Projectile>>,
) {
    // Update projectile direction toward target
}

fn explosive_impact_system(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &SplashRadius, &SplashDamage), (With<Projectile>, With<Explosive>)>,
) {
    // Spawn explosion entity on impact
}
```

### 5. Recording Integration
All ability entities should include recording components when spawned during gameplay:

```rust
use bevy::utils::HashMap;

#[derive(Component)]
pub struct RecordableAction {
    pub action_type: String,
    pub timestamp: f32,
    pub position: Vec3,
    pub parameters: HashMap<String, f32>,
}

// Example spawn with recording
commands.spawn((
    Projectile,
    Origin(character_pos),
    Target(enemy_pos),
    Damage(75.0),
    Duration(travel_time),
    ElapsedTime(0.0),
    RecordableAction {
        action_type: "auto_shot_fire".to_string(),
        timestamp: recording.current_time,
        position: character_pos,
        parameters: HashMap::from([
            ("damage".to_string(), 75.0),
            ("target_x".to_string(), enemy_pos.x),
            ("target_y".to_string(), enemy_pos.y),
        ]),
    },
));
```

## Performance Considerations

### 1. Archetype Efficiency
Group commonly queried components together:

```rust
// Bundle for common projectile components
#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub transform: Transform,
    pub origin: Origin,
    pub target: Target,
    pub elapsed: ElapsedTime,
    pub duration: Duration,
}

// Bundle for area effects
#[derive(Bundle)]
pub struct AreaEffectBundle {
    pub area: AreaEffect,
    pub transform: Transform,
    pub origin: Origin,
    pub radius: Radius,
    pub elapsed: ElapsedTime,
    pub duration: Duration,
}
```

### 2. Query Filtering
Use marker components for efficient filtering:

```rust
// Efficient: Filter with markers
Query<&Transform, (With<Projectile>, With<Homing>, Without<Explosive>)>

// Efficient: Change detection
Query<Entity, (With<Buff>, Changed<Duration>)>
```

### 3. Entity Spawning
Compose all components at spawn to avoid archetype moves:

```rust
// Good: All components at once
commands.spawn((component1, component2, component3, component4));

// Bad: Adding components incrementally
let entity = commands.spawn(component1).id();
commands.entity(entity).insert(component2);
commands.entity(entity).insert(component3);
```

## Summary

This single-use component architecture provides:

1. **Simplicity**: Each component has one clear purpose
2. **Composability**: Complex behaviors from simple components
3. **Performance**: Cache-friendly, archetype-efficient queries
4. **Determinism**: Component state drives all behavior
5. **Maintainability**: Small, focused systems under 50 lines
6. **Flexibility**: Easy to add new abilities by composing existing components

Following these patterns from `holy_nova.rs` and `auto_shot.rs` ensures consistent, performant, and maintainable ability implementations across the entire game.