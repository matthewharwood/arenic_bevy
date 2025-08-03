# Dice

A complete implementation guide for the Merchant's stacking critical chance self-enhancement ability.

## Overview

The **Dice** ability represents the Merchant's mastery over probability manipulation through magical gambling mechanics. Each activation grants a stackable 1% critical strike chance bonus that persists until used, creating a risk-reward system where patience and timing can lead to devastating critical attacks. This self-buffing ability emphasizes strategic accumulation and optimal timing for maximum impact during crucial combat moments.

## Game Design Philosophy

This ability demonstrates accumulation design through stackable probability enhancement:

**Patience Rewards**: The stacking mechanism rewards players who can delay gratification and accumulate bonuses for optimal timing rather than immediate use.

**Probability Manipulation**: The critical strike focus creates exciting moments where accumulated luck transforms routine attacks into game-changing critical hits.

**Resource-Free Investment**: The instant cast and lack of resource costs encourage frequent use during downtime, creating ongoing tactical decisions about when to accumulate versus when to deploy.

## Implementation Architecture

### Component-Based Design

```rust
Dice {
    crit_chance_per_stack: 0.01,        // 1% critical chance per stack
    max_stacks: 50,                     // Maximum 50 stacks (50% crit chance)
    stack_duration: f32::INFINITY,      // Stacks persist until used
    cast_time: 0.0,                     // Instant activation
    cooldown: 0.5,                      // 0.5 second cooldown between casts
    consumption_trigger: CriticalHit,   // All stacks consumed on critical hit
}

DiceStacks {
    current_stacks: u32,
    total_crit_chance: f32,
    visual_effect: Entity,
    stack_indicators: Vec<Entity>,
    accumulated_luck: f32,
}

CriticalStrike {
    base_damage: f32,
    critical_multiplier: f32,           // Typically 2.0x damage
    dice_enhanced: bool,
    stacks_consumed: u32,
    visual_effect: Entity,
}
```

### Event-Driven Systems

The ability operates through five gambling-themed systems:
1. **Stack Accumulation** - Manages dice roll stacking and visual indicators
2. **Critical Calculation** - Modifies base critical hit chance with accumulated stacks
3. **Stack Consumption** - Removes all stacks when critical hit occurs
4. **Visual Management** - Shows stack count and probability enhancement effects
5. **Luck Tracking** - Monitors accumulated probability for strategic decision-making

## Step-by-Step Gameplay

### Phase 1: Stack Accumulation (Repeated Activation)
- **Input Method**: Tap to add 1 stack of critical chance (1% per stack)
- **Instant Effect**: Stack immediately applies to Merchant's critical hit probability
- **Visual Feedback**: Dice appear around Merchant showing current stack count
- **Strategic Decision**: Continue stacking versus using current accumulated chance

### Phase 2: Enhanced Probability (Passive Modification)
- **Critical Calculation**: All attacks modified by accumulated critical chance percentage
- **Stack Persistence**: Stacks remain active until critical hit occurs
- **Risk Assessment**: Higher stacks create higher reward potential but longer investment
- **Timing Consideration**: Evaluate when accumulated chance justifies aggressive combat

### Phase 3: Critical Hit Trigger (Attack Resolution)
- **Probability Roll**: Enhanced critical chance applied to all offensive actions
- **Critical Success**: When critical hit occurs, all accumulated stacks provide benefit
- **Damage Amplification**: Critical hits deal enhanced damage with visual flair
- **Satisfaction Payoff**: Accumulated patience pays off with devastating attack

### Phase 4: Stack Consumption (Post-Critical)
- **Complete Reset**: All accumulated stacks consumed in single critical hit
- **Visual Dissipation**: Dice effects dramatically dissolve after critical strike
- **Cycle Restart**: Merchant can begin accumulating new stacks immediately
- **Strategic Reset**: New accumulation phase begins with fresh tactical decisions

## Stack Management Strategy

### Accumulation Tactics
```rust
fn optimal_stacking_strategy(combat_intensity: f32, enemy_health: f32) -> StackDecision {
    let current_stacks = get_dice_stacks();
    let current_crit_chance = current_stacks as f32 * 0.01;
    
    if combat_intensity < 0.3 && current_stacks < 20 {
        StackDecision::KeepStacking
    } else if enemy_health < 30.0 && current_crit_chance > 0.15 {
        StackDecision::UseNow
    } else if current_stacks >= 40 {
        StackDecision::UseNext
    } else {
        StackDecision::Evaluate
    }
}

fn calculate_expected_value(stacks: u32, time_investment: f32) -> f32 {
    let crit_chance = stacks as f32 * 0.01;
    let expected_damage_bonus = crit_chance * CRITICAL_MULTIPLIER;
    let opportunity_cost = time_investment * NORMAL_DPS;
    
    expected_damage_bonus - opportunity_cost
}
```

### Timing Optimization
- **Low-Intensity Accumulation**: Stack during safe periods or enemy downtime
- **High-Value Targets**: Save high stacks for dangerous or valuable enemies
- **Emergency Usage**: Use moderate stacks when immediate damage is needed
- **Risk Management**: Balance accumulation greed with tactical necessity

## Probability Mathematics

### Critical Chance Scaling
- **Base Rate**: Merchant's normal critical hit chance (typically 5-10%)
- **Stack Enhancement**: Each stack adds exactly 1% to critical probability
- **Maximum Benefit**: 50 stacks = 50% bonus critical chance (55-60% total)
- **Diminishing Psychology**: Higher stacks feel more valuable due to investment

### Expected Value Calculations
- **1-10 Stacks**: Low investment, reasonable to use for moderate threats
- **11-25 Stacks**: Medium investment, save for significant targets
- **26-40 Stacks**: High investment, reserve for major threats or bosses
- **41-50 Stacks**: Maximum investment, only use for critical moments

## Upgrade Paths

### Tier 1: Loaded Dice
- **Stack Value**: 1% → 1.5% critical chance per stack
- **Cooldown Reduction**: 0.5 → 0.3 seconds between activations
- **Stack Limit**: 50 → 60 maximum stacks
- **Strategic Value**: Faster accumulation with higher ceiling for extreme critical chances

### Tier 2: Lucky Streak
- **Partial Consumption**: Critical hits only consume half of accumulated stacks
- **Chain Criticals**: Critical hits grant temporary 25% critical chance for 3 seconds
- **Lucky Sevens**: Every 7th stack grants bonus damage on next attack regardless of critical
- **Risk Mitigation**: Reduces all-or-nothing nature while adding consistent benefits

### Tier 3: Probability Master
- **Guaranteed Criticals**: At 30+ stacks, next attack is guaranteed critical hit
- **Stack Sharing**: Can transfer up to 20 stacks to nearby allies
- **Critical Overflow**: Critical hits create area explosions proportional to stacks consumed
- **Ultimate Control**: Perfect mastery over probability with team-wide benefits

## Combat Integration and Synergy

### Ability Combinations
- **Coin Toss Setup**: Use coin toss to increase base damage before triggering critical
- **Fortune Timing**: Activate fortune before using accumulated stacks for enhanced rewards
- **Vault Coordination**: Use accumulated criticals within merchant vault areas for maximum impact
- **Team Synergy**: Time critical strikes to coincide with ally abilities

### Strategic Applications
- **Boss Encounters**: Accumulate maximum stacks before major boss fights
- **Finishing Moves**: Use moderate stacks to guarantee kills on low-health enemies
- **Emergency Burst**: Deploy accumulated chance when team needs immediate high damage
- **Economic Efficiency**: Balance time investment in stacking against immediate combat needs

## Visual & Audio Design

### Stack Accumulation
- **Visual**: Floating dice appear around Merchant, increasing in number with each stack
- **Animation**: New dice materialize with satisfying rolling animation
- **Audio**: Satisfying dice rolling sound with increasing pitch for higher stacks
- **UI**: Stack counter shows current stacks and total critical chance percentage

### Probability Enhancement
- **Visual**: Merchant's weapons glow with increasing intensity based on stack count
- **Particle**: Luck-themed sparkles and fortune symbols around Merchant
- **Audio**: Subtle magical humming that builds with accumulated probability
- **Status**: Critical chance indicator prominently displayed in UI

### Critical Hit Payoff
- **Visual**: Explosive critical effects enhanced by number of stacks consumed
- **Animation**: Dramatic strike animation with enhanced particle effects
- **Audio**: Satisfying critical impact sound with gambling win audio cues
- **Screen Effect**: Brief screen flash and shake proportional to stacks used

### Stack Consumption
- **Visual**: All dice dramatically explode in burst of golden light
- **Animation**: Spectacular dissolution of accumulated luck into critical strike
- **Audio**: Jackpot-style sound effect as stacks convert to critical damage
- **Feedback**: Clear indication that investment has paid off with massive damage