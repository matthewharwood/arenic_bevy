# Transmute

A complete implementation guide for the Alchemist's resource conversion utility ability.

## Overview

The **Transmute** ability represents the Alchemist's mastery over matter transformation, allowing them to convert basic materials and loot into more valuable resources. This utility ability operates on a risk-reward economy system where players can potentially upgrade common drops into rare materials, but with uncertain outcomes. The ability encourages exploration, resource management, and strategic decision-making about when to gamble current resources for potential improvements.

## Game Design Philosophy

This ability demonstrates economic design principles that create meaningful resource management decisions:

**Controlled Randomness with Value**: While outcomes have random elements, the ability guarantees equal or greater value, preventing purely negative results that would frustrate players.

**Inventory Management Pressure**: By requiring proximity to items and having a cooldown, players must make tactical decisions about which items deserve transmutation versus immediate collection.

**Knowledge-Based Optimization**: Experienced players learn optimal timing and item prioritization, creating skill expression through game system mastery rather than mechanical execution.

## Implementation Architecture

### Component-Based Design

```rust
Transmute {
    range: 2.0,                 // 2-tile activation range
    cooldown: 6.0,              // 6 second ability cooldown
    cast_time: 1.5,             // 1.5 second channel duration
    success_rate_base: 0.7,     // 70% chance for upgraded result
    value_guarantee: true,      // Never produces lower value items
}

TransmutationTarget {
    original_item: ItemType,
    target_position: Vec2,
    transmutation_tier: u8,     // 1-3 upgrade potential
    value_multiplier: f32,      // Economic value scaling
}
```

### Event-Driven Systems

The ability operates through five interconnected systems:
1. **Item Detection** - Scans nearby tiles for transmutable objects
2. **Channeling Control** - Manages the 1.5-second casting window
3. **Outcome Calculation** - Determines transmutation results based on item tier
4. **Resource Generation** - Spawns new items and removes originals
5. **Economy Tracking** - Maintains value balance and upgrade statistics

## Step-by-Step Gameplay

### Phase 1: Target Selection (Double-Tap Activation)
- **Input Method**: Double-tap near target item to begin transmutation
- **Range Validation**: Must be within 2 tiles of target item
- **Item Assessment**: UI shows potential upgrade tiers and success probability
- **Strategic Decision**: Evaluate current item value against potential outcomes

### Phase 2: Channeling (1.5 Second Cast)
- **Animation**: Alchemist extends hands with swirling energy effects
- **Vulnerability Window**: Cannot move or use other abilities during channel
- **Visual Feedback**: Energy streams flow from Alchemist to target item
- **Interruption Risk**: Taking damage cancels the transmutation attempt

### Phase 3: Material Analysis (System Processing)
- **Item Evaluation**: System determines base item tier and upgrade potential
- **Success Calculation**: Rolls against 70% base success rate
- **Value Protection**: Failed upgrades still maintain or slightly increase value
- **Outcome Determination**: Selects specific replacement item from upgrade pool

### Phase 4: Transmutation Result (Instant Resolution)
- **Item Replacement**: Original item vanishes, new item appears instantly
- **Visual Effect**: Bright alchemical flash with particle transformation
- **Audio Feedback**: Distinct sounds for success vs. standard transmutation
- **UI Notification**: Brief display showing transmutation outcome and value change

## Upgrade Paths

### Tier 1: Enhanced Probability
- **Success Rate**: 70% → 85% chance for upgraded results
- **Quality Improvement**: Higher likelihood of rare material outcomes
- **Visual Enhancement**: Brighter energy effects during channeling
- **Strategic Value**: Makes transmutation more reliable for high-value items

### Tier 2: Batch Processing
- **Multi-Target**: Can transmute up to 3 items simultaneously within range
- **Efficiency Gain**: Same 1.5-second channel time for multiple targets
- **Resource Management**: Dramatically improves inventory clearing speed
- **Visual Spectacle**: Energy streams connect to multiple items simultaneously

### Tier 3: Alchemical Mastery
- **Guaranteed Upgrades**: 100% success rate with additional tier bonus
- **Value Multiplication**: Successful transmutations gain 1.5x value multiplier
- **Range Extension**: Activation range increases from 2 → 4 tiles
- **Strategic Dominance**: Transforms resource economy for late-game optimization

## Item Transmutation Tables

### Common Item Outcomes (70% Upgrade Success)
- **Success**: Common → Uncommon tier equivalent
- **Standard**: Common → Improved common (1.2x value)
- **Examples**: Iron Ore → Steel Ingot, Basic Potion → Enhanced Potion

### Uncommon Item Outcomes (70% Upgrade Success)  
- **Success**: Uncommon → Rare tier equivalent
- **Standard**: Uncommon → Superior uncommon (1.3x value)
- **Examples**: Steel Ingot → Mithril Ore, Enhanced Potion → Master's Brew

### Rare Item Outcomes (70% Upgrade Success)
- **Success**: Rare → Epic tier equivalent  
- **Standard**: Rare → Perfected rare (1.5x value)
- **Examples**: Mithril Ore → Adamantine Fragment, Master's Brew → Legendary Elixir

## Visual & Audio Design

### Targeting Phase
- **Visual**: Golden highlight outline around valid transmutation targets
- **UI**: Floating tooltip showing item name, current value, and upgrade probability
- **Audio**: Soft chiming sound when valid targets are detected
- **Feedback**: Range indicator shows 2-tile activation radius

### Channeling Phase
- **Visual**: Swirling golden energy streams flowing from hands to target
- **Animation**: Alchemist's robes billow with alchemical wind effects
- **Audio**: Building harmonic resonance with increasing intensity
- **Progress**: Energy intensity grows throughout 1.5-second channel

### Transmutation Resolution
- **Visual**: Brilliant flash of golden light consuming target item
- **Transformation**: Item dissolves into particle stream, reforms as new item
- **Audio**: Success produces clear bell chime, standard produces softer tone
- **UI**: Brief value comparison popup shows economic outcome

### Environmental Effects
- **Particle System**: Golden alchemical symbols float briefly around Alchemist
- **Ground Effect**: Transmutation circle appears temporarily at target location
- **Ambient Enhancement**: Nearby alchemy equipment glows sympathetically
- **Cooldown Indicator**: Ability icon shows 6-second recharge progress