# ARENIC: Complete Game Instructions

## Game Overview

**Arenic** is an innovative tactical strategy game where you command up to 320 heroes across 8 simultaneous 40-person raids. Master the revolutionary **Record & Replay** system to build layered ghost recordings, coordinate complex strategies, and conquer challenging boss encounters. Every action matters in this deterministic world where victory comes through precise timing, strategic positioning, and masterful orchestration of your guild's abilities.

---

## Core Game Concept

### The Innovation: Record & Replay System
The heart of Arenic is its unique recording mechanism:
- **Record** individual hero actions and movements in 2-minute cycles
- **Replay** those recordings as "ghost" characters that automatically repeat their actions
- **Layer** up to 40 ghost recordings per arena to build massive coordinated raids
- **Master** complex strategies by synchronizing multiple heroes across different timelines

### The Goal
Build a guild powerful enough to simultaneously manage 8 different arenas, each with its own boss and unique challenges. Progress through increasingly difficult tiers (Normal → Heroic → Mythic) while growing your roster through strategic gacha recruitment.

---

## Getting Started

### Your First Steps
1. **Arena Selection**: Begin in Arena 1 with your starting hero
2. **Movement**: Use WASD keys to move your hero one tile at a time on the grid
3. **Character Selection**: Press Tab to switch between available heroes in your current arena
4. **Basic Combat**: Your hero will automatically attack nearby enemies based on their abilities

### Understanding the Interface
- **Grid-Based Movement**: Each arena is a 66×31 tile battlefield
- **Multi-Arena View**: Access all 9 arenas through the navigation system
- **Character Status**: Active characters are highlighted with blue materials
- **Boss Visibility**: Enemy bosses appear as large red spheres in each arena

---

## Controls Reference

| Input | Action |
|-------|--------|
| **WASD** | Move active character (tile-by-tile grid movement) |
| **Tab** | Switch to next character in current arena |
| **Shift+Tab** | Switch to previous character in current arena |
| **1-4** | Activate character abilities (when available) |
| **R** | Start recording (character) / Show replay dialog (ghost with recording in arena) |
| **F** | Finalize and commit the current recording |
| **Enter** | Open guild house management menu |
| **Q/E** | Rotate between different arena faces |
| **W** | Zoom between arena view and overworld map |
| **Mouse** | Arena selection and UI interaction |
| **Space** | Interact with objects, chests, or confirm dialogues |

---

## The Record & Replay System (Core Mechanic)

The recording system allows you to capture 2-minute sequences of character actions that replay as autonomous "ghosts" every arena cycle. This core mechanic enables you to build up complex, coordinated strategies across multiple characters and arenas.

### How Recording Works

**Starting a Recording**
- Press R while controlling any non-ghost character to begin recording
- If pressed on a ghost that has a recording in current arena, shows replay dialog instead
- A 3-second countdown prepares you before capture starts
- The arena timer resets to 0:00 when recording begins
- All your movements (WASD) and abilities (1-4 keys) are captured as intent

**During Recording**
- You have exactly 120 seconds to record your strategy
- A red "RECORDING" indicator shows your current state
- The timeline progress bar shows how much time remains
- Recording captures your input intent, not character position - ensuring perfect replay regardless of physics changes

**Ending a Recording**
- Recording automatically stops at the 2-minute mark
- Press R again to manually interrupt recording early
- A confirmation dialog appears with options:
  - **Continue**: Return to active recording (keep recording)
  - **Cancel**: Stop recording and discard progress

**Recording Interruptions**
- **Arena Switching**: Using [ ] or P (zoom) keys immediately stops recording with no confirmation
- **Character Switching**: Pressing Tab shows confirmation dialog first:
  - **Switch Character**: Stop recording and switch to next character
  - **Continue Recording**: Cancel switch and return to recording

### Ghost Playback System

**Autonomous Replay**
- Ghosts automatically replay their recorded timeline every 2-minute arena cycle
- Each arena maintains its own independent timer (0:00 to 2:00)
- Ghosts in off-screen arenas continue advancing their timelines
- When the timer loops back to 0:00, ghosts seamlessly restart

**Visual Indicators**
- Ghosts appear with blue tint and transparency effects
- Ghost trails show recent movement paths
- Character indicators above heads show state (green=active, red=recording, blue=ghost)
- Cannot directly control ghosts - they follow their recorded timeline

**Timeline Accuracy**
- Recording stores movement intent (WASD keys) not positions
- Abilities trigger at exact recorded timestamps
- Death events are captured and replayed
- Perfect deterministic replay every cycle

### Multi-Arena Coordination

**Independent Timers**
- Each of the 9 arenas has its own 2-minute cycle timer
- Ghosts use their parent arena's clock for playback
- Switching arenas ([ ]) doesn't affect other arena timers
- All timelines pause during dialog choices

**Cross-Arena Strategy**
- Record complementary ghosts across multiple arenas
- Use the arena status panel to track ghost counts per arena
- Maximum of 40 ghosts per arena, 320 total across all arenas
- Performance automatically adjusts update rates for distant arenas

### Ghost Replay Feature

**Arena-Specific Recordings**
- Each character can store separate recordings for each of the 9 arenas
- Recordings are stored in a per-character hashmap with arena as the key
- Characters effectively become "multiple ghosts" - one per arena recorded in

**Returning to Previous Arenas**
- When a ghost returns to an arena where they have a previous recording, a dialog appears
- Options presented:
  - **Replay Previous**: Use the stored recording for this arena
  - **Draft New**: Convert ghost back to character for new recording
  - **Cancel**: Continue without replaying
- This allows the same character to have different tactical roles in different arenas

**Strategic Applications**
- Create arena-specific strategies with the same character
- Build up recordings progressively as you learn each arena
- Reuse successful recordings when returning to farm or practice
- Maintain separate recordings for Normal/Heroic/Mythic difficulties

### Recording Best Practices

**Planning Your Recording**
- Think through the full 2-minute sequence before starting
- Consider boss attack patterns and timing windows
- Position yourself safely at the end for smooth looping
- Test ability combinations before committing

**Optimization Tips**
- Recordings automatically compress to save memory
- Redundant movement events are filtered out
- Only significant position changes create keyframes
- Ability events are always preserved at full fidelity

**Common Patterns**
- **Tank Loop**: Record a warrior continuously taunting and blocking
- **Healer Rotation**: Set up heal timings to match damage spikes
- **DPS Burst**: Align multiple damage dealers for boss vulnerability phases
- **Resource Gathering**: Create forager ghosts to maintain mushroom gardens

### Advanced Recording Features

**State Management**
- Recording state machine tracks: Idle → Countdown → Recording → Dialog
- All state transitions logged for debugging
- Global pause during dialogs ensures no missed actions
- Event-driven architecture for reliable state changes

**Performance Scaling**
- Ghosts in current arena update at 60 FPS
- Adjacent arena ghosts update at 30 FPS
- Distant arena ghosts update at 10-15 FPS
- Automatic quality adjustment when performance drops

**Technical Details**
- Timeline events stored as intent (2 bytes) vs transform (48 bytes)
- Binary search for efficient timeline lookups
- Zero-allocation helpers for event queries
- Arc<[T]> for cheap timeline sharing between systems

---

## Character Classes & Abilities

### The 8 Character Classes
Each class brings unique tactical advantages and 4 specialized abilities:

#### **Hunter** - Ranged Precision Specialist
- **Auto Shot**: Automatically fires at closest enemy every 2.5 seconds
- **Poison Shot**: Toxic projectile with knockback and damage over time
- **Sniper**: Long-range precision shots targeting bosses
- **Trap**: Explosive area denial placement system

#### **Alchemist** - Support Through Transformation
- **Ironskin Draft**: Defensive potion granting damage reduction
- **Acid Flask**: Area denial through persistent acid pools
- **Transmute**: Resource conversion and material transformation
- **Siphon**: Life-draining channeled ability targeting allies

#### **Cardinal** - Divine Healer and Protector
- **Heal**: Smart-targeting restoration ability
- **Barrier**: Round-robin ally protection system
- **Beam**: Piercing divine damage with healing properties
- **Resurrect**: Ultimate revival and telegraph enhancement

#### **Warrior** - Frontline Tank and Protector
- **Block**: Directional projectile defense system
- **Bash**: Offensive shield strike with damage mitigation
- **Taunt**: Threat redirection and aggro management
- **Bulwark**: Frontal barrier and area denial defense

#### **Thief** - Stealth and Mobility Expert
- **Shadow Step**: Evasive teleportation with invulnerability frames
- **Smoke Screen**: Concealment and safe passage utility
- **Backstab**: Positional damage enhancement passive
- **Pickpocket**: Resource extraction and buff theft

#### **Bard** - Team Enhancement Specialist
- **Dance**: Rhythm-based offensive quick-time event
- **Helix**: Dual-mode aura providing healing or haste
- **Cleanse**: Team-wide debuff removal utility
- **Mimic**: Passive ability copying from adjacent allies

#### **Forager** - Terrain Manipulation Expert
- **Dig**: Multi-tile excavation and terrain preparation
- **Boulder**: Rolling stone offensive with resource collection
- **Border**: Projectile-deflecting earth barriers
- **Mushroom**: Healing garden creation on prepared terrain

#### **Merchant** - Economic Warfare Specialist
- **Dice**: Stackable critical chance enhancement
- **Coin Toss**: Economic risk-reward projectile system
- **Fortune**: Team luck enhancement aura
- **Vault**: Area-effect critical damage amplification

---

## Arena System & Boss Battles

### Arena Structure
- **Grid Size**: 66×31 tiles per arena (1,254 tiles × 9 arenas = 11,286 total battlefield)
- **Boss Positioning**: Each arena contains one major boss matching its class theme
- **Multi-Arena Management**: All 9 arenas run independently with separate timers
- **Scaling Difficulty**: Normal → Heroic → Mythic progression tiers

### Arena Names & Layout

Arenic features **9 distinct arenas** arranged in a 3×3 grid layout. Each arena has a unique name and thematic identity:

| Index | Arena Name | Grid Position | Theme |
|-------|------------|---------------|-------|
| **0** | **Labyrinth** | Top-Left (0,0) | Hunter-focused with precision and traps |
| **1** | **Guild House** | Top-Center (1,0) | Alchemist-focused with transformation |
| **2** | **Sanctum** | Top-Right (2,0) | Cardinal-focused with divine magic |
| **3** | **Mountain** | Middle-Left (0,1) | Warrior-focused with strength and defense |
| **4** | **Bastion** | Middle-Center (1,1) | Thief-focused with stealth and mobility |
| **5** | **Pawnshop** | Middle-Right (2,1) | Bard-focused with rhythm and support |
| **6** | **Crucible** | Bottom-Left (0,2) | Forager-focused with terrain manipulation |
| **7** | **Casino** | Bottom-Center (1,2) | Merchant-focused with economic warfare |
| **8** | **Gala** | Bottom-Right (2,2) | Mixed themes and ultimate challenges |

#### Arena Navigation
```
Grid Layout (3×3):
[0] Labyrinth  [1] Guild House  [2] Sanctum
[3] Mountain   [4] Bastion      [5] Pawnshop  
[6] Crucible   [7] Casino       [8] Gala
```

#### Technical Implementation
- **ArenaName Enum**: Each arena uses a strongly-typed `ArenaName` enum instead of raw numeric indices
- **Type Safety**: Prevents invalid arena references and provides human-readable names
- **Index Conversion**: `ArenaName::as_u8()` provides the numeric index (0-8) when needed for calculations
- **Error Handling**: Invalid arena indices are handled gracefully with proper error messages

### Arena Navigation & Camera System
- **Arena Selection**: Use [ and ] keys to cycle through arenas (0-8, wraps around)
- **Camera Zoom**: Press P to toggle between single arena view and all-arenas overview
- **Visual Indicators**: Current arena highlighted with black border when zoomed out
- **Smart Focus**: Camera automatically positions on current arena when zooming in
- **Character Memory**: Each arena remembers its last active hero for seamless transitions

### Character Management Systems
- **Active Character Toggle**: Tab cycles through heroes in current arena (requires 2+ heroes)
- **Cross-Arena Movement**: WASD movement seamlessly transitions heroes between adjacent arenas
- **Arena Boundaries**: Movement past edges teleports character to opposite side of adjacent arena
- **Re-parenting System**: Characters automatically become children of their current arena entity
- **State Preservation**: Heroes maintain active/inactive status when switching arenas

### Arena Update Logic
- **Event-Driven Updates**: Arena state refreshes on camera changes or arena transitions
- **Zoom-Out Behavior**: All characters deactivated (gray) for overview visibility
- **Zoom-In Behavior**: Restores last active hero or activates first available character
- **Empty Arena Handling**: Gracefully handles arenas with no characters present
- **Material System**: Blue material for active heroes, gray for inactive ones

### Boss Mechanics
- **2-Minute Cycles**: Bosses operate on the same timing as your recordings
- **Deterministic Patterns**: Each boss has predictable, repeatable attack sequences
- **Telegraphed Attacks**: Visual warnings appear on grid tiles before major attacks
- **Pattern Recognition**: Success requires learning and countering boss rotations
- **No Enrage Timer**: Bosses reset each cycle without becoming stronger over time

### Arena-Specific Boss Types
0. **Labyrinth**: Features a Hunter-class boss with precision ranged attacks and deadly trap mechanics
1. **Guild House**: Alchemist boss using elemental transformation and area denial through toxic pools
2. **Sanctum**: Cardinal boss with healing denial, purification attacks, and divine shields
3. **Mountain**: Warrior boss with heavily armored defenses, charge attacks, and area damage
4. **Bastion**: Thief boss employing stealth mechanics, teleportation strikes, and ambush tactics
5. **Pawnshop**: Bard boss requiring timing-sensitive responses to rhythmic attack patterns
6. **Crucible**: Forager boss that dynamically changes the battlefield through terrain manipulation
7. **Casino**: Merchant boss with risk-reward mechanics and economic warfare strategies
8. **Gala**: Ultimate challenge arena featuring mixed boss mechanics from all other arenas

---

## Guild Management & Progression

### Gacha Recruitment System
- **Arena-Specific Recruitment**: Each arena only recruits heroes matching its class type
- **Battle Triggers**: Active combat in an arena generates gacha opportunities
- **Quality Tiers**: Heroes come in different rarity levels with enhanced abilities
- **Guild House Access**: Open recruitment boxes by pressing Enter and visiting guild house
- **Strategic Collection**: Build balanced rosters across all 8 character classes

### Character Development
- **Experience Growth**: Heroes gain levels through active participation in battles
- **Death Consequences**: Character death results in de-leveling, not permanent loss
- **Ability Evolution**: Higher-tier characters possess enhanced versions of base abilities
- **Equipment Systems**: Gear improvements provide statistical bonuses to character performance

### Guild House Operations
- **Management Hub**: Central location for all administrative functions
- **Recruitment Review**: Open and evaluate new character acquisitions
- **Global Buff Activation**: Use acquired consumables that affect all arenas simultaneously
- **Strategic Planning**: Review arena status and plan multi-arena coordination
- **Travel Coordination**: Manage character movement between different arenas

---

## Advanced Strategies

### Multi-Arena Coordination
- **Temporal Management**: Balance recording time across multiple arenas efficiently
- **Resource Allocation**: Distribute your best characters across priority arenas
- **Progressive Difficulty**: Master easier arenas before advancing to heroic/mythic tiers
- **Cross-Arena Learning**: Apply successful strategies from one arena to others

### Recording Optimization
- **Ability Timing**: Synchronize powerful abilities to create devastating combinations
- **Positioning Mastery**: Plan movement routes that maximize safety and effectiveness
- **Death Recovery**: Build recordings that account for potential character deaths
- **Cycle Efficiency**: Design recordings that smoothly transition between 2-minute cycles

### Team Composition Strategy
- **Tank-Healer-DPS**: Maintain classic MMO role balance in each arena
- **Class Synergy**: Combine complementary abilities for enhanced effectiveness
- **Backup Systems**: Include redundant healing and protection in case of deaths
- **Specialized Builds**: Develop arena-specific team compositions for unique challenges

---

## Combat Mechanics Deep Dive

### Grid-Based Tactical Combat
- **Tile Movement**: All positioning occurs on discrete grid squares
- **Line of Sight**: Abilities and attacks can be blocked by terrain features
- **Area of Effect**: Many abilities affect multiple adjacent tiles
- **Collision Rules**: Multiple characters can occupy the same grid cell
- **Environmental Hazards**: Some tiles contain traps, buffs, or damage zones

### Ability System Details
- **Cooldown Management**: Each ability has individual cooldown periods
- **Resource Costs**: Some abilities consume mana, stamina, or special resources
- **Cast Times**: Abilities have animation periods during which characters are vulnerable
- **Target Requirements**: Abilities may require specific targets (enemies, allies, empty tiles)
- **Upgrade Paths**: Higher-tier characters possess enhanced ability versions

### Death and Revival Mechanics
- **Death Consequences**: Characters lose levels and must restart from guild house
- **Revival Abilities**: Cardinals and other healers can resurrect fallen allies
- **Grid-Based Revival**: Revival spells target specific tiles rather than characters
- **Timing Requirements**: Revival must occur when dead character is present at target location
- **Recording Integration**: Deaths and revivals become part of the permanent timeline

---

## Progression & Victory Conditions

### Arena Mastery Progression
1. **Normal Tier**: Basic boss mechanics and standard difficulty
2. **Heroic Tier**: Enhanced boss abilities and additional mechanics
3. **Mythic Tier**: Maximum challenge with complex multi-phase encounters
4. **Perfect Runs**: Complete mastery demonstrated through flawless execution

### Long-Term Goals
- **Full Guild Development**: Recruit and develop 320+ heroes across all classes
- **Multi-Arena Excellence**: Successfully manage all 8 arenas simultaneously
- **Strategic Mastery**: Create sophisticated recording combinations across multiple cycles
- **Narrative Discovery**: Uncover the deeper story behind the arena conflicts

### Success Metrics
- **Boss Defeat Frequency**: Consistent victories across multiple 2-minute cycles
- **Character Survival**: Minimize deaths and level loss through strategic planning
- **Efficiency Optimization**: Achieve maximum damage/healing with minimal resource expenditure
- **Creative Problem Solving**: Develop innovative solutions to complex encounter mechanics

---

## Tips for New Players

### Essential Early Game Strategy
1. **Start Simple**: Master basic movement and single-character combat before recording
2. **Learn One Arena**: Focus on understanding one boss thoroughly before expanding
3. **Record Conservatively**: Create safe, reliable recordings rather than ambitious ones
4. **Study Patterns**: Observe boss timing carefully before committing to recordings
5. **Plan Positioning**: Always end recordings in safe locations for the next cycle

### Common Mistakes to Avoid
- **Rushing Recordings**: Take time to understand arena dynamics before committing
- **Ignoring Death Positions**: Don't leave ghosts in positions where they'll die repeatedly
- **Overcomplicating Early**: Simple, effective recordings outperform complex failures
- **Neglecting Other Arenas**: Passive arena management is crucial for overall progress
- **Poor Resource Management**: Balance advancement speed with character safety

### Mastery Development Path
1. **Movement Mastery**: Perfect grid-based positioning and timing
2. **Single-Character Combat**: Excel with individual heroes before team coordination
3. **Basic Recording**: Create simple, effective 2-minute action sequences
4. **Multi-Character Coordination**: Layer multiple recordings for enhanced effectiveness
5. **Advanced Strategy**: Develop sophisticated cross-arena management techniques

---

## Design Philosophy & Player Experience

### Intended Experience
Arenic transforms the complexity of managing massive raids into a solo experience that rewards strategic thinking, pattern recognition, and creative problem-solving. The game emphasizes:

- **Tactical Depth**: Every decision has cascading consequences across multiple timelines
- **Creative Expression**: Players develop unique solutions through recording combinations
- **Progressive Mastery**: Continuous improvement through iteration and refinement
- **Strategic Patience**: Success rewards careful planning over reactive gameplay

### Emotional Journey
- **Discovery Phase**: Wonder and experimentation with the recording system
- **Mastery Phase**: Growing confidence as patterns become familiar
- **Coordination Phase**: Satisfaction from successful multi-arena management
- **Innovation Phase**: Creative fulfillment from developing advanced strategies

The game rewards both analytical optimization and creative experimentation, providing a deeply satisfying experience for players who enjoy complex strategic challenges presented through elegant, accessible mechanics.

---

## Technical Architecture Principles

### Active Character Query Pattern

**Architectural Rule**: All recording system operations should query for the Active Character dynamically rather than storing or passing character entities as parameters.

#### Implementation Principle
```rust
// ✅ CORRECT - Query Active Character when needed
pub enum RecordingRequest {
    Start,        // Query active_character_q.single() when processing
    Stop { reason: StopReason },
    ShowDialog,   // Query active_character_q.single() when showing dialog
    Commit,       // Query active_character_q.single() when committing
    Clear,        // Query active_character_q.single() when clearing
}

// ❌ INCORRECT - Storing/passing entities
pub enum RecordingRequest {
    Start { entity: Entity },           // Don't store entities
    ShowDialog { character: Entity },   // Don't pass character data
}
```

#### Rationale
1. **Single Source of Truth**: The `Active` component determines which character receives input
2. **R Key Behavior**: "R always operates on whoever is CURRENTLY active when pressed" 
3. **Dynamic Dialog Behavior**: Dialog should switch if player changes active character
4. **Type Safety**: `active_character_q.single()` guarantees exactly one Active Character
5. **Simplicity**: Eliminates entity synchronization and parameter passing complexity

#### Query Pattern
```rust
// Standard query for Active Character operations
active_character_q: Single<(Entity, Option<&Ghost>), (With<Character>, With<Active>)>

// Usage in recording systems
let (character_entity, ghost_marker) = active_character_q.single();
```

#### Benefits
- **Architectural Consistency**: All recording operations follow the same pattern
- **Reduced Coupling**: No entity parameters between systems  
- **Automatic Updates**: Dialog and UI automatically reflect current active character
- **Performance**: O(1) queries with minimal overhead
- **Maintainability**: Eliminates entity storage synchronization bugs

#### Application Scope
This pattern applies to:
- All `RecordingRequest` variants
- Dialog state management (`RecordingMode::DialogPaused`)
- Recording state tracking
- UI systems that need character context
- Any system that operates on "the current character"

---

## Conclusion

Arenic offers a unique gaming experience that combines the strategic depth of MMO raiding with innovative single-player mechanics. Through mastering the Record & Replay system, understanding character class synergies, and developing sophisticated multi-arena strategies, players can achieve the satisfaction of commanding massive coordinated raids while maintaining complete control over every aspect of their guild's performance.

Success in Arenic comes not from quick reflexes, but from careful planning, pattern recognition, and the ability to think several moves ahead across multiple simultaneous battlefields. Every recording matters, every position counts, and every strategic decision ripples across your entire guild's effectiveness.

Welcome to Arenic—where strategic mastery meets creative expression in the ultimate raid simulation experience.