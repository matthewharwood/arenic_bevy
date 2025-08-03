# Trap

A complete implementation guide for the Hunter's explosive area denial utility ability.

## Overview

The **Trap** ability demonstrates the Hunter's mastery over battlefield control through strategic explosive placement. When activated, the Hunter instantly places a concealed trap on their current grid position with a 3-second cooldown. The trap triggers when any enemy steps on it, creating a 2x2 area explosion that deals significant damage. This ability excels at area denial, enemy routing, and strategic positioning control.

## Game Design Philosophy

This ability showcases tactical preparation design through predictive placement mechanics:

**Predictive Skill Expression**: Success requires understanding enemy movement patterns and anticipating future positions rather than reactive combat responses.

**Risk-Free Placement**: The instant cast and short cooldown encourage experimentation and tactical creativity without punishing failed predictions.

**Area Control**: The 2x2 explosion creates meaningful space control that affects enemy routing and formation decisions.

## Implementation Architecture

### Component-Based Design

```rust
Trap {
    placement_time: 0.0,                // Instant placement
    cooldown: 3.0,                      // 3 second ability cooldown
    trigger_type: EnemyContact,         // Activates when enemy steps on tile
    explosion_area: GridArea::new(2, 2), // 2x2 explosion radius
    explosion_damage: 150.0,            // 150 damage to all enemies in area
    trap_lifetime: 60.0,                // 60 second maximum trap duration
    concealment: true,                  // Invisible to enemies, visible to allies
}

PlacedTrap {
    position: GridPos,
    hunter_owner: Entity,
    trigger_area: GridArea,
    armed_status: bool,
    lifetime_remaining: f32,
    visual_effect: Entity,              // Ally-only visual indicator
}

TrapExplosion {
    center_position: GridPos,
    affected_area: GridArea,
    damage_amount: f32,
    affected_enemies: Vec<Entity>,
    visual_effect: Entity,
    audio_source: Entity,
}
```

### Event-Driven Systems

The ability operates through five coordinated systems:
1. **Trap Placement** - Handles instant deployment at Hunter's current position
2. **Trigger Detection** - Monitors enemy movement for trap activation conditions
3. **Explosion Management** - Creates area damage effect when trap is triggered
4. **Concealment System** - Manages trap visibility (hidden from enemies, visible to allies)
5. **Lifetime Tracking** - Handles trap expiration and automatic cleanup

### Required Components

```rust
// Core Components
InstantCast
Cooldown(Timer::from_seconds(3.0, TimerMode::Once))
PlaceObject { 
    object_type: "explosive_trap".to_string(), 
    duration: Some(Timer::from_seconds(60.0, TimerMode::Once)), 
    health: Some(1.0) 
}
AreaDamage { radius: 2.0, damage: 150.0, falloff: 0.0 }
TargetPosition(hunter_position)

// Trap Object Components (for placed traps)
Duration(Timer::from_seconds(60.0, TimerMode::Once))
AreaOfEffect(2.0)
ExplodeOnImpact { explosion_radius: 2.0, explosion_damage: 150.0 }
EnemyOnly
IsStealthed  // Hidden from enemies
AllyOnly  // Visible to allies

// Explosion Components (triggered state)
AreaDamage { radius: 2.0, damage: 150.0, falloff: 0.0 }
VisualEffect { effect_type: "trap_explosion".to_string(), scale: 2.0, color: Color::srgb(1.0, 0.3, 0.0), duration: Timer::from_seconds(1.0, TimerMode::Once) }
AudioEffect { sound_file: "trap_explosion.ogg".to_string(), volume: 1.0, pitch: 1.0 }
ScreenShake { intensity: 8.0, duration: Timer::from_seconds(0.5, TimerMode::Once) }

// Upgrade Components
Upgrade 1:
- AreaDamage { radius: 3.0, damage: 200.0, falloff: 0.0 }
- PlaceObject { object_type: "enhanced_trap".to_string(), duration: Some(Timer::from_seconds(60.0, TimerMode::Once)), health: Some(1.0) }

Upgrade 2:
- ExpandingArea { initial_radius: 1.0, final_radius: 2.0, expansion_rate: 10.0 }  // Proximity trigger
- ChainTarget { max_chains: 3, current_chains: 0, chain_damage_falloff: 0.0 }  // Chain explosions

Upgrade 3:
- PeriodicEffect { interval: Timer::from_seconds(5.0, TimerMode::Repeating), effect_type: "place_trap".to_string(), remaining_ticks: None }
- ResourceGeneration { mana_per_second: 0.0, health_per_second: 0.0 }  // Self-replacing
- DamageBonus { flat_bonus: 0.0, multiplier: 1.5, duration: Timer::from_seconds(8.0, TimerMode::Once) }  // Debuff on explosion
```

### High-Level Implementation Plan

1. **Trap Placement System**
   ```rust
   fn trap_placement_system(
       mut commands: Commands,
       input: Res<ButtonInput<KeyCode>>,
       mut hunters: Query<(Entity, &Transform, &mut Cooldown), With<TrapAbility>>,
       recording: Res<RecordingState>,
   ) {
       if input.just_pressed(KeyCode::Digit4) {
           for (entity, transform, mut cooldown) in hunters.iter_mut() {
               if cooldown.0.finished() {
                   let grid_pos = world_to_grid(transform.translation.truncate());
                   
                   // Spawn trap at hunter position
                   commands.spawn((
                       ExplosiveTrap,
                       Transform::from_translation(transform.translation),
                       Duration(Timer::from_seconds(60.0, TimerMode::Once)),
                       AreaOfEffect(2.0),
                       ExplodeOnImpact { explosion_radius: 2.0, explosion_damage: 150.0 },
                       IsStealthed,  // Hidden from enemies
                       RecordableAction {
                           action_type: "place_trap".to_string(),
                           timestamp: recording.current_time,
                           position: transform.translation,
                           parameters: HashMap::from([
                               ("grid_x".to_string(), grid_pos.x as f32),
                               ("grid_y".to_string(), grid_pos.y as f32),
                           ]),
                       },
                   ));
                   
                   cooldown.0.reset();
               }
           }
       }
   }
   ```

2. **Enemy Proximity Detection System**
   ```rust
   fn trap_trigger_system(
       mut commands: Commands,
       traps: Query<(Entity, &Transform, &AreaOfEffect, &ExplodeOnImpact), With<ExplosiveTrap>>,
       enemies: Query<(Entity, &Transform), With<Enemy>>,
       mut explosion_events: EventWriter<TrapExplosionEvent>,
   ) {
       for (trap_entity, trap_transform, aoe, explode) in traps.iter() {
           for (enemy_entity, enemy_transform) in enemies.iter() {
               let distance = trap_transform.translation.distance(enemy_transform.translation);
               
               // Trigger when enemy steps on trap tile
               if distance < TILE_SIZE * 0.5 {
                   explosion_events.write(TrapExplosionEvent {
                       trap_entity,
                       explosion_center: trap_transform.translation,
                       radius: explode.explosion_radius,
                       damage: explode.explosion_damage,
                       triggering_enemy: enemy_entity,
                   });
                   break;
               }
           }
       }
   }
   ```

3. **Explosion Handling System**
   ```rust
   fn trap_explosion_system(
       mut commands: Commands,
       mut explosion_events: EventReader<TrapExplosionEvent>,
       enemies: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
       mut damage_events: EventWriter<DamageEvent>,
   ) {
       for event in explosion_events.read() {
           // Remove the trap
           commands.entity(event.trap_entity).despawn_recursive();
           
           // Find all enemies in explosion radius
           for (enemy_entity, enemy_transform, mut health) in enemies.iter() {
               let distance = event.explosion_center.distance(enemy_transform.translation);
               
               if distance <= event.radius * TILE_SIZE {
                   damage_events.write(DamageEvent {
                       target: enemy_entity,
                       amount: event.damage,
                       damage_type: DamageType::Explosive,
                   });
               }
           }
           
           // Spawn explosion effects
           commands.spawn((
               VisualEffect {
                   effect_type: "trap_explosion".to_string(),
                   scale: 2.0,
                   color: Color::srgb(1.0, 0.3, 0.0),
                   duration: Timer::from_seconds(1.0, TimerMode::Once),
               },
               Transform::from_translation(event.explosion_center),
           ));
       }
   }
   ```

4. **Trap Lifetime Management System**
   ```rust
   fn trap_lifetime_system(
       mut commands: Commands,
       time: Res<Time>,
       mut traps: Query<(Entity, &mut Duration), With<ExplosiveTrap>>,
   ) {
       for (entity, mut duration) in traps.iter_mut() {
           duration.0.tick(time.delta());
           
           if duration.0.finished() {
               commands.entity(entity).despawn_recursive();
           }
       }
   }
   ```

5. **Stealth Visibility System**
   ```rust
   fn trap_visibility_system(
       traps: Query<(Entity, &Transform), (With<ExplosiveTrap>, With<IsStealthed>)>,
       allies: Query<&Transform, With<Player>>,
       mut visibility_events: EventWriter<ShowTrapToAllyEvent>,
   ) {
       for (trap_entity, trap_transform) in traps.iter() {
           for ally_transform in allies.iter() {
               let distance = trap_transform.translation.distance(ally_transform.translation);
               
               // Show trap indicator to nearby allies
               if distance < VIEW_DISTANCE {
                   visibility_events.write(ShowTrapToAllyEvent {
                       trap_entity,
                       ally_can_see: true,
                   });
               }
           }
       }
   }
   ```

6. **Recording Integration**
   - Trap placement is deterministic based on hunter position
   - Trigger detection uses exact grid coordinates
   - Explosion timing and area calculations are frame-perfect
   - All damage is applied deterministically to grid-based positions

## Step-by-Step Gameplay

### Phase 1: Strategic Placement (Tap Activation)
- **Input Method**: Tap to instantly place trap at Hunter's current grid position
- **Immediate Deployment**: Trap appears instantly without cast time or animation delay
- **Position Validation**: Trap placed on Hunter's exact tile location at activation moment
- **Concealment Activation**: Trap becomes invisible to enemies but visible to allies

### Phase 2: Armed Monitoring (Until Triggered or Expired)
- **Enemy Detection**: System continuously monitors for enemy movement onto trap tile
- **Trigger Conditions**: Any enemy unit moving onto or through trap tile activates explosion
- **Visual Status**: Allied units can see subtle trap indicator for tactical awareness
- **Duration Tracking**: Trap remains active for maximum 60 seconds before auto-expiration

### Phase 3: Explosion Trigger (Enemy Contact)
- **Instant Activation**: Trap explodes immediately when enemy enters trigger area
- **Area Damage**: All enemies within 2x2 grid centered on trap take 150 damage
- **Visual Explosion**: Dramatic explosion effect with debris and fire particles
- **Audio Impact**: Powerful explosion sound with satisfying tactical feedback

### Phase 4: Post-Explosion Cleanup (Immediate Resolution)
- **Trap Removal**: Triggered trap disappears completely from battlefield
- **Damage Application**: All affected enemies receive damage simultaneously
- **Cooldown Reset**: 3-second cooldown begins for next trap placement
- **Area Clearance**: Explosion area returns to normal terrain after visual effects fade

## Trap Mechanics and Interactions

### Trigger Conditions
```rust
fn check_trap_trigger(trap: &PlacedTrap, enemy_positions: &[Vec2]) -> bool {
    let trap_pos = grid_to_world(trap.position);
    
    for enemy_pos in enemy_positions {
        let distance = Vec2::distance(trap_pos, *enemy_pos);
        if distance < 0.5 {  // Enemy must be very close to trap center
            return true;
        }
    }
    
    false
}

fn trigger_trap_explosion(trap: &PlacedTrap) -> Vec<Entity> {
    let explosion_area = GridArea::new(2, 2).centered_on(trap.position);
    let affected_enemies = get_enemies_in_area(explosion_area);
    
    for enemy in &affected_enemies {
        apply_damage(*enemy, trap.explosion_damage);
    }
    
    spawn_explosion_effects(trap.position, explosion_area);
    despawn_trap(trap);
    
    affected_enemies
}
```

### Concealment and Visibility
- **Enemy Invisibility**: Enemies cannot see traps and will walk through normally
- **Ally Awareness**: Friendly units see subtle visual indicators of trap locations
- **Placement Feedback**: Hunter receives clear confirmation of successful trap placement
- **Tactical Communication**: Trap positions visible to team for coordination

## Strategic Applications

### Predictive Placement
- **Chokepoint Control**: Place traps in narrow passages enemies must use
- **Retreat Routes**: Block enemy escape paths with explosive surprises
- **Formation Breaking**: Force enemies to spread out or take significant damage
- **Resource Denial**: Protect valuable areas or objectives with trap coverage

### Timing and Pattern Recognition
- **Enemy Pathing**: Study enemy movement patterns to predict optimal trap locations
- **Combat Flow**: Place traps during lull periods for future engagement phases
- **Team Coordination**: Coordinate with allies to drive enemies into trap positions
- **Layered Defense**: Create multiple trap zones for comprehensive area denial

## Upgrade Paths

### Tier 1: Enhanced Explosives
- **Damage Increase**: 150 → 200 damage per explosion
- **Area Expansion**: 2x2 → 3x3 explosion area coverage
- **Multiple Charges**: Can maintain up to 2 active traps simultaneously
- **Strategic Value**: Higher damage and larger area control with increased tactical flexibility

### Tier 2: Advanced Triggers
- **Proximity Detonation**: Traps trigger when enemies come within 1 tile (not just contact)
- **Chain Explosions**: Trap explosions can trigger other nearby traps
- **Smart Targeting**: Traps only trigger for enemy units, ignoring allied movement
- **Tactical Evolution**: More reliable triggering with cascading explosion potential

### Tier 3: Master Trapper
- **Auto-Placement**: Traps automatically place at Hunter's location every 5 seconds
- **Persistent Field**: Destroyed traps respawn at same location after 10 seconds
- **Damage Amplification**: Enemies damaged by traps take 50% more damage for 8 seconds
- **Ultimate Control**: Continuous area denial with battlefield-wide impact

## Team Coordination and Communication

### Allied Awareness
- **Visual Indicators**: Clear but subtle trap markers visible only to team
- **Positioning Coordination**: Team can use trap locations for tactical planning
- **Enemy Herding**: Allies can drive enemies toward trap positions
- **Safety Protocols**: Team awareness prevents accidental trap triggering strategies

### Tactical Integration
- **Tank Positioning**: Use traps to protect flanks while tank holds front
- **Healer Support**: Create safe zones around support characters with trap coverage
- **Damage Dealer Setup**: Use trap explosions to soften enemies for finishing attacks
- **Formation Control**: Channel enemy movement through trap-controlled corridors

## Visual & Audio Design

### Trap Placement
- **Visual**: Hunter quickly deploys small device with subtle ground disturbance
- **Animation**: Swift, practiced movement showing Hunter's trap expertise
- **Audio**: Quiet mechanical click indicating successful deployment
- **Ground Effect**: Minimal visual disturbance that enemies cannot detect

### Armed Status (Ally Vision)
- **Visual**: Subtle red outline or glow visible only to allied units
- **UI**: Small trap icon appears in tactical overlay for team awareness
- **Audio**: Very quiet beeping that only allies can hear
- **Persistence**: Visual indicator remains throughout trap's 60-second lifetime

### Explosion Trigger
- **Visual**: Dramatic explosion with fire, smoke, and debris effects
- **Animation**: 2x2 area explosion with realistic blast radius visualization
- **Audio**: Powerful explosion sound with satisfying tactical impact
- **Screen Effect**: Brief screen shake proportional to distance from explosion

### Tactical Feedback
- **Visual**: Damage numbers appear for all affected enemies
- **UI**: Brief notification shows successful trap trigger and damage dealt
- **Audio**: Satisfying confirmation sound for successful tactical prediction
- **Aftermath**: Scorch marks and debris briefly mark explosion site before fading