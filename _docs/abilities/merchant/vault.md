# Vault

A complete implementation guide for the Merchant's area-effect damage amplification zone ability.

## Overview

The **Vault** ability represents the Merchant's mastery over territorial wealth enhancement through magical economic zones. When activated, the ability creates a 4x4 grid area that doubles all critical hit damage for any character standing within it for 10 seconds. This territorial buff ability encourages strategic positioning and team coordination while providing significant damage amplification that stacks with multiple Merchants for exponential benefits.

## Game Design Philosophy

This ability demonstrates territorial enhancement design through zone-based damage amplification:

**Territorial Strategy**: The fixed area encourages strategic positioning decisions where players must balance optimal damage zones with tactical safety and mobility needs.

**Multiplicative Team Benefits**: The critical damage doubling affects all characters, creating team-wide value that rewards coordination and formation planning.

**Stacking Merchant Synergy**: Multiple Merchants can create overlapping vault zones, encouraging merchant team compositions and cooperative economic strategies.

## Implementation Architecture

### Component-Based Design

```rust
Vault {
    area_size: GridArea::new(4, 4),     // 4x4 grid coverage
    duration: 10.0,                     // 10 second effect duration
    crit_damage_multiplier: 2.0,        // 2x critical hit damage
    cooldown: 30.0,                     // 30 second ability cooldown
    placement_range: 3.0,               // Can place up to 3 tiles away
    stacking_allowed: true,             // Multiple vaults can overlap
    visual_prominence: High,            // Clear visual boundaries
}

VaultZone {
    center_position: GridPos,
    coverage_area: GridArea,
    duration_remaining: f32,
    damage_multiplier: f32,
    characters_inside: HashSet<Entity>,
    visual_effect: Entity,
    boundary_indicators: Vec<Entity>,
}

CriticalDamageBonus {
    base_critical_multiplier: f32,      // Character's normal crit multiplier
    vault_bonus: f32,                   // Additional multiplier from vault
    total_multiplier: f32,              // Combined critical damage multiplier
    vault_sources: Vec<Entity>,         // Which vaults are affecting character
}
```

### Event-Driven Systems

The ability operates through five zone-management systems:
1. **Zone Placement** - Handles vault positioning and area validation
2. **Occupancy Tracking** - Monitors which characters are within vault boundaries
3. **Damage Amplification** - Modifies critical hit damage calculations for zone occupants
4. **Stacking Management** - Handles multiple overlapping vault effects
5. **Visual Coordination** - Manages zone boundaries and effect indicators

## Step-by-Step Gameplay

### Phase 1: Vault Placement (Tap Activation)
- **Input Method**: Tap to place 4x4 vault zone at target location
- **Range Validation**: Can place vault up to 3 tiles away from Merchant's position
- **Area Selection**: 4x4 grid appears at target location with clear boundary indicators
- **Strategic Consideration**: Evaluate optimal placement for team positioning and safety

### Phase 2: Zone Activation (Immediate Effect)
- **Instant Activation**: Vault zone becomes active immediately upon placement
- **Visual Manifestation**: Golden treasure vault appearance with clear 4x4 boundaries
- **Damage Boost**: All characters within zone gain 2x critical hit damage multiplier
- **Team Communication**: Clear visual and audio cues indicate active vault zone

### Phase 3: Tactical Positioning (10 Second Duration)
- **Formation Optimization**: Team positions within vault boundaries for damage benefits
- **Mobility Decisions**: Balance staying in vault versus tactical movement needs
- **Combat Coordination**: Time critical hit abilities while positioned in vault zone
- **Threat Assessment**: Evaluate safety of maintaining vault positioning versus enemy threats

### Phase 4: Zone Expiration (Duration End)
- **Visual Fade**: Vault zone effects gradually diminish over final 2 seconds
- **Benefit Cessation**: Critical damage multiplier returns to normal for all characters
- **Tactical Transition**: Team repositions as needed without vault positioning constraints
- **Cooldown Start**: 30-second cooldown begins for next vault placement

## Zone Coverage and Positioning

### Area Management
```rust
fn is_character_in_vault(character_pos: GridPos, vault_center: GridPos) -> bool {
    let vault_area = GridArea::new(4, 4).centered_on(vault_center);
    vault_area.contains(character_pos)
}

fn calculate_total_crit_multiplier(character: Entity) -> f32 {
    let base_multiplier = get_base_crit_multiplier(character);
    let vault_bonuses: Vec<f32> = get_affecting_vaults(character)
        .iter()
        .map(|vault| vault.damage_multiplier)
        .collect();
    
    // Vault bonuses stack multiplicatively
    let total_vault_bonus = vault_bonuses.iter().product::<f32>();
    
    base_multiplier * total_vault_bonus
}

fn handle_vault_stacking(overlapping_vaults: &[VaultZone]) -> f32 {
    // Multiple vaults stack multiplicatively: 2x * 2x = 4x damage
    overlapping_vaults
        .iter()
        .map(|vault| vault.damage_multiplier)
        .product()
}
```

### Strategic Placement Considerations
- **Team Formation**: Position vault to cover maximum team members during combat
- **Escape Routes**: Ensure vault placement doesn't trap team in dangerous positions
- **Enemy Patterns**: Consider enemy area attacks that might threaten vault occupants
- **Objective Control**: Place vaults near important battlefield objectives or choke points

## Critical Damage Amplification

### Damage Calculation
- **Base Critical**: Character's normal critical hit damage (typically 2x normal damage)
- **Vault Enhancement**: Vault doubles the critical multiplier (2x becomes 4x)
- **Stacking Vaults**: Multiple vaults multiply bonuses (2 vaults = 8x critical damage)
- **Maximum Potential**: Theoretical maximum with multiple merchants creating overlapping zones

### Synergy with Other Abilities
- **Dice Stacking**: Combine with Merchant dice ability for guaranteed high-damage criticals
- **Team Criticals**: Works with any character's critical hits, not just Merchant abilities
- **Burst Windows**: Coordinate with team ultimate abilities for devastating damage phases
- **Economic Synergy**: High damage in vault zones improves fortune loot generation

## Team Coordination Strategy

### Optimal Formation
- **Central Clustering**: Team groups within 4x4 area during high-damage phases
- **Role Distribution**: Ensure damage dealers prioritize vault positioning
- **Support Coverage**: Healers and support characters benefit from vault positioning
- **Tank Integration**: Tanks can use vault zones to amplify threat generation through damage

### Timing Coordination
- **Ability Synchronization**: Coordinate team ultimate abilities with vault placement
- **Enemy Vulnerability**: Place vaults during enemy vulnerability windows
- **Formation Transitions**: Plan vault usage around team formation changes
- **Resource Management**: Balance vault cooldown with other team ultimate timings

## Upgrade Paths

### Tier 1: Enhanced Vault
- **Duration Extension**: 10 → 15 seconds vault lifetime
- **Size Increase**: 4x4 → 5x5 grid coverage area
- **Damage Multiplier**: 2x → 2.5x critical damage enhancement
- **Strategic Value**: Larger, longer-lasting zones with higher damage potential

### Tier 2: Compound Interest
- **Stacking Bonus**: Each overlapping vault adds +0.5x instead of full multiplication
- **Movement Bonus**: Characters in vault gain 25% movement speed
- **Ability Haste**: 15% faster cooldowns for characters within vault
- **Tactical Evolution**: Adds mobility and utility benefits beyond pure damage

### Tier 3: Treasure Fortress
- **Permanent Zones**: Vaults last until Merchant places a new one (no duration limit)
- **Defensive Bonus**: Characters in vault take 25% less damage
- **Resource Generation**: Vault zones generate 1 gold per second for occupants
- **Ultimate Territory**: Creates permanent economic and tactical advantages

## Multi-Merchant Synergy

### Vault Stacking Mechanics
- **Multiplicative Stacking**: Multiple vault bonuses multiply together
- **Exponential Scaling**: 2+ Merchants can create extremely high damage zones
- **Coordination Requirements**: Multiple Merchants must coordinate placement timing
- **Resource Investment**: Multiple Merchants using cooldowns for single powerful zone

### Team Composition Benefits
- **Merchant Teams**: 2-3 Merchants create devastating damage amplification zones
- **Mixed Compositions**: Single Merchant provides significant team damage boost
- **Economic Synergy**: Multiple Merchants amplify each other's economic abilities
- **Tactical Flexibility**: Overlapping vaults create multiple strategic positioning options

## Visual & Audio Design

### Zone Placement
- **Visual**: Merchant opens magical vault portal at target location
- **Animation**: Treasure vault materializes with golden light and wealth effects
- **Audio**: Satisfying vault opening sound with magical enhancement tones
- **Boundaries**: Clear golden outline marks 4x4 area with corner indicators

### Active Vault Zone
- **Visual**: Persistent golden aura with floating treasure and coin particles
- **Boundary**: Shimmering golden walls mark vault boundaries clearly
- **Audio**: Subtle background sound of wealth and prosperity
- **Occupancy**: Characters within zone gain golden outline and treasure particle effects

### Damage Amplification
- **Visual**: Critical hits within vault show enhanced particle effects
- **Animation**: Amplified damage numbers with golden styling and larger size
- **Audio**: Enhanced critical hit sounds with wealth-themed audio layers
- **Feedback**: Screen effects emphasize the increased damage potential

### Zone Expiration
- **Visual**: Vault gradually fades with treasure particles dispersing
- **Animation**: Vault portal closes with diminishing golden effects
- **Audio**: Vault closing sound with echo indicating effect end
- **Transition**: Clear indication that damage bonuses have ended