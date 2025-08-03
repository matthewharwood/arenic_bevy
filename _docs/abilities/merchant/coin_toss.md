# Coin Toss

A complete implementation guide for the Merchant's risk-reward economic projectile ability.

## Overview

The **Coin Toss** ability demonstrates the Merchant's mastery over economic warfare through high-stakes gambling mechanics. Players spend actual currency to launch a skill-shot projectile that deals significant damage on successful hits while potentially returning the investment plus profit. The ability features up to 5-second charge time for increased damage and a 10-second cooldown, creating meaningful resource management decisions.

## Game Design Philosophy

This ability showcases economic risk-reward design through literal monetary investment:

**Real Stakes Gambling**: Using actual currency creates genuine risk-reward tension where missed shots have meaningful economic consequences beyond just cooldown loss.

**Skill-Reward Scaling**: The charge mechanism and skill-shot targeting ensure that economic investment requires mechanical skill to justify the risk.

**Economic Positive-Sum**: The money-back potential on hits creates sustainable economic gameplay where skilled players can maintain or grow their wealth through combat.

## Implementation Architecture

### Component-Based Design

```rust
CoinToss {
    base_cost: 25,                      // 25 gold base cost per toss
    charge_time_max: 5.0,               // Maximum 5 second charge duration
    damage_scaling: PowerCurve::Quadratic, // Damage scales quadratically with charge
    base_damage: 100.0,                 // 100 damage at minimum charge
    max_damage: 400.0,                  // 400 damage at maximum charge
    money_return_chance: 0.8,           // 80% chance to return money on hit
    profit_bonus: 0.5,                  // 50% bonus money on successful hit
    cooldown: 10.0,                     // 10 second ability cooldown
}

CoinProjectile {
    position: Vec2,
    velocity: Vec2,
    charge_level: f32,                  // 0.0 to 1.0 charge progression
    damage_amount: f32,
    cost_invested: u32,
    visual_effect: Entity,
    trail_particles: Vec<Entity>,
}

EconomicResult {
    hit_success: bool,
    damage_dealt: f32,
    money_invested: u32,
    money_returned: u32,
    profit_earned: u32,
}
```

### Event-Driven Systems

The ability operates through six economic systems:
1. **Investment Management** - Handles currency deduction and validation
2. **Charge Mechanics** - Manages hold-to-charge timing and damage scaling
3. **Skill Shot Physics** - Handles projectile trajectory and collision detection
4. **Economic Resolution** - Calculates money return and profit on successful hits
5. **Visual Coordination** - Manages coin projectile effects and economic feedback
6. **Risk Assessment** - Tracks economic performance and investment success rates

## Step-by-Step Gameplay

### Phase 1: Economic Investment (Tap and Hold)
- **Currency Check**: Verify Merchant has sufficient funds (25+ gold)
- **Investment Commitment**: Gold immediately deducted upon ability activation
- **Charge Initiation**: Begin holding input to charge projectile power
- **Economic Risk**: Money is spent regardless of success, creating real stakes

### Phase 2: Power Charging (Hold Duration 0-5 Seconds)
- **Damage Scaling**: Damage increases quadratically from 100 to 400 based on charge time
- **Visual Buildup**: Coin projectile grows in size and intensity during charge
- **Strategic Decision**: Balance damage potential against precision difficulty
- **Maximum Threshold**: Charge caps at 5 seconds for maximum damage potential

### Phase 3: Skill Shot Release (Release Input)
- **Trajectory Determination**: Projectile launches in aimed direction at release moment
- **Precision Requirement**: Success depends on player's aiming skill and timing
- **High Stakes**: Missed shots result in complete loss of invested currency
- **Economic Pressure**: Real money on the line creates tension and focus

### Phase 4: Economic Resolution (Impact or Miss)
- **Hit Success**: 80% chance to return investment plus 50% profit bonus
- **Hit Failure**: 20% chance to lose investment despite successful hit
- **Miss Penalty**: Complete loss of invested gold with no compensation
- **Economic Feedback**: Clear indication of financial result and running profit/loss

## Damage and Economic Scaling

### Charge Time to Damage Conversion
```rust
fn calculate_coin_damage(charge_time: f32, base_damage: f32, max_damage: f32) -> f32 {
    let charge_ratio = (charge_time / 5.0).clamp(0.0, 1.0);
    // Quadratic scaling rewards longer charges exponentially
    let damage_multiplier = charge_ratio * charge_ratio;
    
    base_damage + (max_damage - base_damage) * damage_multiplier
}

fn calculate_economic_result(hit: bool, charge_level: f32, base_cost: u32) -> EconomicResult {
    let cost_invested = base_cost;
    
    if !hit {
        return EconomicResult {
            hit_success: false,
            money_invested: cost_invested,
            money_returned: 0,
            profit_earned: 0,
            ..Default::default()
        };
    }
    
    let return_chance = 0.8;
    let returns_money = random() < return_chance;
    
    if returns_money {
        let profit_bonus = (cost_invested as f32 * 0.5) as u32;
        EconomicResult {
            hit_success: true,
            money_invested: cost_invested,
            money_returned: cost_invested,
            profit_earned: profit_bonus,
            ..Default::default()
        }
    } else {
        EconomicResult {
            hit_success: true,
            money_invested: cost_invested,
            money_returned: 0,
            profit_earned: 0,
            ..Default::default()
        }
    }
}
```

### Economic Efficiency Analysis
- **Break-Even**: Need 80% hit rate to maintain economic neutrality
- **Profit Zone**: Hit rates above 80% generate positive economic returns
- **Loss Mitigation**: Even successful hits have 20% chance of no money return
- **Risk Assessment**: High-skill players can maintain profitable combat economics

## Strategic Economic Management

### Investment Timing
- **Safe Targets**: Use coin toss on stationary or predictable enemies
- **High-Value Opportunities**: Invest during enemy vulnerability windows
- **Economic State**: Consider current wealth before making risky investments
- **Opportunity Cost**: Balance coin toss against other economic abilities

### Risk Mitigation Strategies
- **Conservative Charging**: Use shorter charges for higher accuracy on difficult targets
- **Aggressive Charging**: Use maximum charge on easy targets for maximum damage/profit
- **Economic Reserves**: Maintain minimum currency reserves for non-combat purchases
- **Diversified Strategy**: Mix coin toss with other merchant abilities for balanced risk

## Upgrade Paths

### Tier 1: Better Odds
- **Return Chance**: 80% → 90% chance to return money on successful hits
- **Profit Increase**: 50% → 75% profit bonus on money-returning hits
- **Cost Reduction**: 25 → 20 gold base cost per coin toss
- **Strategic Value**: Improved economic efficiency makes ability more sustainable

### Tier 2: Golden Opportunity
- **Multi-Hit**: Coin bounces once to hit additional enemy for 50% damage
- **Charge Efficiency**: Damage scaling improved, reaching max damage at 3 seconds instead of 5
- **Economic Bonus**: Multiple hits can trigger multiple money returns
- **Tactical Evolution**: Transforms from single-target to area-effect economic tool

### Tier 3: Midas Touch
- **Guaranteed Returns**: 100% chance to return money plus profit on any hit
- **Scaling Profits**: Profit percentage increases with charge level (50% to 150%)
- **Currency Generation**: Enemies killed by coin toss drop bonus gold
- **Economic Mastery**: Eliminates financial risk while maximizing profit potential

## Skill Shot Mechanics and Precision

### Aiming and Trajectory
- **Manual Targeting**: Player must aim projectile manually for optimal accuracy
- **Charge Difficulty**: Longer charges create larger, slower projectiles (easier to hit)
- **Range Scaling**: Charge level affects maximum projectile travel distance
- **Environmental Interaction**: Coins can ricochet off certain surfaces at higher charge levels

### Accuracy Factors
- **Target Movement**: Moving enemies require prediction and leading shots
- **Charge Tradeoff**: Higher damage but larger projectile size affects precision needs
- **Environmental Obstacles**: Walls and barriers can block coin trajectory
- **Distance Compensation**: Longer shots require higher charge for effective damage

## Visual & Audio Design

### Investment Phase
- **Visual**: Merchant pulls out gleaming gold coin with magical enhancement effects
- **Animation**: Dramatic coin flip with slow-motion emphasis on investment decision
- **Audio**: Satisfying coin flip sound with magical enhancement chimes
- **UI**: Current gold amount and investment cost clearly displayed

### Charge Building
- **Visual**: Coin grows larger and more brilliant with increasing charge time
- **Particle**: Golden energy swirls around coin with increasing intensity
- **Audio**: Building magical hum with increasing pitch and richness
- **Feedback**: Charge level indicator shows optimal damage timing

### Projectile Flight
- **Visual**: Brilliant golden coin spinning through air with particle trail
- **Animation**: Realistic coin rotation with magical enhancement effects
- **Audio**: Whistling coin sound with magical energy undertones
- **Trail**: Golden particle stream marks projectile path

### Economic Resolution
- **Success Visual**: Explosion of golden light with currency symbols on hit
- **Success Audio**: Satisfying impact sound followed by cash register success chime
- **Failure Visual**: Coin shatters with disappointed particle effects on miss
- **Failure Audio**: Sad trombone or similar "loss" sound effect

### Economic Feedback
- **Visual**: Clear profit/loss indicators show economic result
- **Animation**: Currency symbols float up for gains, sink down for losses
- **Audio**: Distinct success vs. failure audio cues for economic outcomes
- **UI**: Running profit/loss tracker shows cumulative economic performance