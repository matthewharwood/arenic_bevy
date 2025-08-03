# Sniper

A complete implementation guide for the Hunter's long-range boss-targeting precision ability.

## Overview

The **Sniper** ability represents the Hunter's ultimate ranged mastery through unlimited-distance boss targeting with precision aiming mechanics. This high-damage ability fires exclusively at boss enemies with a 4-second cooldown, featuring a brief wind-up phase and targeting reticle system. The ability excels at consistent boss damage output while requiring tactical positioning and timing to maximize effectiveness.

## Game Design Philosophy

This ability demonstrates specialized role design through boss-focused targeting:

**Boss Specialist Role**: The exclusive boss targeting creates a unique tactical niche, making Hunters particularly valuable during major encounters while maintaining balance in regular combat.

**Precision Over Speed**: The wind-up and aiming phase rewards patience and timing over rapid firing, creating distinct gameplay feel from other ranged abilities.

**Unlimited Range Advantage**: The infinite range creates interesting positioning opportunities and rewards map knowledge and sightline awareness.

## Implementation Architecture

### Component-Based Design

```rust
Sniper {
    wind_up_time: 0.5,                  // 0.5 second aiming wind-up
    projectile_speed: 15.0,             // 15 tiles per second (faster than normal)
    damage: 300.0,                      // 300 damage per shot
    range: f32::INFINITY,               // Unlimited targeting range
    target_filter: BossOnly,            // Can only target boss enemies
    cooldown: 4.0,                      // 4 second ability cooldown
    requires_line_of_sight: true,       // Must have clear shot to target
}

SniperShot {
    target: Entity,
    wind_up_progress: f32,
    aiming_reticle: Entity,
    charge_visual: Entity,
    shot_trajectory: Vec2,
}

SniperProjectile {
    position: Vec2,
    velocity: Vec2,
    target: Entity,
    damage: f32,
    visual_effect: Entity,
    impact_prediction: Vec2,
}
```

### Event-Driven Systems

The ability operates through five precision systems:
1. **Boss Detection** - Identifies valid boss targets regardless of distance
2. **Aiming System** - Manages 0.5-second wind-up with targeting reticle
3. **Projectile Physics** - Handles high-speed projectile with perfect accuracy
4. **Line of Sight** - Validates clear shooting lanes to prevent impossible shots
5. **Visual Coordination** - Manages sniper scope effects and precision feedback

### Required Components

```rust
// Core Components
Damage(300.0)
ProjectileSpeed(15.0)
AttackRange(f32::INFINITY)
CastTime(Timer::from_seconds(0.5, TimerMode::Once))
Cooldown(Timer::from_seconds(4.0, TimerMode::Once))
RequiresLOS
TargetFilter { 
    include_allies: false, 
    include_enemies: true, 
    include_self: false, 
    min_health_percent: 0.0, 
    max_health_percent: 1.0 
}
IsBoss  // Target marker component

// Projectile Components
HomingProjectile { turn_rate: f32::INFINITY, max_lifetime: Timer::from_seconds(5.0, TimerMode::Once) }
ProjectileTarget(boss_position)
ProjectileLifetime(Timer::from_seconds(5.0, TimerMode::Once))
EnemyOnly
IgnoresCover  // Sniper shots go through obstacles

// Visual Components
Telegraph { shape: "line".to_string(), size: Vec2::new(1.0, 100.0), warning_duration: Timer::from_seconds(0.5, TimerMode::Once), danger_color: Color::srgb(1.0, 0.2, 0.2) }
AudioEffect { sound_file: "sniper_shot.ogg".to_string(), volume: 1.0, pitch: 1.0 }
VisualEffect { effect_type: "sniper_trail".to_string(), scale: 1.2, color: Color::srgb(1.0, 0.8, 0.2), duration: Timer::from_seconds(0.8, TimerMode::Once) }

// Upgrade Components
Upgrade 1:
- Damage(400.0)
- CastTime(Timer::from_seconds(0.3, TimerMode::Once))
- Cooldown(Timer::from_seconds(3.0, TimerMode::Once))

Upgrade 2:
- TrueDamage(200.0)  // 200 of the damage ignores armor
- CritChance(0.25)
- CritMultiplier(2.0)
- DamageReduction(0.15)  // Debuff on target

Upgrade 3:
- TargetAllInRange  // Modified to target all bosses
- ChainTarget { max_chains: 99, current_chains: 0, chain_damage_falloff: 0.0 }  // Kill chains to other bosses
- CritChance(1.0)  // Always crits
```

### High-Level Implementation Plan

1. **Boss Target Detection System**
   ```rust
   fn sniper_target_system(
       hunters: Query<(Entity, &Transform), With<SniperAbility>>,
       bosses: Query<(Entity, &Transform, &Health), (With<IsBoss>, With<Enemy>)>,
       mut target_events: EventWriter<SniperTargetEvent>,
   ) {
       for (hunter_entity, hunter_transform) in hunters.iter() {
           // Find all valid boss targets
           let mut valid_targets: Vec<(Entity, f32)> = bosses
               .iter()
               .filter_map(|(boss_entity, boss_transform, health)| {
                   if health.current > 0.0 && has_line_of_sight(hunter_transform, boss_transform) {
                       let distance = hunter_transform.translation.distance(boss_transform.translation);
                       Some((boss_entity, distance))
                   } else {
                       None
                   }
               })
               .collect();
           
           // Sort by distance for priority targeting
           valid_targets.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
           
           if let Some((target, _)) = valid_targets.first() {
               target_events.write(SniperTargetEvent {
                   hunter: hunter_entity,
                   target: *target,
               });
           }
       }
   }
   ```

2. **Wind-Up and Aiming System**
   ```rust
   fn sniper_aiming_system(
       mut commands: Commands,
       time: Res<Time>,
       mut aiming_query: Query<(Entity, &mut CastTime, &SniperTarget, &Transform), With<SniperAiming>>,
       recording: Res<RecordingState>,
   ) {
       for (entity, mut cast_time, target, transform) in aiming_query.iter_mut() {
           cast_time.0.tick(time.delta());
           
           if cast_time.0.just_finished() {
               // Fire sniper shot
               commands.spawn((
                   SniperProjectile,
                   Damage(300.0),
                   ProjectileSpeed(15.0),
                   ProjectileTarget(target.position),
                   HomingProjectile { turn_rate: f32::INFINITY, max_lifetime: Timer::from_seconds(5.0, TimerMode::Once) },
                   Transform::from_translation(transform.translation),
                   RecordableAction {
                       action_type: "sniper_shot".to_string(),
                       timestamp: recording.current_time,
                       position: transform.translation,
                       parameters: HashMap::from([
                           ("target_id".to_string(), target.entity.index() as f32),
                           ("damage".to_string(), 300.0),
                       ]),
                   },
               ));
               
               // Clean up aiming state
               commands.entity(entity).remove::<SniperAiming>();
           }
       }
   }
   ```

3. **Projectile Tracking System**
   ```rust
   fn sniper_projectile_system(
       mut commands: Commands,
       time: Res<Time>,
       mut projectiles: Query<(Entity, &mut Transform, &ProjectileSpeed, &ProjectileTarget, &mut HomingProjectile)>,
       bosses: Query<&Transform, With<IsBoss>>,
   ) {
       for (entity, mut transform, speed, target, mut homing) in projectiles.iter_mut() {
           // Perfect homing to boss target
           if let Ok(boss_transform) = bosses.get(target.0) {
               let direction = (boss_transform.translation - transform.translation).normalize();
               transform.translation += direction * speed.0 * TILE_SIZE * time.delta_seconds();
               
               // Check for impact
               if transform.translation.distance(boss_transform.translation) < COLLISION_THRESHOLD {
                   commands.entity(entity).despawn_recursive();
               }
           }
           
           homing.max_lifetime.tick(time.delta());
           if homing.max_lifetime.finished() {
               commands.entity(entity).despawn_recursive();
           }
       }
   }
   ```

4. **Damage Application System**
   ```rust
   fn sniper_damage_system(
       mut commands: Commands,
       projectiles: Query<(Entity, &Transform, &Damage), With<SniperProjectile>>,
       mut bosses: Query<(Entity, &Transform, &mut Health), With<IsBoss>>,
       mut damage_events: EventWriter<DamageEvent>,
   ) {
       for (proj_entity, proj_transform, damage) in projectiles.iter() {
           for (boss_entity, boss_transform, mut health) in bosses.iter_mut() {
               if is_collision(proj_transform, boss_transform) {
                   damage_events.write(DamageEvent {
                       target: boss_entity,
                       amount: damage.0,
                       damage_type: DamageType::Physical,
                       is_critical: false,  // Base sniper shots don't crit
                   });
                   
                   commands.entity(proj_entity).despawn_recursive();
                   break;
               }
           }
       }
   }
   ```

5. **Visual Telegraph System**
   ```rust
   fn sniper_telegraph_system(
       mut commands: Commands,
       aiming: Query<(&Transform, &SniperTarget), Added<SniperAiming>>,
   ) {
       for (hunter_transform, target) in aiming.iter() {
           let direction = (target.position - hunter_transform.translation).normalize();
           let distance = hunter_transform.translation.distance(target.position);
           
           commands.spawn((
               Telegraph {
                   shape: "line".to_string(),
                   size: Vec2::new(1.0, distance),
                   warning_duration: Timer::from_seconds(0.5, TimerMode::Once),
                   danger_color: Color::srgb(1.0, 0.2, 0.2),
               },
               Transform::from_translation(hunter_transform.translation)
                   .looking_at(target.position, Vec3::Y),
           ));
       }
   }
   ```

6. **Recording Integration**
   - Boss targeting is deterministic based on distance and line of sight
   - Wind-up timing is frame-perfect for consistent replays
   - Projectile paths are deterministic with perfect homing
   - All damage calculations are predictable and replayable

## Step-by-Step Gameplay

### Phase 1: Boss Target Acquisition (Tap Activation)
- **Input Method**: Tap to begin sniper shot targeting sequence
- **Boss Scanning**: System automatically identifies all boss enemies on battlefield
- **Target Selection**: If multiple bosses exist, targets closest or most recently damaged
- **Line of Sight**: Validates clear shooting path to selected boss target

### Phase 2: Aiming Wind-Up (0.5 Second Duration)
- **Aiming Stance**: Hunter assumes precision shooting position with enhanced focus
- **Reticle Appearance**: Targeting crosshair appears over selected boss enemy
- **Charge Buildup**: Visual energy builds around Hunter indicating shot preparation
- **Vulnerability Window**: Hunter cannot move during wind-up but can cancel shot

### Phase 3: Precision Shot (Instant Release)
- **Perfect Accuracy**: Shot travels directly to boss target regardless of distance
- **High Velocity**: Projectile moves at 15 tiles/second for minimal travel time
- **Piercing Capability**: Shot ignores all obstacles and enemies between Hunter and boss
- **Visual Trail**: Distinctive sniper bullet trail with enhanced particle effects

### Phase 4: Impact Resolution (On Contact)
- **Guaranteed Hit**: Boss target receives full 300 damage upon projectile contact
- **Critical Visual**: Enhanced impact effects emphasizing precision and power
- **Audio Feedback**: Satisfying long-range impact sound with echo effects
- **Cooldown Start**: 4-second cooldown begins immediately after successful shot

## Boss Detection and Targeting

### Target Identification System
```rust
fn find_boss_targets() -> Vec<Entity> {
    get_all_enemies()
        .iter()
        .filter(|enemy| is_boss(**enemy) && is_alive(**enemy))
        .cloned()
        .collect()
}

fn select_sniper_target(hunter_pos: Vec2, bosses: &[Entity]) -> Option<Entity> {
    if bosses.is_empty() {
        return None;
    }
    
    // Priority: Most recently damaged boss, then closest boss
    if let Some(recent_target) = get_most_recently_damaged_boss(bosses) {
        if has_line_of_sight(hunter_pos, get_position(recent_target)) {
            return Some(recent_target);
        }
    }
    
    // Fallback to closest boss with line of sight
    bosses.iter()
        .filter(|boss| has_line_of_sight(hunter_pos, get_position(**boss)))
        .min_by(|a, b| {
            let dist_a = Vec2::distance(hunter_pos, get_position(**a));
            let dist_b = Vec2::distance(hunter_pos, get_position(**b));
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}
```

### Line of Sight Validation
- **Obstacle Detection**: Raycast between Hunter and boss to check for blocking terrain
- **Projectile Path**: Ensures clear trajectory exists for successful shot delivery
- **Dynamic Checking**: Revalidates line of sight during wind-up in case of movement
- **Failure Handling**: Cancels shot if line of sight is lost during aiming phase

## Tactical Positioning and Usage

### Optimal Positioning Strategy
- **High Ground Advantage**: Elevated positions often provide better line of sight to bosses
- **Cover Utilization**: Use partial cover that blocks boss attacks but allows sniper shots
- **Range Exploitation**: Position at maximum distance to minimize return fire threat
- **Escape Routes**: Maintain positioning that allows retreat after high-threat shots

### Timing and Coordination
- **Boss Phase Awareness**: Time shots during boss vulnerability windows for maximum impact
- **Team Synchronization**: Coordinate with tank positioning to maintain clear sight lines
- **Cooldown Management**: Plan shots around boss attack patterns and defensive phases
- **Priority Targeting**: Focus fire on most dangerous or low-health bosses first

## Upgrade Paths

### Tier 1: Enhanced Precision
- **Damage Increase**: 300 → 400 damage per sniper shot
- **Wind-Up Reduction**: 0.5 → 0.3 seconds aiming time
- **Cooldown Improvement**: 4 → 3 seconds between shots
- **Strategic Value**: Higher DPS with faster firing for improved boss damage output

### Tier 2: Armor Piercing
- **Defense Penetration**: Ignores 50% of boss armor and defensive abilities
- **Weak Point Targeting**: 25% chance for critical hit dealing double damage
- **Debuff Application**: Successful hits reduce boss damage output by 15% for 8 seconds
- **Tactical Evolution**: Transforms from pure damage to boss debuffing support tool

### Tier 3: Master Marksman
- **Multi-Target**: Can target all bosses simultaneously with single activation
- **Chain Reaction**: Boss kills trigger automatic sniper shots at remaining bosses
- **Perfect Timing**: Sniper shots automatically fire during optimal boss vulnerability windows
- **Ultimate Precision**: Guarantees critical hits and applies permanent damage stacking debuffs

## Integration with Hunter Kit

### Ability Synergy
- **Auto Shot Coordination**: Auto shots continue providing general damage while sniper focuses bosses
- **Trap Setup**: Use traps to control boss positioning for better sniper angles
- **Poison Shot Synergy**: Combine sustained poison with high-damage sniper shots for total boss pressure
- **Positioning Tools**: Other abilities help maintain optimal sniper positioning

### Combat Flow Integration
- **Boss Priority**: Sniper shots take precedence during major enemy encounters
- **Resource Management**: No resource costs allow consistent boss pressure throughout encounters
- **Tactical Flexibility**: Instant cast (after wind-up) allows reactive boss damage application
- **Team Support**: Consistent boss damage supports team strategy without competing for resources

## Visual & Audio Design

### Target Acquisition
- **Visual**: Hunter weapon extends with scope attachment and enhanced precision mechanisms
- **UI**: Boss targets highlighted with distinctive sniper reticle overlay
- **Audio**: Mechanical scope adjustment sounds with precision engineering theme
- **Feedback**: Clear indication when valid boss target is acquired and locked

### Aiming Wind-Up
- **Visual**: Hunter assumes stable shooting stance with enhanced focus effects
- **Animation**: Weapon stabilization with breathing control and precision aiming
- **Audio**: Subtle mechanical charging sound building toward shot release
- **Reticle**: Crosshair appears over target with range and wind information

### Precision Shot
- **Visual**: Muzzle flash with enhanced sniper rifle effects and recoil animation
- **Projectile**: High-velocity bullet trail with precision trajectory visualization
- **Audio**: Sharp crack of sniper rifle with realistic long-range acoustic properties
- **Environment**: Brief lens flare or scope glint effect for dramatic emphasis

### Impact and Aftermath
- **Visual**: Enhanced impact effects emphasizing precision and stopping power
- **Animation**: Boss target recoils with significant impact reaction
- **Audio**: Satisfying long-range impact sound with environmental echo
- **Feedback**: Damage numbers display with special sniper shot styling and emphasis