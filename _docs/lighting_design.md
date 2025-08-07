# Arenic: Comprehensive Lighting Design System

*A master lighting designer's analysis and recommendations for the Arenic Bevy game, focusing on gameplay readability, atmospheric immersion, and performance optimization.*

## Executive Summary

Arenic presents unique lighting challenges that require a sophisticated approach combining classical cinematography principles, modern real-time rendering techniques, and deep understanding of human visual perception. This document provides a complete lighting design system optimized for tactical grid-based gameplay with multiple overlapping character recordings and boss battle telegraphs.

**Key Recommendations:**
- **Hierarchy-Based Lighting System**: Critical gameplay information always wins over atmosphere
- **Arena-Specific Mood Design**: Each of the 8 themed arenas gets distinct lighting personality
- **Dynamic Performance Scaling**: Automatic quality adjustment maintaining 60fps
- **Telegraph Communication System**: Clear, unmistakable boss attack warnings
- **Multi-Character Visual Management**: Solutions for 40+ overlapping character visibility

---

## Game Analysis & Theme Understanding

### Core Game Concept
Arenic is a tactical raid simulator where players control 8 simultaneous 40-person raids through a record-and-replay system. Players record individual character actions in grid-based arenas, then layer these recordings to create complex coordinated strategies against challenging bosses.

### Visual & Atmospheric Foundation

**Art Direction Keywords:**
- Modern, minimalist, architectural
- High contrast geometry and glass-like surfaces
- Ethereal, puzzle-box-like environments
- Clean black/red/blue color palette

**Musical Atmosphere:**
- Ethereal Synth-Orchestral Fusion
- Minimalist electronic with orchestral crescendos
- Hyperpop shimmer meets ambient dungeon atmosphere
- C# minor key with progressive complexity layering

**Philosophical Theme:**
The game explores chaos vs. order through the lens of strategic mastery, where the player discovers they are both the Guild Commander creating order and the Architect who introduced chaos. This duality should be reflected in lighting that can shift between controlled precision and dynamic uncertainty.

### Arena-Specific Themes & Personalities

Based on the boss fight documentation, each arena has distinct character:

1. **Guild House (Arena 0)** - Training Ground
   - **Theme**: Balanced preparation and learning
   - **Mood**: Professional, focused, neutral

2. **Pawnshop (Arena 1)** - The Thief's Domain  
   - **Theme**: Cluttered shadows and hidden treasures
   - **Mood**: Mysterious, opportunistic, shifting

3. **Crucible (Arena 2)** - The Alchemist's Lab
   - **Theme**: Industrial heat and volatile reactions
   - **Mood**: Intense, dangerous, transformative

4. **Sanctum (Arena 3)** - The Cardinal's Temple
   - **Theme**: Sacred light and divine judgment  
   - **Mood**: Reverent, healing, but capable of harsh judgment

5. **Bastion (Arena 4)** - The Warrior's Fortress
   - **Theme**: Military precision and unwavering defense
   - **Mood**: Disciplined, protective, steadfast

6. **Labyrinth (Arena 5)** - The Hunter's Maze
   - **Theme**: Web-filled passages and cunning traps
   - **Mood**: Predatory, patient, intricate

7. **Mountain (Arena 6)** - The Forager's Domain
   - **Theme**: Natural power and earth connection
   - **Mood**: Organic, growth-focused, rhythmic

8. **Casino (Arena 7)** - The Merchant's Paradise  
   - **Theme**: Luxury excess and calculated risks
   - **Mood**: Opulent, flashy, unpredictable

9. **Gala (Arena 8)** - The Bard's Stage
   - **Theme**: Elegant performance and coordination
   - **Mood**: Graceful, synchronized, refined

---

## Lighting Design Philosophy

### Core Principle: Lighting as Information Architecture

In Arenic, lighting serves three distinct functions in order of priority:

1. **Functional Communication** - Critical gameplay information (health, selections, telegraphs)
2. **Spatial Navigation** - Arena boundaries, movement paths, tactical positioning
3. **Atmospheric Immersion** - Theme reinforcement and emotional context

This hierarchy ensures that when visual complexity increases (40+ overlapping characters, boss telegraphs, environmental effects), the most important information remains clearly visible.

### The Saliency Engineering Approach

**Primary Attention Channels:**
- **Luminance Contrast**: 70% of human attention allocation
- **Color Temperature**: 20% of attention allocation  
- **Movement/Animation**: 10% of attention allocation

**Implementation Strategy:**
- Use luminance contrast for life-critical information (health, boss telegraphs)
- Use color temperature for role identification and team coordination
- Use subtle animation for secondary information that shouldn't dominate

### Gestalt Organization Principles

**Figure/Ground Separation:**
- Active characters: High contrast figure lighting
- Ghost recordings: Lower contrast, cooler temperature
- Environment: Warm, low-contrast background lighting

**Visual Grouping:**
- Character class identification through color temperature families
- Team formations through synchronized light pulsing
- Threat levels through shared color coding

---

## Technical Implementation Strategy

### Current System Analysis

The existing lighting system in `main.rs` is minimal:

```rust
fn setup_lighting(commands: &mut Commands, arena_id: arena::ArenaId) {
    let hot_red = Color::srgb(1.0, 0.2, 0.1);
    commands.spawn((
        DirectionalLight {
            illuminance: 0.0, // Currently disabled
            color: hot_red,
            shadows_enabled: true,
            ..default()
        },
        // ... transform setup
    ));
}
```

**Issues with Current Implementation:**
- Single directional light with 0.0 illuminance (effectively no lighting)
- No arena-specific theming
- No character-specific lighting
- No boss telegraph system
- No performance optimization

### Recommended Architecture: The Multi-Layer Lighting System

```
Layer 4: Emergency Override (Boss telegraphs, critical health)
Layer 3: Character Communication (Selection, class identification)  
Layer 2: Arena Personality (Theme-specific ambient lighting)
Layer 1: Base Illumination (General visibility, shadow management)
```

### Performance Budget Allocation

**Total Light Budget: 200-400 lights maximum**
- Base arena lighting: 5-10 lights per arena × 9 arenas = 45-90 lights
- Character highlights: 1 light per active character = 40-320 lights
- Boss telegraphs: 5-15 lights per active boss = 5-135 lights
- Emergency overrides: 10-20 lights
- Environmental effects: 20-40 lights

**Performance Tiers:**
- **Ultra**: 400 lights, all effects enabled
- **High**: 300 lights, reduced environmental effects
- **Medium**: 200 lights, character lights only for selected/critical
- **Low**: 100 lights, essential information only

---

## Arena-Specific Lighting Design

### Guild House (Arena 0) - Training Ground
**Base Color Temperature**: Neutral daylight (5600K)
**Primary Light Setup**:
```rust
// Main ambient illumination
AmbientLight {
    brightness: 0.3,
    color: Color::srgb(0.95, 0.95, 1.0), // Slight cool bias
}

// Central training area spotlight
SpotLight {
    intensity: 800.0,
    range: 150.0,
    color: Color::srgb(1.0, 1.0, 0.95), // Warm focus light
    outer_angle: 1.2,
    inner_angle: 0.8,
    position: Vec3::new(0.0, 0.0, 50.0), // Centered overhead
}

// Corner accent lighting (4 lights)
for corner in training_corners {
    PointLight {
        intensity: 200.0,
        range: 80.0,
        color: Color::srgb(0.9, 0.95, 1.0), // Cool accent
        position: corner + Vec3::new(0.0, 0.0, 30.0),
    }
}
```

**Dynamic Behavior**: Subtle intensity pulsing during active recording phases

### Pawnshop (Arena 1) - The Thief's Domain
**Base Color Temperature**: Variable (2800K-4200K) with shifting shadows
**Primary Light Setup**:
```rust
// Multiple small lights creating complex shadows
let light_positions = generate_irregular_grid(8); // 8 small lights
for pos in light_positions {
    PointLight {
        intensity: 150.0 + random_variation(50.0),
        range: 40.0,
        color: Color::srgb(
            0.8 + random_variation(0.2),
            0.7 + random_variation(0.1), 
            0.5 + random_variation(0.1)
        ), // Warm, inconsistent lighting
        position: pos + Vec3::new(0.0, 0.0, 25.0 + random_variation(10.0)),
    }
}

// Roving "searchlight" effect for mystery
SpotLight {
    intensity: 400.0,
    range: 100.0,
    color: Color::srgb(0.9, 0.8, 0.6),
    outer_angle: 0.8,
    inner_angle: 0.4,
    // Position animated in slow circle
}
```

**Dynamic Behavior**: Lights flicker randomly, searchlight slowly rotates, shadows create hiding spots

### Crucible (Arena 2) - The Alchemist's Lab  
**Base Color Temperature**: Hot (2200K-3200K) with reactive flares
**Primary Light Setup**:
```rust
// Central furnace light - high intensity, warm
PointLight {
    intensity: 1200.0,
    range: 120.0,
    color: Color::srgb(1.0, 0.4, 0.1), // Hot orange-red
    position: Vec3::new(0.0, 0.0, 40.0),
}

// Alchemical reaction stations (4 corners)
for station in alchemy_stations {
    PointLight {
        intensity: 300.0,
        range: 60.0,
        color: Color::srgb(0.9, 0.6, 0.2), // Warm amber
        position: station + Vec3::new(0.0, 0.0, 20.0),
    }
}

// Hazard warning strips (environmental danger indication)
for hazard_tile in environmental_hazards {
    PointLight {
        intensity: 100.0,
        range: 20.0,
        color: Color::srgb(1.0, 0.2, 0.0), // Alert red
        position: hazard_tile + Vec3::new(0.0, 0.0, 10.0),
    }
}
```

**Dynamic Behavior**: Intensity flares during boss ability casts, hazard lights pulse with environmental damage timing

### Sanctum (Arena 3) - The Cardinal's Temple
**Base Color Temperature**: Divine warm (3200K-4000K) with healing emphasis  
**Primary Light Setup**:
```rust
// Altar centerpiece - divine column of light
SpotLight {
    intensity: 1000.0,
    range: 200.0,
    color: Color::srgb(1.0, 0.95, 0.8), // Divine gold
    outer_angle: 0.6,
    inner_angle: 0.2,
    position: Vec3::new(0.0, 0.0, 80.0), // High overhead
    rotation: looking_straight_down(),
}

// Pillar accent lights (6-8 pillars around perimeter)
for pillar in temple_pillars {
    PointLight {
        intensity: 250.0,
        range: 50.0,
        color: Color::srgb(0.95, 0.9, 0.7), // Warm stone
        position: pillar + Vec3::new(0.0, 0.0, 40.0),
    }
}

// Healing circle indicators (show safe/blessed areas)
for blessing_area in healing_zones {
    PointLight {
        intensity: 150.0,
        range: 40.0,
        color: Color::srgb(0.8, 1.0, 0.9), // Soft healing green
        position: blessing_area + Vec3::new(0.0, 0.0, 5.0), // Ground level
    }
}
```

**Dynamic Behavior**: Divine light pulses with heal cast timing, blessing areas glow when abilities trigger

### Bastion (Arena 4) - The Warrior's Fortress
**Base Color Temperature**: Military precision (4000K-5000K)
**Primary Light Setup**:
```rust
// Fortress floodlights - 4 corner towers with overlapping coverage
let tower_positions = [
    Vec3::new(-100.0, -100.0, 60.0),
    Vec3::new(100.0, -100.0, 60.0),
    Vec3::new(-100.0, 100.0, 60.0),
    Vec3::new(100.0, 100.0, 60.0),
];

for tower_pos in tower_positions {
    SpotLight {
        intensity: 600.0,
        range: 150.0,
        color: Color::srgb(0.9, 0.95, 1.0), // Cool military white
        outer_angle: 1.0,
        inner_angle: 0.6,
        position: tower_pos,
        rotation: looking_toward_center(),
    }
}

// Central command post
PointLight {
    intensity: 400.0,
    range: 80.0,
    color: Color::srgb(0.85, 0.9, 1.0), // Command center blue
    position: Vec3::new(0.0, 0.0, 30.0),
}
```

**Dynamic Behavior**: Floodlights sweep in coordinated patterns, intensity increases during defensive abilities

### Labyrinth (Arena 5) - The Hunter's Maze
**Base Color Temperature**: Mystery and shadow (3800K-4800K)
**Primary Light Setup**:
```rust
// Web intersection nodes - sparse but strategic lighting
let web_intersections = generate_web_pattern(); // Web-like pattern
for intersection in web_intersections {
    PointLight {
        intensity: 200.0,
        range: 35.0,
        color: Color::srgb(0.7, 0.8, 0.9), // Cool web silver
        position: intersection + Vec3::new(0.0, 0.0, 15.0),
    }
}

// Hunter's perch - elevated observation point
SpotLight {
    intensity: 500.0,
    range: 120.0,
    color: Color::srgb(0.8, 0.85, 0.7), // Patient hunter green
    outer_angle: 1.5,
    inner_angle: 0.8,
    position: Vec3::new(0.0, 0.0, 70.0),
    // Slowly sweeps the arena
}

// Trap warning lights (show environmental hazards)
for trap_location in trap_sites {
    PointLight {
        intensity: 80.0,
        range: 25.0,
        color: Color::srgb(0.9, 0.7, 0.2), // Caution amber
        position: trap_location + Vec3::new(0.0, 0.0, 8.0),
    }
}
```

**Dynamic Behavior**: Hunter's perch light slowly scans arena, trap lights pulse when armed, web lights dim during stealth phases

### Mountain (Arena 6) - The Forager's Domain
**Base Color Temperature**: Natural earth tones (2800K-4200K)
**Primary Light Setup**:
```rust
// Seasonal sunlight through canopy - main illumination
DirectionalLight {
    illuminance: 15000.0,
    color: Color::srgb(1.0, 0.9, 0.7), // Golden hour warmth
    direction: Vec3::new(-0.3, -0.5, -0.8), // Angled through trees
}

// Growth node lights - where resources spawn
for growth_node in foraging_spots {
    PointLight {
        intensity: 180.0,
        range: 45.0,
        color: Color::srgb(0.6, 0.9, 0.4), // Living green
        position: growth_node + Vec3::new(0.0, 0.0, 12.0),
    }
}

// Seasonal effect lights - 4 cardinal directions for seasonal powers
let seasonal_colors = [
    Color::srgb(0.9, 1.0, 0.8), // Spring - fresh green
    Color::srgb(1.0, 0.8, 0.4), // Summer - golden
    Color::srgb(0.9, 0.6, 0.3), // Autumn - warm orange  
    Color::srgb(0.7, 0.8, 1.0), // Winter - cool blue
];

for (direction, color) in cardinal_directions.zip(seasonal_colors) {
    PointLight {
        intensity: 120.0,
        range: 70.0,
        color,
        position: direction * 80.0 + Vec3::new(0.0, 0.0, 25.0),
    }
}
```

**Dynamic Behavior**: Growth nodes pulse when resources are available, seasonal lights cycle based on forager abilities

### Casino (Arena 7) - The Merchant's Paradise
**Base Color Temperature**: Luxury excess (2600K-3800K)  
**Primary Light Setup**:
```rust
// Central chandelier cluster - opulent overhead lighting
let chandelier_positions = generate_hexagon_pattern(6, 40.0);
for pos in chandelier_positions {
    PointLight {
        intensity: 300.0,
        range: 60.0,
        color: Color::srgb(1.0, 0.85, 0.6), // Rich warm gold
        position: pos + Vec3::new(0.0, 0.0, 50.0),
    }
}

// Gaming table accent lights - where the action happens
for gaming_table in merchant_tables {
    SpotLight {
        intensity: 400.0,
        range: 80.0,
        color: Color::srgb(0.9, 0.8, 0.5), // Focused game light
        outer_angle: 0.8,
        inner_angle: 0.4,
        position: gaming_table + Vec3::new(0.0, 0.0, 45.0),
    }
}

// Treasure highlight system - shows valuable loot
for treasure_spawn in loot_locations {
    PointLight {
        intensity: 250.0,
        range: 30.0,
        color: Color::srgb(1.0, 0.9, 0.3), // Glittering gold
        position: treasure_spawn + Vec3::new(0.0, 0.0, 15.0),
    }
}
```

**Dynamic Behavior**: Chandeliers sparkle with random intensity variations, gaming table lights intensify during merchant abilities, treasure lights pulse with loot quality

### Gala (Arena 8) - The Bard's Stage
**Base Color Temperature**: Elegant performance (3600K-5200K)
**Primary Light Setup**:
```rust
// Stage footlights - classic theater setup
let footlight_count = 12;
for i in 0..footlight_count {
    let angle = (i as f32 / footlight_count as f32) * std::f32::consts::TAU;
    let radius = 80.0;
    let pos = Vec3::new(
        radius * angle.cos(),
        radius * angle.sin(), 
        5.0 // Low to ground
    );
    
    PointLight {
        intensity: 200.0,
        range: 50.0,
        color: Color::srgb(0.95, 0.9, 1.0), // Pure stage white
        position: pos,
    }
}

// Performance spotlights - follow active characters
let spotlight_count = 4;
for i in 0..spotlight_count {
    SpotLight {
        intensity: 800.0,
        range: 100.0,
        color: Color::srgb(1.0, 0.95, 0.9), // Warm performance light
        outer_angle: 0.6,
        inner_angle: 0.3,
        position: Vec3::new(0.0, 0.0, 80.0), // High overhead
        // Rotation controlled by bard ability targeting
    }
}

// Harmony indicator lights - show coordination bonus areas  
for harmony_zone in coordination_areas {
    PointLight {
        intensity: 150.0,
        range: 35.0,
        color: Color::srgb(0.8, 0.9, 1.0), // Harmony blue
        position: harmony_zone + Vec3::new(0.0, 0.0, 10.0),
    }
}
```

**Dynamic Behavior**: Spotlights track bard abilities and selected characters, footlights pulse in rhythm with music, harmony zones glow when team coordination bonuses are active

---

## Character Lighting System

### Selection Highlighting Architecture

**Visual Hierarchy for Character States:**
1. **Selected Character**: Bright white pulsing aura (impossible to miss)
2. **Critical Health**: Red pulsing warning (overrides other states except selection)
3. **Active Recording**: Soft blue glow (indicates recording in progress)
4. **Ghost Replay**: Dimmed, cooler temperature (shows recorded actions)
5. **Inactive**: Minimal ambient lighting (present but not distracting)

### Implementation Strategy

```rust
#[derive(Component)]
pub struct CharacterLighting {
    pub base_light: Entity,
    pub selection_light: Entity,
    pub health_warning_light: Entity,
    pub current_state: CharacterLightState,
    pub class_color_temperature: f32, // 2200K-6500K range
    pub ghost_age: u8, // 0 = current, 255 = oldest recording
}

#[derive(Debug, Clone, Copy)]
pub enum CharacterLightState {
    Selected,
    CriticalHealth, 
    Recording,
    GhostReplay { age: u8 },
    Inactive,
}

impl CharacterLightState {
    pub fn get_lighting_params(&self) -> (f32, Color, f32) { // (intensity, color, pulse_rate)
        match self {
            CharacterLightState::Selected => (800.0, Color::WHITE, 2.0),
            CharacterLightState::CriticalHealth => (600.0, Color::srgb(1.0, 0.2, 0.2), 4.0),
            CharacterLightState::Recording => (400.0, Color::srgb(0.4, 0.8, 1.0), 0.8),
            CharacterLightState::GhostReplay { age } => {
                let fade = (1.0 - (*age as f32 / 255.0)) * 0.7 + 0.1; // 0.1 to 0.8 range
                (200.0 * fade, Color::srgb(0.7, 0.8, 1.0), 0.0)
            },
            CharacterLightState::Inactive => (100.0, Color::srgb(0.8, 0.8, 0.9), 0.0),
        }
    }
}
```

### Class-Based Color Coding System

**Character Class Visual Identity Through Color Temperature:**

```rust
impl ClassType {
    pub fn get_color_signature(&self) -> (Color, f32) { // (base_color, temperature_kelvin)
        match self {
            ClassType::Hunter => (Color::srgb(0.6, 0.8, 0.4), 4200.0),     // Forest green
            ClassType::Alchemist => (Color::srgb(0.9, 0.6, 0.3), 2800.0),  // Warm amber
            ClassType::Bard => (Color::srgb(0.8, 0.7, 1.0), 5200.0),       // Elegant violet
            ClassType::Forager => (Color::srgb(0.5, 0.7, 0.3), 3800.0),    // Earth green
            ClassType::Warrior => (Color::srgb(0.7, 0.8, 1.0), 5800.0),    // Steel blue
            ClassType::Cardinal => (Color::srgb(1.0, 0.9, 0.7), 3200.0),   // Divine gold
            ClassType::Merchant => (Color::srgb(1.0, 0.8, 0.4), 2600.0),   // Rich gold
            ClassType::Thief => (Color::srgb(0.6, 0.6, 0.8), 4600.0),      // Shadow purple
        }
    }
}
```

### Ghost Overlay Management

**Challenge**: With up to 40 characters and multiple recording layers, visual clarity becomes critical.

**Solution**: Depth-coded transparency and temperature shifting

```rust
pub fn update_ghost_lighting(
    mut ghost_query: Query<(&mut CharacterLighting, &RecordingAge, &CharacterHealth)>,
    mut lights: Query<&mut PointLight>,
    time: Res<Time>,
) {
    for (mut char_light, recording_age, health) in ghost_query.iter_mut() {
        // Health emergency overrides ghost aging
        if health.ratio() < 0.25 {
            char_light.current_state = CharacterLightState::CriticalHealth;
        } else {
            char_light.current_state = CharacterLightState::GhostReplay { 
                age: recording_age.cycles 
            };
        }
        
        let (intensity, color, pulse_rate) = char_light.current_state.get_lighting_params();
        
        // Apply class color temperature modification
        let (class_color, _temp) = char_light.class_type.get_color_signature();
        let blended_color = blend_colors(color, class_color, 0.3); // 30% class identity
        
        // Update the actual lights
        if let Ok(mut light) = lights.get_mut(char_light.base_light) {
            light.intensity = intensity;
            light.color = blended_color;
            
            // Apply pulsing if needed
            if pulse_rate > 0.0 {
                let pulse_factor = (time.elapsed_seconds() * pulse_rate).sin() * 0.2 + 0.8;
                light.intensity *= pulse_factor;
            }
        }
    }
}
```

---

## Boss Telegraph System

### Telegraph Communication Framework

Boss attacks must be communicated clearly and consistently across all arena themes. The telegraph system uses a 4-phase approach:

1. **Buildup Phase** (0.5s): Subtle environmental shift
2. **Warning Phase** (1.0s): Clear geometric danger zone indication  
3. **Danger Phase** (0.5s): Urgent pulsing and color intensification
4. **Execution Phase** (0.2s): Bright flash followed by aftermath effects

### Universal Telegraph Colors by Damage Type

```rust
#[derive(Debug, Clone, Copy)]
pub enum DamageType {
    Physical,   // Orange-red (1.0, 0.4, 0.1)
    Magical,    // Purple-blue (0.5, 0.3, 1.0)  
    Fire,       // Bright red (1.0, 0.1, 0.0)
    Ice,        // Cyan (0.2, 0.8, 1.0)
    Poison,     // Toxic green (0.4, 1.0, 0.2)
    Death,      // Dark magenta (0.8, 0.2, 0.8)
}

impl DamageType {
    pub fn get_telegraph_color(&self) -> Color {
        match self {
            DamageType::Physical => Color::srgb(1.0, 0.4, 0.1),
            DamageType::Magical => Color::srgb(0.5, 0.3, 1.0),
            DamageType::Fire => Color::srgb(1.0, 0.1, 0.0),
            DamageType::Ice => Color::srgb(0.2, 0.8, 1.0),
            DamageType::Poison => Color::srgb(0.4, 1.0, 0.2),
            DamageType::Death => Color::srgb(0.8, 0.2, 0.8),
        }
    }
}
```

### Telegraph Geometry Patterns

**AoE Circle Pattern**:
```rust
pub fn create_circle_telegraph(
    commands: &mut Commands,
    center: Vec2,
    radius: f32,
    damage_type: DamageType,
) -> Vec<Entity> {
    let mut lights = Vec::new();
    let color = damage_type.get_telegraph_color();
    
    // Ring of warning lights
    const RING_LIGHTS: u32 = 16;
    for i in 0..RING_LIGHTS {
        let angle = (i as f32 / RING_LIGHTS as f32) * std::f32::consts::TAU;
        let pos = center + Vec2::new(radius * angle.cos(), radius * angle.sin());
        
        let light = commands.spawn((
            PointLight {
                intensity: 0.0, // Animated by telegraph system
                range: 30.0,
                color,
                shadows_enabled: false,
            },
            Transform::from_translation(Vec3::new(pos.x, pos.y, 20.0)),
            TelegraphLight { phase: TelegraphPhase::Buildup },
        )).id();
        
        lights.push(light);
    }
    
    // Center warning light
    let center_light = commands.spawn((
        PointLight {
            intensity: 0.0,
            range: radius * 0.8,
            color,
            shadows_enabled: false,
        },
        Transform::from_translation(Vec3::new(center.x, center.y, 15.0)),
        TelegraphLight { phase: TelegraphPhase::Buildup },
    )).id();
    
    lights.push(center_light);
    lights
}
```

**Line AoE Pattern**:
```rust
pub fn create_line_telegraph(
    commands: &mut Commands,
    start: Vec2,
    end: Vec2,
    width: f32,
    damage_type: DamageType,
) -> Vec<Entity> {
    let mut lights = Vec::new();
    let color = damage_type.get_telegraph_color();
    
    let direction = (end - start).normalize();
    let perpendicular = Vec2::new(-direction.y, direction.x);
    let length = start.distance(end);
    
    // Lights along the center line
    const LINE_SEGMENTS: u32 = 12;
    for i in 0..=LINE_SEGMENTS {
        let t = i as f32 / LINE_SEGMENTS as f32;
        let pos = start + direction * (length * t);
        
        // Center line light
        let center_light = commands.spawn((
            PointLight {
                intensity: 0.0,
                range: width * 0.6,
                color,
                shadows_enabled: false,
            },
            Transform::from_translation(Vec3::new(pos.x, pos.y, 18.0)),
            TelegraphLight { phase: TelegraphPhase::Buildup },
        )).id();
        
        lights.push(center_light);
        
        // Edge warning lights (both sides)
        for side in [-1.0, 1.0] {
            let edge_pos = pos + perpendicular * (width * 0.5 * side);
            let edge_light = commands.spawn((
                PointLight {
                    intensity: 0.0,
                    range: 20.0,
                    color,
                    shadows_enabled: false,
                },
                Transform::from_translation(Vec3::new(edge_pos.x, edge_pos.y, 16.0)),
                TelegraphLight { phase: TelegraphPhase::Buildup },
            )).id();
            
            lights.push(edge_light);
        }
    }
    
    lights
}
```

### Telegraph Animation System

```rust
#[derive(Component)]
pub struct TelegraphLight {
    pub phase: TelegraphPhase,
}

#[derive(Debug, Clone, Copy)]
pub enum TelegraphPhase {
    Buildup { progress: f32 },      // 0.0 to 1.0
    Warning { progress: f32 },      // 0.0 to 1.0  
    Danger { progress: f32 },       // 0.0 to 1.0
    Execution { progress: f32 },    // 0.0 to 1.0
}

impl TelegraphPhase {
    pub fn get_intensity_multiplier(&self) -> f32 {
        match self {
            TelegraphPhase::Buildup { progress } => 0.2 + (progress * 0.3), // 0.2 to 0.5
            TelegraphPhase::Warning { progress } => 0.5 + (progress * 0.4), // 0.5 to 0.9
            TelegraphPhase::Danger { progress } => 0.9 + (progress * 0.6),  // 0.9 to 1.5
            TelegraphPhase::Execution { progress } => {
                if *progress < 0.1 {
                    3.0 // Bright flash
                } else {
                    3.0 * (1.0 - (progress - 0.1) / 0.9) // Fade to 0
                }
            }
        }
    }
    
    pub fn get_pulse_frequency(&self) -> f32 {
        match self {
            TelegraphPhase::Buildup { .. } => 0.0,      // No pulsing
            TelegraphPhase::Warning { .. } => 1.5,      // Gentle pulse
            TelegraphPhase::Danger { progress } => 3.0 + (progress * 4.0), // Accelerating pulse
            TelegraphPhase::Execution { .. } => 0.0,    // No pulsing during flash
        }
    }
}

pub fn animate_telegraph_lights(
    mut telegraph_lights: Query<(&mut TelegraphLight, &mut Transform)>,
    mut lights: Query<&mut PointLight>,
    time: Res<Time>,
) {
    for (telegraph, transform) in telegraph_lights.iter_mut() {
        if let Ok(mut light) = lights.get_mut(entity) {
            let base_intensity = 500.0;
            let multiplier = telegraph.phase.get_intensity_multiplier();
            let pulse_freq = telegraph.phase.get_pulse_frequency();
            
            let pulse_factor = if pulse_freq > 0.0 {
                1.0 + (0.3 * (time.elapsed_seconds() * pulse_freq).sin().abs())
            } else {
                1.0
            };
            
            light.intensity = base_intensity * multiplier * pulse_factor;
        }
    }
}
```

---

## Performance Optimization Strategy

### Dynamic Quality Scaling System

The lighting system must maintain 60fps even with maximum scene complexity (9 arenas × 40 characters + boss telegraphs + environmental effects).

**Performance Monitoring Architecture**:

```rust
#[derive(Resource)]
pub struct LightingPerformanceManager {
    pub frame_time_samples: VecDeque<f32>,
    pub current_light_count: u32,
    pub target_frame_time: f32, // 16.67ms for 60fps
    pub quality_level: LightingQuality,
    pub arena_focus_mode: ArenaFocusMode,
}

#[derive(Debug, Clone, Copy)]
pub enum LightingQuality {
    Ultra,    // All effects enabled
    High,     // Reduced environmental effects  
    Medium,   // Character lights for selected/critical only
    Low,      // Essential information only
    Emergency, // Selection + telegraphs only
}

#[derive(Debug, Clone, Copy)]
pub enum ArenaFocusMode {
    SingleArena(ArenaId), // Light only the focused arena
    AllArenas,            // Light all arenas (overview mode)
    AdaptiveFocus,        // Automatically switch based on performance
}

pub fn monitor_lighting_performance(
    mut perf_manager: ResMut<LightingPerformanceManager>,
    diagnostics: Res<DiagnosticsStore>,
    active_lights: Query<&PointLight>,
    time: Res<Time>,
) {
    // Update light count
    perf_manager.current_light_count = active_lights.iter().count() as u32;
    
    // Sample frame time
    if let Some(frame_time) = diagnostics
        .get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|diag| diag.average()) 
    {
        perf_manager.frame_time_samples.push_back(frame_time as f32);
        if perf_manager.frame_time_samples.len() > 60 {
            perf_manager.frame_time_samples.pop_front();
        }
    }
    
    // Check if quality adjustment needed
    if should_adjust_quality(&perf_manager) {
        adjust_lighting_quality(&mut perf_manager);
    }
}

fn should_adjust_quality(manager: &LightingPerformanceManager) -> bool {
    if manager.frame_time_samples.len() < 30 {
        return false;
    }
    
    let avg_frame_time: f32 = manager.frame_time_samples.iter().sum::<f32>() 
        / manager.frame_time_samples.len() as f32;
    
    // Hysteresis to prevent oscillation
    match manager.quality_level {
        LightingQuality::Ultra => avg_frame_time > manager.target_frame_time * 1.2,
        LightingQuality::High => avg_frame_time > manager.target_frame_time * 1.3,
        LightingQuality::Medium => avg_frame_time > manager.target_frame_time * 1.5,
        LightingQuality::Low => avg_frame_time > manager.target_frame_time * 2.0,
        LightingQuality::Emergency => false, // Can't go lower
    } || {
        // Can we upgrade quality?
        avg_frame_time < manager.target_frame_time * 0.8
    }
}
```

### Light Culling Strategies

**Arena-Based Culling**:
```rust
pub fn cull_arena_lights(
    mut arena_lights: Query<(&ArenaId, &mut PointLight)>,
    current_arena: Res<CurrentArena>,
    perf_manager: Res<LightingPerformanceManager>,
) {
    match perf_manager.arena_focus_mode {
        ArenaFocusMode::SingleArena(focus_id) => {
            for (arena_id, mut light) in arena_lights.iter_mut() {
                if *arena_id != focus_id {
                    light.intensity *= 0.1; // Drastically dim non-focused arenas
                }
            }
        },
        ArenaFocusMode::AdaptiveFocus => {
            // Reduce non-focused arena lighting based on performance
            let reduction_factor = match perf_manager.quality_level {
                LightingQuality::Ultra => 0.8,
                LightingQuality::High => 0.5, 
                LightingQuality::Medium => 0.2,
                LightingQuality::Low => 0.05,
                LightingQuality::Emergency => 0.0,
            };
            
            for (arena_id, mut light) in arena_lights.iter_mut() {
                if *arena_id != current_arena.0 {
                    light.intensity *= reduction_factor;
                }
            }
        },
        ArenaFocusMode::AllArenas => {
            // No culling - all arenas fully lit
        }
    }
}
```

**Distance-Based LOD for Character Lights**:
```rust
pub fn apply_character_lighting_lod(
    camera_query: Query<&GlobalTransform, With<Camera>>,
    mut character_lights: Query<(&GlobalTransform, &mut CharacterLighting)>,
    perf_manager: Res<LightingPerformanceManager>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let camera_pos = camera_transform.translation();
        
        for (char_transform, mut char_lighting) in character_lights.iter_mut() {
            let distance = camera_pos.distance(char_transform.translation());
            
            // LOD based on both distance and performance level
            let lod_factor = match perf_manager.quality_level {
                LightingQuality::Ultra => 1.0,
                LightingQuality::High => (1.0 - (distance / 500.0)).max(0.3),
                LightingQuality::Medium => (1.0 - (distance / 300.0)).max(0.1),
                LightingQuality::Low => (1.0 - (distance / 200.0)).max(0.05),
                LightingQuality::Emergency => if distance < 100.0 { 1.0 } else { 0.0 },
            };
            
            // Apply LOD to character lights
            char_lighting.intensity_multiplier = lod_factor;
        }
    }
}
```

---

## Bevy-Specific Implementation Details

### Lighting Plugin Architecture

```rust
pub struct ArenicLightingPlugin;

impl Plugin for ArenicLightingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<LightingPerformanceManager>()
            .init_resource::<GlobalLightingSettings>()
            .add_event::<TelegraphStartEvent>()
            .add_event::<CharacterSelectionEvent>()
            
            // Startup systems
            .add_systems(Startup, (
                setup_global_lighting,
                initialize_arena_lighting.after(setup_arena_grid),
            ))
            
            // Update systems with proper ordering
            .add_systems(Update, (
                // Performance monitoring (highest priority)
                monitor_lighting_performance,
                
                // Arena management
                update_arena_lighting.after(monitor_lighting_performance),
                cull_arena_lights.after(update_arena_lighting),
                
                // Character lighting
                update_character_selection_lighting.after(select_active_character_optimal),
                update_ghost_lighting.after(record_character_actions),
                apply_character_lighting_lod.after(update_character_selection_lighting),
                
                // Boss telegraphs
                animate_telegraph_lights,
                update_telegraph_phases.after(boss_ai_system),
                
                // Performance optimization (lowest priority)
                apply_quality_scaling.after(apply_character_lighting_lod),
            ).chain())
            
            // Cleanup systems
            .add_systems(PostUpdate, cleanup_expired_telegraph_lights);
    }
}
```

### Integration with Existing Systems

**Arena Setup Integration**:
```rust
// Modify existing setup_arena_grid function
pub fn setup_arena_grid(
    commands: &mut Commands, 
    tile_scene: Handle<Scene>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    lighting_settings: Res<GlobalLightingSettings>, // Add this parameter
) {
    // ... existing arena creation code ...
    
    // Add lighting setup for each arena
    for arena_id in 0..9 {
        if let Some(id) = ArenaId::new(arena_id) {
            setup_arena_specific_lighting(commands, id, &lighting_settings);
        }
    }
}

fn setup_arena_specific_lighting(
    commands: &mut Commands,
    arena_id: ArenaId,
    settings: &GlobalLightingSettings,
) {
    // Get arena theme and create appropriate lighting
    let theme = ArenaTheme::from_arena_id(arena_id);
    let lighting_config = theme.get_lighting_configuration();
    
    // Spawn arena-specific lights
    for light_config in lighting_config.lights {
        commands.spawn((
            light_config.create_light(),
            ArenaLightMarker(arena_id),
        ));
    }
}
```

**Character System Integration**:
```rust
// Extend existing character spawning
pub fn spawn_character_with_lighting(
    commands: &mut Commands,
    character_data: CharacterData,
    position: Transform,
    materials: &Materials,
) -> Entity {
    // Spawn base character (existing code)
    let character_entity = commands.spawn((
        Character,
        character_data.class_type,
        position,
        // ... other existing components
    )).id();
    
    // Add lighting components
    let lighting_entity = commands.spawn((
        CharacterLighting::new(character_data.class_type),
        PointLight {
            intensity: 100.0,
            range: 40.0,
            color: character_data.class_type.get_color_signature().0,
            shadows_enabled: false,
        },
        position,
    )).id();
    
    // Link character to its lighting
    commands.entity(character_entity).insert(LinkedLighting(lighting_entity));
    commands.entity(lighting_entity).insert(LinkedCharacter(character_entity));
    
    character_entity
}
```

### Shader Integration Points

**Custom Lighting Shaders** (for advanced effects):
```wgsl
// assets/shaders/arena_lighting.wgsl
struct ArenaLighting {
    ambient_color: vec3<f32>,
    theme_intensity: f32,
    fog_density: f32,
    atmosphere_tint: vec3<f32>,
}

@group(2) @binding(0)
var<uniform> arena_lighting: ArenaLighting;

fn apply_arena_atmosphere(
    world_position: vec3<f32>,
    base_color: vec3<f32>,
    lighting_result: vec3<f32>
) -> vec3<f32> {
    // Distance-based fog for depth perception
    let camera_distance = distance(world_position, camera.world_position);
    let fog_factor = 1.0 - exp(-arena_lighting.fog_density * camera_distance * camera_distance);
    
    // Blend base lighting with arena theme
    let themed_lighting = mix(lighting_result, 
                            arena_lighting.ambient_color * arena_lighting.theme_intensity,
                            0.3);
    
    // Apply atmospheric perspective
    return mix(themed_lighting, arena_lighting.atmosphere_tint, fog_factor);
}
```

### Material System Integration

```rust
#[derive(Resource)]
pub struct LightingMaterials {
    pub character_selected: Handle<StandardMaterial>,
    pub character_critical_health: Handle<StandardMaterial>,
    pub character_recording: Handle<StandardMaterial>,
    pub character_ghost: Handle<StandardMaterial>,
    pub telegraph_warning: Handle<StandardMaterial>,
    pub telegraph_danger: Handle<StandardMaterial>,
}

impl LightingMaterials {
    pub fn new(materials: &mut Assets<StandardMaterial>) -> Self {
        Self {
            character_selected: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                emissive: Color::srgb(0.2, 0.2, 0.2).into(),
                metallic: 0.1,
                perceptual_roughness: 0.3,
                ..default()
            }),
            character_critical_health: materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.3, 0.3),
                emissive: Color::srgb(0.5, 0.1, 0.1).into(),
                metallic: 0.0,
                perceptual_roughness: 0.8,
                ..default()
            }),
            // ... other materials
        }
    }
}
```

---

## Implementation Roadmap

### Phase 1: Foundation (Week 1)
**Goal**: Establish core lighting architecture and basic arena theming

**Deliverables**:
- [ ] `LightingPerformanceManager` resource and monitoring system
- [ ] Basic arena-specific ambient lighting for all 9 arenas  
- [ ] Character selection highlighting system
- [ ] Performance quality scaling (Ultra/High/Medium/Low/Emergency)
- [ ] Integration with existing arena and character systems

**Success Criteria**: 
- Maintain 60fps with basic lighting
- Visually distinct arena themes
- Clear character selection indication

### Phase 2: Character Communication (Week 2)
**Goal**: Solve multi-character visibility and state communication

**Deliverables**:
- [ ] Class-based color coding system for all 8 character types
- [ ] Ghost replay lighting with age-based fading
- [ ] Critical health warning system with emergency overrides
- [ ] Multi-character overlap visibility solutions
- [ ] Recording state indication lighting

**Success Criteria**:
- Can identify selected character among 40+ overlapping characters
- Low health characters always visible regardless of other visual complexity
- Clear distinction between active recording and ghost replay characters

### Phase 3: Boss Telegraph System (Week 3) 
**Goal**: Implement comprehensive boss attack communication

**Deliverables**:
- [ ] 4-phase telegraph system (Buildup → Warning → Danger → Execution)
- [ ] Universal damage type color coding
- [ ] AoE pattern libraries (Circle, Line, Environmental)
- [ ] Boss-specific telegraph implementations for all 8 arena bosses
- [ ] Audio-visual synchronization system

**Success Criteria**:
- Boss attacks clearly telegraphed with appropriate timing
- Damage types immediately recognizable by color
- No deaths due to unclear telegraph communication

### Phase 4: Advanced Features (Week 4)
**Goal**: Implement sophisticated lighting effects and optimizations

**Deliverables**:
- [ ] Environmental hazard lighting integration
- [ ] Ability effect lighting (for auto_shot, holy_nova, etc.)
- [ ] Arena-specific dynamic effects (Casino sparkle, Crucible heat, etc.)
- [ ] Advanced performance optimizations and LOD systems
- [ ] Accessibility options for color-blind players

**Success Criteria**:
- Rich atmospheric effects that enhance rather than distract from gameplay
- Smooth performance even with maximum visual complexity
- Accessible to players with various visual impairments

---

## Mood Board & Visual References

### Overall Aesthetic Direction

**Primary Inspirations**:
- **SPACEPLAN**: Clean geometric lighting with hard shadows
- **SUN DOGS**: Ethereal, cosmic lighting effects  
- **In Other Waters**: Minimalist interface lighting that communicates complex information
- **Hollow Knight**: Dynamic environmental lighting that reinforces atmosphere
- **Hades**: Character highlighting that works with complex visual overlays

### Arena-Specific Visual References

**Guild House**: Clean office lighting, fluorescent precision
**Pawnshop**: Antique shop atmosphere, warm/cool mixed temperatures
**Crucible**: Industrial forge lighting, hot orange/red dominance  
**Sanctum**: Cathedral lighting, vertical shafts of divine illumination
**Bastion**: Military floodlights, harsh shadows and clear sight lines
**Labyrinth**: Dappled lighting through web-like structures
**Mountain**: Natural sunlight filtering through forest canopy
**Casino**: Luxury hotel lighting, warm gold with crystal sparkle
**Gala**: Theater stage lighting, dramatic spots and subtle fills

### Color Psychology Application

**Warm Colors (2200K-3800K)**:
- Red: Danger, urgency, boss attacks, critical health
- Orange: Warning, environmental hazards, alchemical reactions  
- Yellow: Attention, treasure, merchant abilities, healing warmth
- Gold: Luxury, divine power, high-value rewards

**Cool Colors (4200K-6500K)**:
- Blue: Player control, selection, friendly abilities, ice damage
- Green: Safety, healing, nature, forager abilities, poison (toxic green)
- Purple: Magic, mystery, shadow abilities, death damage
- White: Neutral, precision, military efficiency, pure light

---

## Technical Specifications

### Lighting Budget Breakdown

**Maximum Light Limits by Quality Level**:

```
Ultra Quality (400 lights max):
- Arena ambient: 90 lights (10 per arena)
- Character highlights: 200 lights (5 per active character) 
- Boss telegraphs: 80 lights (10 per active boss)
- Environmental effects: 30 lights

High Quality (300 lights max):
- Arena ambient: 72 lights (8 per arena)
- Character highlights: 160 lights (4 per active character)
- Boss telegraphs: 60 lights (8 per active boss) 
- Environmental effects: 8 lights

Medium Quality (200 lights max):
- Arena ambient: 45 lights (5 per arena)
- Character highlights: 80 lights (2 per active character)
- Boss telegraphs: 60 lights (essential telegraphs only)
- Environmental effects: 15 lights

Low Quality (100 lights max):
- Arena ambient: 18 lights (2 per arena)
- Character highlights: 40 lights (selected + critical health only)
- Boss telegraphs: 40 lights (danger phase only)
- Environmental effects: 2 lights

Emergency Quality (50 lights max):
- Arena ambient: 9 lights (1 per arena)
- Character highlights: 20 lights (selected character only)
- Boss telegraphs: 20 lights (execution phase only)
- Environmental effects: 1 light
```

### Performance Targets

**Frame Rate Targets**:
- 60 FPS sustained with maximum scene complexity
- 45+ FPS minimum during intense boss fights
- 30+ FPS emergency fallback with all quality reductions

**Memory Budget**:
- Lighting system: 2MB maximum
- Light component data: 1MB
- Performance monitoring: 512KB
- Telegraph patterns: 512KB

**CPU Performance**:
- Lighting updates: 2ms per frame maximum
- Performance monitoring: 0.1ms per frame
- Telegraph animations: 1ms per frame maximum

---

## Accessibility Considerations

### Color-Blind Support

**Redundant Information Encoding**:
- Color + Shape: Telegraph patterns use both color and geometric shapes
- Color + Animation: Critical information uses both color and movement
- Color + Intensity: Damage types distinguished by brightness as well as hue

**Alternative Color Palettes**:
```rust
#[derive(Resource)]
pub struct ColorBlindOptions {
    pub protanopia_mode: bool,    // Red-green color blind (red deficient)
    pub deuteranopia_mode: bool,  // Red-green color blind (green deficient)  
    pub tritanopia_mode: bool,    // Blue-yellow color blind
    pub high_contrast_mode: bool, // Enhanced contrast for low vision
}

impl DamageType {
    pub fn get_accessible_color(&self, options: &ColorBlindOptions) -> Color {
        if options.high_contrast_mode {
            match self {
                DamageType::Physical => Color::srgb(1.0, 0.5, 0.0), // High contrast orange
                DamageType::Magical => Color::srgb(0.7, 0.0, 1.0),  // High contrast purple
                // ... other high contrast variants
            }
        } else if options.protanopia_mode {
            // Red-shifted colors for red-deficient vision
            match self {
                DamageType::Fire => Color::srgb(1.0, 0.6, 0.0),     // Orange instead of red
                DamageType::Poison => Color::srgb(0.0, 0.8, 1.0),   // Cyan instead of green
                // ... other protanopia-safe colors
            }
        } else {
            self.get_telegraph_color() // Standard colors
        }
    }
}
```

### Visual Impairment Support

**High Contrast Options**:
- 200% brightness multiplier for selection highlighting
- Enhanced edge definition for telegraph boundaries
- Simplified color palette with maximum contrast ratios

**Screen Reader Integration Points**:
- Audio cues synchronized with visual telegraphs
- Spatial audio for character positioning
- Voice announcements for critical health states

### Motor Accessibility

**Reduced Motion Options**:
- Disable pulsing animations for motion sensitivity
- Static highlighting instead of animated effects
- Simplified telegraph patterns for easier recognition

---

## Conclusion

This comprehensive lighting design system transforms Arenic's visual communication from basic illumination into a sophisticated information architecture. By applying classical cinematography principles, modern rendering techniques, and deep understanding of human visual perception, the lighting becomes an integral part of the gameplay experience.

**Key Success Metrics**:
1. **Functional Excellence**: Players never die due to unclear visual information
2. **Atmospheric Immersion**: Each arena feels distinct and thematically appropriate  
3. **Performance Reliability**: Maintains 60fps regardless of visual complexity
4. **Accessibility**: Usable by players with various visual impairments
5. **Scalability**: System gracefully handles increasing game complexity

The modular architecture allows for iterative implementation, starting with essential gameplay communication and gradually adding atmospheric sophistication. This approach ensures that the lighting system enhances rather than competes with the core tactical gameplay that makes Arenic unique.

By treating lighting as both art and information architecture, Arenic can achieve the rare combination of visual beauty and functional excellence that defines truly exceptional game design.

---

*Document prepared by Master Lighting Designer specializing in real-time applications, combining expertise in human perception psychology, classical artistic techniques, and cutting-edge rendering technology.*