# Backstab

A complete implementation guide for the Thief's positional damage enhancement passive ability.

## Overview

The **Backstab** ability represents the Thief's mastery over positional combat through enhanced damage from advantageous positions. This passive ability automatically triggers enhanced damage when the Thief attacks enemies from behind or while concealed, rewarding tactical positioning and stealth gameplay. The ability emphasizes spatial awareness and positioning skill over timing-based mechanics, creating consistent strategic value throughout combat encounters.

## Game Design Philosophy

This ability demonstrates positional design through spatial skill expression:

**Spatial Skill Rewards**: Success depends on understanding positioning, enemy facing, and battlefield spatial relationships rather than timing or resource management.

**Consistent Passive Value**: The always-active nature ensures positioning skills are continuously rewarded without requiring active ability management or cooldown timing.

**Risk-Reward Positioning**: Achieving optimal backstab positions often requires tactical risk-taking, creating interesting decisions about safety versus damage potential.

## Implementation Architecture

### Component-Based Design

```rust
Backstab {
    damage_multiplier: 1.75,            // 75% damage increase from behind
    concealment_multiplier: 2.0,        // 100% damage increase while concealed
    detection_angle: 120.0,             // 120-degree cone for "behind" detection
    bleed_damage: 10.0,                 // 10 damage per second bleed effect
    bleed_duration: 6.0,                // 6 second bleed duration
    activation_type: Passive,           // Always active, no input required
}

PositionalAttack {
    attacker: Entity,
    target: Entity,
    attack_angle: f32,
    target_facing: f32,
    is_behind: bool,
    is_concealed: bool,
    damage_multiplier: f32,
}

BleedEffect {
    damage_per_tick: f32,
    duration_remaining: f32,
    tick_interval: 1.0,                 // Damage every 1 second
    source_entity: Entity,
    visual_effect: Entity,
}
```

### Event-Driven Systems

The ability operates through four positioning systems:
1. **Position Analysis** - Calculates attack angles and determines backstab validity
2. **Damage Modification** - Applies positional damage multipliers to Thief attacks
3. **Bleed Application** - Manages damage over time effects from successful backstabs
4. **Visual Coordination** - Shows positional advantages and backstab success feedback

## Step-by-Step Gameplay

### Phase 1: Positional Assessment (Continuous Analysis)
- **Spatial Awareness**: System continuously tracks Thief position relative to enemies
- **Angle Calculation**: Determines attack angle compared to enemy facing direction
- **Concealment Check**: Evaluates if Thief is concealed by smoke screen or other effects
- **Opportunity Identification**: Highlights optimal positioning for backstab opportunities

### Phase 2: Attack Execution (Normal Attack Input)
- **Standard Input**: Use normal attack controls - backstab triggers automatically
- **Position Validation**: System confirms backstab conditions at moment of attack
- **Damage Calculation**: Apply appropriate multiplier based on position and concealment status
- **Automatic Activation**: No special input required - positioning determines enhancement

### Phase 3: Damage Application (Enhanced Impact)
- **Multiplier Application**: 75% damage increase from behind, 100% while concealed
- **Bleed Initiation**: Successful backstabs apply 6-second bleeding effect
- **Visual Enhancement**: Distinctive backstab effects show positioning success
- **Audio Feedback**: Satisfying backstab sound confirms positional advantage

### Phase 4: Bleed Effect (6 Second DOT)
- **Periodic Damage**: Target takes 10 damage every second for 6 seconds
- **Visual Persistence**: Bleeding effects continue throughout duration
- **Stacking Potential**: Multiple backstabs can apply multiple bleed effects
- **Strategic Value**: Provides ongoing damage even after Thief repositions

## Position Detection Mechanics

### Behind Detection Algorithm
```rust
fn is_backstab_position(attacker_pos: Vec2, target_pos: Vec2, target_facing: f32) -> bool {
    let attack_vector = (attacker_pos - target_pos).normalize();
    let target_facing_vector = Vec2::from_angle(target_facing);
    
    // Calculate angle between target's facing and attack direction
    let angle_difference = attack_vector.angle_between(target_facing_vector).abs();
    
    // Backstab if attack comes from behind (within 120 degree cone)
    angle_difference > (180.0 - 60.0).to_radians()
}

fn calculate_backstab_damage(base_damage: f32, attacker: Entity, target: Entity) -> f32 {
    let is_behind = is_backstab_position(
        get_position(attacker),
        get_position(target),
        get_facing_angle(target)
    );
    
    let is_concealed = has_concealment_effect(attacker);
    
    if is_concealed {
        base_damage * 2.0  // 100% damage increase while concealed
    } else if is_behind {
        base_damage * 1.75 // 75% damage increase from behind
    } else {
        base_damage        // Normal damage
    }
}
```

### Concealment Integration
- **Smoke Screen Synergy**: Concealment from smoke screen enables maximum damage multiplier
- **Stealth Mechanics**: Any concealment effect triggers enhanced backstab damage
- **Visual Clarity**: Clear indicators show when concealment backstab bonus is active
- **Strategic Coordination**: Coordinate with smoke screen ability for optimal damage timing

## Tactical Positioning Strategy

### Optimal Movement Patterns
- **Flanking Maneuvers**: Circle around enemies to attack from behind
- **Formation Exploitation**: Use enemy clustering to find backstab opportunities
- **Concealment Timing**: Coordinate movement with smoke screen for concealed attacks
- **Hit-and-Run**: Attack from behind, then reposition for continued advantage

### Battlefield Awareness
- **Enemy Facing**: Monitor enemy orientation for backstab opportunities
- **Formation Gaps**: Identify spaces in enemy formations for positioning
- **Mobility Usage**: Use shadow step and other movement to reach optimal positions
- **Risk Assessment**: Balance backstab positioning with personal safety

## Bleed Effect Management

### DOT Stacking Strategy
- **Multiple Applications**: Layer bleed effects through repeated backstabs
- **Target Prioritization**: Focus bleeds on high-health or priority targets
- **Damage Optimization**: Combine immediate backstab damage with sustained bleed
- **Resource Efficiency**: Bleed provides ongoing value after repositioning

### Bleed Mechanics
- **Damage Type**: Physical damage that bypasses armor reduction
- **Duration**: 6 seconds per application, multiple bleeds run simultaneously
- **Visual Tracking**: Clear indicators show which enemies are bleeding
- **Strategic Value**: Provides pressure even when Thief cannot maintain positioning

## Upgrade Paths

### Tier 1: Enhanced Positioning
- **Damage Multiplier**: Behind attacks: 75% → 100% damage increase
- **Concealment Bonus**: Concealed attacks: 100% → 150% damage increase
- **Bleed Improvement**: 10 → 15 damage per second bleed effect
- **Strategic Value**: Higher damage rewards for successful positioning

### Tier 2: Bleeding Mastery
- **Bleed Enhancement**: Each bleed tick increases by 2 damage (escalating damage)
- **Duration Extension**: Bleed duration: 6 → 9 seconds
- **Bleed Spread**: Enemy death with active bleed spreads effect to nearby enemies
- **DOT Evolution**: Transforms bleed into powerful area denial tool

### Tier 3: Shadow Assassin
- **Guaranteed Criticals**: All backstab attacks are guaranteed critical hits
- **Concealment Extension**: Successful backstabs grant 2 seconds of concealment
- **Chain Backstabs**: Killing enemy with backstab grants 3 seconds of enhanced movement speed
- **Ultimate Positioning**: Combines massive damage with mobility and stealth benefits

## Spatial Awareness and Positioning

### Enemy Facing Recognition
- **Visual Indicators**: Clear indicators show enemy facing direction
- **Positioning Feedback**: UI shows optimal positioning for backstab opportunities
- **Angle Calculation**: Real-time feedback on attack angle relative to backstab threshold
- **Movement Prediction**: Anticipate enemy turning to maintain positional advantage

### Formation Analysis
- **Enemy Clusters**: Identify opportunities to backstab multiple enemies
- **Positioning Windows**: Recognize temporary openings for backstab positioning
- **Escape Routes**: Plan positioning that allows retreat after backstab execution
- **Team Coordination**: Work with allies to create backstab opportunities

## Visual & Audio Design

### Position Feedback
- **Visual**: Enemy outlines change color when Thief is in backstab position
- **UI**: Directional indicators show optimal movement for backstab positioning
- **Audio**: Subtle audio cues when moving into backstab range
- **Feedback**: Real-time positioning feedback helps players learn spatial relationships

### Backstab Execution
- **Visual**: Distinctive attack animation showing precise, lethal positioning
- **Animation**: Enhanced strike effects emphasizing positioning advantage
- **Audio**: Satisfying backstab sound effect distinct from normal attacks
- **Impact**: Special particle effects show positioning-based damage enhancement

### Bleed Application
- **Visual**: Blood effects appear on target with ongoing bleed indicators
- **Animation**: Periodic damage visualization every second during bleed
- **Audio**: Subtle damage sounds with each bleed tick
- **Status**: Clear bleed icon shows duration and damage per tick

### Concealment Backstab
- **Visual**: Dramatic concealed strike effects with shadow/stealth themes
- **Animation**: Attack appears to come from concealment with surprise emphasis
- **Audio**: Enhanced backstab sound with stealth/concealment audio layers
- **Feedback**: Maximum damage numbers with special concealment styling