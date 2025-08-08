# Transmute

A complete implementation guide for the Alchemist's resource conversion utility ability using single-component architecture.

## Overview

The **Transmute** ability represents the Alchemist's mastery over matter transformation, allowing them to convert basic materials and loot into more valuable resources. This utility ability operates on a risk-reward economy system where players can potentially upgrade common drops into rare materials, but with uncertain outcomes. The ability encourages exploration, resource management, and strategic decision-making about when to gamble current resources for potential improvements.

## Game Design Philosophy

This ability demonstrates economic design principles that create meaningful resource management decisions:

**Controlled Randomness with Value**: While outcomes have random elements, the ability guarantees equal or greater value, preventing purely negative results that would frustrate players.

**Inventory Management Pressure**: By requiring proximity to items and having a cooldown, players must make tactical decisions about which items deserve transmutation versus immediate collection.

**Knowledge-Based Optimization**: Experienced players learn optimal timing and item prioritization, creating skill expression through game system mastery rather than mechanical execution.

## Component Architecture

### Entity Composition Pattern

Following the single-component architecture, Transmute uses simple, single-value components:

```rust
// Transmute Ability Activation
commands.spawn((
    // Marker Components
    TransmuteAction,
    ChannelCast,
    
    // Ability Components
    Range(2.0),
    Cooldown(6.0),
    CastTime(1.5),
    
    // Target Components
    TargetEntity(item_entity),
    TargetPosition(item_pos),
    
    // Effect Components
    SuccessRate(0.7),
    ValueMultiplier(1.0),
));

// Channeling Entity
commands.entity(alchemist_entity).with_child((
    // Marker Components
    TransmuteChannel,
    Channeling,
    
    // Time Components
    Duration(1.5),
    ElapsedTime(0.0),
    
    // Visual Components
    EmissiveIntensity(2.0),
    EmissiveColor(Color::srgb(0.8, 0.6, 0.2)),
    ParticleCount(30),
));

// Transmutation Result Entity
commands.spawn((
    // Marker Components
    TransmutedItem,
    
    // Item Components
    ItemTier(new_tier),
    ItemValue(new_value),
    Transform::from_translation(item_pos),
    
    // Visual Components
    FlashIntensity(3.0),
    Duration(0.5),
    ElapsedTime(0.0),
));
```

### Core Components Used

All components are single-value/single-purpose:

```rust
// Ability Components
pub struct Range(pub f32);
pub struct Cooldown(pub f32);
pub struct CastTime(pub f32);
pub struct SuccessRate(pub f32);
pub struct ValueMultiplier(pub f32);

// Target Components
pub struct TargetEntity(pub Entity);
pub struct TargetPosition(pub Vec3);

// Time Components
pub struct Duration(pub f32);
pub struct ElapsedTime(pub f32);

// Item Components
pub struct ItemTier(pub u8);
pub struct ItemValue(pub f32);
pub struct OriginalItem(pub Entity);

// Visual Components
pub struct EmissiveIntensity(pub f32);
pub struct EmissiveColor(pub Color);
pub struct ParticleCount(pub u32);
pub struct FlashIntensity(pub f32);

// Marker Components (zero-sized)
pub struct TransmuteAction;
pub struct TransmuteChannel;
pub struct Channeling;
pub struct ChannelCast;
pub struct TransmutedItem;
pub struct Interruptible;
```

### System Implementation

Systems query only the components they need:

```rust
// Target selection system
fn transmute_target_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    alchemists: Query<(Entity, &Transform), With<TransmuteAbility>>,
    items: Query<(Entity, &Transform, &ItemTier), With<GroundItem>>,
) {
    if input.just_pressed(KeyCode::KeyT) {
        for (alchemist_entity, alchemist_transform) in alchemists.iter() {
            // Find nearest item within range
            let mut closest: Option<(Entity, f32)> = None;
            
            for (item_entity, item_transform, tier) in items.iter() {
                let distance = alchemist_transform.translation.distance(item_transform.translation);
                
                if distance <= 2.0 * TILE_SIZE {
                    if closest.is_none() || distance < closest.unwrap().1 {
                        closest = Some((item_entity, distance));
                    }
                }
            }
            
            if let Some((target, _)) = closest {
                // Start channeling
                commands.entity(alchemist_entity).with_child((
                    TransmuteChannel,
                    Channeling,
                    TargetEntity(target),
                    Duration(1.5),
                    ElapsedTime(0.0),
                    Interruptible,
                ));
            }
        }
    }
}

// Channeling progress system
fn channel_progress_system(
    mut commands: Commands,
    time: Res<Time>,
    mut channels: Query<(
        Entity,
        &mut ElapsedTime,
        &Duration,
        &TargetEntity,
        &Parent
    ), With<TransmuteChannel>>,
) {
    for (channel_entity, mut elapsed, duration, target, parent) in channels.iter_mut() {
        elapsed.0 += time.delta_secs();
        
        if elapsed.0 >= duration.0 {
            // Channel complete - trigger transmutation
            commands.spawn((
                TransmuteAction,
                TargetEntity(target.0),
                SuccessRate(0.7),
                ValueMultiplier(1.0),
            ));
            
            // Remove channel
            commands.entity(channel_entity).despawn();
        }
    }
}

// Transmutation resolution system
fn transmute_resolution_system(
    mut commands: Commands,
    actions: Query<(&TargetEntity, &SuccessRate, &ValueMultiplier), Added<TransmuteAction>>,
    items: Query<(&Transform, &ItemTier, &ItemValue), With<GroundItem>>,
    mut rng: ResMut<GlobalRng>,
) {
    for (target, success_rate, multiplier) in actions.iter() {
        if let Ok((transform, tier, value)) = items.get(target.0) {
            let roll = rng.gen_range(0.0..1.0);
            
            let (new_tier, new_value) = if roll < success_rate.0 {
                // Success - upgrade tier
                (tier.0 + 1, value.0 * 1.5)
            } else {
                // Standard - maintain value
                (tier.0, value.0 * 1.2)
            };
            
            // Spawn new item
            commands.spawn((
                TransmutedItem,
                GroundItem,
                ItemTier(new_tier),
                ItemValue(new_value),
                Transform::from_translation(transform.translation),
                FlashIntensity(3.0),
                Duration(0.5),
                ElapsedTime(0.0),
            ));
            
            // Remove original
            commands.entity(target.0).despawn();
        }
    }
}

// Visual effect system
fn transmute_visual_system(
    channels: Query<(&Parent, &ElapsedTime, &Duration), With<TransmuteChannel>>,
    mut alchemists: Query<&mut Handle<ColorMaterial>>,
) {
    for (parent, elapsed, duration) in channels.iter() {
        if let Ok(mut material) = alchemists.get_mut(parent.get()) {
            // Increase glow intensity as channel progresses
            let progress = elapsed.0 / duration.0;
            // Update material emissive based on progress
        }
    }
}

// Interrupt system
fn channel_interrupt_system(
    mut commands: Commands,
    damaged: Query<Entity, (With<TookDamage>, With<Channeling>)>,
    channels: Query<Entity, With<TransmuteChannel>>,
) {
    for damaged_entity in damaged.iter() {
        // Find and remove any channels on damaged entity
        for channel in channels.iter() {
            commands.entity(channel).despawn();
        }
    }
}
```

## Upgrade Paths

### Tier 1: Enhanced Probability
Adds success enhancement components:
```rust
// Additional components
SuccessRateBonus(0.15)  // +15% success rate
QualityBonus(1)         // +1 tier potential
```

### Tier 2: Batch Processing
Adds multi-target components:
```rust
// Additional components
MaxTargets(3)
AreaOfEffect(2.0)
SharedChannel         // All targets use same channel time
```

### Tier 3: Alchemical Mastery
Adds guaranteed success components:
```rust
// Additional components
GuaranteedSuccess
ValueMultiplierBonus(0.5)  // +50% value
RangeBonus(2.0)            // +2 tile range
```

## Item Transmutation Tables

Transmutation uses tier-based components:

```rust
// Common tier transmutation
if item_tier.0 == 1 {
    commands.spawn((
        ItemTier(2),  // Upgrade to uncommon
        ItemValue(base_value * 1.5),
    ));
}

// Uncommon tier transmutation  
if item_tier.0 == 2 {
    commands.spawn((
        ItemTier(3),  // Upgrade to rare
        ItemValue(base_value * 2.0),
    ));
}

// Rare tier transmutation
if item_tier.0 == 3 {
    commands.spawn((
        ItemTier(4),  // Upgrade to epic
        ItemValue(base_value * 3.0),
    ));
}
```

## Visual & Audio Design

Visual effects use single-value components:

```rust
// Targeting phase
commands.spawn((
    TargetHighlight,
    EmissiveColor(Color::srgb(0.8, 0.6, 0.2)),
    EmissiveIntensity(1.0),
    PulseRate(2.0),
));

// Channeling phase
commands.spawn((
    ChannelBeam,
    Origin(alchemist_pos),
    Target(item_pos),
    BeamWidth(0.2),
    BeamColor(Color::srgb(0.8, 0.6, 0.2)),
    ParticleCount(50),
    Duration(1.5),
));

// Transmutation resolution
commands.spawn((
    TransmuteFlash,
    FlashIntensity(3.0),
    FlashColor(Color::srgb(1.0, 0.8, 0.0)),
    Duration(0.5),
    ParticleCount(100),
    AudioVolume(0.8),
));
```

## Strategic Applications

Component-based transmutation strategies:

```rust
// High-value targeting
fn prioritize_rare_items(items: Query<(Entity, &ItemTier)>) -> Option<Entity> {
    items.iter()
        .filter(|(_, tier)| tier.0 >= 2)
        .map(|(e, _)| e)
        .next()
}

// Batch efficiency
fn transmute_item_cluster(position: Vec3, radius: f32) {
    // Transmute all items in area
}

// Economic optimization
fn calculate_transmute_value(tier: &ItemTier, value: &ItemValue) -> f32 {
    let potential_gain = value.0 * 0.5;  // Expected value increase
    let time_cost = 1.5 + 6.0;           // Cast + cooldown
    potential_gain / time_cost
}
```

## Recording Integration

All components are deterministic and recordable:

```rust
commands.spawn((
    RecordableAction,
    ActionType::Transmute,
    Timestamp(recording.current_time),
    TargetEntity(item_entity),
    SuccessRoll(roll_value),
));
```

This single-component architecture ensures:
- **Simple RNG handling** with single success rate values
- **Clean channel interruption** through component removal
- **Flexible item tiers** with single-value tier components
- **Efficient batch processing** with area components
- **Deterministic outcomes** for recording system