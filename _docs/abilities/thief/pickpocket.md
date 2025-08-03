# Pickpocket

A complete implementation guide for the Thief's resource extraction and debuff theft utility ability.

## Overview

The **Pickpocket** ability showcases the Thief's mastery over stealth and resource acquisition through close-quarters theft mechanics. When activated adjacent to enemies, the Thief can steal gold, buffs, or minor items without interrupting enemy behavior or sequence. The hold-duration mechanic creates risk-reward decisions where longer attempts yield better rewards but increase vulnerability exposure, making this ability ideal for economic warfare and tactical debuffing.

## Game Design Philosophy

This ability demonstrates stealth utility design through risk-reward resource acquisition:

**Risk-Time Investment**: The hold mechanic creates tension between safety and reward quality, requiring players to evaluate threat levels against potential gains.

**Non-Disruptive Theft**: Enemies continue their normal behavior during pickpocketing, maintaining deterministic gameplay while allowing strategic resource acquisition.

**Economic Warfare**: The ability to steal buffs and resources creates unique tactical opportunities for weakening enemies while strengthening the Thief.

## Implementation Architecture

### Component-Based Design

```rust
Pickpocket {
    range: 1.0,                         // Must be adjacent to target (1 tile)
    min_hold_time: 0.5,                 // Minimum 0.5 seconds for basic success
    max_hold_time: 3.0,                 // Maximum 3 seconds for optimal rewards
    success_chance_base: 0.6,           // 60% base success chance
    success_chance_per_second: 0.15,    // +15% success per second held
    cooldown: 8.0,                      // 8 second ability cooldown
    detection_risk: 0.1,                // 10% chance of enemy detection per second
}

PickpocketAttempt {
    target: Entity,
    thief: Entity,
    hold_duration: f32,
    success_probability: f32,
    potential_rewards: Vec<StealableItem>,
    detection_risk: f32,
    visual_effect: Entity,
}

StealableItem {
    item_type: TheftType,               // Gold, Buff, Item, or Debuff
    value: u32,
    rarity: ItemRarity,
    steal_difficulty: f32,
    description: String,
}
```

### Event-Driven Systems

The ability operates through five stealth systems:
1. **Target Validation** - Confirms adjacency and valid theft targets
2. **Hold Duration Tracking** - Manages timing and success probability calculation
3. **Theft Resolution** - Determines success and selects stolen items/effects
4. **Buff Transfer** - Handles removal from target and application to Thief
5. **Detection Management** - Tracks detection risk and enemy awareness

## Step-by-Step Gameplay

### Phase 1: Target Selection (Adjacent Positioning)
- **Proximity Requirement**: Thief must be adjacent (within 1 tile) of target enemy
- **Target Assessment**: System evaluates available items, buffs, and resources to steal
- **Risk Evaluation**: Consider enemy threat level and detection consequences
- **Strategic Timing**: Choose optimal moment when enemy is distracted or vulnerable

### Phase 2: Theft Initiation (Tap and Hold)
- **Input Method**: Tap and hold while adjacent to begin pickpocket attempt
- **Stealth Mode**: Thief enters subtle theft animation without alerting target
- **Success Building**: Success probability starts at 60% and increases 15% per second held
- **Risk Accumulation**: Detection risk increases by 10% each second of hold duration

### Phase 3: Hold Duration Decision (0.5-3.0 Seconds)
- **Minimum Threshold**: Must hold at least 0.5 seconds for any chance of success
- **Progressive Rewards**: Longer holds unlock access to higher-value theft targets
- **Detection Pressure**: Risk of discovery increases with extended hold duration
- **Strategic Release**: Balance success probability against detection risk

### Phase 4: Theft Resolution (Release Input)
- **Success Calculation**: Roll against accumulated success probability
- **Reward Selection**: Choose from available items based on hold duration
- **Buff Transfer**: Stolen buffs removed from target and applied to Thief
- **Economic Gain**: Currency and items added to Thief's inventory

## Theft Mechanics and Rewards

### Success Probability Calculation
```rust
fn calculate_theft_success(hold_duration: f32) -> f32 {
    let base_chance = 0.6;
    let time_bonus = hold_duration * 0.15;
    let total_chance = (base_chance + time_bonus).clamp(0.0, 0.95); // Max 95% success
    
    total_chance
}

fn determine_theft_rewards(target: Entity, hold_duration: f32) -> Vec<StealableItem> {
    let mut available_items = Vec::new();
    
    // Gold theft (always available)
    available_items.push(StealableItem {
        item_type: TheftType::Gold,
        value: calculate_gold_amount(target, hold_duration),
        rarity: ItemRarity::Common,
        steal_difficulty: 0.1,
        description: "Enemy currency".to_string(),
    });
    
    // Buff theft (requires longer hold times)
    if hold_duration >= 1.5 {
        for buff in get_active_buffs(target) {
            available_items.push(StealableItem {
                item_type: TheftType::Buff,
                value: buff.power_level,
                rarity: ItemRarity::Uncommon,
                steal_difficulty: 0.4,
                description: format!("Stolen {}", buff.name),
            });
        }
    }
    
    // Rare items (requires maximum hold time)
    if hold_duration >= 2.5 {
        available_items.push(StealableItem {
            item_type: TheftType::RareItem,
            value: 100,
            rarity: ItemRarity::Rare,
            steal_difficulty: 0.8,
            description: "Valuable carried item".to_string(),
        });
    }
    
    available_items
}
```

### Theft Categories and Rewards
- **Currency (0.5+ seconds)**: 15-50 gold depending on enemy type and hold duration
- **Minor Items (1.0+ seconds)**: Consumables, crafting materials, basic equipment
- **Buff Effects (1.5+ seconds)**: Active buffs transferred from enemy to Thief
- **Rare Treasures (2.5+ seconds)**: High-value items, rare materials, special equipment

## Risk Management and Detection

### Detection Consequences
- **Enemy Alert**: Detected theft causes enemy to become aggressive toward Thief
- **Increased Awareness**: Target becomes immune to pickpocket for 30 seconds
- **Combat Initiation**: Detected Thief may trigger immediate enemy attack
- **Stealth Compromise**: Detection may alert nearby enemies to Thief's presence

### Risk Mitigation Strategies
- **Timing Windows**: Attempt theft during enemy casting or distraction periods
- **Quick Extractions**: Use shorter holds for safer but smaller rewards
- **Escape Planning**: Position for quick retreat in case of detection
- **Team Coordination**: Use ally distractions to reduce detection risk

## Buff Theft and Transfer

### Transferable Effects
- **Damage Buffs**: Attack power increases, critical strike bonuses
- **Defensive Buffs**: Armor increases, damage reduction effects
- **Utility Buffs**: Movement speed, ability haste, resource regeneration
- **Temporary Abilities**: Special powers or enhanced capabilities

### Transfer Mechanics
- **Immediate Application**: Stolen buffs apply to Thief instantly upon successful theft
- **Duration Preservation**: Remaining buff duration transfers with the effect
- **Effect Scaling**: Some buffs may scale differently when applied to Thief
- **Visual Indication**: Clear feedback shows which buffs have been stolen and applied

## Upgrade Paths

### Tier 1: Master Thief
- **Success Rate**: Base 60% → 75% success chance
- **Hold Efficiency**: +15% → +20% success bonus per second held
- **Detection Resistance**: 10% → 5% detection risk per second
- **Strategic Value**: Safer, more reliable theft with improved success rates

### Tier 2: Buff Bandit
- **Buff Duration**: Stolen buffs last 50% longer on Thief than original duration
- **Multi-Theft**: Can steal multiple buffs in single successful attempt
- **Buff Enhancement**: Stolen buffs gain +25% effectiveness when applied to Thief
- **Tactical Evolution**: Transforms into powerful buff management and enhancement tool

### Tier 3: Grand Larceny
- **Passive Integration**: Successful backstabs automatically trigger pickpocket attempt
- **Permanent Theft**: Some stolen buffs become permanent character enhancements
- **Area Theft**: Can steal from all adjacent enemies simultaneously
- **Ultimate Acquisition**: Creates persistent advantage through accumulated stolen benefits

## Strategic Applications

### Economic Warfare
- **Resource Denial**: Reduce enemy economic capabilities through systematic theft
- **Wealth Accumulation**: Build Thief's resources through successful theft operations
- **Team Economy**: Share stolen resources with allies for team economic advantage
- **Opportunity Cost**: Weaken enemies while strengthening Thief simultaneously

### Tactical Debuffing
- **Buff Removal**: Strip important buffs from priority enemies
- **Power Transfer**: Gain enemy advantages for Thief's use
- **Formation Disruption**: Weaken enemy formations through selective buff theft
- **Combat Advantage**: Turn enemy strengths into Thief advantages

## Visual & Audio Design

### Theft Initiation
- **Visual**: Subtle hand movements showing careful approach to target's possessions
- **Animation**: Stealthy, practiced motions emphasizing skill and finesse
- **Audio**: Barely audible sounds of careful manipulation and stealth
- **UI**: Hold progress bar shows success probability and detection risk

### Hold Duration Feedback
- **Visual**: Increasing hand dexterity and confidence in theft motions
- **Animation**: More elaborate theft attempts as hold duration increases
- **Audio**: Tension-building subtle audio cues indicating risk and reward
- **Feedback**: Real-time probability display shows success chance improvement

### Successful Theft
- **Visual**: Smooth, practiced completion of theft with acquired items appearing
- **Animation**: Satisfied gesture as Thief successfully acquires stolen goods
- **Audio**: Quiet satisfaction sound with subtle success audio cues
- **Effect**: Stolen buffs transfer with distinctive particle effects

### Detection and Failure
- **Visual**: Startled enemy reaction with alert animations
- **Animation**: Thief caught in act with embarrassed or surprised reaction
- **Audio**: Sharp detection sound with enemy alert audio cues
- **Consequence**: Clear indication of detection penalties and increased awareness