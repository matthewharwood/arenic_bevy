# Cleanse

A complete implementation guide for the Bard's debuff removal utility ability.

## Overview

The **Cleanse** ability represents the Bard's mastery over purifying harmonies, allowing them to remove all debuffs from allies within a 4x4 grid area. This powerful utility ability serves as the team's primary counter to enemy status effects, creating windows of opportunity during boss encounters with heavy debuff mechanics. The ability emphasizes timing, positioning, and team coordination to maximize its impact.

## Game Design Philosophy

This ability demonstrates counterplay design principles that create dynamic interaction with enemy mechanics:

**Counterplay Over Immunity**: Rather than preventing debuffs entirely, Cleanse creates opportunities for tactical recovery, maintaining the threat of enemy abilities while providing skilled counterplay options.

**Area Effect Team Support**: The 4x4 grid encourages team clustering and coordination, creating interesting positioning puzzles where allies must balance safety with cleanse coverage.

**Timing-Based Skill Expression**: The 10-second cooldown and instant cast create decision points about when to cleanse versus when to save the ability for more critical moments.

## Implementation Architecture

### Component-Based Design

```rust
Cleanse {
    range: GridArea::new(4, 4),     // 4x4 grid coverage
    cast_time: 0.0,                 // Instant activation
    cooldown: 10.0,                 // 10 second ability cooldown
    effect_type: DebuffRemoval::All, // Removes all debuff types
    visual_duration: 2.0,           // 2 second purification effect
}

CleanseArea {
    center_position: GridPos,
    affected_allies: Vec<Entity>,
    purification_wave: Entity,
    debuffs_removed: HashMap<Entity, Vec<DebuffType>>,
}
```

### Event-Driven Systems

The ability operates through four coordinated systems:
1. **Area Detection** - Identifies all allies within the 4x4 grid area
2. **Debuff Scanning** - Catalogs all active debuffs on detected allies
3. **Purification Application** - Removes debuffs and applies cleanse immunity
4. **Visual Orchestration** - Manages the harmony wave and particle effects

## Step-by-Step Gameplay

### Phase 1: Tactical Assessment (Pre-Activation)
- **Debuff Analysis**: Identify which allies have dangerous debuffs
- **Positioning Check**: Ensure maximum allies within 4x4 grid coverage
- **Timing Decision**: Balance immediate need against future threat potential
- **Coordination Signal**: Communicate cleanse timing to team for positioning

### Phase 2: Instant Activation (Tap Input)
- **Input Method**: Single tap triggers immediate area effect
- **Range Validation**: All allies within 4x4 grid automatically targeted
- **Effect Application**: All debuffs removed simultaneously from valid targets
- **Visual Initiation**: Harmonic wave begins expanding from Bard's position

### Phase 3: Purification Wave (2 Second Visual)
- **Wave Expansion**: Golden harmonic energy spreads through affected area
- **Ally Highlighting**: Cleansed allies gain brief golden aura effect
- **Debuff Visualization**: Removed debuffs appear as dissipating dark particles
- **Audio Feedback**: Clear harmonic chord progression indicates successful cleanse

### Phase 4: Recovery Window (10 Second Cooldown)
- **Ability Recharge**: Cleanse unavailable for 10 seconds
- **Strategic Positioning**: Team repositions for potential future cleanses
- **Debuff Monitoring**: Track new debuff applications for next cleanse timing
- **Cooldown Communication**: Inform team of cleanse availability status

## Debuff Types Affected

### Status Effect Removal
- **Damage Over Time**: Poison, burn, bleed, and acid effects
- **Movement Impairment**: Slow, root, and movement speed reduction
- **Combat Penalties**: Damage reduction, accuracy debuffs, and silence effects
- **Resource Drain**: Mana burn, energy siphon, and regeneration blocking

### Special Interactions
- **Curse Effects**: Removes magical curses but not permanent transformations
- **Boss Mechanics**: Cleanses most boss-applied debuffs but not phase-specific markers
- **Environmental**: Removes environmental debuffs like toxic atmosphere or corruption
- **Stacking Effects**: Removes all stacks of stackable debuffs instantly

## Upgrade Paths

### Tier 1: Extended Coverage
- **Area Increase**: 4x4 grid → 6x6 grid coverage area
- **Range Flexibility**: Allows cleansing of more spread-out team formations
- **Visual Enhancement**: Larger harmonic wave with increased particle density
- **Strategic Value**: Reduces positioning requirements for effective team coverage

### Tier 2: Protective Harmony
- **Debuff Immunity**: Cleansed allies gain 3-second debuff immunity
- **Tactical Window**: Creates safe period for aggressive positioning or abilities
- **Visual Indicator**: Cleansed allies maintain faint golden aura during immunity
- **Strategic Depth**: Allows team to engage in risky tactics after cleanse

### Tier 3: Resonant Recovery
- **Health Restoration**: Cleanse also heals allies for 25% of maximum health
- **Dual Utility**: Combines debuff removal with significant healing support
- **Mana Efficiency**: Single ability addresses multiple team support needs
- **Visual Spectacle**: Healing orbs accompany harmonic wave expansion

## Positioning Strategy

### Optimal Formation
- **Bard Centralization**: Position Bard at center of ally cluster for maximum coverage
- **Tank Proximity**: Ensure tank within cleanse range during boss debuff phases
- **Damage Dealer Coverage**: Balance DPS positioning needs with cleanse accessibility
- **Escape Routes**: Maintain positioning that allows retreat after cleanse

### Emergency Clustering
- **Debuff Crisis**: Team converges on Bard position when multiple debuffs threaten
- **Communication Signals**: Establish clear signals for emergency clustering
- **Movement Coordination**: Practice rapid formation changes without enemy exposure
- **Recovery Dispersion**: Quick spread after cleanse to avoid area attacks

## Visual & Audio Design

### Pre-Activation
- **Visual**: 4x4 grid outline appears around Bard showing cleanse coverage
- **UI**: Debuff icons highlight on affected allies to show cleanse targets
- **Audio**: Soft harmonic buildup as Bard prepares the purification melody
- **Feedback**: Valid targets show subtle golden outline preview

### Activation Moment
- **Visual**: Brilliant flash of golden light emanates from Bard's position
- **Audio**: Perfect harmonic chord strikes with crystalline clarity
- **Effect**: Debuff icons disappear from all affected allies simultaneously
- **Particle**: Dark debuff particles dissolve into sparkling golden motes

### Purification Wave
- **Visual**: Expanding ring of golden harmonic energy flows outward
- **Animation**: Musical notes and treble clef symbols dance within the wave
- **Audio**: Sustained harmonic progression with ascending melody line
- **Duration**: 2-second wave expansion with gradual fade

### Recovery Phase
- **Visual**: Bard's instrument glows softly during cooldown period
- **UI**: Cleanse icon shows 10-second countdown with musical note theme
- **Audio**: Gentle background harmony indicates ability recharging
- **Feedback**: Ability icon pulses when cleanse becomes available again