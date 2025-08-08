# Sacrifice

A complete implementation guide for the Cardinal's health-conversion ability that transforms personal vitality into healing power for allies.

## Overview

**Sacrifice** represents the Cardinal's ultimate devotion - converting their own life force into restorative energy for allies. This health-conversion mechanic creates a unique resource management system where the Cardinal becomes both healer and resource, forcing critical decisions about when to trade personal safety for team survival. The ability's efficiency scales inversely with the Cardinal's health, rewarding dangerous play with exponentially better healing output.

## Orthogonal Design Analysis

**Unique Mechanic**: Health-to-healing conversion with inverse scaling efficiency. No other ability uses the caster's HP as a resource or rewards low-health operation.

**Strategic Niche**: Creates a "living battery" playstyle where the Cardinal's HP pool becomes the team's most valuable resource. Forces unique protection dynamics.

**Counterplay**: Enemies must decide between focusing the low-HP Cardinal or their allies. Channel interruption and burst damage become critical tactics.

## Implementation Architecture

### Component-Based Design (Single-Value Pattern)

```rust
// Sacrifice ability marker on Cardinal
#[derive(Component)]
pub struct Sacrifice;

// Channel entity components (spawned while channeling)
#[derive(Component)]
pub struct SacrificeChannel;  // Marker

#[derive(Component)]
pub struct SourceEntity(pub Entity);  // Cardinal entity

#[derive(Component)]
pub struct TargetEntity(pub Entity);  // Ally receiving healing

#[derive(Component)]
pub struct HealthDrain(pub f32);  // HP per second from Cardinal

#[derive(Component)]
pub struct BaseConversion(pub f32);  // Base healing per HP

#[derive(Component)]
pub struct MinimumHP(pub f32);  // Safety threshold (0.1 = 10%)

// Sacrifice channel entity composition
commands.spawn((
    SacrificeChannel,
    SourceEntity(cardinal_entity),
    TargetEntity(ally_entity),
    HealthDrain(25.0),
    BaseConversion(2.0),
    MinimumHP(0.1),
    ElapsedTime(0.0),
    Channeled,
));

// Life stream VFX entity
commands.spawn((
    SacrificeVfx,
    Origin(cardinal_pos),
    Target(ally_pos),
    StreamWidth(0.5),
    ParticleCount(30),
    Duration(0.1),
    ElapsedTime(0.0),
));
```

### Focused Systems

```rust
// System 1: Health drain and conversion
pub fn sacrifice_drain_system(
    mut commands: Commands,
    time: Res<Time>,
    mut channels: Query<(
        Entity,
        &SourceEntity,
        &TargetEntity,
        &HealthDrain,
        &BaseConversion,
        &MinimumHP,
        &mut ElapsedTime,
    ), With<SacrificeChannel>>,
    mut healths: Query<&mut Health>,
) {
    for (entity, source, target, drain, conversion, min_hp, mut elapsed) in channels.iter_mut() {
        elapsed.0 += time.delta_secs();
        
        if let Ok(mut source_health) = healths.get_mut(source.0) {
            let hp_percent = source_health.current / source_health.max;
            
            // Stop at minimum threshold
            if hp_percent <= min_hp.0 {
                commands.entity(entity).despawn();
                continue;
            }
            
            // Calculate efficiency (inverse scaling)
            let efficiency = match hp_percent {
                x if x > 0.7 => 2.0,
                x if x > 0.5 => 2.5,
                x if x > 0.3 => 3.0,
                _ => 4.0,  // Danger zone bonus
            };
            
            // Apply drain
            let drain_amount = drain.0 * time.delta_secs();
            source_health.current -= drain_amount;
            
            // Apply healing
            if let Ok(mut target_health) = healths.get_mut(target.0) {
                let heal_amount = drain_amount * efficiency;
                target_health.current = (target_health.current + heal_amount)
                    .min(target_health.max);
            }
        }
    }
}

// System 2: Channel input management
pub fn sacrifice_input_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    cardinal_query: Query<(Entity, &Transform), (With<Character>, With<Sacrifice>)>,
    ally_query: Query<(Entity, &Transform), (With<Character>, Without<Sacrifice>)>,
    existing_channels: Query<Entity, With<SacrificeChannel>>,
) {
    // Stop channel if button released
    if input.just_released(KeyCode::KeyE) {
        for channel in existing_channels.iter() {
            commands.entity(channel).despawn();
        }
        return;
    }
    
    // Start new channel if button pressed
    if input.just_pressed(KeyCode::KeyE) {
        if let Ok((cardinal_entity, cardinal_transform)) = cardinal_query.get_single() {
            // Find nearest damaged ally
            if let Some((ally_entity, _)) = find_nearest_damaged_ally(&ally_query, cardinal_transform) {
                commands.spawn((
                    SacrificeChannel,
                    SourceEntity(cardinal_entity),
                    TargetEntity(ally_entity),
                    HealthDrain(25.0),
                    BaseConversion(2.0),
                    MinimumHP(0.1),
                    ElapsedTime(0.0),
                ));
            }
        }
    }
}

// System 3: Visual feedback
pub fn sacrifice_vfx_system(
    mut commands: Commands,
    time: Res<Time>,
    channels: Query<(&SourceEntity, &TargetEntity), With<SacrificeChannel>>,
    mut vfx: Query<(Entity, &mut ElapsedTime, &Duration), With<SacrificeVfx>>,
    transforms: Query<&Transform>,
) {
    // Update existing VFX
    for (entity, mut elapsed, duration) in vfx.iter_mut() {
        elapsed.0 += time.delta_secs();
        if elapsed.0 >= duration.0 {
            commands.entity(entity).despawn();
        }
    }
    
    // Spawn VFX for active channels
    for (source, target) in channels.iter() {
        if let (Ok(source_transform), Ok(target_transform)) = 
            (transforms.get(source.0), transforms.get(target.0)) {
            // Spawn particle stream every 0.1 seconds
            commands.spawn((
                SacrificeVfx,
                Origin(source_transform.translation),
                Target(target_transform.translation),
                StreamWidth(0.5),
                ParticleCount(30),
                Duration(0.1),
                ElapsedTime(0.0),
            ));
        }
    }
}
```

## Gameplay Mechanics

### Health Conversion Scaling

- **100-70% HP**: 2:1 ratio (25 HP/sec drain → 50 HP/sec healing)
- **70-50% HP**: 2.5:1 ratio (25 HP/sec → 62.5 HP/sec)  
- **50-30% HP**: 3:1 ratio (25 HP/sec → 75 HP/sec)
- **30-10% HP**: 4:1 ratio (25 HP/sec → 100 HP/sec) - "Danger Zone"
- **Below 10%**: Channel automatically stops (safety threshold)

### Channel Mechanics

- **Input**: Hold button to maintain channel
- **Movement**: 50% movement speed while channeling
- **Interruption**: Any damage breaks the channel
- **Range**: 5 tiles to initiate, no range limit once started
- **Target Lock**: Channel maintains even if target moves away

## Strategic Depth

### Risk Management

The inverse efficiency scaling creates constant risk-reward decisions:
- Safe play (high HP) provides reliable but inefficient healing
- Risky play (low HP) delivers massive healing but extreme vulnerability
- The "sweet spot" around 30% HP maximizes efficiency while maintaining escape potential

### Team Dynamics

Sacrifice fundamentally changes team dynamics:
- Allies must actively protect their Cardinal to maintain healing capacity
- Enemies face difficult target prioritization decisions
- Cardinal HP becomes a visible, trackable team resource

### Counterplay Options

- **Burst Damage**: Eliminate low-HP Cardinals before they can react
- **Channel Disruption**: Any damage interrupts the life transfer
- **Isolation**: Separate Cardinal from allies to prevent sacrifice value
- **HP Pressure**: Force Cardinal to choose self-preservation over healing

## Upgrade Paths

### Tier 1: Blood Bond
- **Damage Share**: 20% of damage to healed ally transfers to Cardinal
- **Efficiency Boost**: +0.5 to all conversion ratios
- **Mobile Channel**: 75% movement speed while channeling

### Tier 2: Life Echo
- **Regeneration**: Cardinal gains 5 HP/sec for 10 seconds after channel
- **Overflow Shield**: Excess healing creates 50 HP shield on ally
- **Critical Efficiency**: 5:1 conversion below 30% HP

### Tier 3: Martyrdom
- **Death Prevention**: Cannot die from sacrifice (remain at 1 HP)
- **Phoenix Burst**: At 1 HP, heal all allies within 3 tiles for 200 HP
- **Soul Link**: If healed ally dies within 3 seconds, Cardinal dies instead and ally revives

## Balance Considerations

### Power Budget
- **Drain Rate**: 25 HP/sec balances risk with reaction time
- **Efficiency Curve**: Exponential scaling rewards mastery without being oppressive
- **Safety Threshold**: 10% minimum prevents accidental deaths
- **Movement Penalty**: 50% speed maintains vulnerability during channel

### Skill Expression
- **HP Management**: Mastery involves maintaining optimal HP thresholds
- **Target Priority**: Choosing between multiple damaged allies
- **Channel Timing**: Knowing when to commit to lengthy channels
- **Recovery Windows**: Finding safe moments to regenerate HP

## Visual Design

### Life Stream Effect
- **Color**: Deep crimson particles flowing from Cardinal to ally
- **Intensity**: Particle density increases with efficiency (more at low HP)
- **Path**: Curved arc between source and target
- **Impact**: Gentle golden glow on receiving ally

### Cardinal State Indicators
- **HP Glow**: Cardinal's model tints red as HP decreases
- **Efficiency Aura**: Pulsing aura intensity shows current conversion rate
- **Channel Beam**: Visible life force connection during sacrifice

## Audio Design

### Channel Audio
- **Heartbeat**: Rhythmic pulse that intensifies at lower HP
- **Life Drain**: Ethereal whooshing of life force transfer
- **Warning Tones**: Urgent audio cues below 30% HP

## Conclusion

Sacrifice transforms the Cardinal from traditional healer into a living resource manager, creating unprecedented tactical depth through health economics. The inverse efficiency scaling generates natural risk-reward cycles that reward mastery while maintaining clear counterplay. This ability exemplifies truly orthogonal design - no other ability in the game uses HP as a tradeable resource or rewards low-health operation.

The component architecture follows the repository's single-value pattern perfectly: simple components composed into channel entities, processed by focused sub-50-line systems. Each system has one job, making the implementation clean, testable, and deterministic.