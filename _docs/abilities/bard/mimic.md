# Mimic

A complete implementation guide for the Bard's reactive ability copying system.

## Overview

The **Mimic** ability showcases the Bard's adaptability through passive observation and reproduction of ally abilities. When positioned adjacent to teammates, the Bard has a 10% chance to automatically recreate any offensive ability used by those allies, effectively doubling the team's damage output potential. This passive ability rewards strategic positioning and creates emergent synergies with different team compositions.

## Game Design Philosophy

This ability demonstrates emergent gameplay design through reactive mechanics:

**Positioning-Based Synergy**: The adjacency requirement creates spatial puzzles where team positioning affects ability synergy potential, adding strategic depth to movement decisions.

**Passive Skill Expression**: Rather than requiring active input, mastery comes through understanding ability interactions and maintaining optimal positioning for maximum benefit.

**Multiplicative Team Enhancement**: The ability scales with team composition quality, rewarding diverse offensive abilities and creating positive feedback loops for tactical coordination.

## Implementation Architecture

### Component-Based Design

```rust
Mimic {
    trigger_chance: 0.10,           // 10% chance to mimic abilities
    adjacency_range: 1.0,           // Must be within 1 tile of ability user
    ability_types: OffensiveOnly,   // Only copies offensive abilities
    cooldown_bypass: true,          // Mimicked abilities ignore original cooldowns
    damage_modifier: 0.8,           // 80% damage of original ability
}

MimicTracker {
    watched_allies: HashSet<Entity>,
    recent_abilities: VecDeque<AbilityEvent>,
    position_cache: HashMap<Entity, GridPos>,
    mimic_queue: Vec<PendingMimic>,
}

PendingMimic {
    ability_id: AbilityId,
    original_caster: Entity,
    target_position: GridPos,
    damage_modifier: f32,
    visual_delay: f32,
}
```

### Event-Driven Systems

The ability coordinates through five reactive systems:
1. **Proximity Monitoring** - Tracks adjacency relationships between Bard and allies
2. **Ability Detection** - Listens for offensive ability usage by adjacent allies
3. **Chance Evaluation** - Determines if mimic triggers based on 10% probability
4. **Ability Reconstruction** - Recreates the original ability with modified parameters
5. **Visual Distinction** - Creates unique effects to distinguish mimicked abilities

## Step-by-Step Gameplay

### Phase 1: Positioning Strategy (Ongoing)
- **Adjacency Awareness**: Maintain position within 1 tile of key damage dealers
- **Ability Prediction**: Anticipate when allies will use powerful offensive abilities
- **Movement Coordination**: Balance mimic positioning with personal safety
- **Team Communication**: Coordinate with allies about ability timing and positioning

### Phase 2: Ability Detection (Passive Monitoring)
- **Event Listening**: System automatically detects offensive ability usage
- **Range Validation**: Confirm Bard is adjacent to ability user
- **Ability Classification**: Verify ability qualifies as offensive type
- **Timing Capture**: Record ability parameters and target information

### Phase 3: Chance Evaluation (10% Trigger)
- **Probability Roll**: Random number generation determines mimic activation
- **Success Indication**: Brief visual flash indicates successful mimic trigger
- **Queue Management**: Add successful mimic to execution queue
- **Cooldown Independence**: Mimicked ability bypasses original cooldown restrictions

### Phase 4: Mimic Execution (0.5 Second Delay)
- **Ability Reconstruction**: Recreate original ability with 80% damage scaling
- **Target Acquisition**: Use original target or select new valid target
- **Visual Distinction**: Apply unique "echo" visual effects to distinguish mimic
- **Audio Feedback**: Musical accompaniment overlays original ability sounds

## Ability Interactions and Synergies

### Compatible Offensive Abilities
- **Direct Damage**: Fireballs, lightning bolts, weapon strikes
- **Area Effects**: Explosions, cone attacks, area denial abilities  
- **Projectiles**: Arrows, thrown weapons, magical missiles
- **Channeled Attacks**: Beam weapons, sustained damage effects

### Enhanced Synergy Targets
- **High-Damage Ultimates**: Maximizes the value of rare, powerful abilities
- **Quick Cooldown Abilities**: Frequent triggering opportunities increase total mimics
- **Area Effect Abilities**: Doubled area coverage can dramatically improve battlefield control
- **Combo Abilities**: Mimicking setup abilities can enable extended combo sequences

## Upgrade Paths

### Tier 1: Improved Resonance
- **Trigger Chance**: 10% → 15% chance to mimic adjacent abilities
- **Damage Retention**: 80% → 90% of original ability damage
- **Visual Enhancement**: More pronounced musical effects during mimic execution
- **Strategic Value**: Increased reliability makes positioning investment more worthwhile

### Tier 2: Extended Harmony
- **Range Expansion**: Adjacency requirement expands to 2-tile radius
- **Multi-Target**: Can potentially mimic multiple abilities simultaneously if multiple allies use abilities
- **Cooldown Reduction**: Successful mimics reduce Bard's own ability cooldowns by 1 second
- **Tactical Flexibility**: Larger range allows safer positioning while maintaining synergy potential

### Tier 3: Perfect Echo
- **Guaranteed Mimic**: 100% chance to copy abilities from allies within 1 tile
- **Full Power**: Mimicked abilities deal 100% damage of original
- **Ability Storage**: Can store one ability to use manually within 10 seconds
- **Master Synergy**: Complete control over when and how to leverage team synergies

## Team Composition Synergies

### Optimal Ally Pairings
- **Burst Damage Characters**: Hunter, Alchemist offensive abilities benefit greatly from duplication
- **Area Controllers**: Forager boulder, Warrior area attacks become twice as effective
- **Combo Initiators**: Thief backstab setups can chain with mimicked follow-up attacks
- **Ultimate Abilities**: High-cooldown, high-impact abilities gain maximum value from free mimics

### Positioning Formations
- **Adjacent Support**: Bard stays next to primary damage dealer for consistent mimic opportunities
- **Central Hub**: Bard positioned centrally to maintain adjacency with multiple allies
- **Dynamic Following**: Bard shadows the most active offensive ally throughout encounters
- **Rotation Strategy**: Team rotates positioning to ensure Bard adjacency during key ability usage

## Visual & Audio Design

### Proximity Indicators
- **Visual**: Subtle connecting lines appear between Bard and adjacent allies
- **UI**: Mimic readiness indicator shows which allies are in mimic range
- **Audio**: Soft harmonic resonance when properly positioned for mimic potential
- **Feedback**: Adjacency indicator pulses when allies prepare offensive abilities

### Mimic Activation
- **Visual**: Musical note symbols appear around Bard when mimic triggers
- **Flash Effect**: Brief golden flash indicates successful mimic activation
- **Audio**: Quick musical flourish announces mimic trigger
- **UI**: Mimic success notification with ability name and damage preview

### Mimicked Ability Execution
- **Visual**: Original ability effect overlaid with musical note particles and golden trim
- **Animation**: Bard performs abbreviated version of original ability's animation
- **Audio**: Original ability sound layered with harmonic musical accompaniment
- **Distinction**: Clear visual indicators prevent confusion between original and mimic

### Positioning Feedback
- **Visual**: Glowing outline around allies within mimic range
- **Movement Trails**: Subtle particle trails help track positioning for optimal adjacency
- **Audio**: Positioning feedback through harmonic chord changes
- **Communication**: Clear indicators help team coordinate for maximum mimic potential