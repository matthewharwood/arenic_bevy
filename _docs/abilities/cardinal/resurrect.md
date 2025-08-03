# Resurrect

A complete implementation guide for the Cardinal's ultimate revival and tactical enhancement ability.

## Overview

The **Resurrect** ability serves as the Cardinal's most powerful support tool, providing both ally revival and enhanced enemy telegraph vision. When activated within a 4x4 grid area, the ability can restore fallen allies to life while simultaneously improving the entire team's ability to anticipate and avoid enemy attacks through enhanced visual warnings. This dual-purpose ultimate ability requires careful timing and positioning but can dramatically shift the tide of difficult encounters.

## Game Design Philosophy

This ability demonstrates ultimate ability design through combined utility and tactical enhancement:

**High-Impact Scarcity**: The 1-minute cooldown ensures resurrect remains a pivotal moment rather than routine gameplay, creating meaningful decision points about timing.

**Dual Utility Design**: Combining revival with enemy telegraph enhancement creates value even when no allies need resurrection, preventing the ability from becoming situational dead weight.

**Team Coordination Catalyst**: The enhanced enemy visibility benefits the entire team, creating moments where the Cardinal's ultimate transforms group tactics and positioning.

## Implementation Architecture

### Component-Based Design

```rust
Resurrect {
    range: GridArea::new(4, 4),     // 4x4 grid coverage area
    revival_health: 0.5,            // Revive allies at 50% maximum health
    channel_time: 2.0,              // 2 second channel duration
    cooldown: 60.0,                 // 60 second (1 minute) cooldown
    telegraph_enhancement_duration: 15.0, // 15 second enhanced vision
    telegraph_visibility_boost: 2.0, // 2x longer enemy warning times
}

ResurrectEffect {
    revival_targets: Vec<Entity>,
    affected_area: GridArea,
    channel_progress: f32,
    visual_intensity: f32,
    telegraph_enhancement: Entity,
}

EnhancedTelegraph {
    duration_remaining: f32,
    visibility_multiplier: f32,
    affected_players: HashSet<Entity>,
    enhanced_warnings: HashMap<Entity, f32>,
}
```

### Event-Driven Systems

The ability coordinates through six sophisticated systems:
1. **Area Scanning** - Identifies fallen allies and living allies within 4x4 range
2. **Channel Management** - Handles 2-second channel with full vulnerability
3. **Revival Processing** - Restores fallen allies to 50% health upon completion
4. **Telegraph Enhancement** - Improves enemy attack warnings for entire team
5. **Visual Orchestration** - Manages divine resurrection effects and aura enhancement
6. **Ultimate Tracking** - Monitors cooldown and availability for strategic timing

## Step-by-Step Gameplay

### Phase 1: Strategic Assessment (Pre-Activation)
- **Tactical Evaluation**: Assess value of revival versus telegraph enhancement
- **Positioning Analysis**: Move to optimal location covering maximum fallen allies
- **Timing Consideration**: Evaluate safety window for 2-second channel vulnerability
- **Team Communication**: Coordinate ultimate usage with team strategy

### Phase 2: Channel Initiation (Tap Activation)
- **Input Method**: Single tap begins 2-second resurrection channel
- **Area Effect**: All fallen allies within 4x4 grid marked for revival
- **Movement Lock**: Cardinal cannot move during channel but can be interrupted
- **Visual Buildup**: Intense divine energy accumulates with increasing brightness

### Phase 3: Vulnerable Channeling (2 Second Duration)
- **Interruption Risk**: Any damage to Cardinal cancels the resurrection attempt
- **Visual Intensity**: Divine light builds to brilliant crescendo throughout channel
- **Audio Building**: Choir-like harmonics rise toward divine climax
- **Team Protection**: Allies often provide cover during Cardinal's vulnerability

### Phase 4: Resurrection Resolution (Instant Effect)
- **Ally Revival**: All fallen allies in area return to life at 50% maximum health
- **Telegraph Enhancement**: All team members gain enhanced enemy warning vision
- **Visual Climax**: Explosion of divine light followed by sustained golden aura
- **Audio Completion**: Triumphant divine chord signaling successful resurrection

### Phase 5: Enhanced Vision Period (15 Second Duration)
- **Team Benefit**: All allies see enemy attack telegraphs 2x longer than normal
- **Tactical Advantage**: Improved positioning and dodge timing for entire team
- **Visual Indicator**: Team members gain subtle golden aura indicating enhancement
- **Strategic Window**: 15 seconds of superior battlefield awareness

## Revival Mechanics

### Target Selection
```rust
fn identify_revival_targets(cardinal_pos: GridPos) -> Vec<Entity> {
    let area = GridArea::new(4, 4).centered_on(cardinal_pos);
    
    get_entities_in_area(area)
        .into_iter()
        .filter(|entity| is_fallen_ally(*entity))
        .filter(|entity| can_be_revived(*entity))
        .collect()
}

fn revive_ally(target: Entity) {
    set_health(target, get_max_health(target) * 0.5);
    set_status(target, AliveStatus::Living);
    apply_revival_immunity(target, 3.0); // 3 seconds of damage immunity
    spawn_revival_effects(target);
}
```

### Revival Conditions
- **Recent Death**: Allies must have fallen within the last 30 seconds
- **Valid Corpse**: Target must be in fallen state, not completely removed
- **Area Coverage**: Must be within 4x4 grid centered on Cardinal
- **No Obstruction**: Clear line from Cardinal to fallen ally required

## Telegraph Enhancement System

### Enhanced Warning Mechanics
- **Duration Extension**: Enemy attack warnings last 2x normal time
- **Clarity Improvement**: Warning indicators become brighter and more distinct
- **Early Detection**: Some previously hidden attacks become visible
- **Team-Wide Benefit**: All allies receive enhancement regardless of position

### Enhanced Telegraphs Examples
- **Boss Abilities**: Charge-up times appear longer with clearer visual cues
- **Area Attacks**: Danger zones show more detailed boundary information
- **Projectile Paths**: Incoming attacks display trajectory lines earlier
- **Environmental Hazards**: Trap triggers and floor collapses more obvious

## Upgrade Paths

### Tier 1: Empowered Revival
- **Health Restoration**: 50% → 75% maximum health upon revival
- **Revival Immunity**: 3 seconds → 5 seconds damage immunity after resurrection
- **Channel Protection**: Cardinal gains 50% damage reduction during channel
- **Enhanced Safety**: Reduces risk and improves revival effectiveness

### Tier 2: Extended Grace
- **Telegraph Duration**: 15 seconds → 25 seconds enhanced vision period
- **Area Expansion**: 4x4 grid → 6x6 grid coverage for revival area
- **Cooldown Reduction**: 60 seconds → 45 seconds ultimate cooldown
- **Strategic Flexibility**: More frequent access with larger area coverage

### Tier 3: Divine Ascension
- **Mass Revival**: Revives ALL fallen allies regardless of position
- **Perfect Health**: Revived allies return at 100% health and mana
- **Permanent Enhancement**: Telegraph improvements last until next ultimate use
- **Ultimate Mastery**: Transforms resurrect into game-changing team restoration

## Strategic Timing and Usage

### Optimal Activation Moments
- **Multiple Casualties**: When 2+ allies have fallen in covered area
- **Critical Phase Preparation**: Before difficult boss phases for enhanced vision
- **Team Recovery**: After major enemy ultimates that caused multiple downs
- **Tactical Reset**: When team positioning has deteriorated significantly

### Risk Management
- **Channel Safety**: Ensure allies can protect Cardinal during 2-second vulnerability
- **Positioning Priority**: Move to cover maximum revival targets before activation
- **Cooldown Awareness**: Consider 60-second recharge when deciding usage timing
- **Team Coordination**: Communicate ultimate usage for optimal team response

## Visual & Audio Design

### Pre-Activation
- **Visual**: 4x4 grid outline appears showing resurrection coverage area
- **Target Highlighting**: Fallen allies within range glow with revival potential
- **UI**: Ultimate ability icon pulses to indicate full charge availability
- **Audio**: Distant divine choir building in anticipation

### Channel Phase
- **Visual**: Brilliant column of divine light descends from above onto Cardinal
- **Animation**: Cardinal kneels with arms raised, robes billowing with divine wind
- **Audio**: Powerful choir harmonics building toward resurrection crescendo
- **Environment**: Area around Cardinal brightens with supernatural illumination

### Resurrection Moment
- **Visual**: Explosive burst of divine light followed by golden energy waves
- **Revival Effects**: Fallen allies surrounded by ascending light columns
- **Audio**: Triumphant divine chord with angelic chorus celebrating return to life
- **Particle**: Golden light motes drift throughout the area for several seconds

### Enhanced Vision Period
- **Visual**: All team members gain subtle golden aura around their outlines
- **Telegraph Enhancement**: Enemy warnings appear with golden trim and extended duration
- **Audio**: Continuous background divine harmony during 15-second enhancement
- **UI**: Enhanced vision indicator shows remaining duration with divine theme