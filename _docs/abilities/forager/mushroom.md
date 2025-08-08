# Symbiosis

A complete implementation guide for the Forager's resource-fed healing node ability that grows stronger with investment.

## Overview

**Symbiosis** transforms the Forager from simple healer into resource cultivator. By planting symbiotic nodes and feeding them resources (mana, gold, or items), the Forager creates powerful healing zones that grow stronger with investment. Each resource type provides different benefits - mana increases healing rate, gold extends duration, and items add special effects. This resource-conversion mechanic creates strategic decisions about when to invest in healing infrastructure versus immediate needs.

## Orthogonal Design Analysis

**Unique Mechanic**: Resource-fed growth system. No other ability requires ongoing resource investment to increase power - most abilities have fixed costs and effects.

**Strategic Niche**: Creates "healing investment" gameplay where resources compound into powerful healing zones. Transforms resource management into healing potential.

**Counterplay**: Enemies can destroy nodes before maturation, starve Forager of resources, or force early harvesting before full growth.

## Implementation Architecture

### Component-Based Design (Single-Value Pattern)

```rust
// Symbiosis ability marker on Forager
#[derive(Component)]
pub struct Symbiosis;

// Symbiotic node entity components
#[derive(Component)]
pub struct SymbioticNode;  // Marker

#[derive(Component)]
pub struct NodeOwner(pub Entity);  // Forager who planted

#[derive(Component)]
pub struct GrowthLevel(pub f32);  // Current growth (0.0-5.0)

#[derive(Component)]
pub struct ManaInvested(pub f32);  // Total mana fed

#[derive(Component)]
pub struct GoldInvested(pub f32);  // Total gold fed

#[derive(Component)]
pub struct HealingPower(pub f32);  // HP/sec base healing

#[derive(Component)]
pub struct NodeRadius(pub f32);  // Healing zone radius

// Node entity composition
commands.spawn((
    SymbioticNode,
    NodeOwner(forager_entity),
    GrowthLevel(0.0),
    ManaInvested(0.0),
    GoldInvested(0.0),
    HealingPower(10.0),  // Base 10 HP/sec
    NodeRadius(2.0 * TILE_SIZE),
    Transform::from_translation(position),
));

// Feeding interaction entity
commands.spawn((
    FeedingChannel,
    SourceEntity(forager_entity),
    TargetEntity(node_entity),
    ResourceType::Mana,
    DrainRate(10.0),  // 10 mana/sec
    ElapsedTime(0.0),
));

// Healing pulse entity (spawned periodically)
commands.spawn((
    HealingPulse,
    Origin(node_position),
    Radius(node_radius),
    HealAmount(healing_power),
    Duration(0.1),
    ElapsedTime(0.0),
));
```

### Focused Systems

```rust
// System 1: Node planting
pub fn plant_symbiosis_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    forager_query: Query<(Entity, &Transform), With<Symbiosis>>,
) {
    if !input.just_pressed(KeyCode::KeyE) {
        return;
    }
    
    for (forager_entity, transform) in forager_query.iter() {
        // Plant node at Forager's position
        commands.spawn((
            SymbioticNode,
            NodeOwner(forager_entity),
            GrowthLevel(0.0),
            ManaInvested(0.0),
            GoldInvested(0.0),
            HealingPower(10.0),
            NodeRadius(2.0 * TILE_SIZE),
            Transform::from_translation(transform.translation),
        ));
    }
}

// System 2: Resource feeding
pub fn feed_node_system(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut channels: Query<(
        Entity,
        &SourceEntity,
        &TargetEntity,
        &ResourceType,
        &DrainRate,
        &mut ElapsedTime,
    ), With<FeedingChannel>>,
    mut nodes: Query<(
        &mut GrowthLevel,
        &mut ManaInvested,
        &mut GoldInvested,
        &mut HealingPower,
        &mut NodeRadius,
    ), With<SymbioticNode>>,
    mut resources: Query<&mut Resources>,
) {
    // Handle feeding input
    if input.pressed(KeyCode::KeyF) {
        // Create or maintain feeding channel
        // (implementation details)
    }
    
    // Process active feeding
    for (entity, source, target, resource_type, rate, mut elapsed) in channels.iter_mut() {
        elapsed.0 += time.delta_secs();
        
        if let Ok(mut source_resources) = resources.get_mut(source.0) {
            if let Ok((
                mut growth,
                mut mana_invested,
                mut gold_invested,
                mut healing_power,
                mut radius
            )) = nodes.get_mut(target.0) {
                let amount = rate.0 * time.delta_secs();
                
                match resource_type {
                    ResourceType::Mana => {
                        if source_resources.mana >= amount {
                            source_resources.mana -= amount;
                            mana_invested.0 += amount;
                            
                            // Mana increases healing power
                            healing_power.0 = 10.0 + (mana_invested.0 * 0.5);
                            growth.0 = (growth.0 + amount * 0.1).min(5.0);
                        }
                    },
                    ResourceType::Gold => {
                        if source_resources.gold >= amount as u32 {
                            source_resources.gold -= amount as u32;
                            gold_invested.0 += amount;
                            
                            // Gold increases radius
                            radius.0 = 2.0 * TILE_SIZE + (gold_invested.0 * 0.1);
                            growth.0 = (growth.0 + amount * 0.05).min(5.0);
                        }
                    },
                }
            }
        }
    }
}

// System 3: Healing pulse generation
pub fn symbiosis_healing_system(
    mut commands: Commands,
    time: Res<Time>,
    mut nodes: Query<(
        Entity,
        &Transform,
        &GrowthLevel,
        &HealingPower,
        &NodeRadius,
        &mut Local<Timer>,
    ), With<SymbioticNode>>,
) {
    for (entity, transform, growth, power, radius, mut timer) in nodes.iter_mut() {
        // Only mature nodes heal (growth > 1.0)
        if growth.0 < 1.0 {
            continue;
        }
        
        // Pulse timer based on growth level
        if timer.duration() == Duration::ZERO {
            let interval = 2.0 / growth.0.max(1.0);  // Faster with growth
            *timer = Timer::from_seconds(interval, TimerMode::Repeating);
        }
        
        timer.tick(time.delta());
        
        if timer.just_finished() {
            // Spawn healing pulse
            commands.spawn((
                HealingPulse,
                Origin(transform.translation),
                Radius(radius.0),
                HealAmount(power.0),
                Duration(0.1),
                ElapsedTime(0.0),
            ));
        }
    }
}

// System 4: Apply healing pulses
pub fn apply_healing_pulse_system(
    mut commands: Commands,
    pulses: Query<(Entity, &Origin, &Radius, &HealAmount), With<HealingPulse>>,
    mut allies: Query<(&Transform, &mut Health), With<Character>>,
) {
    for (pulse_entity, origin, radius, heal_amount) in pulses.iter() {
        for (transform, mut health) in allies.iter_mut() {
            let distance = transform.translation.distance(origin.0);
            if distance <= radius.0 {
                health.current = (health.current + heal_amount.0).min(health.max);
            }
        }
        
        // Despawn pulse after application
        commands.entity(pulse_entity).despawn();
    }
}

// System 5: Node visual growth
pub fn symbiosis_visual_system(
    nodes: Query<(&Transform, &GrowthLevel, &HealingPower), With<SymbioticNode>>,
    mut gizmos: Gizmos,
) {
    for (transform, growth, power) in nodes.iter() {
        // Visual representation scales with growth
        let size = 0.5 + growth.0 * 0.3;
        let color = Color::rgb(0.2, 0.8 + growth.0 * 0.04, 0.2);
        
        gizmos.sphere(
            transform.translation,
            Quat::IDENTITY,
            size,
            color,
        );
        
        // Pulsing aura based on healing power
        let aura_size = size * (1.0 + (power.0 / 100.0));
        gizmos.circle(
            transform.translation,
            Vec3::Y,
            aura_size,
            color.with_a(0.3),
        );
    }
}
```

## Gameplay Mechanics

### Growth Stages

- **Stage 0 (Seed)**: 0.0-1.0 growth, no healing
- **Stage 1 (Sprout)**: 1.0-2.0 growth, basic healing
- **Stage 2 (Young)**: 2.0-3.0 growth, enhanced healing
- **Stage 3 (Mature)**: 3.0-4.0 growth, rapid healing
- **Stage 4 (Ancient)**: 4.0-5.0 growth, maximum power

### Resource Conversion

- **Mana**: 10 mana → 5 HP/sec healing power increase
- **Gold**: 10 gold → 0.1 tile radius increase
- **Items**: Consumables add special effects (regen, cleanse, shield)

### Investment Returns

- Early investment compounds over time
- Mature nodes provide exponentially better healing
- Resource diversity creates balanced, powerful nodes

## Strategic Depth

### Economic Planning

Symbiosis creates resource allocation decisions:
- Invest early for long-term healing infrastructure
- Balance immediate needs vs future potential
- Coordinate team resources for mega-nodes

### Defensive Gardening

Creating healing strongholds:
- Plant nodes at strategic choke points
- Layer multiple nodes for overlapping coverage
- Time growth for anticipated battles

### Risk Management

Protecting investments:
- Nodes vulnerable during growth phase
- Enemies can force premature harvesting
- Resource drain leaves Forager vulnerable

## Upgrade Paths

### Tier 1: Accelerated Growth
- **Growth Rate**: +50% growth from all resources
- **Quick Sprout**: Nodes start at Stage 1 (immediate healing)
- **Resource Efficiency**: 20% less resources for same growth

### Tier 2: Symbiotic Network
- **Node Linking**: Nodes within 5 tiles share resources
- **Compound Growth**: Each node boosts nearby node growth by 25%
- **Shared Healing**: Overlapping zones stack healing

### Tier 3: World Tree
- **Mega Node**: Can grow to Stage 6 (10.0 growth)
- **Resource Absorption**: Nodes collect resources from defeated enemies
- **Permanent Roots**: Nodes persist through Forager death

## Balance Considerations

### Power Budget
- **High Resource Cost**: Significant investment for full power
- **Vulnerable Growth**: Nodes weak during early stages
- **Stationary Healing**: No mobile healing option
- **Setup Time**: Requires planning and preparation

### Skill Expression
- **Resource Management**: Optimizing investment timing
- **Placement Strategy**: Choosing node locations
- **Growth Timing**: When to start investing
- **Harvest Decisions**: When to stop feeding and use

## Visual Design

### Node Appearance
- **Organic Growth**: Visibly expands with investment
- **Resource Glow**: Different colors for mana (blue) vs gold (yellow)
- **Pulsing Life**: Healing pulses visible as green waves
- **Root Network**: Visible connections between nearby nodes

### Growth Stages
- **Seed**: Small green sprout
- **Young**: Glowing mushroom cap
- **Mature**: Large bioluminescent structure
- **Ancient**: Towering organic spire with particle effects

## Audio Design

### Growth Sounds
- **Planting**: Soft earth-moving sound
- **Feeding**: Gentle absorption hum
- **Growth**: Organic expansion creaks
- **Healing Pulse**: Natural chime with each wave

## Conclusion

Symbiosis creates unique "investment healing" gameplay where resources compound into powerful healing infrastructure. By requiring ongoing resource feeding, it transforms the Forager from reactive healer into strategic cultivator. The mechanic is completely orthogonal - no other ability grows stronger through continuous resource investment.

The implementation uses clean entity composition: node entities with investment trackers, feeding channels for resource transfer, and periodic healing pulses. Each system handles one aspect: planting, feeding, growth, healing, or visuals. This maintains the single-responsibility principle while creating deep, emergent gameplay from simple components.