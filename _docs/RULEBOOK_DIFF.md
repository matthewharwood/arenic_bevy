# RULEBOOK COMPARISON: Missing Features & Nuances Analysis

## Overview
This document compares the current **RULEBOOK.md** with the **_OLD/_OLD_RULEBOOK.md** to identify features, mechanics, and design nuances that may be missing or under-developed in the new rulebook.

---

## 1. MAJOR MISSING SYSTEMS & FEATURES

### 1.1 Gacha System Details
**Missing from Current Rulebook:**
- **Upgrade Path System**: OLD mentions abilities can have upgrade paths via gacha that apply to single heroes
- **Arena-Specific Recruitment Mechanics**: Detailed explanation that each arena only produces recruits from its themed class
- **Gacha Buff Stacking Rules**: Multiple global buffs running in parallel with independent timers
- **Buff Tag & Level System**: Buffs have associated tags and levels, with higher levels overwriting lower ones while different buffs stack
- **Pity System Considerations**: Discussion of guaranteed higher-rarity recruits vs static probability with tier progression

### 1.2 Offline Idle Progression
**Completely Missing System:**
- **Snapshot-Based Calculation**: Game captures arena state snapshots on exit, calculates offline time divided by 2-minute cycles
- **Active Character Scaling**: Offline rewards scale based on number of active raiding characters
- **Background Simulation**: Characters continue participating in battles during offline periods
- **Chat Log Integration**: Offline events (deaths, victories) captured in filterable global chat logs

### 1.3 Death, Revival & Travel Mechanics
**Missing Nuances:**
- **Grid-Based Revival System**: Revival spells target specific tiles, not characters directly
- **Revival Miss Mechanics**: If no dead character is present at target location, counts as "revive miss"
- **Travel Attack System**: Heroes traveling between arenas can take damage and die en route via background simulation
- **Character State During Travel**: Characters remain in background simulation but don't formally record timeline during travel

### 1.4 Determinism & Future RNG
**Missing Technical Framework:**
- **RNG Implementation Plans**: Acknowledgment of planned non-deterministic elements with proper testing approach
- **Critical Hit Systems**: Future consideration of small RNG elements (critical hits) while maintaining timeline integrity
- **Collision Rule Ambiguity**: Current rulebook states multiple characters can occupy same grid, but OLD shows this was undecided

---

## 2. USER INTERFACE & CONTROLS GAPS

### 2.1 Advanced UI Features
**Missing from Current Rulebook:**
- **Chat Filter System**: Advanced filtering for boss kills, recruit notifications, lore reveals
- **Global HUD for Buffs**: Centralized buff timer display vs per-arena timers
- **Contextual UI Cues**: Tooltips, highlights, and prompts for loot/interactive objects
- **Timeline Idle Indicators**: Clear UI showing when characters are idling during remaining recording time
- **Offline Report System**: Structured offline event reporting through chat log filters

### 2.2 Developer Tools Integration
**Missing Development Features:**
- **Timeline Viewer**: In-engine debugging tool for stepping through frames
- **Persistent Snapshot System**: Dev snapshots stored as loadable files
- **Console Commands**: Frame-jumping and timeline debugging capabilities
- **Performance Scaling Display**: Visual indicators of arena update rates (60/30/10-15 FPS)

---

## 3. ARENA & BOSS MECHANICS REFINEMENTS

### 3.1 Arena Management Details
**Missing Operational Nuances:**
- **Arena Synchronization Rules**: Arenas can be out of sync, timers reset independently
- **Performance Scaling Logic**: Detailed explanation of current/adjacent/distant arena update rates
- **Arena Transition Mechanics**: Character re-parenting system and state preservation
- **Empty Arena Handling**: Graceful handling of arenas with no characters

### 3.2 Boss Scaling & Mechanics
**Missing Design Decisions:**
- **Static Boss Difficulty**: Explicit statement that bosses don't scale with character count to preserve determinism
- **Boss Timeline Integration**: Confirmation that bosses follow same 2-minute recording system rather than hand-coded patterns
- **Enrage Timer Removal**: OLD mentions "enrage at 2:00" was removed as design feature
- **Multi-Boss Scenarios**: Possibility of multiple bosses, mini-bosses, or creeps in single arena

---

## 4. TECHNICAL IMPLEMENTATION DETAILS

### 4.1 Data Storage & Memory
**Missing Technical Specifications:**
- **Timeline Data Structure**: Action start/end with duration vs frame-by-frame storage
- **Compression Strategy**: Timeline events stored as intent (2 bytes) vs transform (48 bytes)
- **Memory Optimization**: Zero-allocation helpers and Arc<[T]> for timeline sharing
- **Persistent Storage**: Codex/lore stored in local text files rather than database

### 4.2 Recording System Internals
**Missing Implementation Details:**
- **State Machine Tracking**: Idle → Countdown → Recording → Dialog state transitions
- **Input Event Types**: Click-and-hold captured as keydown/keyup events, multi-tap as N tap events
- **Timeline Accuracy**: Movement intent vs position storage for perfect deterministic replay
- **Recording Compression**: Automatic filtering of redundant movement events

---

## 5. PROGRESSION & NARRATIVE SYSTEMS

### 5.1 Character Development
**Missing Progression Details:**
- **Experience Growth System**: Heroes gain levels through active battle participation
- **De-leveling Consequences**: Death results in level loss, not permanent character loss
- **Equipment Integration**: Gear improvements provide statistical bonuses
- **Ability Evolution**: Higher-tier characters have enhanced ability versions

### 5.2 Narrative Integration
**Missing Story Systems:**
- **Echo Guild Commentary**: Flavor text and chatter systems for immersion
- **Codex Generation**: AI-generated lore with thousands of lines of randomized content
- **Narrative Triggers**: Boss defeats and progression milestones advance storyline
- **Dialogue System**: Real-time message bubbles on characters with global chat integration

### 5.3 Guild Management
**Missing Management Features:**
- **Global Buff Activation**: Use consumables from guild house that affect all arenas
- **Travel Coordination**: Manage character movement between arenas
- **Recruitment Box System**: Physical interaction with gacha rewards via guild house
- **Strategic Planning Hub**: Review arena status and plan multi-arena coordination

---

## 6. ADVANCED GAMEPLAY SYSTEMS

### 6.1 Multi-Arena Coordination
**Missing Strategic Elements:**
- **Resource Allocation Strategy**: Distribute best characters across priority arenas
- **Progressive Difficulty Management**: Master easier arenas before advancing tiers
- **Cross-Arena Learning**: Apply successful strategies between different arenas
- **Temporal Management**: Balance recording time efficiently across multiple arenas

### 6.2 Environmental Interactions
**Missing Environmental Systems:**
- **Environmental Hazards**: Traps, damage zones, and terrain features affecting combat
- **Buff Tiles**: Environmental bonuses and strategic positioning elements
- **Terrain Manipulation**: Forager abilities that dynamically change battlefield
- **Interactive Objects**: Chests, loot containers, and confirmation dialogues

---

## 7. QUALITY OF LIFE & POLISH FEATURES

### 7.1 Player Experience Enhancements
**Missing QoL Features:**
- **Replay Highlight System**: Review interesting events from past cycles (P1 feature)
- **Timeline History**: Persistent recording history for strategic rollback options
- **Visual Timeline Editor**: Minimalistic approach to show recorded actions
- **Performance Auto-Adjustment**: Automatic quality scaling when performance drops

### 7.2 Accessibility & Usability
**Missing Usability Features:**
- **Color-Blind Considerations**: Rare loot highlighting and visual distinction systems
- **Input Conflict Resolution**: Priority handling for simultaneous ability triggers
- **Movement Cooldown**: Small animation delays to prevent twitchy movement
- **Contextual Help**: In-game guidance for complex timeline management

---

## 8. DESIGN PHILOSOPHY GAPS

### 8.1 Player Psychology & Engagement
**Missing Design Considerations:**
- **Risk-Reward Balance**: Economic warfare mechanics and fortune systems
- **Creative Problem Solving**: Emphasis on innovative solution development
- **Strategic Patience Rewards**: Careful planning over reactive gameplay benefits
- **Iterative Mastery Path**: Progressive improvement through refinement cycles

### 8.2 Long-term Vision Elements
**Missing Future Planning:**
- **User-Created Raids**: Testing boss mechanics through player-generated content (P2)
- **Multiplayer Considerations**: Optional co-op features for future implementation (P2)
- **Modding Potential**: Timeline system extensibility for community content
- **Competitive Elements**: Potential for shared strategies or community challenges

---

## 9. RECOMMENDATIONS

### 9.1 High Priority Additions
1. **Offline Idle System**: Critical for player retention and progression feel
2. **Gacha Enhancement Details**: Upgrade paths and buff stacking rules
3. **Advanced UI Features**: Chat filters and global buff display
4. **Death/Revival Nuances**: Grid-based revival and travel mechanics

### 9.2 Medium Priority Additions
1. **Developer Tools Integration**: Timeline debugging and snapshot systems
2. **Environmental Interactions**: Hazards, buff tiles, and terrain manipulation
3. **Narrative Systems**: Echo guild commentary and dialogue integration
4. **Performance Scaling**: Detailed explanation of arena update optimization

### 9.3 Future Consideration Items
1. **RNG Integration Framework**: Planned approach for non-deterministic elements
2. **User-Created Content**: Raid building tools and community features
3. **Advanced Analytics**: Player behavior tracking and optimization metrics
4. **Multiplayer Architecture**: Foundation for future co-op implementation

---

## CONCLUSION

The current RULEBOOK.md provides excellent high-level gameplay guidance but lacks many of the nuanced systems, technical details, and quality-of-life features discussed in the OLD rulebook. The most critical gaps are in **offline progression**, **gacha system details**, **advanced UI features**, and **environmental/narrative integration**.

These missing elements represent significant depth that could greatly enhance player engagement and provide the polish necessary for a compelling final product. The OLD rulebook's extensive FAQ and design decision documentation reveals sophisticated thinking about edge cases and player experience that should be preserved and integrated into the current design.