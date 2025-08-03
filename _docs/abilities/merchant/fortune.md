# Fortune

A complete implementation guide for the Merchant's luck enhancement aura utility ability.

## Overview

The **Fortune** ability showcases the Merchant's mastery over probability manipulation through a powerful luck-enhancement aura. When activated, the Merchant and all adjacent allies gain increased chances to obtain gold and rare loot from their offensive attacks for a limited duration. This team-wide economic enhancement ability transforms combat into a wealth generation opportunity while providing strategic value through improved resource acquisition.

## Game Design Philosophy

This ability demonstrates economic utility design through team-wide luck enhancement:

**Cooperative Economics**: The aura affects adjacent allies, encouraging team clustering and coordination for mutual economic benefit rather than purely selfish wealth accumulation.

**Combat Integration**: Linking luck enhancement to offensive attacks creates interesting tactical decisions about when to activate fortune for maximum economic impact.

**Sustainable Wealth**: The improved loot generation provides long-term economic benefits that extend beyond immediate combat encounters.

## Implementation Architecture

### Component-Based Design

```rust
Fortune {
    aura_radius: 2.0,                   // 2-tile radius around Merchant
    duration: 20.0,                     // 20 second effect duration
    gold_drop_bonus: 0.4,               // 40% increased gold drop chance
    loot_quality_improvement: 1,        // +1 tier to loot quality rolls
    rare_item_chance: 0.15,             // 15% chance for rare items on kills
    cooldown: 45.0,                     // 45 second ability cooldown
    stack_with_merchant: true,          // Multiple merchants stack effects
}

FortuneAura {
    center_position: Vec2,
    affected_allies: HashSet<Entity>,
    duration_remaining: f32,
    luck_enhancement: f32,
    visual_effect: Entity,
    particle_systems: Vec<Entity>,
}

LuckModification {
    entity: Entity,
    gold_multiplier: f32,
    loot_tier_bonus: u8,
    rare_chance_bonus: f32,
    fortune_source: Entity,
}
```

### Event-Driven Systems

The ability operates through five luck-based systems:
1. **Aura Management** - Tracks adjacent allies and maintains effect radius
2. **Luck Enhancement** - Modifies drop chances and loot quality for affected entities
3. **Economic Tracking** - Monitors wealth generation and resource acquisition improvements
4. **Duration Control** - Manages 20-second effect duration and cooldown timing
5. **Visual Coordination** - Shows fortune effects and luck enhancement indicators

## Step-by-Step Gameplay

### Phase 1: Fortune Activation (Double-Tap Input)
- **Input Method**: Double-tap to activate fortune aura around Merchant
- **Immediate Effect**: All allies within 2-tile radius gain luck enhancement
- **Visual Manifestation**: Golden aura appears around Merchant with radiating effects
- **Team Notification**: Clear indicators show which allies are receiving fortune benefits

### Phase 2: Luck Enhancement Period (20 Second Duration)
- **Gold Drop Improvement**: 40% increased chance for enemies to drop gold when killed
- **Loot Quality Boost**: All loot drops improved by +1 tier (common becomes uncommon, etc.)
- **Rare Item Chance**: 15% bonus chance for rare/epic items from enemy defeats
- **Team Coordination**: Allies position to maintain adjacency for continued benefits

### Phase 3: Economic Optimization (Active Management)
- **Positioning Strategy**: Team clusters around Merchant to maximize aura coverage
- **Target Prioritization**: Focus on high-value enemies during fortune window
- **Ability Synergy**: Coordinate offensive abilities to maximize kill count during effect
- **Resource Monitoring**: Track improved loot generation and wealth accumulation

### Phase 4: Effect Expiration (Duration End)
- **Visual Fade**: Fortune aura gradually diminishes over final 2 seconds
- **Benefit Cessation**: Luck enhancements end, returning to normal drop rates
- **Cooldown Start**: 45-second cooldown begins for next fortune activation
- **Economic Assessment**: Evaluate wealth gained during fortune period

## Luck Enhancement Mechanics

### Gold Drop Modifications
```rust
fn apply_fortune_to_gold_drops(enemy: Entity, killer: Entity) -> u32 {
    let base_gold = calculate_base_gold_drop(enemy);
    let has_fortune = is_affected_by_fortune(killer);
    
    if has_fortune {
        let fortune_bonus = 0.4;
        let enhanced_chance = get_base_drop_chance(enemy) + fortune_bonus;
        
        if random() < enhanced_chance {
            // Fortune increases both chance and amount
            let gold_multiplier = 1.2 + random() * 0.3; // 1.2x to 1.5x gold
            (base_gold as f32 * gold_multiplier) as u32
        } else {
            0
        }
    } else {
        // Normal gold drop calculation
        if random() < get_base_drop_chance(enemy) {
            base_gold
        } else {
            0
        }
    }
}

fn enhance_loot_quality(base_loot: LootDrop, has_fortune: bool) -> LootDrop {
    if !has_fortune {
        return base_loot;
    }
    
    let mut enhanced_loot = base_loot;
    
    // Improve tier by 1 (common -> uncommon -> rare -> epic)
    enhanced_loot.tier = match enhanced_loot.tier {
        LootTier::Common => LootTier::Uncommon,
        LootTier::Uncommon => LootTier::Rare,
        LootTier::Rare => LootTier::Epic,
        LootTier::Epic => LootTier::Epic, // Already maximum
    };
    
    // Additional rare item chance
    if random() < 0.15 {
        enhanced_loot.tier = LootTier::Rare.max(enhanced_loot.tier);
    }
    
    enhanced_loot
}
```

### Aura Coverage and Positioning
- **Dynamic Range**: 2-tile radius updates as Merchant moves during effect
- **Real-Time Tracking**: Allies entering/leaving aura gain/lose benefits immediately
- **Formation Incentive**: Encourages tight team formation for maximum economic benefit
- **Tactical Mobility**: Merchant can reposition to cover different allies as needed

## Strategic Applications

### Optimal Activation Timing
- **Enemy Density**: Activate when facing multiple high-value enemies
- **Team Readiness**: Ensure allies can capitalize on enhanced loot opportunities
- **Combat Phases**: Time activation for phases with high enemy kill potential
- **Resource Needs**: Activate when team needs specific resources or currency

### Formation and Positioning
- **Central Merchant**: Position Merchant at center of team formation
- **Cluster Benefits**: Team groups tightly around Merchant for maximum coverage
- **Mobile Aura**: Merchant follows team movement to maintain coverage
- **Priority Coverage**: Ensure highest-DPS allies remain within aura range

## Team Economic Benefits

### Wealth Distribution
- **Shared Prosperity**: All affected allies benefit from enhanced drop rates
- **Economic Teamwork**: Creates incentive for coordinated combat positioning
- **Resource Generation**: Dramatically improves team's overall resource acquisition
- **Sustainable Growth**: Regular fortune usage enables long-term wealth accumulation

### Loot Quality Improvements
- **Tier Advancement**: Common loot becomes uncommon, uncommon becomes rare
- **Rare Item Access**: 15% bonus chance provides access to high-tier equipment
- **Crafting Materials**: Enhanced loot quality improves crafting resource availability
- **Economic Diversification**: Improved variety of valuable items and resources

## Upgrade Paths

### Tier 1: Enhanced Fortune
- **Duration Extension**: 20 → 30 seconds effect duration
- **Radius Increase**: 2-tile → 3-tile aura coverage
- **Gold Bonus**: 40% → 60% increased gold drop chance
- **Strategic Value**: Longer duration and larger coverage for improved team benefits

### Tier 2: Golden Opportunity
- **Loot Tier Bonus**: +1 → +2 tier improvement to all loot drops
- **Rare Chance**: 15% → 25% bonus rare item chance
- **Stacking Benefit**: Multiple fortune auras can stack for exponential benefits
- **Economic Evolution**: Dramatically improves loot quality and rare item acquisition

### Tier 3: Midas Aura
- **Persistent Effect**: Fortune aura remains active for 60 seconds
- **Combat Rewards**: Killing enemies during fortune grants immediate gold to killer
- **Quality Guarantee**: All loot drops guaranteed to be at least uncommon tier
- **Ultimate Prosperity**: Transforms team into wealth-generating economic powerhouse

## Economic Impact and Sustainability

### Resource Generation Metrics
- **Gold Accumulation**: Track total gold gained during fortune windows
- **Loot Quality**: Monitor tier improvements and rare item acquisition
- **Team Wealth**: Measure overall team economic growth from fortune usage
- **Return on Investment**: Evaluate fortune cooldown cost against economic benefits

### Long-Term Economic Strategy
- **Regular Usage**: Use fortune on cooldown for consistent wealth generation
- **Target Selection**: Focus fortune windows on high-value enemy encounters
- **Resource Planning**: Coordinate fortune with team resource needs and goals
- **Economic Sustainability**: Build team wealth through strategic fortune application

## Visual & Audio Design

### Fortune Activation
- **Visual**: Brilliant golden aura emanates from Merchant with coin and gem particles
- **Animation**: Merchant performs dramatic gesture invoking financial prosperity
- **Audio**: Rich, satisfying sound of wealth and abundance with magical undertones
- **Team Effect**: Golden sparkles appear around all affected allies

### Active Fortune Aura
- **Visual**: Persistent golden glow around Merchant with floating currency symbols
- **Particle**: Continuous stream of gold coins, gems, and treasure particles
- **Audio**: Subtle background sound of prosperity with occasional coin chimes
- **Coverage**: Visual indicators clearly show 2-tile radius and affected allies

### Enhanced Loot Events
- **Visual**: Enhanced loot drops have golden glow and sparkle effects
- **Animation**: Loot appears with more dramatic materialization effects
- **Audio**: Satisfying treasure discovery sounds when enhanced loot appears
- **Quality**: Different particle effects for different loot tier improvements

### Economic Success Feedback
- **Visual**: Gold and treasure symbols float upward when wealth is generated
- **Animation**: Coin counter increases with satisfying numerical progression
- **Audio**: Success chimes and wealth accumulation sound effects
- **UI**: Running total shows economic benefits gained during fortune period