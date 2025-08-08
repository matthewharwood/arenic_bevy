# Dice

A complete implementation guide for the Merchant's stacking critical chance self-enhancement ability using single-component architecture.

## Overview

The **Dice** ability represents the Merchant's mastery over probability manipulation through magical gambling mechanics. Each activation grants a stackable 1% critical strike chance bonus that persists until used, creating a risk-reward system where patience and timing can lead to devastating critical attacks. This self-buffing ability emphasizes strategic accumulation and optimal timing for maximum impact during crucial combat moments.

## Game Design Philosophy

This ability demonstrates accumulation design through stackable probability enhancement:

**Patience Rewards**: The stacking mechanism rewards players who can delay gratification and accumulate bonuses for optimal timing rather than immediate use.

**Probability Manipulation**: The critical strike focus creates exciting moments where accumulated luck transforms routine attacks into game-changing critical hits.

**Resource-Free Investment**: The instant cast and lack of resource costs encourage frequent use during downtime, creating ongoing tactical decisions about when to accumulate versus when to deploy.

## Component Architecture

### Entity Composition Pattern

Following the single-component architecture, Dice uses simple, single-value components:

```rust
// Dice Activation Entity
commands.spawn((
    // Marker Components
    DiceRoll,
    InstantCast,
    
    // Ability Components
    Cooldown(0.5),
    CritChancePerStack(0.01),
    
    // Owner
    Owner(merchant_entity),
));

// Dice Stack Entity (child of Merchant)
commands.entity(merchant_entity).with_child((
    // Marker Components
    DiceStacks,
    CriticalEnhancement,
    
    // Stack Components
    CurrentStacks(current),
    MaxStacks(50),
    TotalCritChance(current as f32 * 0.01),
    
    // Persistence
    PersistUntilUsed,
    ConsumeOnCrit,
));

// Visual Dice Entity (per stack)
commands.spawn((
    // Marker Components
    DiceVisual,
    
    // Stack Reference
    StackIndex(index),
    Owner(merchant_entity),
    
    // Visual Components
    Transform::from_translation(orbit_pos),
    RotationSpeed(2.0),
    OrbitRadius(1.5),
    OrbitAngle(angle),
    EmissiveIntensity(0.5),
));

// Critical Strike Entity (on trigger)
commands.spawn((
    // Marker Components
    CriticalStrike,
    DiceEnhanced,
    
    // Effect Components
    BaseDamage(base),
    CritMultiplier(2.0),
    StacksConsumed(stacks),
    
    // Visual Components
    FlashIntensity(3.0),
    FlashColor(Color::srgb(1.0, 0.8, 0.0)),
    Duration(0.5),
    ElapsedTime(0.0),
));
```

### Core Components Used

All components are single-value/single-purpose:

```rust
// Stack Components
pub struct CurrentStacks(pub u32);
pub struct MaxStacks(pub u32);
pub struct StackIndex(pub u32);
pub struct CritChancePerStack(pub f32);
pub struct TotalCritChance(pub f32);
pub struct StacksConsumed(pub u32);

// Ability Components
pub struct Cooldown(pub f32);
pub struct BaseDamage(pub f32);
pub struct CritMultiplier(pub f32);

// Visual Components
pub struct RotationSpeed(pub f32);
pub struct OrbitRadius(pub f32);
pub struct OrbitAngle(pub f32);
pub struct FlashIntensity(pub f32);
pub struct FlashColor(pub Color);
pub struct EmissiveIntensity(pub f32);

// Time Components
pub struct Duration(pub f32);
pub struct ElapsedTime(pub f32);

// Ownership
pub struct Owner(pub Entity);

// Marker Components (zero-sized)
pub struct DiceRoll;
pub struct DiceStacks;
pub struct DiceVisual;
pub struct CriticalEnhancement;
pub struct CriticalStrike;
pub struct DiceEnhanced;
pub struct InstantCast;
pub struct PersistUntilUsed;
pub struct ConsumeOnCrit;
```

### System Implementation

Systems query only the components they need:

```rust
// Stack accumulation system
fn dice_accumulation_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    merchants: Query<Entity, With<DiceAbility>>,
    mut stacks: Query<(&mut CurrentStacks, &MaxStacks, &CritChancePerStack), With<DiceStacks>>,
) {
    if input.just_pressed(KeyCode::KeyD) {
        for merchant_entity in merchants.iter() {
            // Find or create stack component
            let mut found = false;
            
            for (mut current, max, per_stack) in stacks.iter_mut() {
                if current.0 < max.0 {
                    current.0 += 1;
                    found = true;
                    
                    // Spawn visual dice
                    let angle = (current.0 as f32 / max.0 as f32) * TAU;
                    commands.spawn((
                        DiceVisual,
                        StackIndex(current.0),
                        Owner(merchant_entity),
                        Transform::default(),
                        OrbitRadius(1.5),
                        OrbitAngle(angle),
                        RotationSpeed(2.0),
                        EmissiveIntensity(0.5),
                    ));
                }
            }
            
            if !found {
                // Create initial stack
                commands.entity(merchant_entity).with_child((
                    DiceStacks,
                    CriticalEnhancement,
                    CurrentStacks(1),
                    MaxStacks(50),
                    TotalCritChance(0.01),
                    PersistUntilUsed,
                    ConsumeOnCrit,
                ));
            }
        }
    }
}

// Critical chance calculation system
fn critical_calculation_system(
    mut stacks: Query<(&CurrentStacks, &CritChancePerStack, &mut TotalCritChance), With<DiceStacks>>,
) {
    for (current, per_stack, mut total) in stacks.iter_mut() {
        total.0 = current.0 as f32 * per_stack.0;
    }
}

// Attack critical check system
fn attack_critical_system(
    mut commands: Commands,
    attacks: Query<(Entity, &BaseDamage, &Owner), Added<Attack>>,
    stacks: Query<(&CurrentStacks, &TotalCritChance, &Parent), With<DiceStacks>>,
    mut rng: ResMut<GlobalRng>,
) {
    for (attack_entity, base_damage, owner) in attacks.iter() {
        // Check if attacker has dice stacks
        for (current, crit_chance, parent) in stacks.iter() {
            if parent.get() == owner.0 {
                let roll = rng.gen_range(0.0..1.0);
                
                if roll < crit_chance.0 {
                    // Critical hit!
                    commands.entity(attack_entity).insert((
                        CriticalStrike,
                        DiceEnhanced,
                        CritMultiplier(2.0),
                        StacksConsumed(current.0),
                    ));
                    
                    // Queue stack consumption
                    commands.entity(parent.get()).insert(ConsumeStacks);
                }
            }
        }
    }
}

// Stack consumption system
fn stack_consumption_system(
    mut commands: Commands,
    mut to_consume: Query<(Entity, &mut CurrentStacks), (With<DiceStacks>, With<ConsumeStacks>)>,
    visuals: Query<(Entity, &StackIndex, &Owner), With<DiceVisual>>,
) {
    for (entity, mut stacks) in to_consume.iter_mut() {
        // Despawn all visual dice
        for (visual_entity, index, owner) in visuals.iter() {
            if owner.0 == entity {
                commands.entity(visual_entity).despawn();
            }
        }
        
        // Reset stacks
        stacks.0 = 0;
        
        // Remove consumption marker
        commands.entity(entity).remove::<ConsumeStacks>();
    }
}

// Visual orbit system
fn dice_orbit_system(
    time: Res<Time>,
    mut visuals: Query<(
        &mut Transform,
        &mut OrbitAngle,
        &OrbitRadius,
        &RotationSpeed,
        &Owner
    ), With<DiceVisual>>,
    merchants: Query<&Transform, (With<Merchant>, Without<DiceVisual>)>,
) {
    for (mut dice_transform, mut angle, radius, speed, owner) in visuals.iter_mut() {
        if let Ok(merchant_transform) = merchants.get(owner.0) {
            // Update orbit angle
            angle.0 += speed.0 * time.delta_secs();
            
            // Calculate orbit position
            let x = angle.0.cos() * radius.0;
            let z = angle.0.sin() * radius.0;
            let y = 1.0 + (angle.0 * 2.0).sin() * 0.3;  // Bobbing motion
            
            dice_transform.translation = merchant_transform.translation + Vec3::new(x, y, z);
            
            // Rotate dice
            dice_transform.rotate_y(speed.0 * time.delta_secs());
        }
    }
}

// Critical damage application system
fn critical_damage_system(
    criticals: Query<(&BaseDamage, &CritMultiplier, &StacksConsumed), With<CriticalStrike>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (base, multiplier, stacks) in criticals.iter() {
        let final_damage = base.0 * multiplier.0;
        
        damage_events.send(DamageEvent {
            damage: final_damage,
            is_critical: true,
            bonus_effects: vec![
                BonusEffect::ScreenShake(0.5),
                BonusEffect::CriticalText(format!("{}x CRIT!", stacks.0)),
            ],
        });
    }
}
```

## Upgrade Paths

### Tier 1: Loaded Dice
Adds enhanced stacking components:
```rust
// Additional components
CritChanceBonus(0.005)     // +0.5% per stack (1.5% total)
CooldownReduction(0.2)     // 0.3s cooldown
MaxStacksBonus(10)         // 60 max stacks
```

### Tier 2: Lucky Streak
Adds partial consumption components:
```rust
// Additional components
PartialConsume(0.5)        // Only consume half stacks
ChainCritWindow(3.0)       // 3 second window
ChainCritBonus(0.25)       // +25% crit during window
LuckySevens                // Every 7th stack bonus damage
```

### Tier 3: Probability Master
Adds mastery components:
```rust
// Additional components
GuaranteedCritThreshold(30) // 30+ stacks = guaranteed crit
StackTransfer              // Can transfer to allies
TransferAmount(20)         // Max 20 stacks to transfer
CriticalExplosion          // Area damage on crit
ExplosionScale(0.1)        // Damage per stack consumed
```

## Visual & Audio Design

Visual effects use single-value components:

```rust
// Stack accumulation
commands.spawn((
    DiceRollEffect,
    Duration(0.5),
    ElapsedTime(0.0),
    RotationSpeed(10.0),
    EmissiveIntensity(2.0),
    AudioPitch(1.0 + (stacks as f32 * 0.02)),  // Higher pitch with more stacks
));

// Probability enhancement
commands.spawn((
    LuckAura,
    Owner(merchant_entity),
    GlowIntensity(stacks as f32 * 0.02),
    ParticleCount(stacks * 2),
    ParticleColor(Color::srgb(1.0, 0.8, 0.0)),
));

// Critical hit payoff
commands.spawn((
    CriticalExplosionVfx,
    Duration(1.0),
    ElapsedTime(0.0),
    StartRadius(0.5),
    EndRadius(3.0 + (stacks as f32 * 0.1)),
    ParticleCount(100 + stacks * 10),
    FlashIntensity(2.0 + (stacks as f32 * 0.05)),
    ScreenShakeIntensity(0.3 + (stacks as f32 * 0.01)),
));

// Stack consumption
commands.spawn((
    DiceExplosion,
    ParticleCount(stacks * 5),
    ExplosionForce(10.0),
    Duration(1.0),
    AudioVolume(1.0),
));
```

## Strategic Applications

Component-based stacking strategies:

```rust
// Accumulation decision system
fn should_keep_stacking(
    current: &CurrentStacks,
    combat_intensity: f32,
    enemy_health: f32,
) -> bool {
    if current.0 >= 40 {
        false  // Use at high stacks
    } else if enemy_health < 30.0 && current.0 >= 15 {
        false  // Use for finishing blow
    } else if combat_intensity < 0.3 {
        true   // Keep stacking in low combat
    } else {
        current.0 < 20  // Moderate threshold
    }
}

// Expected value calculation
fn calculate_dice_value(stacks: &CurrentStacks, multiplier: &CritMultiplier) -> f32 {
    let crit_chance = stacks.0 as f32 * 0.01;
    let expected_multiplier = 1.0 + (crit_chance * (multiplier.0 - 1.0));
    expected_multiplier
}
```

## Recording Integration

All components are deterministic and recordable:

```rust
commands.spawn((
    RecordableAction,
    ActionType::DiceRoll,
    Timestamp(recording.current_time),
    StackCount(current_stacks),
));

commands.spawn((
    RecordableAction,
    ActionType::CriticalStrike,
    Timestamp(recording.current_time),
    StacksUsed(consumed),
    DamageDealt(final_damage),
));
```

This single-component architecture ensures:
- **Efficient stack tracking** with single counter values
- **Clean visual management** through indexed dice entities
- **Flexible probability math** with single chance components
- **Smooth consumption** through marker-based systems
- **Deterministic critical rolls** for recording system