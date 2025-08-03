# Smoke Screen

A complete implementation guide for the Thief's concealment and safe passage utility ability.

## Overview

The **Smoke Screen** ability demonstrates the Thief's mastery over stealth and battlefield control through tactical concealment deployment. When activated, the Thief throws a smoke bomb onto a targeted tile, creating a concealment zone where any ally within the smoke can walk through enemies without taking damage. This defensive utility ability excels at creating safe passages, enabling tactical repositioning, and providing emergency escape routes during dangerous combat situations.

## Game Design Philosophy

This ability showcases defensive utility design through temporary invulnerability mechanics:

**Safe Passage Creation**: The smoke provides temporary immunity to enemy damage, enabling tactical movement that would otherwise be impossible or extremely dangerous.

**Team-Wide Utility**: Any ally can benefit from the smoke screen, encouraging team coordination and tactical movement planning around concealment zones.

**Strategic Positioning**: The targeted placement allows for predictive tactical planning, rewarding players who can anticipate team movement needs and enemy positioning.

## Implementation Architecture

### Component-Based Design

```rust
SmokeScreen {
    throw_range: 5.0,                   // 5-tile maximum throw distance
    cast_time: 0.5,                     // 0.5 second throw animation
    smoke_radius: 2.0,                  // 2-tile radius concealment area
    duration: 8.0,                      // 8 second smoke persistence
    damage_immunity: true,              // Complete damage immunity while in smoke
    movement_freedom: true,             // Can move through enemies while concealed
    cooldown: 15.0,                     // 15 second ability cooldown
}

SmokeZone {
    center_position: GridPos,
    coverage_area: CircularArea,
    duration_remaining: f32,
    concealed_allies: HashSet<Entity>,
    visual_effect: Entity,
    particle_system: Entity,
}

Concealment {
    entity: Entity,
    damage_immunity: bool,
    enemy_collision_bypass: bool,
    smoke_source: Entity,
    concealment_timer: f32,
}
```

### Event-Driven Systems

The ability operates through five concealment systems:
1. **Projectile Deployment** - Handles smoke bomb throwing and impact mechanics
2. **Zone Creation** - Establishes concealment area with defined boundaries
3. **Concealment Management** - Tracks which allies are within smoke and grants immunity
4. **Collision Override** - Allows movement through enemies without triggering damage
5. **Visual Coordination** - Manages smoke effects and concealment indicators

## Step-by-Step Gameplay

### Phase 1: Smoke Deployment (Tap Activation)
- **Input Method**: Tap to begin smoke bomb throw targeting
- **Range Selection**: Target any tile within 5-tile range for smoke bomb placement
- **Throw Animation**: 0.5-second animation as Thief throws smoke bomb to target location
- **Strategic Planning**: Consider ally movement needs and enemy positioning for optimal placement

### Phase 2: Smoke Zone Creation (Impact Activation)
- **Immediate Effect**: Smoke bomb creates 2-tile radius concealment zone on impact
- **Visual Manifestation**: Dense smoke cloud appears with clear boundary indicators
- **Area Coverage**: Circular area provides concealment for any character within range
- **Duration Start**: 8-second concealment period begins immediately

### Phase 3: Active Concealment (8 Second Duration)
- **Damage Immunity**: Allies within smoke take no damage from any source
- **Movement Freedom**: Concealed allies can move through enemy positions without collision
- **Tactical Opportunities**: Team uses smoke for repositioning, escapes, or aggressive positioning
- **Dynamic Coverage**: Allies entering/leaving smoke gain/lose concealment benefits immediately

### Phase 4: Smoke Dissipation (Natural Expiration)
- **Visual Fade**: Smoke gradually thins and disperses over final 2 seconds
- **Benefit Loss**: Concealment effects end as smoke clears
- **Tactical Transition**: Team must establish new positioning as concealment ends
- **Cooldown Start**: 15-second cooldown begins for next smoke screen deployment

## Concealment Mechanics

### Damage Immunity System
```rust
fn apply_damage_to_entity(target: Entity, damage: f32, source: DamageSource) -> f32 {
    if is_concealed_by_smoke(target) {
        // Complete damage immunity while in smoke
        spawn_immunity_effect(target, "smoke_protection");
        return 0.0;
    }
    
    // Normal damage application
    apply_damage_normal(target, damage, source)
}

fn update_concealment_status(smoke_zone: &SmokeZone) {
    let entities_in_area = get_entities_in_circle(smoke_zone.center_position, smoke_zone.coverage_area.radius);
    
    for entity in entities_in_area {
        if is_ally(entity) {
            add_concealment_effect(entity, smoke_zone);
        }
    }
    
    // Remove concealment from entities no longer in smoke
    for concealed_entity in &smoke_zone.concealed_allies {
        if !entities_in_area.contains(concealed_entity) {
            remove_concealment_effect(*concealed_entity);
        }
    }
}
```

### Movement Through Enemies
- **Collision Bypass**: Concealed allies ignore enemy collision boundaries
- **Positioning Freedom**: Can move to any tile regardless of enemy occupation
- **Strategic Flanking**: Enables movement to optimal tactical positions behind enemy lines
- **Emergency Escapes**: Allows movement through enemy formations to safety

## Strategic Applications

### Tactical Movement
- **Formation Repositioning**: Enable team to move through enemy formations safely
- **Flanking Maneuvers**: Allow damage dealers to reach optimal positioning behind enemies
- **Escape Routes**: Create safe passages for retreating from dangerous situations
- **Objective Control**: Provide safe movement to capture points or important battlefield positions

### Team Coordination
- **Planned Movements**: Coordinate smoke placement with team tactical plans
- **Emergency Response**: Use smoke reactively to save allies in dangerous positions
- **Formation Transitions**: Enable complex formation changes without enemy interference
- **Healer Protection**: Provide safe zones for support characters during repositioning

## Upgrade Paths

### Tier 1: Dense Smoke
- **Duration Extension**: 8 → 12 seconds smoke persistence
- **Radius Increase**: 2-tile → 3-tile concealment area
- **Enhanced Immunity**: Also provides immunity to debuffs and status effects
- **Strategic Value**: Larger, longer-lasting concealment zones with improved protection

### Tier 2: Tactical Smoke
- **Multi-Deployment**: Can maintain 2 active smoke screens simultaneously
- **Movement Bonus**: Concealed allies gain 50% movement speed increase
- **Stealth Enhancement**: Concealed allies become invisible to enemy targeting
- **Tactical Evolution**: Improved mobility and concealment effectiveness

### Tier 3: Master Infiltrator
- **Persistent Concealment**: Allies retain 3 seconds of concealment after leaving smoke
- **Damage Reflection**: Attacks against concealed allies reflect 50% damage back to attacker
- **Area Expansion**: Smoke zones slowly expand over their duration
- **Ultimate Stealth**: Provides comprehensive protection with offensive capabilities

## Positioning and Timing Strategy

### Optimal Placement
- **Chokepoint Coverage**: Place smoke in narrow passages where team must move through
- **Retreat Paths**: Position smoke to cover likely escape routes during tactical withdrawals
- **Objective Access**: Enable safe approach to important battlefield objectives or positions
- **Formation Centers**: Place smoke at team center to enable radial repositioning

### Timing Considerations
- **Predictive Placement**: Deploy smoke before movement needs arise for optimal coverage
- **Emergency Response**: React quickly to dangerous situations with defensive smoke placement
- **Coordination Windows**: Time smoke deployment with team tactical plans and movements
- **Cooldown Management**: Balance immediate needs with future tactical requirements

## Visual & Audio Design

### Smoke Deployment
- **Visual**: Thief produces and throws distinctive smoke bomb with practiced motion
- **Animation**: Realistic throwing arc with smoke bomb spinning through air
- **Audio**: Soft whoosh of thrown object followed by muffled impact sound
- **Trajectory**: Clear arc visualization shows where smoke will impact

### Smoke Zone Creation
- **Visual**: Dramatic smoke explosion creating dense, opaque concealment cloud
- **Animation**: Realistic smoke dispersal with natural billowing and swirling effects
- **Audio**: Muffled pop of smoke bomb activation followed by hissing smoke release
- **Boundary**: Clear visual indicators show concealment area boundaries

### Active Concealment
- **Visual**: Dense, swirling smoke with mysterious shadowy depths
- **Animation**: Continuous smoke movement with allies appearing as shadowy silhouettes
- **Audio**: Subtle background smoke sounds with muffled movement audio
- **Concealment**: Allies within smoke show faded/translucent appearance to enemies

### Concealment Benefits
- **Visual**: Damage immunity shown through sparkle effects when attacks are negated
- **Animation**: Attacks pass harmlessly through concealed allies
- **Audio**: Distinctive deflection sounds when damage immunity activates
- **Feedback**: Clear indicators show which allies are receiving concealment benefits

### Smoke Dissipation
- **Visual**: Gradual smoke thinning and dispersal with natural wind effects
- **Animation**: Smoke breaks apart and fades over 2-second transition period
- **Audio**: Gentle wind sounds as smoke clears and normal battlefield audio returns
- **Transition**: Clear indication when concealment effects end and normal gameplay resumes