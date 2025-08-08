# Misdirection

A complete implementation guide for the Thief's ability redirection mechanic that causes enemy abilities to target their own allies.

## Overview

**Misdirection** transforms the Thief from evasive survivor into tactical manipulator. By creating a zone of confusion, enemy abilities fired through or within the area have their targeting reversed - healing becomes damage to allies, damage hits friendly targets, and buffs affect enemies instead. This redirection mechanic creates chaotic reversals that punish ability spam and reward clever positioning.

## Orthogonal Design Analysis

**Unique Mechanic**: Ability redirection and reversal. No other ability manipulates enemy targeting or reverses ability effects - most defensive abilities block or evade.

**Strategic Niche**: Creates "friendly fire" scenarios that turn enemy aggression against themselves. Transforms defensive play into offensive opportunity.

**Counterplay**: Enemies can avoid using abilities near misdirection zones, wait for expiration, or use basic attacks which aren't redirected.

## Implementation Architecture

### Component-Based Design (Single-Value Pattern)

```rust
// Misdirection ability marker on Thief
#[derive(Component)]
pub struct Misdirection;

// Misdirection zone entity components
#[derive(Component)]
pub struct MisdirectionZone;  // Marker

#[derive(Component)]
pub struct ZoneRadius(pub f32);  // Area of effect

#[derive(Component)]
pub struct RedirectionChance(pub f32);  // Probability of redirection (0.0-1.0)

#[derive(Component)]
pub struct ZoneDuration(pub f32);  // Seconds remaining

// Misdirection zone entity composition
commands.spawn((
    MisdirectionZone,
    Origin(target_position),
    ZoneRadius(3.0 * TILE_SIZE),
    RedirectionChance(1.0),  // 100% redirection
    ZoneDuration(6.0),
    ElapsedTime(0.0),
));

// Confused projectile marker (attached to redirected abilities)
#[derive(Component)]
pub struct Redirected;

#[derive(Component)]
pub struct OriginalTarget(pub Entity);

#[derive(Component)]
pub struct NewTarget(pub Entity);

// Zone VFX entity
commands.spawn((
    MisdirectionVfx,
    Origin(zone_center),
    Radius(3.0 * TILE_SIZE),
    SwirlingParticles(100),
    DistortionIntensity(0.5),
    Duration(6.0),
    ElapsedTime(0.0),
));
```

### Focused Systems

```rust
// System 1: Zone spawning
pub fn cast_misdirection_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    thief_query: Query<&Transform, With<Misdirection>>,
    cursor: Res<CursorWorldPos>,
) {
    if !input.just_pressed(KeyCode::KeyQ) {
        return;
    }
    
    for thief_transform in thief_query.iter() {
        let cast_range = 6.0 * TILE_SIZE;
        let target_pos = cursor.0.clamp_distance(
            thief_transform.translation,
            cast_range
        );
        
        // Spawn misdirection zone
        commands.spawn((
            MisdirectionZone,
            Origin(target_pos),
            ZoneRadius(3.0 * TILE_SIZE),
            RedirectionChance(1.0),
            ZoneDuration(6.0),
            ElapsedTime(0.0),
        ));
        
        // Spawn VFX
        commands.spawn((
            MisdirectionVfx,
            Origin(target_pos),
            Radius(3.0 * TILE_SIZE),
            Duration(6.0),
            ElapsedTime(0.0),
        ));
    }
}

// System 2: Projectile redirection
pub fn redirect_projectiles_system(
    mut commands: Commands,
    zones: Query<(&Origin, &ZoneRadius, &RedirectionChance), With<MisdirectionZone>>,
    mut projectiles: Query<(
        Entity,
        &Transform,
        &mut Target,
        Option<&TargetEntity>
    ), (With<Projectile>, Without<Redirected>)>,
    enemies: Query<Entity, With<Enemy>>,
    allies: Query<Entity, With<Character>>,
) {
    for (zone_origin, radius, chance) in zones.iter() {
        for (proj_entity, proj_transform, mut target, target_entity) in projectiles.iter_mut() {
            // Check if projectile is within zone
            let distance = proj_transform.translation.distance(zone_origin.0);
            if distance <= radius.0 {
                // Roll for redirection
                if rand::random::<f32>() <= chance.0 {
                    // Find new target (enemy projectile -> ally, ally projectile -> enemy)
                    let new_target = find_redirected_target(
                        &enemies,
                        &allies,
                        target_entity,
                        proj_transform
                    );
                    
                    if let Some(new_target_entity) = new_target {
                        // Update projectile target
                        target.0 = get_entity_position(new_target_entity);
                        
                        // Mark as redirected
                        commands.entity(proj_entity).insert((
                            Redirected,
                            OriginalTarget(target_entity.unwrap_or(Entity::PLACEHOLDER)),
                            NewTarget(new_target_entity),
                        ));
                    }
                }
            }
        }
    }
}

// System 3: Ability reversal for area effects
pub fn reverse_area_abilities_system(
    zones: Query<(&Origin, &ZoneRadius), With<MisdirectionZone>>,
    mut area_effects: Query<(&Transform, &mut TargetAllegiance), With<AreaEffect>>,
) {
    for (zone_origin, radius) in zones.iter() {
        for (effect_transform, mut allegiance) in area_effects.iter_mut() {
            let distance = effect_transform.translation.distance(zone_origin.0);
            if distance <= radius.0 {
                // Reverse targeting allegiance
                *allegiance = match *allegiance {
                    TargetAllegiance::Allies => TargetAllegiance::Enemies,
                    TargetAllegiance::Enemies => TargetAllegiance::Allies,
                    TargetAllegiance::All => TargetAllegiance::All,
                };
            }
        }
    }
}

// System 4: Zone duration and cleanup
pub fn misdirection_duration_system(
    mut commands: Commands,
    time: Res<Time>,
    mut zones: Query<(Entity, &mut ZoneDuration, &mut ElapsedTime), With<MisdirectionZone>>,
) {
    for (entity, mut duration, mut elapsed) in zones.iter_mut() {
        elapsed.0 += time.delta_secs();
        duration.0 -= time.delta_secs();
        
        if duration.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// System 5: Visual distortion effect
pub fn misdirection_vfx_system(
    time: Res<Time>,
    mut vfx: Query<(&mut Transform, &DistortionIntensity, &ElapsedTime), With<MisdirectionVfx>>,
) {
    for (mut transform, intensity, elapsed) in vfx.iter_mut() {
        // Swirling distortion effect
        let rotation = elapsed.0 * 2.0;
        transform.rotate_y(rotation * time.delta_secs());
        
        // Pulsing intensity
        let pulse = (elapsed.0 * 3.0).sin() * 0.2 + 1.0;
        transform.scale = Vec3::splat(pulse * intensity.0);
    }
}
```

## Gameplay Mechanics

### Zone Properties

- **Duration**: 6 seconds of active redirection
- **Radius**: 3-tile radius sphere of influence
- **Cast Range**: 6 tiles from Thief position
- **Redirection Rate**: 100% of abilities passing through

### Redirection Rules

- **Projectiles**: Change target to nearest enemy of original caster
- **Area Effects**: Reverse ally/enemy targeting
- **Buffs/Debuffs**: Apply to opposite allegiance
- **Healing**: Becomes damage to allies if from enemy source

### Exceptions

- **Basic Attacks**: Not redirected (melee/auto-attacks)
- **Self-Targeted**: Abilities targeting self are unaffected
- **Environmental**: Terrain effects pass through normally

## Strategic Depth

### Defensive Usage

Misdirection excels at neutralizing ability-heavy enemies:
- Place between team and enemy casters
- Force enemies to reposition or hold abilities
- Create safe zones during ability barrages

### Offensive Opportunities

Turn enemy power against themselves:
- Redirect high-damage ultimates back at enemy team
- Convert enemy healing into friendly fire
- Make enemies buff your team instead

### Mind Games

The threat of misdirection changes enemy behavior:
- Enemies hesitate to use abilities
- Forces predictable positioning away from zones
- Creates psychological pressure on enemy casters

## Upgrade Paths

### Tier 1: Lingering Confusion
- **Duration**: 6 seconds → 9 seconds
- **Size**: 3-tile → 4-tile radius
- **Persistence**: Zone remains for 2 seconds after Thief death

### Tier 2: Chaos Amplification
- **Damage Boost**: Redirected abilities deal 25% more damage
- **Double Redirect**: 25% chance to redirect twice (bounces between enemies)
- **Confusion Spread**: Redirected enemies are slowed by 30%

### Tier 3: Mirror Realm
- **Duplicate Zone**: Creates second zone at Thief's position
- **Perfect Reflection**: Redirected abilities also copy themselves
- **Reality Tear**: Zone deals 10 damage/second to enemies inside

## Balance Considerations

### Power Budget
- **No Direct Damage**: Zone itself deals no damage
- **Requires Enemy Abilities**: Useless against basic attack teams
- **Cooldown**: 20 second cooldown prevents spam
- **Visibility**: Zone clearly visible to all players

### Skill Expression
- **Predictive Placement**: Anticipating ability trajectories
- **Timing**: When to deploy for maximum disruption
- **Positioning**: Forcing enemies into bad positions
- **Baiting**: Encouraging enemies to waste abilities

## Visual Design

### Zone Appearance
- **Swirling Vortex**: Purple-black energy spiral
- **Distortion Effect**: Space appears warped within zone
- **Particle System**: Confusing directional particles
- **Edge Definition**: Clear boundary visualization

### Redirection Effect
- **Color Shift**: Projectiles turn purple when redirected
- **Trail Bend**: Visible trajectory change
- **Confusion Particles**: Sparkles indicating redirection

## Audio Design

### Zone Audio
- **Activation**: Reality-warping "whoosh" sound
- **Ambient**: Disorienting whispers and echoes
- **Redirection**: Sharp "reversal" sound effect

## Conclusion

Misdirection creates unique tactical scenarios through ability redirection, transforming the Thief from simple evader into battlefield manipulator. By turning enemy abilities against themselves, it punishes ability spam while rewarding clever positioning. The mechanic is completely orthogonal - no other ability manipulates enemy targeting in this way.

The implementation uses clean single-value components: zone entities with radius and duration, redirection markers on affected projectiles, and focused systems for each behavior. This maintains the repository's pattern of ephemeral entities with composed components while creating genuinely unique gameplay.