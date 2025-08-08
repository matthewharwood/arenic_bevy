# Mark Target

A complete implementation guide for the Hunter's damage amplification ability that marks enemies for increased damage from all sources.

## Overview

**Mark Target** transforms the Hunter from pure damage dealer into tactical coordinator. By marking an enemy, the Hunter amplifies all damage that target receives from any source, creating powerful focus-fire opportunities. This amplification mechanic rewards team coordination and target prioritization, making the Hunter essential for eliminating high-value threats.

## Orthogonal Design Analysis

**Unique Mechanic**: Damage amplification from all sources. No other ability increases incoming damage universally - most abilities deal damage directly or buff allies.

**Strategic Niche**: Creates focus-fire coordination without dealing direct damage. Transforms Hunter into a force multiplier rather than solo damage dealer.

**Counterplay**: Marked targets can retreat, use defensive abilities, or force target switches. Mark visibility enables tactical responses.

## Implementation Architecture

### Component-Based Design (Single-Value Pattern)

```rust
// Mark Target ability marker on Hunter
#[derive(Component)]
pub struct MarkTarget;

// Mark entity components (attached to marked enemy)
#[derive(Component)]
pub struct Marked;  // Marker component

#[derive(Component)]
pub struct MarkSource(pub Entity);  // Hunter who applied mark

#[derive(Component)]
pub struct AmplificationFactor(pub f32);  // Damage multiplier (1.5 = 50% more)

#[derive(Component)]
pub struct MarkDuration(pub f32);  // Seconds remaining

// Mark application entity (spawned on cast)
commands.spawn((
    MarkApplication,
    SourceEntity(hunter_entity),
    TargetEntity(enemy_entity),
    AmplificationFactor(1.5),
    Duration(8.0),
    Range(10.0 * TILE_SIZE),
));

// Mark VFX entity (child of marked enemy)
commands.entity(enemy_entity).with_child((
    MarkVfx,
    PulseIntensity(2.0),
    PulseRate(1.0),
    IconHeight(2.0),
    Duration(8.0),
    ElapsedTime(0.0),
));
```

### Focused Systems

```rust
// System 1: Mark application
pub fn apply_mark_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    hunter_query: Query<(Entity, &Transform), With<MarkTarget>>,
    enemy_query: Query<(Entity, &Transform), Without<Character>>,
    existing_marks: Query<&MarkSource, With<Marked>>,
) {
    if !input.just_pressed(KeyCode::KeyE) {
        return;
    }
    
    for (hunter_entity, hunter_transform) in hunter_query.iter() {
        // Find nearest enemy within range
        if let Some((enemy_entity, _)) = find_nearest_enemy(
            &enemy_query, 
            hunter_transform, 
            10.0 * TILE_SIZE
        ) {
            // Remove existing mark from this hunter
            for (entity, mark_source) in existing_marks.iter() {
                if mark_source.0 == hunter_entity {
                    commands.entity(entity).remove::<Marked>();
                    commands.entity(entity).remove::<MarkSource>();
                    commands.entity(entity).remove::<AmplificationFactor>();
                    commands.entity(entity).remove::<MarkDuration>();
                }
            }
            
            // Apply new mark
            commands.entity(enemy_entity).insert((
                Marked,
                MarkSource(hunter_entity),
                AmplificationFactor(1.5),
                MarkDuration(8.0),
            ));
            
            // Spawn VFX
            commands.entity(enemy_entity).with_child((
                MarkVfx,
                PulseIntensity(2.0),
                PulseRate(1.0),
                Duration(8.0),
                ElapsedTime(0.0),
            ));
        }
    }
}

// System 2: Damage amplification
pub fn amplify_damage_system(
    damage_events: EventReader<DamageEvent>,
    marked_query: Query<&AmplificationFactor, With<Marked>>,
) {
    for event in damage_events.iter() {
        if let Ok(amplification) = marked_query.get(event.target) {
            // Amplify the damage
            event.amount *= amplification.0;
        }
    }
}

// System 3: Mark duration management
pub fn mark_duration_system(
    mut commands: Commands,
    time: Res<Time>,
    mut marked_query: Query<(Entity, &mut MarkDuration), With<Marked>>,
) {
    for (entity, mut duration) in marked_query.iter_mut() {
        duration.0 -= time.delta_secs();
        
        if duration.0 <= 0.0 {
            // Remove mark components
            commands.entity(entity).remove::<Marked>();
            commands.entity(entity).remove::<MarkSource>();
            commands.entity(entity).remove::<AmplificationFactor>();
            commands.entity(entity).remove::<MarkDuration>();
            
            // Despawn VFX children
            commands.entity(entity).despawn_descendants();
        }
    }
}

// System 4: Visual feedback update
pub fn mark_vfx_system(
    time: Res<Time>,
    mut vfx_query: Query<(&PulseIntensity, &PulseRate, &mut Transform), With<MarkVfx>>,
) {
    for (intensity, rate, mut transform) in vfx_query.iter_mut() {
        // Pulsing scale effect
        let pulse = (time.elapsed_secs() * rate.0).sin() * 0.5 + 1.0;
        transform.scale = Vec3::splat(pulse * intensity.0);
    }
}
```

## Gameplay Mechanics

### Mark Properties

- **Duration**: 8 seconds base duration
- **Amplification**: 50% increased damage from all sources
- **Range**: 10 tiles to apply mark
- **Visibility**: Marked enemies have visible indicator above them
- **Exclusivity**: Each Hunter can only mark one target at a time

### Damage Amplification

- **Universal**: Amplifies damage from ANY source (allies, environment, self)
- **Multiplicative**: Stacks multiplicatively with other damage modifiers
- **No Damage Cap**: Amplifies both small and large damage equally
- **Instant Application**: Mark takes effect immediately

## Strategic Depth

### Target Priority

Mark Target creates critical decisions about target selection:
- High-health enemies become manageable with focused fire
- Dangerous enemies can be eliminated quickly
- Healers/supports become vulnerable when marked
- Tanks lose their durability advantage

### Team Coordination

The ability naturally encourages team coordination:
- Visible mark signals focus-fire target to entire team
- Amplification rewards coordinated attacks
- Creates implicit shot-calling through mark placement

### Counterplay Options

- **Defensive Abilities**: Shields, invulnerability, damage reduction
- **Repositioning**: Marked targets can retreat out of range
- **Target Switching**: Force Hunter to waste mark on less valuable targets
- **Cleanse Effects**: Remove mark with cleanse abilities

## Upgrade Paths

### Tier 1: Hunter's Focus
- **Duration**: 8 seconds → 12 seconds
- **Amplification**: 50% → 65% damage increase
- **Cast Time**: Instant mark application

### Tier 2: Spreading Mark
- **Chain Mark**: On marked target death, mark jumps to nearest enemy
- **Multi-Mark**: Can mark up to 2 targets simultaneously
- **Vision**: Marked targets visible through walls to team

### Tier 3: Executioner's Mark
- **Critical Vulnerability**: Marked targets take 100% more critical damage
- **Health Threshold**: Below 30% HP, amplification doubles
- **Team Reward**: Killing marked target grants 5% damage boost to team for 10 seconds

## Balance Considerations

### Power Budget
- **No Direct Damage**: Mark itself deals zero damage
- **Single Target**: Can only affect one enemy at a time
- **Cooldown**: 12 second cooldown prevents mark spam
- **Range Limited**: Must be within 10 tiles to apply

### Skill Expression
- **Target Selection**: Choosing the right target to mark
- **Timing**: When to mark for maximum team damage
- **Repositioning**: Maintaining mark application range
- **Coordination**: Communicating marks to teammates

## Visual Design

### Mark Indicator
- **Icon**: Glowing red crosshair floating above enemy
- **Pulse Effect**: Rhythmic pulsing to draw attention
- **Team Visibility**: Visible to all allies through walls
- **Intensity Scaling**: Brighter as duration decreases

### Application Effect
- **Cast Animation**: Hunter points at target
- **Beam Effect**: Brief red targeting laser
- **Impact**: Crosshair "locks on" with satisfying click

## Audio Design

### Mark Sounds
- **Application**: Sharp "target acquired" lock-on sound
- **Ambient**: Subtle pulsing hum while marked
- **Expiration**: "Target lost" deactivation sound

## Conclusion

Mark Target transforms the Hunter from damage dealer into tactical coordinator through its unique amplification mechanic. By making enemies take more damage from ALL sources, it creates natural focus-fire coordination without requiring voice communication. The ability rewards good target selection and team coordination while maintaining clear counterplay through positioning and defensive abilities.

The single-value component architecture keeps the implementation clean: mark components attached directly to enemies, processed by focused systems for amplification, duration, and visuals. This follows the repository's patterns perfectly while creating genuinely orthogonal gameplay.