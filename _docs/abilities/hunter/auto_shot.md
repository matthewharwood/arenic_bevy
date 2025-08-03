# Auto Shot

A complete implementation guide for the Hunter's automated targeting ranged ability.

## Overview

The **Auto Shot** ability represents the Hunter's mastery over consistent ranged combat through automated target acquisition and firing. This passive-aggressive ability automatically fires projectiles at the closest enemy on a regular timer, providing steady damage output without requiring active player input. The ability emphasizes positioning and target prioritization while offering reliable damage contribution during complex combat scenarios.

## Game Design Philosophy

This ability demonstrates automation design that enhances rather than replaces player skill:

**Consistent Pressure**: The automatic firing creates steady battlefield pressure that rewards good positioning over precise timing, allowing players to focus on tactical decisions.

**Proximity-Based Targeting**: The closest-enemy selection creates interesting positioning puzzles where Hunter placement affects target prioritization automatically.

**Non-Disruptive Integration**: The ability operates independently without interfering with other abilities, allowing complex ability combinations and tactical planning.

## Implementation Architecture

### Component-Based Design

```rust
AutoShot {
    fire_interval: 2.5,                 // Fires every 2.5 seconds
    projectile_speed: 10.0,             // 10 tiles per second travel speed
    damage: 75.0,                       // 75 damage per auto shot
    range: 8.0,                         // 8 tile maximum targeting range
    target_selection: ClosestEnemy,     // Automatically targets nearest foe
    projectile_type: Standard,          // Normal projectile without special effects
}

AutoShotSystem {
    last_fire_time: f32,
    current_target: Option<Entity>,
    fire_timer: f32,
    active_projectiles: Vec<Entity>,
    targeting_range: f32,
}

AutoProjectile {
    position: Vec2,
    velocity: Vec2,
    target: Entity,
    damage: f32,
    visual_effect: Entity,
    lifetime: f32,
}
```

### Event-Driven Systems

The ability operates through five automated systems:
1. **Target Acquisition** - Continuously scans for closest enemies within range
2. **Timer Management** - Tracks 2.5-second intervals for automatic firing
3. **Projectile Creation** - Spawns and directs auto shots toward selected targets
4. **Collision Detection** - Handles projectile impacts and damage application
5. **Visual Coordination** - Manages automatic firing effects and projectile trails

### Required Components

```rust
// Core Components
Damage(75.0)
ProjectileSpeed(10.0)
AttackRange(8.0)
TargetNearest
PeriodicEffect { 
    interval: Timer::from_seconds(2.5, TimerMode::Repeating), 
    effect_type: "fire_projectile".to_string(), 
    remaining_ticks: None  // Infinite 
}

// Projectile Components
ProjectileTarget(enemy_position)
ProjectileLifetime(Timer::from_seconds(3.0, TimerMode::Once))
EnemyOnly
RequiresLOS

// Visual Components
AudioEffect { sound_file: "auto_shot.ogg".to_string(), volume: 0.7, pitch: 1.0 }
VisualEffect { effect_type: "auto_shot_trail".to_string(), scale: 0.8, color: Color::srgb(0.2, 0.4, 1.0), duration: Timer::from_seconds(0.5, TimerMode::Once) }

// Upgrade Components
Upgrade 1: 
- PeriodicEffect { interval: Timer::from_seconds(2.0, TimerMode::Repeating), effect_type: "fire_projectile".to_string(), remaining_ticks: None }
- Damage(100.0)
- AttackRange(10.0)

Upgrade 2:
- TargetLowestHealth
- Multishot(2)
- PierceCount(1)

Upgrade 3:
- PeriodicEffect { interval: Timer::from_seconds(3.0, TimerMode::Repeating), effect_type: "burst_fire".to_string(), remaining_ticks: None }
- Multishot(3)
- ExplodeOnImpact { explosion_radius: 1.0, explosion_damage: 30.0 }
```

### High-Level Implementation Plan

1. **Spawn System Setup**
   - Create `AutoShotAbility` component bundle with required components
   - Attach to Hunter entity when ability is learned
   - Initialize targeting system with `TargetNearest` and `AttackRange` components
   - Set up `PeriodicEffect` timer for automated firing

2. **Target Acquisition System**
   ```rust
   fn auto_shot_targeting_system(
       mut commands: Commands,
       time: Res<Time>,
       hunters: Query<(Entity, &GridPosition, &AttackRange, &TargetNearest), 
                      (With<Hunter>, With<AutoShotAbility>, With<Active>, Without<Dead>)>,
       bosses: Query<(Entity, &GridPosition, &Health), 
                     (With<Boss>, With<BossActive>, Without<Dead>)>,
       mini_bosses: Query<(Entity, &GridPosition, &Health), 
                          (With<MiniBoss>, With<BossActive>, Without<Dead>)>,
       current_arena: Res<CurrentArena>,
       mut targeting_events: EventWriter<TargetAcquiredEvent>,
   ) {
       for (hunter_entity, hunter_pos, range, _) in hunters.iter() {
           // Find closest boss or mini-boss within range in current arena
           let target = find_closest_boss_in_arena(hunter_pos, range, &bosses, &current_arena)
               .or_else(|| find_closest_mini_boss_in_arena(hunter_pos, range, &mini_bosses, &current_arena));
               
           if let Some((target_entity, target_pos)) = target {
               targeting_events.send(TargetAcquiredEvent {
                   caster: hunter_entity,
                   target: target_entity,
                   position: *target_pos,
               });
           }
       }
   }
   ```

3. **Automated Firing System**
   ```rust
   fn auto_shot_fire_system(
       mut commands: Commands,
       time: Res<Time>,
       mut ability_query: Query<(&mut PeriodicEffect, &Damage, &ProjectileSpeed), With<AutoShotAbility>>,
       mut target_events: EventReader<TargetAcquiredEvent>,
       recording: Res<RecordingState>,
   ) {
       for event in target_events.read() {
           if let Ok((mut periodic, damage, speed)) = ability_query.get_mut(event.ability_entity) {
               periodic.interval.tick(time.delta());
               
               if periodic.interval.just_finished() {
                   // Spawn projectile with recording-friendly deterministic components
                   commands.spawn((
                       ProjectileTarget(event.target_position),
                       ProjectileSpeed(speed.0),
                       Damage(damage.0),
                       ProjectileLifetime(Timer::from_seconds(3.0, TimerMode::Once)),
                       RecordableAction {
                           action_type: "auto_shot_fire".to_string(),
                           timestamp: recording.current_time,
                           position: event.hunter_position,
                           parameters: HashMap::from([
                               ("damage".to_string(), damage.0),
                               ("target_id".to_string(), event.target_entity.index() as f32),
                           ]),
                       },
                   ));
               }
           }
       }
   }
   ```

4. **Projectile Movement System**
   ```rust
   fn auto_shot_projectile_system(
       mut commands: Commands,
       time: Res<Time>,
       mut projectiles: Query<(Entity, &mut Transform, &ProjectileSpeed, &ProjectileTarget, &mut ProjectileLifetime)>,
   ) {
       for (entity, mut transform, speed, target, mut lifetime) in projectiles.iter_mut() {
           // Grid-based movement calculation
           let direction = (*target.0 - transform.translation).normalize();
           let velocity = direction * speed.0 * TILE_SIZE;
           transform.translation += velocity * time.delta_seconds();
           
           lifetime.0.tick(time.delta());
           if lifetime.0.finished() {
               commands.entity(entity).despawn_recursive();
           }
       }
   }
   ```

5. **Damage Application System**
   ```rust
   fn auto_shot_damage_system(
       mut commands: Commands,
       projectiles: Query<(Entity, &Transform, &Damage), With<AutoShotProjectile>>,
       mut enemies: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
       mut damage_events: EventWriter<DamageEvent>,
   ) {
       for (projectile_entity, projectile_transform, damage) in projectiles.iter() {
           for (enemy_entity, enemy_transform, mut health) in enemies.iter_mut() {
               if is_collision(projectile_transform, enemy_transform) {
                   damage_events.write(DamageEvent {
                       target: enemy_entity,
                       amount: damage.0,
                       damage_type: DamageType::Physical,
                   });
                   commands.entity(projectile_entity).despawn_recursive();
               }
           }
       }
   }
   ```

6. **Recording Integration**
   - All projectile spawns include `RecordableAction` component
   - Target acquisition events are deterministic based on grid positions
   - Timer-based firing ensures consistent replay behavior
   - No random elements in targeting or damage calculation

## Step-by-Step Gameplay

### Phase 1: Continuous Target Scanning (Passive Operation)
- **Range Monitoring**: System continuously scans 8-tile radius around Hunter
- **Enemy Detection**: Identifies all hostile targets within effective range
- **Proximity Calculation**: Determines closest enemy based on straight-line distance
- **Target Locking**: Maintains focus on selected target until out of range or defeated

### Phase 2: Automatic Fire Timing (2.5 Second Intervals)
- **Timer Progression**: Internal timer counts down from 2.5 seconds to 0
- **Target Validation**: Confirms selected enemy still within range and alive
- **Fire Decision**: Automatically triggers projectile creation when timer expires
- **Timer Reset**: Immediately restarts 2.5-second countdown for next shot

### Phase 3: Projectile Generation and Launch (Instant Creation)
- **Automatic Aiming**: Projectile aims directly at current target position
- **Launch Physics**: Auto shot travels at 10 tiles/second toward target
- **Visual Creation**: Standard projectile with distinctive auto shot effects
- **Audio Feedback**: Subtle automatic firing sound distinct from manual shots

### Phase 4: Impact and Damage Resolution (On Contact)
- **Collision Detection**: Projectile hits target or travels maximum distance
- **Damage Application**: Target receives 75 damage upon successful contact
- **Visual Impact**: Standard impact effects with auto shot identification
- **Target Continuation**: System immediately begins scanning for next target

## Target Selection Algorithm

### Closest Enemy Calculation
```rust
fn find_closest_enemy(hunter_pos: Vec2, max_range: f32) -> Option<Entity> {
    let enemies_in_range: Vec<(Entity, f32)> = get_all_enemies()
        .iter()
        .filter_map(|enemy| {
            let distance = Vec2::distance(hunter_pos, get_position(*enemy));
            if distance <= max_range && is_alive(*enemy) && has_line_of_sight(hunter_pos, get_position(*enemy)) {
                Some((*enemy, distance))
            } else {
                None
            }
        })
        .collect();
    
    enemies_in_range
        .iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(entity, _)| *entity)
}

fn has_line_of_sight(from: Vec2, to: Vec2) -> bool {
    // Raycast between positions to check for obstacles
    !raycast_obstacles(from, to).any(|obstacle| blocks_projectiles(obstacle))
}
```

### Target Priority Factors
- **Distance Priority**: Closest enemy always receives targeting preference
- **Line of Sight**: Auto shot cannot target enemies behind solid obstacles
- **Alive Status**: Dead or incapacitated enemies excluded from targeting
- **Range Boundary**: Enemies outside 8-tile range ignored regardless of other factors

## Positioning Strategy and Optimization

### Optimal Hunter Placement
- **Central Coverage**: Position to maximize 8-tile range coverage of enemy positions
- **High Priority Targets**: Move closer to important enemies to ensure they become closest target
- **Line of Sight**: Maintain clear view of battlefield to avoid targeting obstruction
- **Safety Distance**: Balance range optimization with personal safety from enemy attacks

### Target Manipulation Techniques
- **Proximity Control**: Adjust Hunter position to influence which enemy is targeted
- **Formation Awareness**: Understand enemy positioning to predict auto shot targets
- **Range Management**: Use 8-tile range limit strategically to exclude unwanted targets
- **Coordination**: Work with team to position enemies for optimal auto shot effectiveness

## Upgrade Paths

### Tier 1: Improved Automation
- **Fire Rate**: 2.5 seconds → 2.0 seconds between shots
- **Damage Increase**: 75 → 100 damage per auto shot
- **Range Extension**: 8 tiles → 10 tiles maximum targeting range
- **Strategic Value**: Higher DPS output with improved target acquisition capabilities

### Tier 2: Smart Targeting
- **Priority Targeting**: Preferentially targets low-health enemies for finishing shots
- **Multi-Target**: Can maintain and fire at up to 2 different enemies simultaneously
- **Piercing Shots**: Auto shots can pass through first target to hit secondary enemies
- **Tactical Evolution**: Transforms from simple automation to intelligent combat assistance

### Tier 3: Barrage Protocol
- **Burst Fire**: Fires 3 shots in rapid succession every 3 seconds instead of single shots
- **Explosive Ammunition**: Auto shots create small area explosions on impact
- **Target Painting**: Manual abilities mark targets for increased auto shot damage
- **Ultimate Automation**: High-intensity automated combat with devastating effectiveness

## Integration with Other Abilities

### Ability Synergy
- **Manual Shot Coordination**: Auto shots continue during manual ability usage
- **Positioning Abilities**: Movement abilities help optimize auto shot targeting
- **Trap Synergy**: Auto shots can drive enemies into manually placed traps
- **Team Coordination**: Auto shot damage supplements team DPS during complex encounters

### Combat Flow Integration
- **Background Damage**: Provides consistent damage while player focuses on other abilities
- **Tactical Freedom**: Automation allows attention to positioning and team coordination
- **Resource Independence**: No mana or cooldown conflicts with other Hunter abilities
- **Seamless Operation**: Functions transparently without disrupting combat rhythm

## Visual & Audio Design

### Lighting Design Philosophy
The Hunter's Auto Shot employs **precision-focused lighting** that emphasizes mechanical reliability and technological mastery. The color palette centers on **cool blues and steel grays** to convey automated efficiency while distinguishing from manual abilities.

**Technical Implementation:**
- **Key Light**: Sharp, directional lighting from weapon position (90-degree angle)
- **Fill Light**: Subtle ambient bounce lighting to maintain form definition
- **Rim Light**: Blue-tinted backlight creates technological silhouette separation
- **Color Temperature**: 4000K-5500K range for cold, mechanical feeling
- **PBR Materials**: High metallic values (0.8-0.9) with low roughness (0.1-0.3) for weapon components

### Automatic Target Acquisition

**Lighting Design:**
- **Target Illumination**: Soft blue rim light (3-unit intensity) around targeted enemy
- **Range Indicator**: Subtle volumetric fog cone showing 8-tile acquisition radius
- **Weapon Glow**: Emissive blue channels in weapon material (RGB: 0.2, 0.4, 0.8, Intensity: 2.0)
- **Performance**: Static lightmap for range indicator, dynamic point light for target highlight

**Visual Effects:**
- **Targeting Reticle**: Minimalist blue HUD element with 50% transparency
- **UI Elements**: Monospace font with slight blue glow for technical aesthetic
- **Particle System**: Sparse blue motes (5-10 particles) around weapon scope
- **Shader Optimization**: UI elements use additive blending, no real-time shadows

**Audio Design:**
- **Target Lock Sound**: Subtle mechanical beep (440Hz, 0.1s duration, 0.3 volume)
- **Acquisition Loop**: Low-frequency hum (80Hz) with slight modulation
- **Spatial Audio**: Point source at weapon position, 8-unit falloff radius
- **Performance**: Cached audio clips, maximum 2 concurrent instances

### Automated Firing

**Lighting Design:**
- **Muzzle Flash**: Brief blue-white strobe (Intensity: 20, Duration: 0.05s)
- **Illumination Radius**: 4-unit sphere, affects nearby surfaces only
- **Color Ramp**: Blue-white center (RGB: 0.8, 0.9, 1.0) to blue edge (RGB: 0.2, 0.4, 0.8)
- **Shadow Casting**: Enabled only for muzzle flash, disabled for projectile trail
- **Performance**: Pre-calculated light cookies for consistent muzzle flash pattern

**Visual Effects:**
- **Animation**: 3-frame firing sequence (0.1s total) with mechanical precision
- **Particle Burst**: 15-20 blue sparks, velocity range 5-10 units/second
- **Projectile Material**: Emissive core with blue trail shader
- **Trail Optimization**: Ribbon trail renderer, maximum 1-second lifetime

**Audio Design:**
- **Fire Sound**: Crisp mechanical snap (frequency range: 2-8kHz)
- **Layer Stack**: Base impact + mechanical click + subtle reverb tail
- **Volume Scaling**: 0.7 base volume, scales with distance (inverse square law)
- **Performance**: Pre-mixed stereo samples, no real-time processing

### Projectile Flight and Impact

**Lighting Design:**
- **Projectile Light**: Small moving point light (Intensity: 1.5, Range: 2 units)
- **Trail Illumination**: Subtle light emission along trajectory path
- **Color Consistency**: Maintains blue theme throughout flight
- **Performance**: Single dynamic light per projectile, culled beyond camera frustum

**Visual Effects:**
- **Trail Renderer**: Blue gradient ribbon (Width: 0.1-0.05 units, Alpha: 1.0-0.0)
- **Speed Lines**: Subtle motion blur effect for projectile core
- **Particle Trail**: 3-5 particles per frame, 0.3-second lifetime
- **Impact Flash**: Blue burst effect (20 particles, 0.2-second duration)

**Audio Design:**
- **Flight Audio**: High-frequency whistle (4-6kHz) with Doppler effect
- **3D Positioning**: Audio source follows projectile with accurate spatial tracking
- **Impact Sound**: Sharp crack (broadband noise) + blue-tinted reverb
- **Performance**: Optimized 3D audio with occlusion detection

### System Status Indicators

**Lighting Design:**
- **Weapon Readiness**: Subtle blue pulse (0.5-second cycle) on weapon components
- **Charge Indicator**: Graduated blue glow intensity based on timer state
- **HUD Backlighting**: Soft blue key light behind UI elements
- **Performance**: Animated material parameters, no additional light sources required

**Visual Effects:**
- **Status Icons**: Clean geometric designs with blue accent lighting
- **Timer Visualization**: Circular progress indicator with blue fill
- **Ready State**: Brief blue flash when auto shot becomes available
- **Accessibility**: High contrast mode available with yellow/blue alternatives

**Audio Design:**
- **Ready Signal**: Subtle two-tone chime (E4-A4 notes, 0.2s duration)
- **Timer Audio**: Optional metronome click for precision timing
- **Ambient Hum**: 60Hz bass frequency indicating active system
- **Performance**: Cached notification sounds, triggered by component state changes

### Performance Optimization for Mass Combat

**Lighting Optimization:**
- **LOD System**: 3 quality levels based on distance from camera
  - **High (0-20 units)**: Full dynamic lighting and shadows
  - **Medium (20-50 units)**: Dynamic lighting, no shadows
  - **Low (50+ units)**: Static baked lighting only
- **Culling**: Frustum culling for all dynamic lights
- **Batching**: Grouped light rendering for similar auto shot effects

**Visual Effects Optimization:**
- **Particle Pooling**: Pre-allocated particle systems (100 projectile trails max)
- **Texture Atlasing**: All Hunter effects use single 2048x2048 texture atlas
- **Shader Variants**: Simplified shaders for distant or off-screen effects
- **Distance Culling**: Effects disabled beyond 75-unit camera distance

**Audio Optimization:**
- **Voice Limiting**: Maximum 8 concurrent auto shot audio sources
- **Audio LOD**: Simplified audio beyond 30-unit range
- **Compression**: OGG Vorbis format for all audio assets
- **Streaming**: Large ambient tracks use compressed streaming

### Deterministic Recording Compatibility

**Visual Synchronization:**
- **Frame-Perfect Timing**: All visual effects triggered by component state, not time
- **Deterministic Randomness**: Seeded RNG for particle variations
- **State-Based Animation**: Animation triggers tied to ability component updates
- **Replay Consistency**: Visual effects identical across all replay instances

**Audio Synchronization:**
- **Event-Driven Audio**: Audio cues triggered by gameplay events, not time
- **Position Accuracy**: Audio source positions updated via deterministic transforms
- **Timing Precision**: Audio timing based on simulation steps, not wall clock
- **Replay Fidelity**: Identical audio playback across all replay sessions

### Accessibility Considerations

**Visual Accessibility:**
- **Colorblind Support**: Blue effects have brightness contrast alternatives
- **High Contrast Mode**: Optional yellow-on-black UI elements
- **Motion Sensitivity**: Reduced particle effects option available
- **Text Scaling**: UI elements support 125%-200% scaling

**Audio Accessibility:**
- **Hearing Impaired**: Visual indicators for all audio cues
- **Audio Description**: Optional narration for complex visual effects
- **Frequency Options**: Alternative high/low frequency audio cues
- **Subtitle Support**: Text descriptions for spatial audio information

### Hunter Class Visual Identity

The Auto Shot ability establishes the Hunter's **technological precision** visual language:
- **Primary Colors**: Cool blues (RGB: 0.2, 0.4, 0.8) and steel grays (RGB: 0.6, 0.6, 0.7)
- **Secondary Accents**: Bright cyan (RGB: 0.0, 0.8, 1.0) for active states
- **Material Palette**: High metallic, low roughness surfaces with subtle wear patterns
- **Lighting Character**: Sharp, directional lighting emphasizing precision and focus
- **Animation Style**: Mechanical, precise movements with consistent timing