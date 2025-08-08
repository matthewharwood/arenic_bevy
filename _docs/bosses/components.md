# Boss Components Architecture

A comprehensive single-value component architecture for Arenic's boss system, following the patterns established in player abilities.

## Core Design Principles

1. **Single-Value Components**: Each component holds exactly one value
2. **Ephemeral Entities**: Short-lived entities for boss mechanics
3. **Marker Components**: Zero-sized components for categorization
4. **Focused Systems**: Small systems with single responsibilities
5. **Entity Composition**: Complex behavior from simple components

## Boss-Specific Marker Components

### Boss Identity Markers
```rust
// Hunter Boss
#[derive(Component)]
pub struct Webweaver;

#[derive(Component)]
pub struct PredictiveTrap;

#[derive(Component)]
pub struct EchoTrap;

#[derive(Component)]
pub struct WebTether;

// Thief Boss
#[derive(Component)]
pub struct Shadowdancer;

#[derive(Component)]
pub struct ShadowClone;

#[derive(Component)]
pub struct FalseTelegraph;

#[derive(Component)]
pub struct DimensionalPhase;

// Alchemist Boss
#[derive(Component)]
pub struct MadAlchemist;

#[derive(Component)]
pub struct ChemicalPool;

#[derive(Component)]
pub struct CascadeReaction;

#[derive(Component)]
pub struct PhilosophersStone;

// Bard Boss
#[derive(Component)]
pub struct Conductor;

#[derive(Component)]
pub struct RhythmDisruptor;

#[derive(Component)]
pub struct Polyrhythm;

#[derive(Component)]
pub struct TemporalFugue;

// Forager Boss
#[derive(Component)]
pub struct Earthshaper;

#[derive(Component)]
pub struct TerrainTransform;

#[derive(Component)]
pub struct GravityWell;

#[derive(Component)]
pub struct FloatingIsland;

// Warrior Boss
#[derive(Component)]
pub struct Ironwall;

#[derive(Component)]
pub struct AttritionAura;

#[derive(Component)]
pub struct FatigueTracker;

#[derive(Component)]
pub struct ResourceVampirism;

// Cardinal Boss
#[derive(Component)]
pub struct Purifier;

#[derive(Component)]
pub struct InversionField;

#[derive(Component)]
pub struct ParadoxEffect;

#[derive(Component)]
pub struct PolarityMarker;

// Merchant Boss
#[derive(Component)]
pub struct Gambler;

#[derive(Component)]
pub struct ProbabilityCascade;

#[derive(Component)]
pub struct GamblingEffect;

#[derive(Component)]
pub struct LuckTracker;
```

## Boss Mechanic Components

### Prediction and Temporal Components
```rust
#[derive(Component)]
pub struct FuturePosition(pub Vec3);  // Predicted future position

#[derive(Component)]
pub struct PastPosition(pub Vec3);  // Historical position

#[derive(Component)]
pub struct PresentPosition(pub Vec3);  // Current position snapshot

#[derive(Component)]
pub struct ActivationDelay(pub f32);  // Seconds until activation

#[derive(Component)]
pub struct TemporalLayer(pub TimeState);  // Time state enum

#[derive(Component)]
pub struct EchoDelay(pub f32);  // Echo effect delay

#[derive(Component)]
pub struct PredictionAccuracy(pub f32);  // Accuracy of prediction (0.0-1.0)
```

### Deception and Information Components
```rust
#[derive(Component)]
pub struct IsRealBoss(pub bool);  // Whether this is the actual boss

#[derive(Component)]
pub struct CloneSource(pub Entity);  // Source entity for clone

#[derive(Component)]
pub struct FakeHealthBar(pub f32);  // False health display

#[derive(Component)]
pub struct DamageImmune;  // Takes no damage marker

#[derive(Component)]
pub struct CloneBehavior(pub BehaviorType);  // AI behavior for clone

#[derive(Component)]
pub struct TelegraphPosition(pub Vec3);  // False telegraph location

#[derive(Component)]
pub struct RealAttackAngle(pub f32);  // Actual attack direction

#[derive(Component)]
pub struct MisdirectionDelay(pub f32);  // Delay before real attack

#[derive(Component)]
pub struct PhaseVisibility(pub f32);  // Visibility in dimensional phase

#[derive(Component)]
pub struct CrossPhaseBleed(pub f32);  // Effect bleed between dimensions
```

### Chemical and Reaction Components
```rust
#[derive(Component)]
pub struct PoolElement(pub ElementType);  // Chemical element type

#[derive(Component)]
pub struct ReactionPotential(pub f32);  // Reaction strength multiplier

#[derive(Component)]
pub struct Volatile(pub bool);  // Whether pool is unstable

#[derive(Component)]
pub struct ExpansionRate(pub f32);  // Pool growth rate

#[derive(Component)]
pub struct ChainDepth(pub u32);  // Cascade reaction depth

#[derive(Component)]
pub struct ReactionMultiplier(pub f32);  // Cascade damage multiplier

#[derive(Component)]
pub struct PropagationDelay(pub f32);  // Time between cascades

#[derive(Component)]
pub struct CatalystType(pub ElementType);  // Triggering element
```

### Rhythm and Temporal Disruption Components
```rust
#[derive(Component)]
pub struct TempoMultiplier(pub f32);  // Speed modification

#[derive(Component)]
pub struct DesyncDuration(pub f32);  // Desynchronization time

#[derive(Component)]
pub struct BeatOffset(pub f32);  // Offset from global beat

#[derive(Component)]
pub struct RhythmPattern(pub Vec<f32>);  // Beat pattern

#[derive(Component)]
pub struct ResonanceLevel(pub f32);  // Sound resonance buildup

#[derive(Component)]
pub struct CacophonyThreshold(pub f32);  // Damage threshold

#[derive(Component)]
pub struct SilenceDuration(pub f32);  // Audio removal time
```

### Terrain and Environmental Components
```rust
#[derive(Component)]
pub struct StartElevation(pub f32);  // Initial terrain height

#[derive(Component)]
pub struct TargetElevation(pub f32);  // Target terrain height

#[derive(Component)]
pub struct TransformDuration(pub f32);  // Terrain change time

#[derive(Component)]
pub struct AffectedTiles(pub Vec<IVec2>);  // Tiles to transform

#[derive(Component)]
pub struct WellCenter(pub Vec3);  // Gravity well center

#[derive(Component)]
pub struct GravityStrength(pub f32);  // Gravity force

#[derive(Component)]
pub struct WellRadius(pub f32);  // Gravity effect radius

#[derive(Component)]
pub struct IslandVelocity(pub Vec3);  // Floating island movement

#[derive(Component)]
pub struct LavaLevel(pub f32);  // Rising lava height
```

### Attrition and Resource Components
```rust
#[derive(Component)]
pub struct DrainRate(pub f32);  // Resource drain per second

#[derive(Component)]
pub struct ResourceType(pub ResourceType);  // Type being drained

#[derive(Component)]
pub struct FatigueLevel(pub f32);  // Current fatigue (0.0-1.0)

#[derive(Component)]
pub struct FatigueRate(pub f32);  // Fatigue accumulation rate

#[derive(Component)]
pub struct MaxFatigue(pub f32);  // Maximum fatigue value

#[derive(Component)]
pub struct FailureThreshold(pub f32);  // Fatigue failure point

#[derive(Component)]
pub struct VampirismAmount(pub f32);  // Resource steal amount

#[derive(Component)]
pub struct EnduranceModifier(pub f32);  // Endurance multiplier
```

### Inversion and Paradox Components
```rust
#[derive(Component)]
pub struct InversionType(pub InversionType);  // What to invert

#[derive(Component)]
pub struct PolarityFlip(pub bool);  // Whether polarity is flipped

#[derive(Component)]
pub struct DualNature(pub bool);  // Has both positive/negative

#[derive(Component)]
pub struct HealAmount(pub f32);  // Paradox heal value

#[derive(Component)]
pub struct SimultaneousApplication(pub bool);  // Apply both effects

#[derive(Component)]
pub struct ParadigmState(pub ParadigmType);  // Current paradigm

#[derive(Component)]
pub struct InversionFieldRadius(pub f32);  // Field effect radius
```

### Probability and Gambling Components
```rust
#[derive(Component)]
pub struct InitialChance(pub f32);  // Starting probability

#[derive(Component)]
pub struct CascadeDepth(pub u32);  // Probability cascade depth

#[derive(Component)]
pub struct SuccessMultiplier(pub f32);  // Success outcome multiplier

#[derive(Component)]
pub struct FailureMultiplier(pub f32);  // Failure outcome multiplier

#[derive(Component)]
pub struct WinChance(pub f32);  // Probability of winning

#[derive(Component)]
pub struct WinResult(pub EffectType);  // Win effect

#[derive(Component)]
pub struct LoseResult(pub EffectType);  // Lose effect

#[derive(Component)]
pub struct DoubleOrNothing(pub bool);  // All or nothing gamble

#[derive(Component)]
pub struct LuckLevel(pub f32);  // Current luck value

#[derive(Component)]
pub struct LuckModifier(pub f32);  // Luck change rate

#[derive(Component)]
pub struct MaxLuck(pub f32);  // Maximum luck value

#[derive(Component)]
pub struct LuckDecay(pub f32);  // Luck reduction rate
```

## Environmental Hazard Components

```rust
#[derive(Component)]
pub struct EnvironmentalHazard;  // Marker for hazards

#[derive(Component)]
pub struct HazardDamage(pub f32);  // Damage per tick

#[derive(Component)]
pub struct HazardRadius(pub f32);  // Effect radius

#[derive(Component)]
pub struct HazardDuration(pub f32);  // Total lifetime

#[derive(Component)]
pub struct HazardTickRate(pub f32);  // Seconds between ticks

#[derive(Component)]
pub struct MovingHazard;  // Marker for mobile hazards

#[derive(Component)]
pub struct HazardVelocity(pub Vec3);  // Movement vector

#[derive(Component)]
pub struct ChainableHazard;  // Can trigger other hazards

#[derive(Component)]
pub struct HazardTriggerRange(pub f32);  // Chain trigger distance
```

## Phase Transition Components

```rust
#[derive(Component)]
pub struct BossPhase(pub u8);  // Current phase number

#[derive(Component)]
pub struct PhaseHealth(pub f32);  // Health threshold for phase

#[derive(Component)]
pub struct PhaseTimer(pub f32);  // Time until next phase

#[derive(Component)]
pub struct PhaseTransition;  // Marker for transition state

#[derive(Component)]
pub struct TransitionDuration(pub f32);  // Transition time

#[derive(Component)]
pub struct PhaseAbilities(pub Vec<AbilityId>);  // Available abilities

#[derive(Component)]
pub struct PhaseModifier(pub f32);  // Damage/speed modifier

#[derive(Component)]
pub struct FinalPhase;  // Marker for last phase
```

## Boss State Components

```rust
#[derive(Component)]
pub struct BossHealth(pub f32, pub f32);  // Current, Max

#[derive(Component)]
pub struct BossArmor(pub f32);  // Damage reduction

#[derive(Component)]
pub struct BossShield(pub f32);  // Shield amount

#[derive(Component)]
pub struct Enraged;  // Enrage state marker

#[derive(Component)]
pub struct EnrageTimer(pub f32);  // Time until enrage

#[derive(Component)]
pub struct EnrageDamageMultiplier(pub f32);  // Damage increase

#[derive(Component)]
pub struct Invulnerable;  // Cannot take damage

#[derive(Component)]
pub struct InvulnerabilityDuration(pub f32);  // Immunity time

#[derive(Component)]
pub struct ThreatTarget(pub Entity);  // Current target

#[derive(Component)]
pub struct ThreatLevel(pub f32);  // Threat value
```

## Arena Modification Components

```rust
#[derive(Component)]
pub struct ArenaZone;  // Marker for arena areas

#[derive(Component)]
pub struct ZoneType(pub ZoneType);  // Type of zone

#[derive(Component)]
pub struct ZoneActive(pub bool);  // Whether zone is active

#[derive(Component)]
pub struct ZoneEffect(pub EffectType);  // Zone's effect

#[derive(Component)]
pub struct ZonePulseRate(pub f32);  // Effect frequency

#[derive(Component)]
pub struct SafeZone;  // Marker for safe areas

#[derive(Component)]
pub struct DangerZone;  // Marker for danger areas

#[derive(Component)]
pub struct ZoneTransition;  // Zone changing state

#[derive(Component)]
pub struct TransitionTarget(pub ZoneType);  // New zone type
```

## Implementation Example

```rust
// Spawning a predictive trap (Hunter Boss)
commands.spawn((
    PredictiveTrap,
    FuturePosition(predicted_pos),
    ActivationDelay(3.0),
    TrapDamage(150.0),
    TemporalLayer(TimeState::Future),
    ElapsedTime(0.0),
    Duration(10.0),
    Transform::from_translation(current_pos),
    Mesh3d(trap_mesh),
    MeshMaterial3d(materials.web.clone()),
));

// Creating an inversion field (Cardinal Boss)
commands.spawn((
    InversionField,
    InversionFieldRadius(10.0),
    InversionType(InversionType::HealingDamage),
    Duration(3.0),
    PolarityFlip(true),
    Transform::from_translation(boss_pos),
));

// Spawning a probability cascade (Merchant Boss)
commands.spawn((
    ProbabilityCascade,
    InitialChance(0.5),
    CascadeDepth(3),
    SuccessMultiplier(2.0),
    FailureMultiplier(0.5),
    ElapsedTime(0.0),
));
```

## System Pattern Example

```rust
fn process_predictive_traps(
    mut commands: Commands,
    time: Res<Time>,
    mut traps: Query<(
        Entity,
        &mut ElapsedTime,
        &ActivationDelay,
        &FuturePosition,
        &TrapDamage,
    ), With<PredictiveTrap>>,
    players: Query<&Transform, With<Player>>,
) {
    for (entity, mut elapsed, delay, future_pos, damage) in traps.iter_mut() {
        elapsed.0 += time.delta_secs();
        
        if elapsed.0 >= delay.0 {
            // Check if any player is at predicted position
            for transform in players.iter() {
                if transform.translation.distance(future_pos.0) < 1.0 {
                    // Apply damage and spawn effect
                    commands.spawn((
                        TrapExplosion,
                        Transform::from_translation(future_pos.0),
                        ExplosionDamage(damage.0),
                        Duration(0.5),
                    ));
                }
            }
            commands.entity(entity).despawn();
        }
    }
}
```