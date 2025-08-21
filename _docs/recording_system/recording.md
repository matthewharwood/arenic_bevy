# Recording System Technical Specification

## Glossary of Terms

### Core Components

- **CurrentArena**: The arena (0-8) that the player is currently viewing and interacting with
- **ActiveCharacter**: The single character within CurrentArena that responds to player input (WASD, abilities)
- **ArenaChild**: Any character entity that is a child of an Arena entity in the ECS hierarchy
- **Ghost**: An ArenaChild that is playing back a previously recorded PublishTimeline
- **PublishTimeline**: The committed, immutable recording of a character's actions over 2 minutes
- **DraftTimeline**: The temporary recording buffer that captures ActiveCharacter actions during a recording session
- **RecordingMode**: The state indicating that ActiveCharacter's actions are being captured to DraftTimeline

### Timeline Components

- **Event**: A single recorded action with a timestamp (e.g., Transform change, ability cast)
- **TransformEvent**: The position and rotation data of a character at a specific time
- **ActionEvent**: An ability activation or interaction at a specific timestamp
- **PlaybackMode**: The state where an ArenaChild replays its PublishTimeline automatically
- **TimelinePosition**: Current playback position (0.0 to 120.0 seconds) for each arena

### System States

- **ConfirmationDialog**: Modal UI that pauses all arena timelines and requires player decision
- **PassiveAbility**: Abilities that trigger automatically without player input (e.g., Hunter's AutoShot)
- **ActiveAbility**: Abilities requiring explicit player activation (e.g., HolyNova)

---

## Recording System Overview

The recording system enables players to create 2-minute action sequences for individual characters that replay
automatically as "ghosts" in subsequent arena cycles. This allows a single player to orchestrate complex multi-character
strategies across 9 independent arenas.

---

## Pre-Recording Setup

### Arena Selection

1. The game automatically selects Arena 0 as the initial CurrentArena on startup
2. Players can switch CurrentArena using `[` (previous) or `]` (next) keys
3. CurrentArena determines which arena receives player input

### Character Selection

1. Within CurrentArena, one ArenaChild is designated as ActiveCharacter
2. Players press `TAB` to cycle through available ArenaChildren in CurrentArena
3. Only ActiveCharacter responds to movement (WASD) and ability inputs
4. Visual indicator: ActiveCharacter uses blue material, inactive ArenaChildren use gray

---

## Recording Initiation

### Starting a Recording (R Key)

When the player presses `R` with an ActiveCharacter selected:

1. **Timeline Reset**: All ArenaChildren in CurrentArena reset to their first TransformEvent (t=0.0)
2. **Initial State Capture**: ActiveCharacter's current Transform is written as the first Event in DraftTimeline
3. **Countdown Display**: A 3-second countdown appears in the console ("3... 2... 1... Recording!")
4. **Mode Transition**: After countdown, RecordingMode activates for ActiveCharacter

### During Countdown

- All arena timelines continue normal playback
- Player input is disabled until countdown completes
- Other arenas continue their independent cycles unaffected

---

## During Recording (RecordingMode Active)

### ActiveCharacter Behavior

- All WASD movements are captured as TransformEvents in DraftTimeline
- All ability activations (keys 1-4) are captured as ActionEvents in DraftTimeline
- Each Event includes precise timestamp relative to recording start (0.0-120.0)
- Player retains full control over ActiveCharacter

### Ghost Playback in CurrentArena

- Non-active ArenaChildren with PublishTimelines enter PlaybackMode
- Ghosts replay their recorded Events at exact timestamps
- Ghost actions are deterministic and frame-perfect reproductions
- Ghosts cannot be controlled or interrupted during playback

### Inactive Characters Without Timelines

- ArenaChildren without PublishTimelines remain stationary at their spawn positions
- PassiveAbilities (e.g., Hunter's AutoShot) continue to function based on proximity
- No Events are recorded for these inactive characters
- They serve as static targets or obstacles

### Other Arenas During Recording

- All non-current arenas continue their independent 2-minute cycles
- Ghosts in other arenas replay their PublishTimelines normally
- Arena timelines are NOT synchronized unless explicitly paused

---

## Recording Time Management

### 2-Minute Timeline Structure

- Each arena operates on a fixed 120-second cycle (0.0 to 120.0)
- All recordings must fit within this window
- Timeline loops seamlessly from 120.0 back to 0.0

### Mid-Recording Interruption (R pressed before 120s)

When player presses `R` during an active recording:

1. **Dialog Trigger**: ConfirmationDialog appears with three options
2. **Global Pause**: ALL arena timelines pause at current positions
3. **Options**:
    - **Commit**: Move DraftTimeline � PublishTimeline, clear DraftTimeline, resume playback
    - **Clear**: Discard DraftTimeline, exit RecordingMode, resume at current time
    - **Cancel**: Keep DraftTimeline, continue recording, resume from pause point

### Full Recording Completion (120 seconds elapsed)

At the 2-minute mark:

1. **Dialog Trigger**: ConfirmationDialog appears with modified options
2. **Global Pause**: ALL arena timelines pause
3. **Options**:
    - **Commit**: Overwrite any existing PublishTimeline with DraftTimeline
    - **Clear**: Discard DraftTimeline, loop to t=0.0, exit RecordingMode
    - **Retry**: Clear DraftTimeline, reset to t=0.0, restart 3-second countdown

---

## Input Restrictions During Recording

### Blocked Actions

While RecordingMode is active, these inputs are handled specially:

**Immediate Recording Stop (No Confirmation):**
- `[` or `]` (arena switching) - Triggers CameraUpdate event, immediately stops recording
- `P` (camera zoom) - Triggers CameraUpdate event, immediately stops recording
- Attempting to move ActiveCharacter outside CurrentArena boundaries

**Confirmation Dialog Required:**
- `TAB` (character switching) - Shows confirmation dialog first:
  - **Switch Character**: Stop recording and proceed with character switch
  - **Continue Recording**: Cancel switch request and return to recording

### Arena Transition Handling

If ActiveCharacter tries to exit CurrentArena during recording:

1. Movement is intercepted and prevented
2. ConfirmationDialog appears
3. Character only transitions to new arena if player selects "Clear"
4. Selecting "Clear" abandons the current recording

---

## Post-Recording Behavior

### Characters with PublishTimelines

Once an ArenaChild has a committed PublishTimeline:

1. **Automatic Playback**: Replays timeline every 2-minute cycle
2. **Input Interception**: Any attempt to control (WASD/abilities/R) triggers ConfirmationDialog
3. **Options**:
    - **Retry**: Initiates new recording (resets arena to t=0.0, starts countdown)
    - **Cancel**: Closes dialog, character continues ghost playback

### Ghost Persistence

- PublishTimelines persist until explicitly overwritten by new recording
- Ghosts continue playback even when player is in different arenas
- All 320 potential ghosts (40 per arena � 8 arenas) can play simultaneously

---

## Technical Considerations

### Performance Optimization

- **Timeline Compression**: Store only keyframe Events, interpolate between them
- **Event Batching**: Group simultaneous Events to reduce memory overhead
- **LOD System**: Reduce playback fidelity for ghosts in non-current arenas

### State Synchronization

- All arenas share a global timer for coordinated pausing
- ConfirmationDialog blocks all timeline advancement
- Frame-perfect determinism ensures consistent ghost behavior

### Error Handling

- **Death During Recording**: Death Events are recorded and replayed (ghost dies at same time each cycle)
- **Invalid Timeline**: Corrupted timelines trigger automatic clearing and notification
- **Memory Limits**: System enforces maximum Event count per timeline

### Resource Management

- Ghost abilities consume resources from a separate "ghost pool" (infinite for now)
- Collision detection between ghosts uses spatial hashing for efficiency
- Timeline storage uses delta compression for Transform changes

---

## Implementation Priority

### Phase 1 (Core Recording)

- [ ] Basic recording/playback for movement only
- [ ] Single arena support
- [ ] Simple commit/cancel dialog

### Phase 2 (Full System)

- [ ] Multi-arena timeline management
- [ ] Ability recording and playback
- [ ] Complete ConfirmationDialog options

### Phase 3 (Optimization)

- [ ] Timeline compression
- [ ] Performance optimizations for 320 ghosts
- [ ] Visual polish (ghost transparency, recording indicators)

---

## Open Questions for Design Decisions

1. **Ghost Collision**: Should ghosts have collision with each other, or phase through?
2. **Ability Targeting**: Should ghost abilities re-target based on current state or use recorded targets?
3. **Timeline Editing**: Should players be able to trim/adjust recordings post-capture?
4. **Visual Clarity**: How to visually distinguish ghosts from active characters (transparency, color, effects)?
5. **Save System**: Should timelines persist between game sessions?