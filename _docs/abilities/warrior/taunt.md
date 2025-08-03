# Taunt

A complete implementation guide for the Warrior's threat redirection and aggro management utility ability.

## Overview

The **Taunt** ability represents the Warrior's mastery over battlefield control through targeted threat redirection. When activated with tap-and-hold mechanics, the ability forces nearby enemies within a 2x2 area to direct their projectiles and attacks toward the Warrior for a limited duration. This aggro management tool maintains enemy movement patterns and timing while redirecting damage away from vulnerable allies, making it essential for team protection and formation control.

## Game Design Philosophy

This ability demonstrates threat management design through targeting redirection mechanics:

**Threat Redistribution**: The ability redirects damage without altering enemy timing or movement, maintaining deterministic gameplay while providing tactical team protection.

**Area-Based Selection**: The 2x2 targeting area creates spatial decisions about which enemies to taunt while rewarding positioning and formation awareness.

**Sacrifice for Team**: The Warrior accepts increased personal risk to protect allies, embodying the classic tank role through strategic threat assumption.

## Implementation Architecture

### Component-Based Design

```rust
Taunt {
    area_size: GridArea::new(2, 2),     // 2x2 grid targeting area
    duration: 6.0,                      // 6 second taunt effect duration
    cast_time: 0.8,                     // 0.8 second hold-to-cast time
    cooldown: 12.0,                     // 12 second ability cooldown
    threat_redirection: true,           // Forces enemy targeting toward Warrior
    movement_preservation: true,        // Enemy movement patterns unchanged
    range: 3.0,                         // Maximum 3-tile placement range
}

TauntEffect {
    center_position: GridPos,
    affected_area: GridArea,
    taunted_enemies: HashSet<Entity>,
    duration_remaining: f32,
    threat_source: Entity,              // The Warrior who applied taunt
    visual_effect: Entity,
}

ThreatRedirection {
    enemy: Entity,
    forced_target: Entity,              // Must target the taunting Warrior
    original_targeting: Option<Entity>, // Store original target for restoration
    redirection_timer: f32,
    visual_indicator: Entity,
}
```

### Event-Driven Systems

The ability operates through five threat management systems:
1. **Area Targeting** - Handles 2x2 area selection and enemy identification
2. **Threat Redirection** - Forces affected enemies to target Warrior
3. **Duration Management** - Tracks taunt effect timing and expiration
4. **Targeting Override** - Modifies enemy attack target selection without changing timing
5. **Visual Coordination** - Shows taunted area and threat redirection effects

## Step-by-Step Gameplay

### Phase 1: Area Selection (Tap and Hold Initiation)
- **Input Method**: Tap and hold to begin taunt area targeting
- **Range Validation**: Can target 2x2 area up to 3 tiles away from Warrior
- **Area Preview**: Targeting reticle shows 2x2 area and affected enemies
- **Hold Duration**: Must maintain hold for 0.8 seconds to complete taunt

### Phase 2: Taunt Casting (0.8 Second Channel)
- **Channel Animation**: Warrior performs threatening gestures and vocalizations
- **Vulnerability Window**: Warrior cannot move during 0.8-second cast time
- **Area Confirmation**: Final 2x2 area determined at moment of cast completion
- **Enemy Assessment**: System identifies all enemies within target area

### Phase 3: Threat Redirection (6 Second Duration)
- **Targeting Override**: All affected enemies must target Warrior for attacks
- **Movement Preservation**: Enemy movement patterns remain completely unchanged
- **Attack Scheduling**: Enemy attack timing stays on original schedule
- **Damage Concentration**: All redirected attacks focus on Warrior

### Phase 4: Taunt Expiration (Natural Duration End)
- **Targeting Restoration**: Enemies return to normal targeting behavior
- **Visual Fade**: Taunt effects gradually diminish over final second
- **Threat Reset**: Enemies resume original target priorities and behavior
- **Cooldown Start**: 12-second cooldown begins for next taunt usage

## Threat Redirection Mechanics

### Target Override System
```rust
fn apply_taunt_effect(warrior: Entity, target_area: GridArea) -> Vec<Entity> {
    let enemies_in_area = get_enemies_in_area(target_area);
    let mut taunted_enemies = Vec::new();
    
    for enemy in enemies_in_area {
        if is_valid_taunt_target(enemy) {
            let original_target = get_current_target(enemy);
            
            // Store original targeting for restoration
            let threat_redirect = ThreatRedirection {
                enemy,
                forced_target: warrior,
                original_targeting: original_target,
                redirection_timer: 6.0,
                visual_indicator: spawn_taunt_indicator(enemy),
            };
            
            // Override enemy targeting
            set_forced_target(enemy, warrior);
            add_status_effect(enemy, threat_redirect);
            taunted_enemies.push(enemy);
        }
    }
    
    taunted_enemies
}

fn update_taunted_enemy_behavior(enemy: Entity, warrior: Entity) {
    // Force all attacks and projectiles to target the Warrior
    if is_about_to_attack(enemy) {
        override_attack_target(enemy, warrior);
    }
    
    // Maintain original movement and timing patterns
    continue_original_movement_pattern(enemy);
    maintain_attack_timing_schedule(enemy);
}
```

### Targeting Rules
- **Forced Target**: Taunted enemies must attack Warrior regardless of original target
- **Range Independence**: Taunt works regardless of distance between enemy and Warrior
- **Projectile Redirection**: All ranged attacks redirect toward Warrior
- **Melee Consideration**: Melee enemies may need to approach Warrior if within taunt range

## Strategic Applications

### Team Protection
- **Damage Dealer Shield**: Protect fragile damage dealers from enemy focus
- **Healer Defense**: Redirect attacks away from support characters
- **Formation Control**: Manage enemy targeting to maintain team formation integrity
- **Crisis Management**: Use during overwhelming enemy assault phases

### Positioning Strategy
- **Central Tanking**: Position Warrior to intercept redirected attacks effectively
- **Formation Leadership**: Use taunt to control battlefield flow and enemy pressure
- **Ally Positioning**: Enable allies to position aggressively while protected
- **Escape Facilitation**: Redirect enemy attention to allow ally repositioning

## Area Selection and Timing

### Optimal Area Placement
- **Enemy Clustering**: Target areas with multiple high-threat enemies
- **Formation Disruption**: Taunt enemies positioned to threaten team formation
- **Priority Threats**: Focus on enemies with powerful attacks or special abilities
- **Tactical Concentration**: Group enemy attacks for more predictable threat management

### Timing Considerations
- **Predictive Taunting**: Cast taunt before enemies enter dangerous attack phases
- **Formation Transitions**: Use during team movement or positioning changes
- **Emergency Response**: React to sudden threats against vulnerable allies
- **Cooldown Planning**: Balance immediate needs with future threat requirements

## Upgrade Paths

### Tier 1: Enhanced Presence
- **Area Expansion**: 2x2 → 3x3 grid coverage for taunt effect
- **Duration Extension**: 6 seconds → 9 seconds taunt duration
- **Cast Speed**: 0.8 → 0.5 seconds channel time
- **Strategic Value**: Larger coverage with longer duration and faster activation

### Tier 2: Commanding Presence
- **Damage Reduction**: Warrior takes 25% less damage from taunted enemies
- **Threat Amplification**: Taunted enemies deal 20% more damage (risk-reward)
- **Cooldown Reduction**: 12 seconds → 8 seconds between taunt uses
- **Tactical Evolution**: Balances increased survivability with heightened risk

### Tier 3: Battlefield Dominance
- **Global Taunt**: Can taunt all enemies on battlefield regardless of position
- **Damage Reflection**: 30% of damage from taunted enemies reflects back to them
- **Persistent Threat**: Enemies killed while taunted spread taunt to nearby enemies
- **Ultimate Control**: Transforms into battlefield-wide threat management system

## Team Coordination and Communication

### Formation Management
- **Tank Positioning**: Warrior positions to handle concentrated attack streams
- **Ally Awareness**: Team recognizes taunt usage and adjusts positioning accordingly
- **Formation Support**: Allies position to support Warrior during taunt periods
- **Escape Routes**: Team maintains positioning for tactical withdrawal if needed

### Communication Strategy
- **Taunt Callouts**: Clear communication about taunt timing and target selection
- **Threat Assessment**: Team identifies priority enemies for taunt targeting
- **Formation Coordination**: Coordinate team movement with taunt usage timing
- **Emergency Protocols**: Establish clear signals for emergency taunt usage

## Visual & Audio Design

### Taunt Activation
- **Visual**: Warrior assumes aggressive stance with intimidating gestures
- **Animation**: Threatening postures and vocalizations directed at target area
- **Audio**: Powerful war cry or challenge directed at enemies
- **Area**: 2x2 targeting area clearly outlined with threat indicators

### Threat Redirection Effects
- **Visual**: Red threat lines connect taunted enemies to Warrior
- **Animation**: Enemy attack animations clearly redirect toward Warrior
- **Audio**: Enemy vocalizations and attack sounds orient toward Warrior
- **Status**: Taunted enemies show clear visual indicators of forced targeting

### Concentrated Defense
- **Visual**: Warrior gains defensive aura indicating heightened threat absorption
- **Animation**: Defensive posture emphasizing readiness for concentrated attacks
- **Audio**: Determined defensive sounds showing readiness for incoming damage
- **Feedback**: Clear indication that Warrior is successfully protecting allies

### Taunt Expiration
- **Visual**: Threat redirection lines fade as enemies return to normal targeting
- **Animation**: Enemies return to original postures and targeting behavior
- **Audio**: Taunt effects fade with enemies returning to normal vocalizations
- **Transition**: Clear indication that threat redirection has ended