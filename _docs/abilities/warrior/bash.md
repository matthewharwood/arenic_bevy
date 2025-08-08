# Bash

A complete implementation guide for the Warrior's offensive shield strike and damage mitigation ability.

## Overview

The **Bash** ability demonstrates the Warrior's mastery over offensive defense through a powerful shield strike that combines immediate damage with damage reduction effects. When activated, the Warrior performs an instant melee attack that deals significant damage while reducing the target's next attack damage output. This hybrid offensive-defensive ability maintains enemy timing schedules while providing both immediate tactical advantage and ongoing damage mitigation.

## Game Design Philosophy

This ability showcases hybrid design through combined offensive and defensive mechanics:

**Offense-Defense Integration**: The ability provides immediate damage output while creating future defensive value through enemy damage reduction.

**Predictable Enemy Interaction**: Enemy attack schedules remain unchanged, but damage output is reduced, maintaining deterministic gameplay while providing defensive benefit.

**Instant Tactical Value**: The immediate cast provides reliable combat response without complex timing or positioning requirements.

## Implementation Architecture

### Component-Based Design (Single-Use Pattern)

```rust
// Bash ability entity composition
commands.spawn((
    BashAbility,                        // Marker
    Instant,                            // No cast time
    Damage(150.0),                      // Base damage
    Range(1.5 * TILE_SIZE),             // Melee range
    DamageReduction(0.3),               // 30% damage reduction debuff
    Duration(8.0),                      // Debuff duration
    Cooldown(6.0),                      // Ability cooldown
    TargetNearest,                      // Auto-target nearest enemy
    TargetEnemy,                        // Can only target enemies
    RequiresLineOfSight,                // Needs clear path
));

// Bash strike effect entity (spawned on activation)
commands.spawn((
    BashStrike,                         // Marker for strike effect
    Origin(warrior_pos),                // Attacker position
    TargetEntity(enemy),                // Target entity
    Damage(150.0),                      // Damage to apply
    DamageReduction(0.3),               // Debuff to apply
    Duration(0.1),                      // Strike duration
    ElapsedTime(0.0),                   // Timer
    ScreenShakeIntensity(3.0),          // Camera shake
    ScreenShakeDuration(0.3),           // Shake duration
));

// Damage reduction debuff entity (applied to target)
commands.entity(target).insert((
    Debuff,                             // Debuff marker
    DamageReduction(0.3),               // 30% reduction
    Duration(8.0),                      // Maximum duration
    ElapsedTime(0.0),                   // Timer
    DebuffVfx,                          // Visual indicator
));
```

### Event-Driven Systems

The ability operates through five combat systems:
1. **Melee Strike** - Handles instant damage application and range validation
2. **Debuff Application** - Applies damage reduction effect to struck enemy
3. **Damage Modification** - Reduces enemy attack damage while debuff is active
4. **Duration Management** - Tracks debuff duration and removal conditions
5. **Visual Coordination** - Manages bash impact effects and debuff indicators

### Required Single-Use Components

```rust
// Core Components (composed at spawn)
BashAbility                 // Marker
Instant                     // No cast time marker
Damage(150.0)               // Base damage value
Range(1.5 * TILE_SIZE)      // Attack range
Cooldown(6.0)               // Cooldown in seconds
TargetNearest               // Targeting behavior marker
TargetEnemy                 // Valid target marker
RequiresLineOfSight         // LOS requirement marker

// Strike Effect Components
BashStrike                  // Strike effect marker
Origin(Vec3)                // Attacker position
TargetEntity(Entity)        // Target entity reference
DamageReduction(0.3)        // Debuff percentage
Duration(0.1)               // Effect duration
ElapsedTime(0.0)            // Timer tracking

// Debuff Components (applied to target)
Debuff                      // Debuff marker
DamageReduction(0.3)        // 30% reduction value
Duration(8.0)               // Max duration in seconds
ElapsedTime(0.0)            // Timer tracking
DebuffVfx                   // Visual effect marker

// Visual/Audio Components
ScreenShakeIntensity(3.0)   // Shake strength
ScreenShakeDuration(0.3)    // Shake duration
FlashIntensity(20.0)        // Impact flash brightness
FlashDuration(0.1)          // Flash duration
SoundVolume(0.9)            // Audio volume
AudioHandle(audio.bash)     // Sound asset

// Upgrade 1 Components
Damage(200.0)               // Increased damage
DamageReduction(0.4)        // Stronger debuff
Duration(12.0)              // Longer debuff

// Upgrade 2 Components
Radius(2.0 * TILE_SIZE)     // AoE radius
AreaEffect                  // AoE marker
MovementReduction(0.25)     // Slow effect
Cooldown(4.0)               // Reduced cooldown

// Upgrade 3 Components
CritChance(0.5)             // 50% crit chance
CritMultiplier(2.0)         // Double damage on crit
Spreading                   // Debuff spreads on kill
ChainCount(2)               // Spreads to 2 enemies
ChainRange(3.0 * TILE_SIZE) // Chain range
```

### High-Level Implementation Plan

1. **Bash Activation System**
   ```rust
   fn bash_activation_system(
       mut commands: Commands,
       input: Res<ButtonInput<KeyCode>>,
       mut warriors: Query<(Entity, &Transform, &mut Cooldown), (With<BashAbility>, With<Shield>)>,
       enemies: Query<(Entity, &Transform), With<Enemy>>,
       recording: Res<RecordingState>,
   ) {
       if input.just_pressed(KeyCode::Digit1) {
           for (warrior_entity, warrior_transform, mut cooldown) in warriors.iter_mut() {
               if cooldown.0.finished() {
                   // Find nearest enemy within range
                   let target = enemies
                       .iter()
                       .filter(|(_, enemy_transform)| {
                           warrior_transform.translation.distance(enemy_transform.translation) <= 1.5 * TILE_SIZE
                       })
                       .min_by(|(_, a), (_, b)| {
                           let dist_a = warrior_transform.translation.distance(a.translation);
                           let dist_b = warrior_transform.translation.distance(b.translation);
                           dist_a.partial_cmp(&dist_b).unwrap()
                       });
                   
                   if let Some((target_entity, target_transform)) = target {
                       commands.spawn((
                           BashStrike,
                           Damage(150.0),
                           Transform::from_translation(warrior_transform.translation),
                           BashTarget(target_entity),
                           RecordableAction {
                               action_type: "bash".to_string(),
                               timestamp: recording.current_time,
                               position: warrior_transform.translation,
                               parameters: HashMap::from([
                                   ("target_id".to_string(), target_entity.index() as f32),
                               ]),
                           },
                       ));
                       
                       cooldown.0.reset();
                   }
               }
           }
       }
   }
   ```

2. **Bash Damage Application System**
   ```rust
   fn bash_damage_system(
       mut commands: Commands,
       bash_strikes: Query<(Entity, &Damage, &BashTarget), With<BashStrike>>,
       mut enemies: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
       mut damage_events: EventWriter<DamageEvent>,
   ) {
       for (strike_entity, damage, target) in bash_strikes.iter() {
           if let Ok((enemy_entity, enemy_transform, mut health)) = enemies.get_mut(target.0) {
               // Apply damage
               damage_events.write(DamageEvent {
                   target: enemy_entity,
                   amount: damage.0,
                   damage_type: DamageType::Physical,
               });
               
               // Apply bash debuff
               commands.entity(enemy_entity).insert(Curse {
                   damage_reduction: 0.3,
                   healing_reduction: 0.0,
                   duration: Timer::from_seconds(8.0, TimerMode::Once),
               });
               
               // Cleanup strike
               commands.entity(strike_entity).despawn_recursive();
           }
       }
   }
   ```

3. **Debuff Duration System**
   ```rust
   fn bash_debuff_system(
       mut commands: Commands,
       time: Res<Time>,
       mut debuffed: Query<(Entity, &mut Curse), With<Enemy>>,
   ) {
       for (entity, mut curse) in debuffed.iter_mut() {
           curse.duration.tick(time.delta());
           
           if curse.duration.finished() {
               commands.entity(entity).remove::<Curse>();
           }
       }
   }
   ```

4. **Damage Modification System**
   ```rust
   fn bash_damage_reduction_system(
       mut damage_events: EventReader<EnemyAttackEvent>,
       debuffed_enemies: Query<&Curse, With<Enemy>>,
       mut modified_damage_events: EventWriter<ModifiedDamageEvent>,
   ) {
       for event in damage_events.read() {
           let final_damage = if let Ok(curse) = debuffed_enemies.get(event.attacker) {
               // Reduce damage by debuff amount
               event.damage * (1.0 - curse.damage_reduction)
           } else {
               event.damage
           };
           
           modified_damage_events.write(ModifiedDamageEvent {
               original_event: *event,
               modified_damage: final_damage,
           });
       }
   }
   ```

5. **Shield Requirement Validation System**
   ```rust
   fn shield_requirement_system(
       mut commands: Commands,
       warriors: Query<(Entity, &Equipment), With<BashAbility>>,
   ) {
       for (entity, equipment) in warriors.iter() {
           let has_shield = equipment.items.iter().any(|item| item.item_type == ItemType::Shield);
           
           if has_shield {
               if !warriors.get::<Shield>(entity).is_ok() {
                   commands.entity(entity).insert(Shield);
               }
           } else {
               if warriors.get::<Shield>(entity).is_ok() {
                   commands.entity(entity).remove::<Shield>();
               }
           }
       }
   }
   ```

6. **Recording Integration**
   - Bash activation is deterministic based on nearest enemy selection
   - Debuff application timing is frame-perfect
   - Damage reduction calculations are consistent across replays
   - All visual and audio effects are tied to deterministic events

## Step-by-Step Gameplay

### Phase 1: Target Acquisition (Tap Activation)
- **Input Method**: Tap to execute instant bash attack on nearest enemy within range
- **Range Validation**: Must have enemy within 1.5-tile melee range
- **Shield Requirement**: Warrior must have shield equipped or active for bash execution
- **Instant Execution**: Attack triggers immediately without wind-up or delay

### Phase 2: Damage Application (Immediate Impact)
- **Melee Strike**: Target receives 150 base damage instantly upon activation
- **Impact Resolution**: Damage calculation includes armor and resistance modifiers
- **Visual Feedback**: Clear impact effects show successful bash strike
- **Audio Confirmation**: Satisfying shield impact sound confirms successful hit

### Phase 3: Debuff Application (Post-Impact Effect)
- **Damage Reduction**: Target's damage output reduced by 30% for next attack
- **Duration Timer**: Effect lasts maximum 8 seconds or until enemy performs attack
- **Visual Indicator**: Clear debuff marker shows which enemy has reduced damage
- **Status Application**: Enemy receives "weakened" status affecting damage calculations

### Phase 4: Damage Mitigation (Until Consumed or Expired)
- **Attack Modification**: Enemy's next attack deals 30% less damage
- **Single-Use Effect**: Debuff is consumed after first enemy attack or 8-second expiration
- **Damage Calculation**: System modifies enemy damage before applying to target
- **Effect Removal**: Debuff visual indicator disappears when effect is consumed

## Damage and Debuff Mechanics

### Damage Calculation
```rust
fn execute_bash_attack(warrior: Entity, target: Entity) -> BashResult {
    let base_damage = 150.0;
    let warrior_attack = get_attack_power(warrior);
    let target_defense = get_defense_rating(target);
    
    let final_damage = calculate_damage_with_modifiers(
        base_damage + warrior_attack,
        target_defense,
        DamageType::Physical
    );
    
    apply_damage(target, final_damage);
    apply_bash_debuff(target);
    
    BashResult {
        damage_dealt: final_damage,
        debuff_applied: true,
        target_entity: target,
    }
}

fn apply_bash_debuff(target: Entity) {
    let debuff = DamageReduction {
        affected_entity: target,
        reduction_percentage: 0.3,
        duration_remaining: 8.0,
        applies_to_next_attack: true,
        visual_indicator: spawn_debuff_indicator(target),
        source_ability: AbilityType::Bash,
    };
    
    add_status_effect(target, debuff);
}
```

### Enemy Damage Modification
- **Pre-Attack Calculation**: Enemy damage calculated with 30% reduction before application
- **Single Application**: Debuff affects only the next attack, then is consumed
- **Timing Independence**: Enemy attack timing unchanged, only damage output affected
- **Stacking Rules**: Multiple bash debuffs do not stack, newer application replaces older

## Strategic Applications

### Defensive Timing
- **Threat Mitigation**: Use bash before anticipated high-damage enemy attacks
- **Tank Protection**: Reduce incoming damage to maintain tank survivability
- **Team Defense**: Protect allies by weakening enemy damage output
- **Formation Support**: Enable more aggressive positioning through damage mitigation

### Offensive Integration
- **Damage Contribution**: 150 damage provides meaningful offensive contribution
- **Combo Potential**: Combine with other abilities for enhanced damage sequences
- **Target Priority**: Focus bash on most dangerous or high-damage enemies
- **Resource Efficiency**: Instant cast allows seamless integration with combat flow

## Timing and Tactical Usage

### Optimal Timing Windows
- **Pre-Emptive Strikes**: Bash enemies before they execute high-damage abilities
- **Formation Defense**: Use when enemies are positioned to threaten multiple allies
- **Tank Support**: Apply debuff to enemies targeting the main tank
- **Emergency Mitigation**: React to incoming threats with immediate damage reduction

### Target Selection Strategy
- **High-Damage Enemies**: Prioritize enemies with powerful attack capabilities
- **Immediate Threats**: Target enemies about to attack within debuff window
- **Formation Disruptors**: Weaken enemies threatening team formation integrity
- **Priority Assessment**: Balance immediate damage versus future damage mitigation

## Upgrade Paths

### Tier 1: Enhanced Strike
- **Damage Increase**: 150 → 200 base damage per bash
- **Reduction Improvement**: 30% → 40% damage reduction on enemy attacks
- **Duration Extension**: 8 seconds → 12 seconds maximum debuff duration
- **Strategic Value**: Higher immediate damage with stronger defensive benefit

### Tier 2: Crushing Blow
- **Area Effect**: Bash affects all enemies within 2-tile radius
- **Stagger Effect**: Bashed enemies have movement speed reduced by 25% for 5 seconds
- **Cooldown Reduction**: 6 seconds → 4 seconds between bash uses
- **Tactical Evolution**: Transforms from single-target to area control ability

### Tier 3: Devastating Impact
- **Critical Strikes**: Bash attacks have 50% chance for critical hit (double damage)
- **Vulnerability**: Debuffed enemies take 25% more damage from all sources
- **Chain Weakness**: Killing debuffed enemy spreads bash debuff to nearby enemies
- **Ultimate Control**: Creates cascading vulnerability effects across enemy formations

## Combat Integration and Synergy

### Ability Combinations
- **Block Coordination**: Use bash between block rotations for offensive pressure
- **Taunt Synergy**: Bash taunted enemies to reduce incoming damage to Warrior
- **Bulwark Integration**: Combine with bulwark for comprehensive damage reduction
- **Team Coordination**: Time bash with ally abilities for maximum damage and protection

### Formation Strategy
- **Tank Leadership**: Use bash to control enemy damage output against formation
- **Frontline Pressure**: Provide offensive contribution while maintaining defensive role
- **Target Management**: Focus bash on enemies threatening team cohesion
- **Damage Control**: Create windows of reduced damage for team tactical adjustments

## Visual & Audio Design

### Lighting Design Philosophy
The Warrior's Bash employs **metallic impact lighting** that emphasizes raw physical power and defensive mastery. The color palette centers on **bronze golds and iron grays** to convey martial prowess and shield expertise.

**Technical Implementation:**
- **Key Light**: Sharp, directional lighting emphasizing muscle definition and shield gleam
- **Fill Light**: Warm bounce lighting from metallic shield surface
- **Rim Light**: Golden edge lighting creating heroic silhouette
- **Color Temperature**: 2800K-3200K range for warm, martial atmosphere
- **PBR Materials**: High metallic values (0.9-1.0) for shield, medium (0.4-0.6) for armor

### Bash Execution

**Lighting Design:**
- **Shield Flash**: Brilliant metallic gleam as shield catches light (Intensity: 12, Duration: 0.2s)
- **Muscle Definition**: Enhanced key lighting emphasizing Warrior's physical power
- **Motion Blur**: Subtle light streaking following shield movement
- **Performance**: Single directional light with animated intensity curve

**Visual Effects:**
- **Shield Trail**: Golden metallic streak following shield arc
- **Power Particles**: 25-35 golden sparks emanating from shield surface
- **Impact Anticipation**: Shield surface develops bright rim lighting before contact
- **Armor Reflection**: Realistic light reflection off metallic armor pieces

**Audio Design:**
- **Shield Preparation**: Metallic scraping as shield is raised (500-2kHz)
- **Power Buildup**: Low-frequency rumble indicating gathered strength (40-100Hz)
- **Battle Cry**: Optional warrior shout with heroic determination
- **Performance**: Layered metallic audio with natural reverb characteristics

### Impact and Damage

**Lighting Design:**
- **Impact Flash**: Intense yellow-white burst at contact point (Intensity: 20, Duration: 0.1s)
- **Shockwave Lighting**: Expanding light ring from impact zone (3-unit radius)
- **Enemy Illumination**: Target briefly becomes light source during impact
- **Performance**: Impact lighting uses pre-calculated light cookies for consistency

**Visual Effects:**
- **Metal Impact**: 40-50 golden sparks exploding from contact point
- **Force Visualization**: Visible shockwave expanding from shield strike
- **Enemy Reaction**: Target staggers with realistic physics response
- **Damage Numbers**: Bold orange numbers with metallic outline styling

**Audio Design:**
- **Shield Impact**: Deep metallic clang with harmonic overtones (200-4kHz)
- **Physical Contact**: Bone-crushing impact with satisfying weight
- **Reverb Tail**: Environmental echo emphasizing power and authority
- **Performance**: High-quality samples with convolution reverb for realistic space

### Debuff Application

**Lighting Design:**
- **Weakness Aura**: Dim amber outline around affected enemy (RGB: 0.6, 0.4, 0.2, Intensity: 1.0)
- **Status Indication**: Pulsing weak light indicating reduced combat effectiveness
- **Duration Visualization**: Gradually fading aura over 8-second duration
- **Performance**: Animated material emission with exponential decay curve

**Visual Effects:**
- **Debuff Symbol**: Cracked shield icon hovering above enemy
- **Weakness Particles**: Occasional amber motes drifting from affected enemy
- **Combat Reduction**: Enemy's attacks display visual diminishment effects
- **Status UI**: Clear debuff timer with metallic shield iconography

**Audio Design:**
- **Debuff Application**: Subtle metal weakening sound (bell-like decay)
- **Status Audio**: Periodic weak chime indicating ongoing debuff
- **Frequency Range**: Mid-range tones (400-1.2kHz) for clear identification
- **Performance**: Compressed audio with distance-based attenuation

### Damage Mitigation

**Lighting Design:**
- **Mitigation Flash**: Brief amber pulse when debuff reduces incoming damage
- **Protection Visualization**: Subtle shield-like light barrier during mitigation
- **Effectiveness Display**: Visual confirmation of successful damage reduction
- **Performance**: Reactive lighting triggered by damage calculation events

**Visual Effects:**
- **Damage Reduction**: Modified damage numbers showing reduced values
- **Shield Echo**: Translucent shield appears briefly during mitigation
- **Protection Particles**: Small golden sparks during successful mitigation
- **Effectiveness Feedback**: Clear visual indication of bash working as intended

**Audio Design:**
- **Mitigation Sound**: Subtle metallic deflection audio (1-3kHz)
- **Protection Audio**: Brief shield resonance indicating successful reduction
- **Feedback Clarity**: Distinct audio confirming bash effectiveness
- **Performance**: Reactive audio with minimal processing overhead

### Performance Optimization for Mass Combat

**Lighting Optimization:**
- **Bash Light LOD**:
  - **High (0-15 units)**: Full metallic reflections and dynamic shadows
  - **Medium (15-35 units)**: Simplified impact lighting, no reflections
  - **Low (35+ units)**: Emissive effects only, no dynamic lighting
- **Light Culling**: Impact lights disabled when outside camera frustum
- **Batch Processing**: Multiple bash impacts use shared lighting calculations

**Visual Effects Optimization:**
- **Particle Pooling**: Pre-allocated pools for 30 concurrent bash effects
- **Shader Variants**: Simplified metallic shaders for distant warriors
- **Effect Scaling**: Particle counts reduced based on camera distance
- **Texture Compression**: Metallic textures use BC7 compression for quality retention

**Audio Optimization:**
- **Voice Limiting**: Maximum 6 concurrent bash audio sources
- **Audio LOD**: Simplified impact audio beyond 20-unit range
- **Compression**: Metallic audio uses lossless compression for quality
- **Memory Management**: Shared buffers for similar metallic impact sounds

### Deterministic Recording Compatibility

**Visual Synchronization:**
- **Impact Timing**: All visual effects triggered by exact collision detection events
- **Effect Consistency**: Particle patterns use seeded randomization for replay accuracy
- **Animation Precision**: Shield movement animations synchronized with component updates
- **Replay Accuracy**: Identical visual sequence across all replay instances

**Audio Synchronization:**
- **Event-Driven Audio**: All audio cues tied to specific bash and damage events
- **Position Accuracy**: Audio sources track exact entity collision positions
- **Timing Precision**: Audio scheduling based on simulation frame timing
- **Replay Fidelity**: Consistent audio playback in all replay modes

### Accessibility Considerations

**Visual Accessibility:**
- **Colorblind Support**: Golden effects include brightness contrast alternatives
- **High Contrast**: Alternative red-orange palette for improved visibility
- **Motion Sensitivity**: Reduced particle effects option maintaining core feedback
- **Text Scaling**: Damage and debuff indicators support 150%-250% scaling

**Audio Accessibility:**
- **Hearing Impaired**: Visual impact indicators for all audio cues
- **Frequency Alternatives**: Lower frequency impact sounds available
- **Subtitle Support**: Text descriptions for metallic audio effects
- **Volume Independence**: Visual impact clarity maintained at all audio levels

### Warrior Class Visual Identity

The Bash ability establishes the Warrior's **martial prowess** visual language:
- **Primary Colors**: Bronze gold (RGB: 0.8, 0.6, 0.2) and iron gray (RGB: 0.5, 0.5, 0.6)
- **Secondary Accents**: Battle red (RGB: 0.8, 0.2, 0.1) for enhanced combat states
- **Material Palette**: High metallic surfaces with realistic wear and battle damage
- **Lighting Character**: Strong directional lighting emphasizing physical power
- **Animation Style**: Powerful, grounded movements with realistic physics

### Metallurgy and Craftsmanship

**Shield Design Integration:**
- **Surface Reflection**: Realistic metallic BRDF with appropriate roughness values
- **Battle Wear**: Authentic scratches and dents showing combat experience
- **Light Interaction**: Proper caustic reflections from polished shield surface
- **Material Authenticity**: Physically accurate metal properties in all lighting conditions