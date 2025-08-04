# How Do You Create Lighting That Actually Helps Players Win?

*A cognitive-science approach to mastering 2D lighting in Arenic that transforms complex concepts into lasting knowledge*

## Your Learning Journey Map

**Time Investment:** 2-3 hours reading + 4-6 hours implementation  
**Prerequisites:** Basic Rust, introductory Bevy experience  
**Learning Outcome:** Build a professional lighting system that enhances gameplay while maintaining 60fps in both binary zoom states: single arena focus (scale 1.0) and all-arena overview (scale 3.0)

### What You'll Master
- **The "Lighting as Communication" Mental Model**: How to think about light as a gameplay information system
- **The 3-Layer Architecture Pattern**: A scalable approach that separates functional from atmospheric lighting
- **The Binary Zoom Optimization Process**: Building systems that excel at both detail and overview modes
- **The Emergency Visibility Framework**: Ensuring critical information is never lost in visual noise

---

## The Mental Model: Lighting as a Communication System

**Core Analogy**: Think of your lighting system like a **traffic control system** for a busy intersection. Just as traffic lights use color, intensity, and timing to safely guide vehicles through complex scenarios, your game lighting uses the same principles to guide players through complex gameplay decisions.

### The Traffic Light Mental Model in Action

```
🚦 Traffic Light System     →    🎮 Game Lighting System
Red = Stop/Danger           →    Red = Boss attacks, low health
Yellow = Caution/Prepare    →    Orange = Telegraphs, warnings  
Green = Go/Safe             →    Blue/Green = Friendly abilities, safe zones
Flashing = Urgent           →    Pulsing = Time-sensitive actions
Brightness = Priority       →    Intensity = Information importance
```

**Why This Analogy Works**: Both systems must communicate critical information instantly, work under stress, and never fail when lives (or characters) are at stake.

### Building Your Core Schema: The 3-Layer Architecture

Before we dive into implementation, establish this mental framework:

**Layer 1: Information (What players must know)**
- Character health and selection status
- Boss attack telegraphs and danger zones
- Critical timing windows

**Layer 2: Navigation (Where players should look)**
- Arena boundaries and focal points
- Movement paths and safe zones
- Resource and objective locations

**Layer 3: Atmosphere (How players should feel)**
- Arena personality and theme
- Tension building and release
- Emotional context and mood

---

## The Arenic Challenge: Why Normal Lighting Fails

### Active Recall Check #1
Before reading further, pause and consider: *What makes Arenic's lighting requirements different from a typical game?*

Think about:
- How many characters can be on screen simultaneously?
- What happens when recordings overlap?
- How fast do players need to make decisions?

<details>
<summary>🧠 Compare Your Answer</summary>

**Arenic's Unique Constraints:**
- **40+ simultaneous characters** in a 320×180 grid (most games handle 5-10)
- **Overlapping ghost recordings** with multiply-blend effects creating visual noise
- **Frame-perfect timing** requirements (16ms decision windows)
- **Binary zoom system** with dramatic rendering load differences: 1 arena vs 9 arenas
- **2-minute deterministic cycles** requiring pattern recognition support

**Why Standard Approaches Fail:**
- Traditional RPG lighting assumes 4-6 party members maximum
- Most lighting systems prioritize atmosphere over functional communication
- Standard performance optimizations break down with Arenic's binary zoom system
</details>

### The Binary Zoom Performance Challenge

Arenic's camera system presents a unique optimization challenge with two dramatically different modes:

**Detail Mode (Scale 1.0)**
- Shows only the current focused arena
- High-quality lighting budget available
- Rich atmospheric effects possible
- Character details clearly visible

**Overview Mode (Scale 3.0)**  
- Shows all 9 arenas simultaneously
- 9x rendering load increase
- Must prioritize critical information
- Performance optimization essential

This binary system requires a fundamentally different approach than gradual zoom—you're designing for two distinct lighting experiences.

### The Visual Chaos Problem

Imagine trying to read 40 overlapping transparent sheets of paper, each with different text, while someone shines colored lights through them randomly. That's what players face without proper lighting design.

**The Solution**: Systematic visual hierarchy that automatically prioritizes information by gameplay importance and adapts to the current zoom mode.

---

## Building Schema: The Emergency Visibility Framework

### Core Principle: Critical Information Always Wins

Your lighting system needs an "emergency broadcast system" that can override everything else when lives are at stake, working at both zoom levels.

**Priority Hierarchy (Memorize This)**:
1. **Immediate Death Threats** (Boss telegraphs) - Always maximum visibility
2. **Character Health Emergencies** - Cuts through all visual noise  
3. **Player Selection State** - Must remain visible for control clarity
4. **Strategic Information** - Visible when screen isn't chaotic
5. **Atmospheric Elements** - Suppressed during intense moments

### Mental Model: The Spotlight Metaphor

Think of your lighting system as a **theater director with a spotlight**:
- **Spotlight follows the most important actor** (critical information)
- **Stage lights provide context** (general illumination)
- **Background lighting sets mood** (atmosphere)
- **Emergency lights override everything** (health crises, boss attacks)

This works in both zoom modes: focused detail for single arena, strategic overview for all arenas.

---

## Implementation Phase 1: Building the Foundation

### The Lighting Manager: Your System's Brain

This is your traffic control center. Every light decision flows through this system and adapts to the binary zoom states.

```rust
use bevy::prelude::*;
use std::collections::VecDeque;
use crate::recording::ArenaIndex; // Import from the existing recording module
use crate::arena_camera::CurrentArena; // Import from the existing arena_camera module

/// The central nervous system for all lighting decisions
#[derive(Resource)]
pub struct LightingManager {
    /// Current performance level - automatically adjusts based on frame rate
    pub performance_level: PerformanceLevel,
    /// Which arena gets full lighting attention - uses existing ArenaIndex type
    pub focused_arena: Option<ArenaIndex>,
    /// Emergency override - forces maximum visibility for critical situations
    pub emergency_mode: bool,
    /// Current camera zoom scale (1.0 = single arena, 3.0 = all arenas)
    pub camera_zoom_scale: f32,
    /// Performance tracking for auto-optimization
    pub frame_time_history: VecDeque<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PerformanceLevel {
    /// All effects enabled for both zoom modes
    Ultra,   
    /// Full quality in detail mode (scale 1.0), optimized in overview mode (scale 3.0)
    High,    
    /// Reduced quality in detail mode, essential-only in overview mode
    Low,     
    /// Critical information only - works at both zoom levels
    Emergency,
}

impl Default for LightingManager {
    fn default() -> Self {
        Self {
            performance_level: PerformanceLevel::Ultra,
            focused_arena: None,
            emergency_mode: false,
            camera_zoom_scale: 1.0, // Default to detail mode
            frame_time_history: VecDeque::with_capacity(60), // 1 second of history
        }
    }
}
```

### Active Recall Challenge #1: Test Your Understanding

**Without looking back**, explain what each `PerformanceLevel` means in the binary zoom system and when the system would use each one.

<details>
<summary>🎯 Solution + Explanation</summary>

- **Ultra**: All lighting enabled for both zoom states (detail and overview modes)
- **High**: Full lighting in detail mode (scale 1.0), strategically reduced in overview mode (scale 3.0)
- **Low**: Reduced lighting in detail mode, critical-only in overview mode
- **Emergency**: Absolute minimum - selected characters + health warnings + boss telegraphs only

**Auto-switching Logic**: System monitors frame time and current zoom level. Overview mode (scale 3.0) is inherently more demanding due to 9x rendering load, so the system is more aggressive about reducing quality. Detail mode (scale 1.0) maintains higher quality since it only renders one arena.
</details>

### The Arena Lighting Component: Personality + Binary Zoom Optimization

Each arena needs both **functional lighting** (gameplay) and **personality lighting** (theme), optimized for both zoom modes.

```rust
use crate::arena::{Bastion, Casino, Crucible, Gala, Labyrinth, Mountain, Pawnshop, Sanctum}; // Import existing arena types

#[derive(Component)]
pub struct ArenaLighting {
    pub theme: ArenaTheme,
    pub functional_priority: bool, // True = gameplay over atmosphere
    pub ambient_lights: Vec<Entity>,
    pub emergency_override: bool,
    // Binary zoom optimization
    pub single_arena_intensity: f32,  // Multiplier for detail mode (scale 1.0)
    pub overview_intensity: f32,      // Multiplier for overview mode (scale 3.0)
}

/// Arena theme enum that mirrors the existing arena structure
/// Note: This maps to the actual arena components in the codebase
#[derive(Debug, Clone, Copy)]
pub enum ArenaTheme {
    Bastion,    // Index 4 - Military precision - cool blues, sharp edges
    Casino,     // Index 7 - Luxury excess - warm golds, glittering
    Crucible,   // Index 2 - Industrial danger - orange-red heat, harsh shadows  
    Gala,       // Index 8 - Elegant sophistication - soft whites, refined
    Labyrinth,  // Index 5 - Mystery - purple-teal, shifting shadows
    Mountain,   // Index 6 - Natural power - earth tones, storm effects
    Pawnshop,   // Index 1 - Cluttered chaos - mixed colors, item highlights
    Sanctum,    // Index 3 - Sacred solemnity - divine golds, pillar lighting
    GuildHouse, // Index 0 - Training ground - balanced green
}

impl ArenaTheme {
    /// Returns the base color that defines this arena's personality
    pub fn ambient_color(&self) -> Color {
        match self {
            ArenaTheme::Bastion =>    Color::srgb(0.4, 0.6, 0.8),   // Cool military blue
            ArenaTheme::Casino =>     Color::srgb(1.0, 0.8, 0.4),   // Rich gold
            ArenaTheme::Crucible =>   Color::srgb(0.8, 0.4, 0.2),   // Industrial orange-red
            ArenaTheme::Gala =>       Color::srgb(0.9, 0.9, 0.95),  // Elegant white
            ArenaTheme::Labyrinth =>  Color::srgb(0.6, 0.4, 0.8),   // Mysterious purple
            ArenaTheme::Mountain =>   Color::srgb(0.6, 0.5, 0.4),   // Earth brown
            ArenaTheme::Pawnshop =>   Color::srgb(0.7, 0.7, 0.6),   // Cluttered gray
            ArenaTheme::Sanctum =>    Color::srgb(1.0, 0.95, 0.8),  // Divine gold
            ArenaTheme::GuildHouse => Color::srgb(0.5, 0.7, 0.5),   // Balanced green
        }
    }
    
    /// How much should this theme's atmosphere be suppressed during intense gameplay?
    pub fn suppression_factor(&self) -> f32 {
        match self {
            // Crucible and Bastion themes complement danger, so suppress less
            ArenaTheme::Crucible | ArenaTheme::Bastion => 0.3,
            // Casino and Gala can be distracting, suppress more
            ArenaTheme::Casino | ArenaTheme::Gala => 0.7,
            // GuildHouse is neutral training ground
            ArenaTheme::GuildHouse => 0.4,
            // Others are neutral
            _ => 0.5,
        }
    }
    
    /// Returns intensity multipliers optimized for each zoom mode
    /// Different arena themes have different optimization profiles
    pub fn zoom_intensity_profile(&self) -> (f32, f32) {
        // Returns (single_arena_multiplier, overview_multiplier)
        match self {
            // High-contrast arenas maintain visibility in overview mode
            ArenaTheme::Crucible => (1.0, 0.8),  // Industrial orange cuts through
            ArenaTheme::Bastion => (1.0, 0.9),   // Military blue remains clear
            
            // Subtle themes need boosting in overview mode
            ArenaTheme::Sanctum => (1.0, 0.6),   // Divine gold gets overwhelming
            ArenaTheme::Mountain => (1.0, 0.5),  // Earth tones blend together
            
            // Attention-grabbing themes reduced in overview to prevent chaos  
            ArenaTheme::Casino => (1.0, 0.4),    // Gold glitter would be chaos
            ArenaTheme::Gala => (1.0, 0.4),      // Elegant white too bright
            
            // Balanced themes
            ArenaTheme::Labyrinth => (1.0, 0.7), // Purple mystery
            ArenaTheme::Pawnshop => (1.0, 0.6),  // Cluttered needs reduction
            ArenaTheme::GuildHouse => (1.0, 0.8), // Training ground stays visible
        }
    }
    
    /// Convert from the arena index to theme type (based on codebase arena indices)
    pub fn from_arena_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(ArenaTheme::GuildHouse),
            1 => Some(ArenaTheme::Pawnshop),
            2 => Some(ArenaTheme::Crucible),
            3 => Some(ArenaTheme::Sanctum),
            4 => Some(ArenaTheme::Bastion),
            5 => Some(ArenaTheme::Labyrinth),
            6 => Some(ArenaTheme::Mountain),
            7 => Some(ArenaTheme::Casino),
            8 => Some(ArenaTheme::Gala),
            _ => None,
        }
    }
}
```

### Setting Up Your First Arena

Let's implement the system that creates lighting when a new arena is loaded, with binary zoom optimization:

```rust
use crate::arena::Arena; // Import the existing Arena component

/// System that automatically sets up lighting when arenas are created
/// This system detects newly spawned arenas and adds appropriate lighting
pub fn setup_arena_lighting(
    mut commands: Commands,
    // Find arenas that were just created but don't have lighting yet
    new_arenas: Query<(Entity, &ArenaIndex), (Added<Arena>, Without<ArenaLighting>)>,
) {
    for (arena_entity, arena_index) in new_arenas.iter() {
        // Get the theme for this arena based on its index
        let theme = ArenaTheme::from_arena_index(arena_index.0)
            .unwrap_or(ArenaTheme::GuildHouse);

        // Get binary zoom optimization profile
        let (single_intensity, overview_intensity) = theme.zoom_intensity_profile();

        // Create the main ambient light for this arena
        let ambient_light = commands.spawn((
            PointLight {
                intensity: 500.0,
                range: 200.0,
                color: theme.ambient_color(),
                shadows_enabled: false, // 2D doesn't need shadows
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        )).id();

        // Create theme-specific accent lighting
        let accent_lights = create_theme_accent_lights(&mut commands, &theme);
        
        // Combine ambient + accent lights
        let mut all_lights = vec![ambient_light];
        all_lights.extend(accent_lights);

        // Attach the lighting component to the arena with binary zoom optimization
        commands.entity(arena_entity).insert(ArenaLighting {
            theme,
            functional_priority: false, // Start with atmosphere, switch during combat
            ambient_lights: all_lights,
            emergency_override: false,
            single_arena_intensity: single_intensity,
            overview_intensity: overview_intensity,
        });
    }
}

/// Creates additional lights that give each arena its unique personality
fn create_theme_accent_lights(commands: &mut Commands, theme: &ArenaTheme) -> Vec<Entity> {
    match theme {
        ArenaTheme::Casino => {
            // Glittering accent lights for luxury feel
            let mut lights = Vec::new();
            for i in 0..6 {
                let angle = (i as f32 / 6.0) * std::f32::consts::TAU;
                let radius = 150.0;
                let pos = Vec2::new(radius * angle.cos(), radius * angle.sin());
                
                let light = commands.spawn((
                    PointLight {
                        intensity: 200.0,
                        range: 50.0,
                        color: Color::srgb(1.0, 0.9, 0.6), // Warm gold
                        shadows_enabled: false,
                    },
                    Transform::from_translation(Vec3::new(pos.x, pos.y, 3.0)),
                )).id();
                lights.push(light);
            }
            lights
        },
        ArenaTheme::Crucible => {
            // Industrial heat sources
            vec![
                commands.spawn((
                    PointLight {
                        intensity: 800.0,
                        range: 100.0,
                        color: Color::srgb(1.0, 0.3, 0.1), // Hot orange-red
                        shadows_enabled: false,
                    },
                    Transform::from_translation(Vec3::new(-100.0, 0.0, 4.0)),
                )).id(),
                commands.spawn((
                    PointLight {
                        intensity: 800.0,
                        range: 100.0,
                        color: Color::srgb(1.0, 0.3, 0.1),
                        shadows_enabled: false,
                    },
                    Transform::from_translation(Vec3::new(100.0, 0.0, 4.0)),
                )).id(),
            ]
        },
        // For this tutorial, we'll implement Casino and Crucible as examples
        // Other themes follow similar patterns
        _ => Vec::new(),
    }
}
```

### What's the Output? Challenge #1

Run the code above with a new Bastion arena in both zoom modes. **Before looking at the answer**, predict:
1. How many lights will be created?
2. What color will the ambient light be?
3. What are the intensity multipliers for each zoom mode?

<details>
<summary>🎯 Verify Your Prediction</summary>

**Results for Bastion Arena:**
1. **1 light total** - Bastion uses default case, so only ambient light created
2. **Cool blue color** - `Color::srgb(0.4, 0.6, 0.8)` from the ambient_color() method
3. **Intensity multipliers**: single_arena = 1.0, overview = 0.9 (maintains visibility in overview)
4. **Position**: Ambient light at `(0, 0, 5)` - centered on arena, elevated above characters

**If it were Casino Arena:**
1. **7 lights total** - 1 ambient + 6 glittering accent lights
2. **Gold base color** with warm gold accents
3. **Intensity multipliers**: single_arena = 1.0, overview = 0.4 (reduces chaos in overview)
4. **Positions**: Ambient at center, accents in hexagonal pattern 150 units from center
</details>

---

## Character Lighting: The Heart of Player Communication

### The Selection Highlight System

Players need to instantly identify which character they've selected, even when 40 characters overlap. This is your highest-priority functional lighting that must work in both zoom modes.

```rust
// Character health component since the existing Character struct has private fields
#[derive(Component, Debug)]
pub struct CharacterHealth {
    pub current: u32,
    pub maximum: u32,
}

impl CharacterHealth {
    pub fn new(max_health: u32) -> Self {
        Self {
            current: max_health,
            maximum: max_health,
        }
    }
    
    pub fn ratio(&self) -> f32 {
        self.current as f32 / self.maximum as f32
    }
    
    pub fn is_critical(&self) -> bool {
        self.ratio() < 0.25
    }
}

#[derive(Component)]
pub struct SelectionHighlight {
    pub light_entity: Entity,
    pub pulse_timer: Timer,
    pub base_intensity: f32,
    pub emergency_boost: f32, // Extra brightness when health is critical
    pub overview_boost: f32,  // Extra brightness when in overview mode
}

#[derive(Component)]
pub struct Selected(pub bool);

/// Creates selection highlighting for characters that don't have it yet
pub fn add_selection_highlighting(
    mut commands: Commands,
    // Find characters without selection highlighting
    characters_needing_lights: Query<Entity, (With<Character>, Without<SelectionHighlight>)>,
) {
    for character_entity in characters_needing_lights.iter() {
        // Create a bright white light that will pulse when selected
        let light_entity = commands.spawn((
            PointLight {
                intensity: 0.0, // Start dim, will be controlled by selection state
                range: 35.0,    // Slightly larger than character sprite
                color: Color::WHITE,
                shadows_enabled: false,
            },
            // Position slightly above character for proper layering
            Transform::from_translation(Vec3::new(0.0, 0.0, 8.0)),
        )).id();

        // Attach the selection highlight to the character
        commands.entity(character_entity).insert(SelectionHighlight {
            light_entity,
            pulse_timer: Timer::from_seconds(1.2, TimerMode::Repeating),
            base_intensity: 400.0,
            emergency_boost: 0.0,
            overview_boost: 1.2, // 20% boost when in overview mode for visibility
        });
        
        // Also add the selection state component
        commands.entity(character_entity).insert(Selected(false));
        
        // Add health component for testing (replace with your actual health system)
        commands.entity(character_entity).insert(CharacterHealth::new(100));
    }
}
```

### The Selection Animation System

This system creates the pulsing effect that makes selected characters unmistakable in both zoom modes:

```rust
/// Updates selection highlighting - runs every frame to create smooth pulsing
/// Works with both detail mode (scale 1.0) and overview mode (scale 3.0)
pub fn update_character_selection_lighting(
    mut highlights: Query<(&mut SelectionHighlight, &Selected, &CharacterHealth)>,
    mut lights: Query<&mut PointLight>,
    time: Res<Time>,
    lighting_manager: Res<LightingManager>,
) {
    // Determine if we're in overview mode (scale 3.0)
    let is_overview_mode = (lighting_manager.camera_zoom_scale - 3.0).abs() < 0.1;
    let zoom_multiplier = if is_overview_mode { 1.2 } else { 1.0 };

    for (mut highlight, selected, health) in highlights.iter_mut() {
        // Update the pulse timer
        highlight.pulse_timer.tick(time.delta());
        
        // Calculate emergency boost for low health characters
        if health.is_critical() {
            highlight.emergency_boost = (0.25 - health.ratio()) * 4.0; // 0.0 to 1.0 boost
        } else {
            highlight.emergency_boost = 0.0;
        }
        
        // Update the actual light
        if let Ok(mut light) = lights.get_mut(highlight.light_entity) {
            if selected.0 {
                // Selected: Pulsing bright white with emergency boost and zoom boost
                let pulse_factor = (highlight.pulse_timer.elapsed_secs() * 3.0).sin() * 0.3 + 0.7;
                let base_intensity = highlight.base_intensity * (1.0 + highlight.emergency_boost);
                light.intensity = base_intensity * pulse_factor * zoom_multiplier;
                light.color = Color::WHITE;
            } else if highlight.emergency_boost > 0.0 {
                // Not selected but low health: Pulsing red emergency light
                let pulse_factor = (highlight.pulse_timer.elapsed_secs() * 5.0).sin().abs();
                light.intensity = 300.0 * pulse_factor * highlight.emergency_boost * zoom_multiplier;
                light.color = Color::srgb(1.0, 0.2, 0.2); // Emergency red
            } else {
                // Normal state: Subtle ambient glow for character identity
                light.intensity = 100.0 * zoom_multiplier;
                light.color = Color::srgb(0.8, 0.8, 0.9); // Subtle blue-white
            }
        }
    }
}
```

### Active Recall Challenge #2: Trace the Logic

A character has 15 health out of 60 maximum, is currently selected, and we're in overview mode (scale 3.0). The pulse timer shows 0.5 seconds elapsed.

**Calculate (don't look ahead):**
1. What's the health ratio?
2. What's the emergency boost value?
3. What's the zoom multiplier?
4. What's the pulse factor?
5. What's the final light intensity?
6. What color will the light be?

<details>
<summary>🧮 Work Through the Math</summary>

**Step-by-step calculation:**
1. **Health ratio**: 15/60 = 0.25 (exactly at the emergency threshold)
2. **Emergency boost**: Since health_ratio == 0.25, boost = 0.0 (no emergency boost)
3. **Zoom multiplier**: Overview mode (scale 3.0) = 1.2
4. **Pulse factor**: sin(0.5 * 3.0) * 0.3 + 0.7 = sin(1.5) * 0.3 + 0.7 = 0.997 * 0.3 + 0.7 ≈ 0.999
5. **Final intensity**: 400.0 * (1.0 + 0.0) * 0.999 * 1.2 ≈ 479.5
6. **Color**: WHITE (because selected.0 is true)

**Key insight**: The 1.2x zoom multiplier in overview mode ensures selected characters remain visible even when competing with 8 other arenas for attention.
</details>

---

## Boss Telegraph System: Dramatic Danger Communication

### The Telegraph Mental Model: Movie Lighting

Think of boss telegraphs like **movie lighting for dramatic scenes** that must work at both zoom levels:
- **Build-up phase**: Subtle color shift (like storm clouds gathering)
- **Warning phase**: Clear geometric shapes (like spotlights on stage)
- **Danger phase**: Urgent pulsing (like emergency alarms)
- **Execution phase**: Bright flash + aftermath (like lightning strike)

In overview mode, telegraphs get extra intensity to compete with visual noise from other arenas.

### The Telegraph Component Architecture

```rust
#[derive(Component)]
pub struct BossTelegraph {
    pub attack_type: AttackType,
    pub current_phase: TelegraphPhase,
    pub phase_timer: Timer,
    pub light_entities: Vec<Entity>,
    pub audio_sync_offset: f32, // Keeps visuals synchronized with audio cues
    pub overview_boost: f32,    // Extra intensity for overview mode
}

#[derive(Debug, Clone)]
pub enum AttackType {
    SingleTarget { 
        target_pos: Vec2,
        damage_type: DamageType,
    },
    AoeCircle { 
        center: Vec2, 
        radius: f32,
        damage_type: DamageType,
    },
    AoeLine { 
        start: Vec2, 
        end: Vec2, 
        width: f32,
        damage_type: DamageType,
    },
    Environmental { 
        affected_tiles: Vec<Vec2>,
        damage_type: DamageType,
    },
}

impl AttackType {
    pub fn damage_type(&self) -> DamageType {
        match self {
            AttackType::SingleTarget { damage_type, .. } => *damage_type,
            AttackType::AoeCircle { damage_type, .. } => *damage_type,
            AttackType::AoeLine { damage_type, .. } => *damage_type,
            AttackType::Environmental { damage_type, .. } => *damage_type,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TelegraphPhase {
    Buildup(f32),   // 0.0 to 1.0 progress - subtle environmental shift
    Warning(f32),   // 0.0 to 1.0 progress - clear geometric telegraph
    Danger(f32),    // 0.0 to 1.0 progress - urgent pulsing
    Execution(f32), // 0.0 to 1.0 progress - bright flash + aftermath
    Idle,           // No active telegraph
}

#[derive(Debug, Clone, Copy)]
pub enum DamageType {
    Physical,   // Orange-red colors
    Magical,    // Blue-purple colors  
    Fire,       // Bright red-orange
    Ice,        // Cyan-blue
    Poison,     // Toxic green
    Death,      // Dark magenta/purple
}

impl DamageType {
    pub fn telegraph_color(&self) -> Color {
        match self {
            DamageType::Physical => Color::srgb(1.0, 0.4, 0.2),  // Orange-red
            DamageType::Magical =>  Color::srgb(0.4, 0.2, 1.0),  // Blue-purple
            DamageType::Fire =>     Color::srgb(1.0, 0.2, 0.0),  // Bright red
            DamageType::Ice =>      Color::srgb(0.2, 0.8, 1.0),  // Cyan
            DamageType::Poison =>   Color::srgb(0.4, 1.0, 0.2),  // Toxic green
            DamageType::Death =>    Color::srgb(0.8, 0.0, 0.8),  // Dark magenta
        }
    }
}
```

### Creating Circle AoE Telegraphs

This is the most common boss attack pattern - let's build it step by step with binary zoom optimization:

```rust
impl AttackType {
    /// Creates the light entities that visualize this attack's danger zone
    /// Optimized for both detail mode (scale 1.0) and overview mode (scale 3.0)
    pub fn create_telegraph_lights(&self, commands: &mut Commands) -> Vec<Entity> {
        match self {
            AttackType::AoeCircle { center, radius, damage_type } => {
                let mut lights = Vec::new();
                
                // Create a ring of lights to show the blast radius
                const LIGHTS_IN_RING: u32 = 16;
                let color = damage_type.telegraph_color();
                
                for i in 0..LIGHTS_IN_RING {
                    let angle = (i as f32 / LIGHTS_IN_RING as f32) * std::f32::consts::TAU;
                    let light_pos = Vec2::new(
                        center.x + radius * angle.cos(),
                        center.y + radius * angle.sin(),
                    );
                    
                    let light = commands.spawn((
                        PointLight {
                            intensity: 0.0, // Will be animated by telegraph system
                            range: 30.0,
                            color,
                            shadows_enabled: false,
                        },
                        Transform::from_translation(
                            Vec3::new(light_pos.x, light_pos.y, 12.0) // High Z for visibility
                        ),
                    )).id();
                    
                    lights.push(light);
                }
                
                // Add a center warning light for extra clarity
                let center_light = commands.spawn((
                    PointLight {
                        intensity: 0.0,
                        range: radius * 0.8, // Covers most of the danger zone
                        color,
                        shadows_enabled: false,
                    },
                    Transform::from_translation(
                        Vec3::new(center.x, center.y, 10.0)
                    ),
                )).id();
                
                lights.push(center_light);
                lights
            },
            
            AttackType::SingleTarget { target_pos, damage_type } => {
                // Single spotlight pointing at the target
                vec![commands.spawn((
                    SpotLight {
                        intensity: 0.0,
                        range: 80.0,
                        color: damage_type.telegraph_color(),
                        outer_angle: 0.3,  // Focused beam
                        inner_angle: 0.15,
                        shadows_enabled: false,
                    },
                    Transform::from_translation(
                        Vec3::new(target_pos.x, target_pos.y, 15.0)
                    ),
                )).id()]
            },
            
            // Line and environmental attacks would follow similar patterns
            _ => Vec::new(),
        }
    }
}
```

### The Telegraph Animation System

This system brings the telegraphs to life with proper timing and visual escalation for both zoom modes:

```rust
/// Animates boss telegraphs through their phases with dramatic lighting changes
/// Includes binary zoom awareness for proper visibility at both detail and overview scales
pub fn animate_boss_telegraphs(
    mut telegraphs: Query<&mut BossTelegraph>,
    mut point_lights: Query<&mut PointLight, Without<SpotLight>>,
    mut spot_lights: Query<&mut SpotLight>,
    time: Res<Time>,
    lighting_manager: Res<LightingManager>,
) {
    // Determine if we're in overview mode
    let is_overview_mode = (lighting_manager.camera_zoom_scale - 3.0).abs() < 0.1;
    let overview_boost = if is_overview_mode { 1.3 } else { 1.0 }; // 30% boost in overview mode

    for mut telegraph in telegraphs.iter_mut() {
        // Update the phase timer
        telegraph.phase_timer.tick(time.delta());
        
        // Determine current phase and progress
        let (intensity_base, pulse_frequency, color_saturation) = match telegraph.current_phase {
            TelegraphPhase::Buildup(progress) => {
                // Subtle buildup - players should notice something is coming
                (200.0 * progress, 0.0, 0.7) // No pulsing, slightly desaturated
            },
            TelegraphPhase::Warning(progress) => {
                // Clear visibility - players must see the exact danger zone
                (400.0 + (200.0 * progress), 1.0, 1.0) // Gentle pulse, full color
            },
            TelegraphPhase::Danger(progress) => {
                // Urgent warning - attack is imminent
                let urgency = 1.0 + progress * 2.0; // Increasing urgency
                (600.0 + (400.0 * progress), 3.0 * urgency, 1.0) // Fast pulsing
            },
            TelegraphPhase::Execution(progress) => {
                // Flash and aftermath
                if progress < 0.1 {
                    (2000.0, 0.0, 1.2) // Bright flash, slightly overexposed
                } else {
                    // Fade to dim aftermath
                    let fade = 1.0 - ((progress - 0.1) / 0.9);
                    (300.0 * fade, 0.0, 0.6)
                }
            },
            TelegraphPhase::Idle => (0.0, 0.0, 1.0), // Lights off
        };
        
        // Apply performance scaling based on both performance level and zoom
        let base_scale = match lighting_manager.performance_level {
            PerformanceLevel::Ultra => 1.0,
            PerformanceLevel::High => 0.8,
            PerformanceLevel::Low => 0.4,
            PerformanceLevel::Emergency => 0.2,
        };
        
        // Apply overview boost for visibility (critical survival information)
        let performance_scale = base_scale * overview_boost;
        let final_intensity = intensity_base * performance_scale;
        
        // Calculate pulsing effect
        let pulse_factor = if pulse_frequency > 0.0 {
            1.0 + (0.4 * (time.elapsed_seconds() * pulse_frequency).sin().abs())
        } else {
            1.0
        };
        
        // Update all lights for this telegraph
        for &light_entity in &telegraph.light_entities {
            // Try point light first
            if let Ok(mut light) = point_lights.get_mut(light_entity) {
                light.intensity = final_intensity * pulse_factor;
                
                // Adjust color saturation for different phases
                let base_color = telegraph.attack_type.damage_type().telegraph_color();
                light.color = adjust_color_saturation(base_color, color_saturation);
            }
            // Then try spot light
            else if let Ok(mut light) = spot_lights.get_mut(light_entity) {
                light.intensity = final_intensity * pulse_factor;
                let base_color = telegraph.attack_type.damage_type().telegraph_color();
                light.color = adjust_color_saturation(base_color, color_saturation);
            }
        }
    }
}

/// Helper function to adjust color saturation for different telegraph phases
fn adjust_color_saturation(base_color: Color, saturation: f32) -> Color {
    let [r, g, b, a] = base_color.to_srgba().to_f32_array();
    
    // Convert to grayscale for desaturation
    let gray = 0.299 * r + 0.587 * g + 0.114 * b;
    
    // Mix between grayscale and original color based on saturation
    Color::srgba(
        gray + (r - gray) * saturation,
        gray + (g - gray) * saturation,
        gray + (b - gray) * saturation,
        a,
    )
}
```

### Active Recall Challenge #3: Telegraph Timing with Binary Zoom

A Fire-type AoE Circle attack is in the Danger phase at 0.6 progress. The current time is 10.5 seconds, we're at High performance level, and in overview mode (scale 3.0).

**Calculate:**
1. What's the intensity_base?
2. What's the pulse_frequency?
3. What's the overview_boost?
4. What's the performance_scale?
5. What's the pulse_factor? (assume sin(31.5) ≈ -0.5)
6. What's the final light intensity?

<details>
<summary>🎯 Work Through the Telegraph Math</summary>

**Step-by-step:**
1. **intensity_base**: 600.0 + (400.0 * 0.6) = 600.0 + 240.0 = 840.0
2. **pulse_frequency**: 3.0 * (1.0 + 0.6 * 2.0) = 3.0 * 2.2 = 6.6
3. **overview_boost**: Overview mode (scale 3.0) = 1.3
4. **performance_scale**: High level (0.8) * overview_boost (1.3) = 1.04
5. **pulse_factor**: 1.0 + (0.4 * abs(sin(10.5 * 6.6))) = 1.0 + (0.4 * abs(-0.5)) = 1.0 + 0.2 = 1.2
6. **final intensity**: 840.0 * 1.04 * 1.2 = 1049.3

**Color**: Fire damage = bright red `Color::srgb(1.0, 0.2, 0.0)` at full saturation

**Key insight**: The overview boost ensures boss telegraphs remain highly visible even when competing with 8 other arenas, making survival information always accessible.
</details>

---

## Performance Optimization: The Binary Zoom Performance System

### The Performance Monitoring Brain

Your lighting system needs to automatically detect when it's causing performance problems and gracefully reduce quality to maintain 60fps, with special handling for the binary zoom system.

```rust
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

#[derive(Resource)]
pub struct PerformanceMonitor {
    /// Rolling average of frame times in milliseconds
    pub frame_time_samples: VecDeque<f32>,
    /// How many lights are currently active
    pub active_light_count: u32,
    /// Target frame time in milliseconds (16.67ms = 60fps)
    pub target_frame_time: f32,
    /// When we last adjusted performance level
    pub last_adjustment: f32,
    /// Minimum time between performance adjustments (prevents oscillation)
    pub adjustment_cooldown: f32,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self {
            frame_time_samples: VecDeque::with_capacity(120), // 2 seconds at 60fps
            active_light_count: 0,
            target_frame_time: 16.67, // 60fps
            last_adjustment: 0.0,
            adjustment_cooldown: 2.0, // 2 second cooldown
        }
    }
}

/// Monitors performance and triggers automatic optimization
/// Includes binary zoom awareness for appropriate performance targets
pub fn monitor_lighting_performance(
    mut performance_monitor: ResMut<PerformanceMonitor>,
    mut lighting_manager: ResMut<LightingManager>,
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
    active_lights: Query<&PointLight>,
) {
    // Update light count
    performance_monitor.active_light_count = active_lights.iter().count() as u32;
    
    // Sample current frame time
    if let Some(frame_time_diag) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(current_frame_time) = frame_time_diag.average() {
            performance_monitor.frame_time_samples.push_back(current_frame_time as f32);
            
            // Keep only recent samples
            if performance_monitor.frame_time_samples.len() > 120 {
                performance_monitor.frame_time_samples.pop_front();
            }
        }
    }
    
    // Check if it's time to adjust performance
    let current_time = time.elapsed_seconds();
    if current_time - performance_monitor.last_adjustment > performance_monitor.adjustment_cooldown {
        if should_adjust_performance(&performance_monitor, &lighting_manager) {
            adjust_performance_level(&mut lighting_manager, &performance_monitor);
            performance_monitor.last_adjustment = current_time;
        }
    }
}

/// Determines if performance adjustment is needed based on frame time trends
/// Includes binary zoom awareness for appropriate performance targets
fn should_adjust_performance(monitor: &PerformanceMonitor, lighting_manager: &LightingManager) -> bool {
    if monitor.frame_time_samples.len() < 60 {
        return false; // Need enough samples for reliable measurement
    }
    
    let avg_frame_time: f32 = monitor.frame_time_samples.iter().sum::<f32>() 
        / monitor.frame_time_samples.len() as f32;
    
    // Overview mode (scale 3.0) is inherently more demanding, so adjust target
    let is_overview_mode = (lighting_manager.camera_zoom_scale - 3.0).abs() < 0.1;
    let zoom_factor = if is_overview_mode { 1.4 } else { 1.0 }; // 40% more lenient in overview
    let adjusted_target = monitor.target_frame_time * zoom_factor;
    
    // Adjust if consistently above or below target with some hysteresis
    avg_frame_time > adjusted_target * 1.15 || // 15% over target
    avg_frame_time < adjusted_target * 0.85    // 15% under target (can upgrade)
}

/// Adjusts performance level up or down based on current performance and zoom mode
fn adjust_performance_level(
    lighting_manager: &mut LightingManager,
    monitor: &PerformanceMonitor,
) {
    let avg_frame_time: f32 = monitor.frame_time_samples.iter().sum::<f32>() 
        / monitor.frame_time_samples.len() as f32;
    
    // Consider zoom level when adjusting performance
    let is_overview_mode = (lighting_manager.camera_zoom_scale - 3.0).abs() < 0.1;
    let zoom_factor = if is_overview_mode { 1.4 } else { 1.0 };
    let adjusted_target = monitor.target_frame_time * zoom_factor;
    
    if avg_frame_time > adjusted_target * 1.15 {
        // Performance is poor, reduce quality
        lighting_manager.performance_level = match lighting_manager.performance_level {
            PerformanceLevel::Ultra => PerformanceLevel::High,
            PerformanceLevel::High => PerformanceLevel::Low,
            PerformanceLevel::Low => PerformanceLevel::Emergency,
            PerformanceLevel::Emergency => PerformanceLevel::Emergency, // Can't go lower
        };
        info!("Lighting performance reduced to {:?} (frame time: {:.2}ms, zoom: {}, mode: {})", 
              lighting_manager.performance_level, avg_frame_time, lighting_manager.camera_zoom_scale,
              if is_overview_mode { "overview" } else { "detail" });
    } else if avg_frame_time < adjusted_target * 0.85 {
        // Performance is good, can increase quality
        lighting_manager.performance_level = match lighting_manager.performance_level {
            PerformanceLevel::Emergency => PerformanceLevel::Low,
            PerformanceLevel::Low => PerformanceLevel::High,
            PerformanceLevel::High => PerformanceLevel::Ultra,
            PerformanceLevel::Ultra => PerformanceLevel::Ultra, // Already at max
        };
        info!("Lighting performance increased to {:?} (frame time: {:.2}ms, zoom: {}, mode: {})", 
              lighting_manager.performance_level, avg_frame_time, lighting_manager.camera_zoom_scale,
              if is_overview_mode { "overview" } else { "detail" });
    }
}
```

### The Binary Arena Management System

Instead of distance-based culling, we now have binary state-based arena lighting management:

```rust
/// Manages arena lighting based on binary zoom states
/// Detail mode (scale 1.0): Focus resources on single arena
/// Overview mode (scale 3.0): Distribute resources across all 9 arenas
pub fn manage_binary_arena_lighting(
    mut arena_lighting: Query<(&mut ArenaLighting, &ArenaIndex)>,
    current_arena: Query<&CurrentArena>,
    lighting_manager: Res<LightingManager>,
    mut lights: Query<&mut PointLight>,
) {
    // Determine current state
    let is_overview_mode = (lighting_manager.camera_zoom_scale - 3.0).abs() < 0.1;
    let focused_arena_index = current_arena.get_single()
        .map(|arena| arena.0)
        .unwrap_or(0);

    for (mut lighting, arena_index) in arena_lighting.iter_mut() {
        let is_focused_arena = arena_index.0 == focused_arena_index;
        
        // Determine lighting strategy based on zoom mode and performance level
        let (should_be_lit, intensity_multiplier) = match (is_overview_mode, lighting_manager.performance_level) {
            // Detail mode strategies
            (false, PerformanceLevel::Ultra) => {
                // Ultra detail: focused arena gets full lighting, others minimal
                if is_focused_arena {
                    (true, lighting.single_arena_intensity)
                } else {
                    (true, 0.1) // Very dim background arenas
                }
            },
            (false, PerformanceLevel::High) => {
                // High detail: only focused arena lit
                if is_focused_arena {
                    (true, lighting.single_arena_intensity)
                } else {
                    (false, 0.0)
                }
            },
            (false, PerformanceLevel::Low) => {
                // Low detail: focused arena with reduced quality
                if is_focused_arena {
                    (true, lighting.single_arena_intensity * 0.7)
                } else {
                    (false, 0.0)
                }
            },
            (false, PerformanceLevel::Emergency) => {
                // Emergency detail: minimal lighting
                if is_focused_arena {
                    (true, 0.3)
                } else {
                    (false, 0.0)
                }
            },
            
            // Overview mode strategies  
            (true, PerformanceLevel::Ultra) => {
                // Ultra overview: all arenas lit with theme-optimized intensity
                (true, lighting.overview_intensity)
            },
            (true, PerformanceLevel::High) => {
                // High overview: all arenas with reduced intensity
                (true, lighting.overview_intensity * 0.8)
            },
            (true, PerformanceLevel::Low) => {
                // Low overview: focused arena emphasized, others minimal
                if is_focused_arena {
                    (true, lighting.overview_intensity * 0.9)
                } else {
                    (true, lighting.overview_intensity * 0.3)
                }
            },
            (true, PerformanceLevel::Emergency) => {
                // Emergency overview: only focused arena
                if is_focused_arena {
                    (true, 0.4)
                } else {
                    (false, 0.0)
                }
            },
        };
        
        // Apply the lighting decision to all lights in this arena
        for &light_entity in &lighting.ambient_lights {
            if let Ok(mut light) = lights.get_mut(light_entity) {
                if should_be_lit {
                    // Calculate base intensity from theme
                    let base_intensity = match lighting.theme {
                        ArenaTheme::Casino => 600.0,     // Bright luxury
                        ArenaTheme::Crucible => 800.0,   // Industrial heat
                        ArenaTheme::Gala => 500.0,       // Elegant ambiance
                        ArenaTheme::Bastion => 400.0,    // Military efficiency
                        _ => 450.0,                      // Standard ambiance
                    };
                    
                    light.intensity = base_intensity * intensity_multiplier;
                } else {
                    light.intensity = 0.0;
                }
            }
        }
    }
}
```

### Emergency Mode: When All Else Fails

In Emergency mode, only the most critical information gets lighting, but it works at both zoom levels:

```rust
/// Emergency lighting system - only essential information gets lit
/// Works at both detail mode (scale 1.0) and overview mode (scale 3.0)
pub fn apply_emergency_lighting(
    mut character_lights: Query<(&mut SelectionHighlight, &Selected, &CharacterHealth)>,
    mut telegraph_lights: Query<&mut BossTelegraph>,
    mut point_lights: Query<&mut PointLight>,
    lighting_manager: Res<LightingManager>,
) {
    if lighting_manager.performance_level != PerformanceLevel::Emergency {
        return; // Only run in emergency mode
    }
    
    // Determine zoom multiplier for emergency visibility
    let is_overview_mode = (lighting_manager.camera_zoom_scale - 3.0).abs() < 0.1;
    let emergency_boost = if is_overview_mode { 1.5 } else { 1.0 }; // Extra boost in overview
    
    // Only light selected characters and critical health characters
    for (highlight, selected, health) in character_lights.iter_mut() {
        if let Ok(mut light) = point_lights.get_mut(highlight.light_entity) {
            if selected.0 {
                // Selected character: minimal lighting for control
                light.intensity = 200.0 * emergency_boost;
                light.color = Color::WHITE;
            } else if health.is_critical() {
                // Critical health: emergency red
                light.intensity = 150.0 * emergency_boost;
                light.color = Color::srgb(1.0, 0.0, 0.0);
            } else {
                // Everything else: no lighting
                light.intensity = 0.0;
            }
        }
    }
    
    // Boss telegraphs still work but at minimum intensity
    for mut telegraph in telegraph_lights.iter_mut() {
        for &light_entity in &telegraph.light_entities {
            if let Ok(mut light) = point_lights.get_mut(light_entity) {
                // Reduce to 20% intensity but still visible, with emergency boost
                if light.intensity > 0.0 {
                    light.intensity = (light.intensity * 0.2 * emergency_boost).max(50.0);
                }
            }
        }
    }
}
```

---

## Advanced Feature: Ghost Overlap Handling

### The Multiply Blend Problem

When multiple ghost recordings overlap, traditional lighting creates unreadable visual soup. The solution: **depth-coded lighting intensity** that works at both zoom levels.

```rust
#[derive(Component)]
pub struct GhostLighting {
    pub recording_age: u32,        // How many cycles old this recording is
    pub base_light: Entity,        // The character's base lighting
    pub depth_factor: f32,         // 0.0 = oldest, 1.0 = current recording
    pub emergency_override: bool,  // Forces visibility regardless of age
    pub overview_boost: f32,       // Extra visibility when in overview mode
}

/// Updates ghost lighting intensity based on recording age and health status
/// Includes binary zoom awareness for proper visibility at both zoom levels
pub fn update_ghost_lighting_depth(
    mut ghost_lighting: Query<(&mut GhostLighting, &CharacterHealth, &Selected)>,
    mut lights: Query<&mut PointLight>,
    time: Res<Time>,
    lighting_manager: Res<LightingManager>,
) {
    // Determine if we're in overview mode
    let is_overview_mode = (lighting_manager.camera_zoom_scale - 3.0).abs() < 0.1;
    let overview_multiplier = if is_overview_mode { 1.3 } else { 1.0 };

    for (mut ghost, health, selected) in ghost_lighting.iter_mut() {
        // Calculate depth factor (newer recordings are brighter)
        ghost.depth_factor = (1.0 - (ghost.recording_age as f32 * 0.08)).max(0.15);
        
        // Health emergency overrides aging
        if health.is_critical() {
            ghost.emergency_override = true;
            ghost.depth_factor = ghost.depth_factor.max(0.8); // Force high visibility
        } else {
            ghost.emergency_override = false;
        }
        
        // Selection overrides everything
        if selected.0 {
            ghost.depth_factor = 1.0;
        }
        
        // Apply to the actual light
        if let Ok(mut light) = lights.get_mut(ghost.base_light) {
            // Base intensity adjusted by depth and zoom
            let base_intensity = 300.0 * ghost.depth_factor * overview_multiplier;
            
            // Add pulsing for emergency health
            if ghost.emergency_override {
                let pulse = (time.elapsed_seconds() * 4.0).sin().abs();
                light.intensity = base_intensity * (0.7 + 0.3 * pulse);
                light.color = Color::srgb(1.0, 0.3, 0.3); // Emergency red tint
            } else {
                light.intensity = base_intensity;
                // Older ghosts get cooler, more desaturated colors
                let warmth = ghost.depth_factor;
                light.color = Color::srgb(
                    0.8 + 0.2 * warmth,     // Red component
                    0.8 + 0.1 * warmth,     // Green component  
                    0.9,                    // Blue stays high (cooler for older)
                );
            }
        }
    }
}
```

---

## Putting It All Together: Your Complete Lighting Plugin

### Camera Integration System

This system synchronizes the lighting manager with the existing camera zoom system:

```rust
/// Synchronizes lighting manager with camera zoom changes
/// Hooks into the existing camera zoom toggle (P key)
pub fn sync_lighting_with_camera(
    camera_query: Query<&Projection, (With<Camera>, Changed<Projection>)>,
    mut lighting_manager: ResMut<LightingManager>,
    current_arena: Query<&CurrentArena, Changed<CurrentArena>>,
) {
    // Track camera zoom changes
    for projection in camera_query.iter() {
        if let Projection::Orthographic(ortho) = projection {
            lighting_manager.camera_zoom_scale = ortho.scale;
        }
    }
    
    // Track arena focus changes
    if let Ok(arena) = current_arena.get_single() {
        lighting_manager.focused_arena = Some(ArenaIndex(arena.0));
    }
}
```

### The Main Plugin Structure

```rust
pub struct ArenicLightingPlugin;

impl Plugin for ArenicLightingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize core resources
            .init_resource::<LightingManager>()
            .init_resource::<PerformanceMonitor>()
            
            // Core lighting systems that run every frame
            .add_systems(
                Update,
                (
                    // Performance monitoring (highest priority)
                    monitor_lighting_performance,
                    
                    // Camera and arena management
                    sync_lighting_with_camera,
                    setup_arena_lighting,
                    manage_binary_arena_lighting,
                    
                    // Character lighting
                    add_selection_highlighting,
                    update_character_selection_lighting,
                    update_ghost_lighting_depth,
                    
                    // Boss telegraphs
                    animate_boss_telegraphs,
                    
                    // Emergency systems
                    apply_emergency_lighting,
                ).chain() // Run in order to avoid frame lag
            );
    }
}
```

### Integration with Your Game

Add this to your main game setup, following the existing project structure:

```rust
// In your main.rs, following the existing pattern
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: GAME_NAME.to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GameStatePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(RecordingPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(ArenicLightingPlugin) // Add the lighting system
        .run();
}
```

---

## Final Active Recall: Test Your Mastery

**Without looking back at the code**, answer these integration questions for the binary zoom system:

### Question 1: Binary System Design
You have 8 arenas, 35 characters each, and frame rate just dropped to 45fps while in overview mode (scale 3.0). What happens automatically, and in what order?

<details>
<summary>🎯 Complete System Response</summary>

**Automatic Response Chain:**
1. **PerformanceMonitor** detects frame time > 23.3ms (40% over 16.67ms target due to overview mode factor)
2. **System recognizes overview mode** (scale 3.0) and applies 1.4x more lenient performance target
3. **LightingManager** drops from current level to next lower level
4. **manage_binary_arena_lighting** adjusts based on new performance level in overview mode:
   - Ultra→High: All arenas get 0.8x intensity multiplier
   - High→Low: Focused arena gets 0.9x, others get 0.3x
   - Low→Emergency: Only focused arena remains lit
5. **Character lighting systems** apply both performance scaling and 1.2x overview boost
6. **Telegraph system** maintains 1.3x overview boost for critical survival information
7. **System waits 2 seconds** before considering another adjustment

**Result**: Frame rate should recover to 60fps with binary zoom-optimized reductions.
</details>

### Question 2: Emergency Scenarios in Binary Zoom
A character has 8 health out of 50 maximum, is not selected, has 12 ghost overlaps, and we're in overview mode (scale 3.0). What lighting does this character get, and why?

<details>
<summary>🎯 Emergency Lighting Logic</summary>

**Character gets enhanced emergency visibility:**
1. **Health ratio**: 8/50 = 0.16 (below 0.25 threshold)
2. **Ghost lighting**: `emergency_override = true`, `depth_factor = max(calculated, 0.8) = 0.8`
3. **Overview mode boost**: 1.3x multiplier for visibility in 9-arena view
4. **Selection highlighting**: Red pulsing emergency light (not white, since not selected)
5. **Final result**: Bright pulsing red light (300.0 * 0.8 * 1.3 = 312 intensity) that cuts through all visual noise

**Why this works**: The binary zoom system recognizes that overview mode makes individual characters harder to see, so emergency systems get extra intensity. This ensures characters near death are always visible, regardless of ghost overlaps or the 9-arena view complexity.
</details>

### Question 3: Telegraph Timing with Binary Performance
A boss telegraph needs perfect synchronization with a 3-second audio warning. We're in overview mode at High performance level. How does the binary system ensure proper visibility across zoom states?

<details>
<summary>🎯 Binary Synchronization Solution</summary>

**Telegraph timing setup:**
```rust
BossTelegraph {
    phase_transitions: [1.0, 2.5, 3.0, 3.5], // Buildup, Warning, Danger, Execution end times
    audio_sync_offset: 0.0, // Adjusted based on audio latency measurement
    overview_boost: 1.3,    // Binary boost for overview mode
    // ... other fields
}
```

**Binary synchronization process:**
1. **Measure audio latency** on game startup
2. **Set audio_sync_offset** to compensate for audio system delay
3. **Use deterministic timing** based on boss cycle timer, not frame-dependent animations
4. **Apply binary zoom boost**: High performance (0.8) × overview boost (1.3) = 1.04 final multiplier
5. **Validate sync** by testing at both zoom levels (detail and overview)

**Result**: Boss telegraphs remain clearly visible and perfectly synchronized at both zoom levels, with overview mode getting extra intensity to compete with 8 other arenas.
</details>

---

## Your Next Steps: Binary Zoom Implementation Roadmap

### Week 1: Foundation (8-12 hours)
**Goal**: Get basic lighting working with proper binary zoom support

**Milestones**:
- [ ] `LightingManager` with binary zoom tracking (scale 1.0 vs 3.0) and `PerformanceMonitor` resources working
- [ ] Arena ambient lighting for at least 2 arena themes with proper zoom multipliers
- [ ] Basic character selection highlighting with overview mode boost
- [ ] Performance auto-adjustment between Ultra/High/Low levels with binary zoom awareness
- [ ] Binary zoom-aware lighting intensity scaling (detail vs overview modes)

**Success criteria**: 60fps at both zoom levels with appropriate visual quality for each mode

### Week 2: Character Systems (10-15 hours)  
**Goal**: Solve the ghost overlap visibility problem at both zoom levels

**Milestones**:
- [ ] Ghost depth-coded lighting working with overview boost
- [ ] Emergency health visibility overrides with binary zoom scaling
- [ ] Selection highlighting works with heavy overlap at both zoom levels
- [ ] Integration with your existing character/recording system

**Success criteria**: Can identify selected character and low-health characters even with 15+ overlapping ghosts, in both detail and overview modes

### Week 3: Boss Telegraphs (12-18 hours)
**Goal**: Clear, dramatic boss ability communication at both zoom levels

**Milestones**:
- [ ] At least 3 attack types (SingleTarget, AoeCircle, AoeLine) with proper binary zoom lighting
- [ ] 4-phase telegraph system with proper timing and overview boost
- [ ] Audio synchronization working at both zoom levels
- [ ] Telegraph lights integrate with performance system and binary zoom

**Success criteria**: Boss attacks are clearly telegraphed at both zoom levels and consistently timed across multiple test cycles

### Week 4: Performance Optimization (8-12 hours)
**Goal**: Smooth 60fps with full game load at both zoom levels

**Milestones**:
- [ ] Emergency mode lighting working at both detail and overview scales
- [ ] Binary arena state management (focused vs distributed lighting)
- [ ] All 5 performance levels functional with binary zoom awareness
- [ ] Graceful degradation under stress for both zoom modes

**Success criteria**: Maintains 60fps in overview mode (scale 3.0) with all 9 arenas visible, 40 characters each, 3 active boss fights, and maintains rich detail in single-arena mode (scale 1.0)

### Binary Zoom Performance Budgets

**Create this reference chart for both zoom modes:**

```
Detail Mode (Scale 1.0):
Ultra: 400 lights max, focused arena + ambient others
High: 250 lights max, focused arena only  
Low: 100 lights max, focused arena reduced quality
Emergency: 30 lights max, critical info only

Overview Mode (Scale 3.0):
Ultra: 300 lights max, all arenas with theme optimization
High: 200 lights max, all arenas reduced (0.8x multiplier)
Low: 80 lights max, focused emphasized, others minimal
Emergency: 25 lights max, focused arena only

Binary Zoom Multipliers:
- Character selection: 1.2x boost in overview mode
- Boss telegraphs: 1.3x boost in overview mode  
- Ghost emergency: 1.5x boost in overview mode
- Performance targets: 1.4x more lenient in overview mode
```

### Troubleshooting Guide

**"Frame rate drops dramatically when switching to overview mode"**
- Check: Are you applying binary zoom multipliers correctly?
- Solution: Ensure overview mode reduces non-critical lighting while boosting essential elements

**"Can't see selected character in overview mode with multiple arenas"**  
- Check: Is the 1.2x overview boost being applied to selection highlighting?
- Solution: Verify binary zoom detection logic using `(scale - 3.0).abs() < 0.1`

**"Boss telegraphs disappear in overview mode"**
- Check: Is the 1.3x overview boost being applied to telegraphs?
- Solution: Telegraph visibility is critical - they should be MORE visible in overview, not less

**"Performance oscillates between detail and overview modes"**
- Check: Are you using different performance targets for each zoom level?
- Solution: Implement 2-second cooldown and zoom-aware hysteresis

### Conclusion: Mastering Binary Zoom Lighting

You've learned to design lighting systems that excel in two distinct modes rather than trying to handle all intermediate states. This binary approach simplifies implementation while providing clear optimization targets:

- **Detail Mode (Scale 1.0)**: Rich, immersive lighting that enhances the tactical experience
- **Overview Mode (Scale 3.0)**: Strategic lighting that emphasizes critical information across all arenas

The binary constraint becomes an advantage when embraced rather than fought. Players get the best of both worlds: intimate detail when focusing on one arena, and clear strategic overview when surveying the entire battlefield.

**Remember**: Great binary systems don't try to be everything to everyone - they excel at their two specific modes and provide clear, predictable transitions between them. Your lighting system now serves both the detailed tactician and the strategic commander within the same player.

That's the difference between compromise and optimization.