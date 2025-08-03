# Siphon

A complete implementation guide for the Alchemist's life-draining channeled ability.

## Overview

The **Siphon** ability demonstrates the darker side of alchemical mastery, allowing the Alchemist to drain health from nearby allies to fuel their own survival or enhance their combat effectiveness. This morally complex ability creates interesting team dynamics where the Alchemist can act as both support and potential burden, requiring careful communication and strategic timing to maximize benefit while minimizing harm to the team.

## Game Design Philosophy

This ability explores sophisticated team interaction design through controlled risk-reward mechanics:

**Consensual Risk-Sharing**: The ability encourages communication and coordination, as effective use requires willing participation from allies who must position themselves appropriately.

**Dynamic Role Flexibility**: Transforms the traditional support role by adding offensive capability through ally resources, creating unique strategic decisions about resource allocation.

**Moral Complexity in Mechanics**: The life-draining aspect creates narrative tension and forces players to consider the ethical implications of their tactical choices.

## Implementation Architecture

### Component-Based Design

```rust
Siphon {
    channel_range: 4.0,         // 4-tile maximum range
    drain_rate: 15.0,           // 15 HP per second from target
    conversion_rate: 0.8,       // 80% of drained health converted
    max_channel_time: 8.0,      // 8 second maximum duration
    cooldown: 20.0,             // 20 second ability cooldown
    target_type: AllyUnit,      // Can only target friendly units
}

SiphonChannel {
    target_entity: Entity,
    channel_duration: f32,
    health_drained: f32,
    energy_gained: f32,
    visual_connection: Entity,
}
```

### Event-Driven Systems

The ability coordinates through six integrated systems:
1. **Target Selection** - Identifies valid allies within range and line of sight
2. **Channel Management** - Handles continuous drain and conversion calculations
3. **Health Transfer** - Manages damage to ally and benefit to Alchemist
4. **Range Monitoring** - Breaks channel if target moves out of range
5. **Visual Effects** - Maintains the energy transfer visualization
6. **Interruption Handling** - Manages channel breaks from damage or movement

## Step-by-Step Gameplay

### Phase 1: Target Acquisition (Tap & Hold Initiation)
- **Input Method**: Tap and hold while targeting ally within 4-tile range
- **Target Validation**: System confirms ally consent and availability
- **Strategic Assessment**: Evaluate ally's current health and tactical value
- **Communication Window**: Brief moment for ally to consent or reposition

### Phase 2: Channel Establishment (0.5 Second Setup)
- **Visual Connection**: Dark red energy beam connects Alchemist to target
- **Audio Feedback**: Low humming sound indicates successful channel start
- **Movement Restriction**: Both Alchemist and target movement speed reduced by 50%
- **Status Indicators**: UI shows drain rate, conversion efficiency, and channel duration

### Phase 3: Active Siphoning (Up to 8 Seconds)
- **Continuous Drain**: Target loses 15 HP per second
- **Health Conversion**: Alchemist gains 12 HP per second (80% efficiency)
- **Range Monitoring**: Channel breaks if distance exceeds 4 tiles
- **Interruption Conditions**: Any damage to Alchemist immediately breaks channel

### Phase 4: Channel Termination (Release or Break)
- **Manual Release**: Player releases input to end channel voluntarily
- **Automatic Break**: Channel ends at 8-second maximum or ally reaches 25% health
- **Visual Dissipation**: Energy beam fades over 0.5 seconds
- **Effect Resolution**: Final health calculations applied instantly

## Upgrade Paths

### Tier 1: Enhanced Efficiency
- **Conversion Rate**: 80% → 95% health conversion efficiency
- **Drain Rate Reduction**: 15 HP/sec → 12 HP/sec from ally
- **Net Improvement**: Alchemist gains more health per ally health sacrificed
- **Strategic Value**: Makes the ability more ally-friendly while maintaining benefit

### Tier 2: Empowerment Siphon
- **Dual Effect**: Drained health also increases damage output by 25% for 10 seconds
- **Damage Stacking**: Multiple channel sessions can stack damage bonus up to 75%
- **Visual Enhancement**: Alchemist gains red energy aura during damage bonus
- **Tactical Evolution**: Transforms from pure survival to offensive enhancement tool

### Tier 3: Benevolent Drain
- **Health Distribution**: Can redirect siphoned health to other allies instead of self
- **Range Extension**: Target selection range increases to 6 tiles
- **Efficiency Boost**: 100% conversion rate with no health loss during transfer
- **Strategic Mastery**: Enables pure support usage without self-benefit requirements

## Team Dynamics and Communication

### Ally Consent Mechanisms
- **Visual Indicator**: Target ally receives clear UI indication of siphon request
- **Movement Consent**: Ally moving closer signals willingness to participate
- **Denial Method**: Ally moving away or using abilities signals refusal
- **Emergency Break**: Target ally can trigger immediate channel break through specific input

### Tactical Coordination
- **Health Threshold**: Recommended ally health above 60% before initiating
- **Positioning Strategy**: Coordinate positioning to maintain range during channel
- **Timing Windows**: Use during low-intensity combat phases to minimize interruption risk
- **Role Distribution**: Tanks ideal targets due to high health pools and damage mitigation

## Visual & Audio Design

### Channel Initiation
- **Visual**: Dark red targeting reticle appears over valid ally targets
- **UI**: Tooltip shows ally's current health and projected drain amount
- **Audio**: Subtle dark energy gathering sound when targeting
- **Feedback**: Range indicator shows 4-tile channel maintenance requirement

### Active Siphoning
- **Visual**: Continuous flowing dark red energy beam between Alchemist and target
- **Particle Effects**: Small health orbs flow along beam from ally to Alchemist
- **Audio**: Deep harmonic humming with occasional energy pulse sounds
- **Status Display**: Both characters show health change indicators

### Channel Termination
- **Visual**: Energy beam fractures and dissipates with spark effects
- **Audio**: Sharp cutting sound for forced breaks, soft fade for voluntary release
- **UI**: Final health transfer numbers display briefly
- **Recovery**: Brief red aura around Alchemist indicates recent siphon benefit

### Environmental Effects
- **Area Darkening**: Subtle light reduction around active siphon
- **Magical Resonance**: Nearby alchemical equipment shows sympathetic dark energy
- **Ground Effects**: Small dark energy symbols appear beneath both participants
- **Cooldown Visualization**: Ability icon shows 20-second recharge with dark energy theme