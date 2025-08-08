# Shared Components Architecture

Components used by both player abilities and boss mechanics in Arenic's ECS architecture.

## Core Time and Duration Components

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

## Spatial and Position Components

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
pub struct Speed(pub f32);  // Movement speed in units/second

#[derive(Component)]
pub struct Distance(pub f32);  // Distance value in world units

#[derive(Component)]
pub struct Height(pub f32);  // Height/altitude value

#[derive(Component)]
pub struct Width(pub f32);  // Width value for areas
```

## Effect Value Components

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
pub struct ArmorValue(pub f32);  // Armor amount

#[derive(Component)]
pub struct ResistanceValue(pub f32);  // Resistance amount

#[derive(Component)]
pub struct ShieldAmount(pub f32);  // Shield/barrier health
```

## Entity Reference Components

```rust
#[derive(Component)]
pub struct SourceEntity(pub Entity);  // Source of an effect

#[derive(Component)]
pub struct TargetEntity(pub Entity);  // Target of an effect

#[derive(Component)]
pub struct NodeOwner(pub Entity);  // Owner of a deployable
```

## Resource Management Components

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

## Stacking and Charges

```rust
#[derive(Component)]
pub struct Stacks(pub u32);  // Current stack count

#[derive(Component)]
pub struct MaxStacks(pub u32);  // Maximum stacks allowed

#[derive(Component)]
pub struct Charges(pub u32);  // Ability charges available

#[derive(Component)]
pub struct MaxCharges(pub u32);  // Maximum charges
```

## Universal Effect Markers

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
pub struct Pulse;  // Marks pulse/nova entities

#[derive(Component)]
pub struct Beam;  // Marks beam entities

#[derive(Component)]
pub struct Pool;  // Marks persistent pool entities
```

## Behavior Markers

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

## Visual Effect Components

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
pub struct LightIntensity(pub f32);  // Light brightness

#[derive(Component)]
pub struct LightRadius(pub f32);  // Light range

#[derive(Component)]
pub struct LightColor(pub Color);  // Light color
```

## Audio Components

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

## Health and Status Components

```rust
#[derive(Component)]
pub struct Health(pub f32, pub f32);  // Current, Max

#[derive(Component)]
pub struct Armor(pub f32);  // Damage reduction value

#[derive(Component)]
pub struct Stunned;  // Stunned status marker

#[derive(Component)]
pub struct Silenced;  // Cannot cast abilities

#[derive(Component)]
pub struct Rooted;  // Cannot move

#[derive(Component)]
pub struct Immune;  // Immune to effects

#[derive(Component)]
pub struct Invisible;  // Cannot be targeted
```

## Transform and Physics

```rust
// Note: These are standard Bevy components but listed for completeness
use bevy::prelude::{Transform, GlobalTransform};

#[derive(Component)]
pub struct Velocity(pub Vec3);  // Movement velocity

#[derive(Component)]
pub struct Acceleration(pub Vec3);  // Movement acceleration

#[derive(Component)]
pub struct RotationSpeed(pub f32);  // Rotation rate
```

## Usage Examples

### Player Ability Using Shared Components
```rust
// Hunter's Auto Shot
commands.spawn((
    Projectile,              // Shared marker
    Transform::from_translation(pos),
    Origin(character_pos),   // Shared spatial
    Target(boss_pos),        // Shared spatial
    ElapsedTime(0.0),        // Shared time
    Duration(travel_time),   // Shared time
    Damage(75.0),            // Shared effect
));
```

### Boss Ability Using Shared Components
```rust
// Boss projectile attack
commands.spawn((
    Projectile,              // Shared marker
    Homing,                  // Shared behavior
    Transform::from_translation(boss_pos),
    TargetEntity(player),    // Shared reference
    Speed(10.0),             // Shared spatial
    Damage(200.0),           // Shared effect
    Duration(5.0),           // Shared time
));
```

### Environmental Effect Using Shared Components
```rust
// Damage pool (used by both players and bosses)
commands.spawn((
    Pool,                    // Shared marker
    AreaEffect,              // Shared marker
    Transform::from_translation(pos),
    Radius(3.0),             // Shared spatial
    Damage(50.0),            // Shared effect
    TickInterval(1.0),       // Shared time
    Duration(10.0),          // Shared time
));
```

## Migration Notes

When refactoring existing code:

1. **Replace duplicated components** with shared versions
2. **Update imports** to reference this shared module
3. **Maintain single-value principle** - never add fields to these components
4. **Use composition** to create complex behaviors from simple shared components

## Component Categories Summary

| Category | Usage | Examples |
|----------|-------|----------|
| **Time** | Duration and timing control | ElapsedTime, Duration, Cooldown |
| **Spatial** | Position and movement | Origin, Target, Radius, Speed |
| **Effects** | Damage and healing values | Damage, Healing, DamageReduction |
| **References** | Entity relationships | SourceEntity, TargetEntity |
| **Resources** | Cost and generation | ManaCost, ResourceDrain |
| **Markers** | Behavior categorization | Projectile, Homing, Explosive |
| **Visual** | Rendering effects | EmissiveIntensity, ParticleCount |
| **Audio** | Sound effects | SoundVolume, AudioHandle |
| **Status** | Character states | Health, Stunned, Immune |