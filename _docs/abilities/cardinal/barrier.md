# Barrier

A complete implementation guide for the Cardinal's protective group defense ability.

## Overview

The **Barrier** ability demonstrates the Cardinal's divine protection powers through selective ally shielding within an 8x8 grid area. Using a round-robin targeting system, the Cardinal automatically applies defensive barriers to the nearest ally who hasn't recently received protection, ensuring equitable distribution of defensive benefits across the team. This ability emphasizes tactical positioning and resource management while providing consistent team survival support.

## Game Design Philosophy

This ability showcases equitable support design through automated fair distribution:

**Round-Robin Fairness**: The automatic targeting system prevents favoritism and ensures all team members benefit from protection over time, eliminating player bias in support distribution.

**Proximity-Based Efficiency**: The 8x8 grid range encourages team cohesion while providing reasonable tactical flexibility for positioning.

**Preventive Defense**: Barriers apply before damage occurs, rewarding anticipation and positioning rather than reactive healing.

## Implementation Architecture

### Component-Based Design

```rust
Barrier {
    range: GridArea::new(8, 8),     // 8x8 grid coverage area
    barrier_strength: 75.0,         // 75 HP damage absorption
    barrier_duration: 12.0,         // 12 second barrier lifetime
    cooldown: 5.0,                  // 5 second ability cooldown
    defense_bonus: 0.2,             // 20% damage reduction while active
}

BarrierSystem {
    round_robin_tracker: VecDeque<Entity>,
    last_barrier_times: HashMap<Entity, f32>,
    eligible_targets: HashSet<Entity>,
    barrier_visual_effects: HashMap<Entity, Entity>,
}

DefensiveBarrier {
    remaining_absorption: f32,
    duration_remaining: f32,
    defense_modifier: f32,
    visual_effect: Entity,
    original_caster: Entity,
}
```

### Event-Driven Systems

The ability operates through five coordinated systems:
1. **Target Selection** - Implements round-robin algorithm for fair ally selection
2. **Proximity Detection** - Identifies valid allies within 8x8 grid range
3. **Barrier Application** - Creates defensive shields with absorption and duration tracking
4. **Damage Interception** - Modifies incoming damage calculations for protected allies
5. **Visual Management** - Maintains protective aura effects and status indicators

## Step-by-Step Gameplay

### Phase 1: Automatic Target Assessment (Pre-Activation)
- **Range Scanning**: System identifies all allies within 8x8 grid area
- **Round-Robin Logic**: Selects ally who has waited longest since last barrier
- **Eligibility Check**: Excludes allies who already have active barriers
- **Priority Override**: Critically injured allies (below 25% health) gain selection priority

### Phase 2: Instant Barrier Application (Tap Activation)
- **Input Method**: Single tap triggers immediate barrier on selected target
- **Effect Application**: Target gains 75 HP absorption shield and 20% damage reduction
- **Visual Feedback**: Golden protective aura materializes around selected ally
- **UI Update**: Barrier status appears in ally's health display with countdown timer

### Phase 3: Active Protection (12 Second Duration)
- **Damage Absorption**: Barrier absorbs up to 75 damage before breaking
- **Defense Enhancement**: All incoming damage reduced by 20% before absorption
- **Visual Persistence**: Golden aura maintains intensity proportional to remaining strength
- **Status Tracking**: UI shows remaining absorption and duration for team awareness

### Phase 4: Barrier Resolution (Expiration or Depletion)
- **Natural Expiration**: Barrier fades after 12 seconds if not depleted
- **Damage Depletion**: Barrier breaks when 75 HP absorption is exhausted
- **Visual Dissipation**: Golden aura gradually fades over 1 second
- **Round-Robin Update**: Target becomes eligible for future barrier selection

## Round-Robin Distribution System

### Selection Algorithm
```rust
fn select_barrier_target(allies: &[Entity], tracker: &mut VecDeque<Entity>) -> Option<Entity> {
    // Remove allies not in range or with active barriers
    let eligible: Vec<Entity> = allies.iter()
        .filter(|ally| is_in_range(**ally) && !has_active_barrier(**ally))
        .cloned()
        .collect();
    
    // Priority for critically injured allies
    if let Some(critical) = eligible.iter().find(|ally| health_percentage(**ally) < 0.25) {
        return Some(*critical);
    }
    
    // Round-robin selection from eligible allies
    while let Some(next) = tracker.pop_front() {
        if eligible.contains(&next) {
            tracker.push_back(next); // Move to end of queue
            return Some(next);
        }
    }
    
    // Add new allies to tracker
    for ally in eligible {
        if !tracker.contains(&ally) {
            tracker.push_back(ally);
        }
    }
    
    tracker.pop_front()
}
```

### Fairness Mechanisms
- **Equal Opportunity**: All allies cycle through selection queue
- **Critical Priority**: Low-health allies bypass normal rotation
- **Cooldown Respect**: Recently protected allies wait longer before re-selection
- **Range Awareness**: Out-of-range allies removed from consideration but retain queue position

## Upgrade Paths

### Tier 1: Enhanced Protection
- **Absorption Increase**: 75 HP → 100 HP damage absorption capacity
- **Duration Extension**: 12 seconds → 16 seconds barrier lifetime
- **Defense Boost**: 20% → 30% damage reduction while barrier active
- **Visual Enhancement**: Brighter golden aura with more pronounced protective effects

### Tier 2: Group Reinforcement
- **Multi-Target**: Applies barriers to 2 nearest eligible allies simultaneously
- **Shared Strength**: If one barrier breaks, remaining barrier gains 25 HP absorption
- **Cooldown Reduction**: 5 seconds → 3.5 seconds between barrier applications
- **Strategic Evolution**: Enables more frequent protection for larger teams

### Tier 3: Divine Aegis
- **Area Protection**: Creates 3x3 protective zone around Cardinal granting 15% damage reduction
- **Barrier Overflow**: When barriers absorb damage, 25% reflects back to attackers
- **Perfect Coverage**: Automatic barrier refresh when allies drop below 50% health
- **Ultimate Defense**: Combines personal and ally protection for comprehensive team security

## Positioning Strategy

### Optimal Cardinal Placement
- **Central Authority**: Position near team center to maximize 8x8 grid coverage
- **Tank Proximity**: Stay close to main tank for consistent barrier application
- **Retreat Routes**: Maintain positioning allowing safe withdrawal while providing support
- **Line of Sight**: Ensure clear view of all allies for effective targeting

### Team Formation Coordination
- **Cluster Benefits**: Tight formation ensures all allies remain within barrier range
- **Tank Priority**: Main tank typically receives first barrier due to damage exposure
- **Damage Dealer Coverage**: Ensure glass cannon allies included in round-robin rotation
- **Support Chain**: Other support characters gain barrier protection for sustained team healing

## Visual & Audio Design

### Pre-Activation
- **Visual**: 8x8 grid outline appears showing barrier coverage area
- **Target Indicator**: Selected ally highlighted with golden outline preview
- **UI**: Round-robin queue display shows next barrier recipients
- **Audio**: Soft divine harmony builds as Cardinal prepares blessing

### Barrier Application
- **Visual**: Brilliant golden light descends from above onto selected ally
- **Animation**: Protective aura materializes as shimmering golden shell
- **Audio**: Clear divine bell chime indicates successful barrier activation
- **Particle**: Golden motes of light swirl protectively around target

### Active Protection
- **Visual**: Steady golden aura pulses gently around protected ally
- **Intensity**: Aura brightness corresponds to remaining absorption strength
- **Audio**: Subtle harmonic resonance when barrier absorbs damage
- **UI**: Barrier strength indicator shows absorption remaining and duration

### Barrier Resolution
- **Visual**: Gradual fade of golden aura over 1-second dissipation period
- **Audio**: Soft chime indicates natural expiration, sharp crack indicates damage break
- **Particle**: Final golden sparkles disperse as barrier completes its protection
- **Feedback**: UI notification confirms barrier completion and round-robin advancement