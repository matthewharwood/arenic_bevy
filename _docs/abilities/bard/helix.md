# Helix

A complete implementation guide for the Bard's dual-purpose aura utility ability.

## Overview

The **Helix** ability represents the Bard's mastery over dual harmonies, creating a togglable aura that provides either rapid healing (Regeneration Mode) or enhanced movement and attack speed (Haste Mode). This versatile utility ability allows the Bard to adapt their support role to match the current tactical situation, switching between sustain and acceleration as battlefield conditions change.

## Game Design Philosophy

This ability demonstrates adaptive support design through modal gameplay mechanics:

**Situational Adaptation Over Static Support**: The toggle mechanism rewards players who can read battlefield conditions and provide the appropriate type of support at the right moment.

**Clear Mechanical Distinction**: The two modes offer fundamentally different benefits that address distinct tactical needs, preventing mode choice from becoming arbitrary.

**Persistent Active Decision-Making**: The toggle nature requires ongoing evaluation and active management rather than set-and-forget gameplay.

## Implementation Architecture

### Component-Based Design

```rust
Helix {
    aura_radius: 3.0,               // 3-tile radius effect
    toggle_cooldown: 2.0,           // 2 second cooldown between mode switches
    mana_drain_per_second: 10.0,    // Continuous mana cost while active
    max_duration: 30.0,             // 30 second maximum uptime per activation
}

HelixAura {
    mode: HelixMode,                // Regeneration or Haste
    affected_allies: HashSet<Entity>,
    duration_remaining: f32,
    visual_effect: Entity,
}

// Regeneration Mode
HelixRegeneration {
    heal_per_second: 8.0,           // 8 HP/sec to allies in range
    bonus_mana_regen: 5.0,          // 5 MP/sec to allies in range
}

// Haste Mode  
HelixHaste {
    movement_speed_bonus: 0.3,      // 30% movement speed increase
    attack_speed_bonus: 0.25,       // 25% attack speed increase
    ability_cooldown_reduction: 0.15, // 15% faster ability recharge
}
```

### Event-Driven Systems

The ability operates through six interconnected systems:
1. **Mode Management** - Handles toggle input and mode transitions
2. **Aura Detection** - Maintains list of allies within effect radius
3. **Regeneration Application** - Applies healing and mana restoration effects
4. **Haste Application** - Modifies movement and combat speed statistics
5. **Resource Management** - Tracks mana consumption and duration limits
6. **Visual Coordination** - Manages distinct visual effects for each mode

## Step-by-Step Gameplay

### Phase 1: Mode Selection (Initial Activation)
- **Input Method**: Tap to activate in Regeneration Mode (default)
- **Mode Toggle**: Tap again during active aura to switch to Haste Mode
- **Strategic Assessment**: Evaluate team needs for healing versus acceleration
- **Resource Check**: Ensure sufficient mana for desired duration

### Phase 2: Regeneration Mode (Healing Focus)
- **Area Effect**: All allies within 3-tile radius receive healing aura
- **Health Restoration**: 8 HP per second continuous healing
- **Mana Support**: 5 MP per second mana regeneration boost
- **Visual Identity**: Soft blue-green swirling energy around affected allies

### Phase 3: Haste Mode (Speed Enhancement)
- **Movement Boost**: 30% faster movement speed for affected allies
- **Combat Acceleration**: 25% faster attack animations and ability usage
- **Cooldown Reduction**: All abilities recharge 15% faster
- **Visual Identity**: Bright yellow-gold energy with quick-moving particles

### Phase 4: Mode Management (Active Toggling)
- **Toggle Cooldown**: 2-second delay between mode switches prevents spam
- **Tactical Switching**: Respond to changing battlefield conditions
- **Resource Monitoring**: Track mana consumption and remaining duration
- **Team Communication**: Signal mode changes to optimize ally positioning

## Mode-Specific Strategies

### Regeneration Mode Applications
- **Sustained Encounters**: Long boss fights requiring health maintenance
- **Post-Damage Recovery**: After taking significant area damage
- **Resource Conservation**: When mana users need regeneration support
- **Defensive Positioning**: Supporting allies during defensive phases

### Haste Mode Applications
- **Pursuit Phases**: Chasing retreating enemies or escaping danger
- **Burst Damage Windows**: Maximizing damage during boss vulnerability phases
- **Mobility Challenges**: Navigating environmental hazards or tight timing sections
- **Aggressive Pushes**: Supporting coordinated team advances

## Upgrade Paths

### Tier 1: Enhanced Efficiency
- **Mana Cost Reduction**: 10 MP/sec → 7 MP/sec drain rate
- **Duration Extension**: 30 seconds → 45 seconds maximum uptime
- **Toggle Improvement**: Mode switch cooldown reduced to 1 second
- **Strategic Value**: Allows longer sustained support with more flexible switching

### Tier 2: Amplified Effects
- **Regeneration Boost**: Healing increases to 12 HP/sec, mana regen to 8 MP/sec
- **Haste Enhancement**: Speed bonuses increase to 40% movement, 35% attack speed
- **Radius Extension**: Aura range increases from 3 tiles to 4 tiles
- **Visual Upgrade**: More intense particle effects matching improved potency

### Tier 3: Harmonic Convergence
- **Dual Mode**: Both effects active simultaneously at 75% efficiency
- **Perfect Balance**: No mode switching required, provides both benefits
- **Resource Optimization**: Mana cost increases to only 12 MP/sec for dual benefits
- **Master Support**: Ultimate flexibility allowing response to any situation

## Positioning and Team Coordination

### Optimal Aura Coverage
- **Central Positioning**: Bard maintains position to cover maximum allies
- **Formation Awareness**: Team clusters appropriately for aura benefits
- **Dynamic Movement**: Bard follows team movement to maintain coverage
- **Range Communication**: Clear signals when allies move outside aura range

### Mode Communication
- **Visual Indicators**: Distinct particle effects clearly communicate current mode
- **Audio Cues**: Different musical themes for each mode
- **UI Notifications**: Mode changes announced to team through interface
- **Tactical Callouts**: Vocal communication about upcoming mode switches

## Visual & Audio Design

### Regeneration Mode
- **Visual**: Soft flowing blue-green energy spirals around affected allies
- **Particles**: Gentle healing orbs that drift upward from healed characters
- **Audio**: Soothing harmonic progressions with nature-inspired tones
- **Animation**: Bard's instrument glows with cool blue-green light

### Haste Mode
- **Visual**: Dynamic yellow-gold energy with rapid particle movement
- **Particles**: Swift streaks of light that emphasize speed and motion
- **Audio**: Energetic musical themes with accelerated tempo
- **Animation**: Bard's movements become more animated and energetic

### Mode Transitions
- **Visual**: Brief rainbow spiral effect during the 2-second toggle cooldown
- **Audio**: Musical bridge phrase connecting the two mode themes
- **Animation**: Bard performs transitional dance move while switching modes
- **UI**: Mode indicator clearly shows current state and toggle availability

### Aura Boundaries
- **Visual**: Subtle ring indicator shows 3-tile radius coverage area
- **Feedback**: Allies entering/leaving aura show brief particle flash
- **Communication**: Range indicator helps teammates maintain positioning
- **Clarity**: Distinct visual boundaries prevent confusion about coverage