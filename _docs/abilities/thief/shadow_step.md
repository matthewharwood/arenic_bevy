# Shadow Step

A complete implementation guide for the Thief's evasive teleportation and positioning ability.

## Overview

The **Shadow Step** ability represents the Thief's mastery over stealth and dimensional movement through instantaneous teleportation with temporary invulnerability. When activated, the Thief dashes forward in their facing direction, passing through hazards and enemy attacks without taking damage while gaining brief invulnerability after reappearing. This mobility ability excels at positioning, escape, and tactical engagement while maintaining enemy scripted behavior for deterministic gameplay.

## Game Design Philosophy

This ability demonstrates evasive mobility design through invulnerability-based movement:

**Predictable Escape Tool**: The forward dash direction creates consistent, learnable mechanics while providing reliable escape options during dangerous situations.

**Invulnerability Windows**: Brief immunity periods reward timing and positioning decisions without creating overpowered defensive capabilities.

**Deterministic Integration**: Enemy scripts continue uninterrupted, maintaining game predictability while allowing Thief tactical repositioning options.

## Implementation Architecture

### Component-Based Design

```rust
ShadowStep {
    dash_distance: 4.0,                 // 4-tile forward dash distance
    dash_speed: 20.0,                   // 20 tiles per second dash velocity
    invulnerability_during: 0.2,        // 0.2 seconds immunity during dash
    invulnerability_after: 0.8,         // 0.8 seconds immunity after dash
    cooldown: 12.0,                     // 12 second ability cooldown
    hazard_immunity: true,              // Immune to environmental hazards during dash
    enemy_passthrough: true,            // Can dash through enemy positions
}

ShadowDash {
    start_position: Vec2,
    end_position: Vec2,
    current_position: Vec2,
    dash_progress: f32,
    direction: Vec2,
    visual_effect: Entity,
    trail_particles: Vec<Entity>,
}

Invulnerability {
    entity: Entity,
    duration_remaining: f32,
    immunity_type: ImmunityType,        // During dash or post-dash
    visual_indicator: Entity,
    damage_negation_count: u32,
}
```

### Event-Driven Systems

The ability operates through five movement systems:
1. **Dash Mechanics** - Handles instantaneous movement and trajectory calculation
2. **Invulnerability Management** - Tracks immunity duration and damage negation
3. **Collision Override** - Allows movement through enemies and hazards during dash
4. **Position Validation** - Ensures dash endpoints are valid and safe
5. **Visual Coordination** - Manages shadow effects and teleportation feedback

## Step-by-Step Gameplay

### Phase 1: Dash Initiation (Double-Tap Activation)
- **Input Method**: Double-tap to instantly begin shadow step in facing direction
- **Direction Lock**: Dash travels in Thief's current facing direction at activation
- **Distance Validation**: 4-tile forward movement with endpoint validation
- **Invulnerability Start**: Immediate immunity to all damage begins with dash

### Phase 2: Shadow Dash Movement (0.2 Second Transit)
- **Rapid Movement**: Thief travels 4 tiles in 0.2 seconds (20 tiles/second)
- **Hazard Immunity**: Can pass through enemy attacks, area effects, and environmental hazards
- **Enemy Passthrough**: Dash movement ignores enemy collision boundaries
- **Visual Effects**: Distinctive shadow trail marks dash path with supernatural speed

### Phase 3: Dash Completion (Landing Position)
- **Position Validation**: Thief appears at valid endpoint or closest available tile
- **Continued Invulnerability**: Additional 0.8 seconds of immunity after dash completion
- **Tactical Positioning**: Thief gains advantageous position behind enemy lines or in safety
- **Visual Materialization**: Dramatic reappearance with shadow effects dissipating

### Phase 4: Post-Dash Recovery (0.8 Second Immunity)
- **Extended Protection**: Brief immunity window allows position assessment and planning
- **Tactical Decision**: Use immunity time to plan next actions or further movement
- **Cooldown Start**: 12-second cooldown begins immediately after dash completion
- **Vulnerability Return**: Normal damage reception resumes after immunity expires

## Dash Mechanics and Physics

### Movement Calculation
```rust
fn execute_shadow_step(thief: Entity, facing_direction: Vec2) -> ShadowDash {
    let start_pos = get_position(thief);
    let dash_vector = facing_direction.normalize() * 4.0;
    let target_pos = start_pos + dash_vector;
    
    // Validate endpoint and find nearest valid position if needed
    let end_pos = find_valid_landing_position(target_pos, start_pos);
    
    ShadowDash {
        start_position: start_pos,
        end_position: end_pos,
        current_position: start_pos,
        dash_progress: 0.0,
        direction: (end_pos - start_pos).normalize(),
        visual_effect: spawn_shadow_trail(start_pos, end_pos),
        trail_particles: create_dash_particles(),
    }
}

fn find_valid_landing_position(target: Vec2, origin: Vec2) -> Vec2 {
    // Check if target position is valid and accessible
    if is_valid_position(target) && !is_occupied_by_obstacle(target) {
        return target;
    }
    
    // Search for nearest valid position within dash range
    let search_positions = generate_nearby_positions(target, 1.0);
    
    for pos in search_positions {
        if is_valid_position(pos) && !is_occupied_by_obstacle(pos) {
            return pos;
        }
    }
    
    // Fallback to original position if no valid landing found
    origin
}
```

### Invulnerability Windows
- **Dash Transit (0.2s)**: Complete immunity while traveling between positions
- **Post-Dash Grace (0.8s)**: Extended immunity for positioning and decision-making
- **Total Protection**: 1.0 second total invulnerability per shadow step usage
- **Damage Negation**: All damage sources ignored during immunity periods

## Strategic Applications

### Escape and Repositioning
- **Emergency Escape**: Instant movement out of dangerous area effects or enemy attacks
- **Flanking Maneuvers**: Dash behind enemy lines for optimal backstab positioning
- **Hazard Navigation**: Pass through environmental dangers without taking damage
- **Formation Breaking**: Move through enemy formations to reach strategic positions

### Combat Integration
- **Backstab Setup**: Use dash to reach optimal positioning for backstab ability
- **Pickpocket Approach**: Dash adjacent to targets for safe pickpocket attempts
- **Smoke Coordination**: Dash into smoke screen areas for concealment benefits
- **Team Support**: Reposition to provide assistance or protection to allies

## Positioning and Timing Strategy

### Optimal Usage Timing
- **Threat Avoidance**: Dash when anticipating incoming area attacks or enemy ultimates
- **Positioning Windows**: Use during combat lulls to reach advantageous positions
- **Emergency Response**: React to sudden dangers with immediate evasive dash
- **Tactical Engagement**: Dash into optimal combat positions for ability combinations

### Directional Strategy
- **Facing Management**: Control Thief facing before activation for optimal dash direction
- **Endpoint Planning**: Visualize dash destination and ensure tactical advantage
- **Obstacle Awareness**: Consider environmental features that might affect landing position
- **Enemy Formation**: Dash through or around enemy formations for optimal positioning

## Upgrade Paths

### Tier 1: Enhanced Mobility
- **Distance Increase**: 4 tiles → 6 tiles dash distance
- **Cooldown Reduction**: 12 seconds → 9 seconds between uses
- **Extended Immunity**: Post-dash invulnerability: 0.8 → 1.2 seconds
- **Strategic Value**: Improved range and frequency with longer protection windows

### Tier 2: Combat Shadow Step
- **Damage Dash**: Enemies passed through during dash take 75 damage
- **Multiple Charges**: Can store up to 2 shadow step charges
- **Directional Control**: Can change direction once during dash movement
- **Tactical Evolution**: Transforms from pure mobility to offensive positioning tool

### Tier 3: Shadow Master
- **Dimensional Strike**: Dash creates shadow clone that attacks from original position
- **Phase Immunity**: Invulnerability extends to 3 seconds with phasing through all obstacles
- **Reset Mechanics**: Killing enemy during post-dash immunity resets shadow step cooldown
- **Ultimate Mobility**: Provides near-constant invulnerability and positioning control

## Environmental Interaction

### Hazard Immunity
- **Area Effects**: Pass through damage zones, poison clouds, and magical hazards
- **Projectiles**: Immunity to all projectiles during dash and post-dash window
- **Environmental Traps**: Dash through spike traps, pressure plates, and similar hazards
- **Boss Abilities**: Can avoid major boss attacks through strategic dash timing

### Collision Mechanics
- **Enemy Passthrough**: Dash movement ignores enemy collision boundaries
- **Obstacle Interaction**: Solid walls and barriers still block dash movement
- **Terrain Navigation**: Can dash over small gaps or elevation changes
- **Safe Landing**: System ensures landing position is accessible and safe

## Visual & Audio Design

### Lighting Design Philosophy
The Thief's Shadow Step employs **dimensional shadow lighting** that emphasizes stealth, mystery, and supernatural movement. The color palette centers on **deep purples and void blacks** to convey otherworldly stealth and dimensional manipulation.

**Technical Implementation:**
- **Key Light**: Dramatic chiaroscuro lighting with sharp light-shadow transitions
- **Fill Light**: Minimal fill to maintain mysterious shadow areas
- **Rim Light**: Purple-tinted edge lighting for supernatural silhouette definition
- **Color Temperature**: 2000K-4000K range for moody, supernatural atmosphere
- **PBR Materials**: Low metallic values (0.1-0.3) with varied roughness for fabric and leather

### Dash Initiation

**Lighting Design:**
- **Shadow Dissolution**: Gradual dimming of all lights affecting Thief (0.5→0.0 intensity)
- **Portal Effect**: Brief purple-black void portal at start position (RGB: 0.3, 0.1, 0.5, Intensity: 4.0)
- **Reality Distortion**: Subtle light bending effects around disappearing Thief
- **Performance**: Animated material opacity with single portal light source

**Visual Effects:**
- **Shadow Particles**: 40-60 dark purple particles swirling around Thief
- **Dissolution Animation**: Thief model fades from bottom-up over 0.1 seconds
- **Void Portal**: Small dimensional rift with swirling shadow effects
- **Shader Optimization**: Dissolve shader with noise texture, no geometry changes

**Audio Design:**
- **Dimensional Tear**: Deep whoosh with supernatural undertones (80-400Hz)
- **Shadow Whispers**: Subtle ethereal whispers during dissolution
- **Spatial Audio**: 3D positioning with slight reverb for otherworldly effect
- **Performance**: Layered audio with compressed mystical effects

### Dash Transit

**Lighting Design:**
- **Shadow Trail**: Moving shadow that dims environmental lights along path
- **Void Corridor**: Subtle purple light trail marking dimensional passage
- **Reality Distortion**: Environmental lights flicker briefly as shadow passes
- **Performance**: Animated shadow volume with simplified lighting calculations

**Visual Effects:**
- **Dimensional Streak**: Purple-black trail with particle ribbon effect
- **Shadow Vortex**: Swirling darkness following movement trajectory
- **Reality Tears**: Small dimensional rifts appearing and closing along path
- **Speed Distortion**: Motion blur effects emphasizing supernatural velocity

**Audio Design:**
- **Dimensional Travel**: Continuous whooshing with Doppler effect
- **Reality Friction**: Subtle crackling as shadow passes through normal space
- **3D Movement**: Audio source follows exact shadow path with spatial accuracy
- **Performance**: Real-time audio positioning with distance-based filtering

### Dash Completion

**Lighting Design:**
- **Materialization Flash**: Brief purple burst as Thief reappears (Intensity: 8, Duration: 0.15s)
- **Shadow Dispersal**: Expanding shadow effects with gradual light restoration
- **Rim Lighting**: Strong purple backlight creating dramatic silhouette
- **Performance**: Single burst light with animated falloff curve

**Visual Effects:**
- **Emergence Particles**: 60-80 purple shadow particles dispersing outward
- **Materialization**: Thief model fades in from top-down over 0.2 seconds
- **Ground Impact**: Small shadow shockwave expanding from landing point
- **Dimensional Closure**: Final void portal sealing with particle collapse

**Audio Design:**
- **Dimensional Snap**: Sharp reality-snapping sound (1-4kHz)
- **Materialization**: Ethereal chord with mystical harmonics
- **Landing Impact**: Subtle thud with supernatural reverb tail
- **Performance**: Layered impact audio with convolution reverb

### Invulnerability Feedback

**Lighting Design:**
- **Phase Aura**: Subtle purple rim light indicating dimensional protection (RGB: 0.4, 0.2, 0.6)
- **Attack Negation**: Incoming attacks create brief light distortions around Thief
- **Immunity Glow**: Gentle pulsing shadow aura (0.8-second cycle)
- **Performance**: Animated shader emission, no additional lighting sources

**Visual Effects:**
- **Shadow Phasing**: Attacks pass through with purple particle dispersion
- **Immunity Aura**: Subtle dark energy field around Thief
- **Damage Negation**: Purple "IMMUNE" text with shadow styling
- **Phase Particles**: Occasional purple motes drifting around protected Thief

**Audio Design:**
- **Phase Deflection**: Mystical chime when attacks are negated (C#4 note)
- **Immunity Ambient**: Low-frequency shadow whispers (40-80Hz)
- **Attack Negation**: Ethereal "miss" sound with dimensional characteristics
- **Performance**: Reactive audio triggered by damage events

### Performance Optimization for Mass Combat

**Lighting Optimization:**
- **Shadow Step LOD**:
  - **High (0-20 units)**: Full dimensional effects and dynamic shadows
  - **Medium (20-45 units)**: Simplified shadow trails, no reality distortion
  - **Low (45+ units)**: Particle effects only, no dynamic lighting
- **Light Culling**: Shadow effects disabled when outside camera view
- **Batch Processing**: Multiple shadow steps use shared shadow volume calculations

**Visual Effects Optimization:**
- **Particle Pooling**: Pre-allocated pools for 25 concurrent shadow steps
- **Shader Variants**: Simplified shaders for distant or background effects
- **Trail Optimization**: Shadow trails use low-poly ribbon geometry
- **Texture Streaming**: Shadow textures use compressed BC4 format

**Audio Optimization:**
- **Voice Limiting**: Maximum 5 concurrent shadow step audio sources
- **Audio LOD**: Simplified mystical effects beyond 25-unit range
- **Compression**: Dimensional audio uses OGG format for size efficiency
- **Memory Management**: Shared audio buffers for similar mystical sounds

### Deterministic Recording Compatibility

**Visual Synchronization:**
- **Movement Precision**: Shadow dash uses fixed-point trajectory calculations
- **Effect Timing**: All visual effects triggered by exact component state changes
- **Particle Determinism**: Shadow particles use seeded RNG for consistent placement
- **Replay Accuracy**: Identical visual sequence across all replay instances

**Audio Synchronization:**
- **Event-Based Audio**: All audio cues tied to specific movement and damage events
- **Position Tracking**: Audio sources follow precise entity transform data
- **Timing Consistency**: Audio scheduling based on simulation timesteps
- **Replay Fidelity**: Synchronized dimensional audio in all replay modes

### Accessibility Considerations

**Visual Accessibility:**
- **Colorblind Support**: Purple effects include brightness contrast alternatives
- **High Contrast**: Optional bright cyan outlines for better visibility
- **Motion Sensitivity**: Reduced particle effects with maintained core visibility
- **Text Scaling**: Immunity indicators support 125%-300% scaling

**Audio Accessibility:**
- **Hearing Impaired**: Visual phase indicators for all audio cues
- **Frequency Alternatives**: Higher frequency mystical sounds available
- **Subtitle Support**: Text descriptions for dimensional audio effects
- **Volume Independence**: Visual phase clarity maintained at all audio levels

### Thief Class Visual Identity

The Shadow Step ability establishes the Thief's **dimensional stealth** visual language:
- **Primary Colors**: Deep purple (RGB: 0.3, 0.1, 0.5) and void black (RGB: 0.1, 0.1, 0.15)
- **Secondary Accents**: Mystical silver (RGB: 0.7, 0.7, 0.8) for dimensional rifts
- **Material Palette**: Dark fabrics, worn leather, and ethereal shadow effects
- **Lighting Character**: Dramatic chiaroscuro emphasizing stealth and mystery
- **Animation Style**: Fluid, supernatural movements with dimensional phasing

### Dimensional Physics

**Shadow Realm Integration:**
- **Phase Mechanics**: Visual representation of movement between dimensions
- **Reality Interaction**: Shadows interact with environmental lighting realistically
- **Void Aesthetics**: Portal effects suggest access to shadow realm
- **Mystical Consistency**: All shadow effects follow consistent otherworldly rules