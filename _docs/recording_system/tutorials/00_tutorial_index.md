# Recording System Tutorial Series

## Lead Developer's Architectural Philosophy - REFINED

### Core Principles for These Tutorials

1. **Simplicity Over Cleverness**: We use simple, understandable patterns that junior developers can follow. No academic algorithms where basic ones suffice.

2. **Idiomatic Bevy**: We embrace Bevy's patterns - marker components, change detection, event-driven systems. We don't fight the framework.

3. **Appropriate Optimization**: We optimize where it matters (320 ghosts) but reject premature optimization (SIMD for lerp, custom allocators).

4. **No External Dependencies**: These tutorials use only Bevy. No state machine crates, no compression libraries, no UI frameworks. Learn the fundamentals.

5. **Production-Ready, Not Production-Perfect**: Code that could ship but doesn't include every possible optimization.

### Pragmatic Improvements Integrated (Post-Review)

After Jon's production-focused review, these patterns have been refined:

- **Type-Safe Newtypes**: `Timer`, `ArenaIdx`, `GridPos` with From/Display traits
- **Arc<[T]> for Published Data**: Immutable timelines with zero-cost cloning
- **Intent Recording Only**: Record WASD keys, never Transform changes
- **Event-Driven Transitions**: `RecordingTransition`, `DialogTransition` for traceability
- **SystemSets with .chain()**: Deterministic ordering with both patterns shown
- **Zero-Alloc Helpers**: `events_in_range()`, `slice()`, binary search throughout
- **Const Keymaps**: `const KEY_ABILITIES: [(KeyCode, AbilityId); 4]`
- **Changed<T> Reactive Systems**: State-driven UI updates
- **GlobalTimelinePause**: Proper pause state for all clocks
- **VecDeque for Trails**: O(1) front/back operations for buffers
- **Material Mutation**: Update materials in-place, no handle churn
- **UpdateFrequency Component**: Per-ghost update rates based on distance

### What We Explicitly Reject

- **Over-abstraction**: No trait objects for 3 dialog types
- **Academic Solutions**: No R-trees, octrees, or kd-trees for 320 entities
- **Premature Optimization**: No SIMD, custom allocators, or memory pools
- **External Dependencies**: No egui, no state machine crates, no compression libs
- **Feature Creep**: No undo/redo, no timeline editing, no save systems

## Overview

This tutorial series guides you through building a complete recording and playback system for Arenic, enabling players to record 2-minute character action sequences that replay as autonomous "ghosts". The system scales to support 320 simultaneous ghosts across 8 arenas while maintaining performant gameplay.

## Tutorial Structure

Each tutorial is designed to be completed in approximately 30 minutes and includes:
- Clear objectives and prerequisites
- Step-by-step implementation with code
- Unit tests for verification
- Practical testing instructions
- Key takeaways and next steps

## Tutorial Sequence

### Foundation (Tutorials 01-03)
Build the core data structures and recording mechanism.

#### [Tutorial 01: Timeline Foundation](01_timeline_foundation.md)
- **Time**: 30 minutes
- **Focus**: Core timeline data structures with type safety
- **Key Components**: TimelineEvent, DraftTimeline, PublishTimeline, TimelineClock
- **Key Patterns**: Newtypes with From/Display, Arc<[T]>, binary search, slice() helper
- **Outcome**: Type-safe foundation for storing and organizing recorded events

#### [Tutorial 02: Recording State Machine](02_recording_state.md)
- **Time**: 30 minutes
- **Focus**: Event-driven state management
- **Key Components**: RecordingState, RecordingTransition events, GlobalTimelinePause
- **Key Patterns**: Event boundaries, let-else returns, transition testing
- **Outcome**: Traceable state transitions with full event history

#### [Tutorial 03: Movement Capture](03_movement_capture.md)
- **Time**: 30 minutes
- **Focus**: Recording player intent, not transforms
- **Key Components**: Movement intent capture, const KEY_ABILITIES array, monotonic assertions
- **Key Patterns**: "Why not Changed<Transform>?" explanation, intent vs result
- **Outcome**: Deterministic action capture that replays perfectly

### Playback & UI (Tutorials 04-05)
Implement ghost playback and user interface.

#### [Tutorial 04: Playback System](04_playback_system.md)
- **Time**: 30 minutes
- **Focus**: Ghost replay with proper arena clocks
- **Key Components**: GhostArena, per-arena clocks, wrap-around handling
- **Key Patterns**: .chain() vs .after() examples, range queries with wrapping
- **Outcome**: Ghosts using parent arena clocks, not CurrentArena

#### [Tutorial 05: Commit Dialog System](05_commit_dialog.md)
- **Time**: 30 minutes
- **Focus**: Event-driven dialog system
- **Key Components**: Choice→Action mapping, DialogTransition events
- **Key Patterns**: Input layer emits events, comprehensive transition tests
- **Outcome**: Clean dialog system with full action mapping documentation

### Scaling & Polish (Tutorials 06-08)
Extend to multiple arenas and optimize performance.

#### [Tutorial 06: Multi-Arena Support](06_multi_arena_support.md)
- **Time**: 30 minutes
- **Focus**: Managing ghosts across arenas efficiently
- **Key Components**: ArenaGhostRegistry, iter_many_mut batching, clock sync modes
- **Key Patterns**: Multiple iter_many_mut examples, simple sync modes
- **Outcome**: Efficient multi-arena ghost management

#### [Tutorial 07: Visual Polish & Feedback](07_visual_polish.md)
- **Time**: 30 minutes
- **Focus**: State-driven visual feedback
- **Key Components**: Changed<RecordingState> UI, VecDeque trails, in-place materials
- **Key Patterns**: TimelineClock for UI, material mutation without handle churn
- **Outcome**: Performant visual feedback with minimal allocations

#### [Tutorial 08: Performance Optimization](08_performance_optimization.md)
- **Time**: 30 minutes
- **Focus**: Practical optimizations for 320 ghosts
- **Key Components**: Simple RLE compression, UpdateFrequency component, FrameTimeDiagnosticsPlugin
- **Key Patterns**: No complex algorithms, "Further Reading" for advanced topics
- **Outcome**: 60+ FPS with hundreds of ghosts using simple techniques

## Implementation Roadmap

### Phase 1: Core Recording (Tutorials 01-03)
**Goal**: Basic recording and storage of character actions
```
Week 1, Days 1-2:
□ Tutorial 01: Timeline Foundation
□ Tutorial 02: Recording State Machine  
□ Tutorial 03: Movement Capture
→ Milestone: Can record character movement
```

### Phase 2: Playback & UI (Tutorials 04-05)
**Goal**: Replay recordings as ghosts with user control
```
Week 1, Days 3-4:
□ Tutorial 04: Playback System
□ Tutorial 05: Commit Dialog
→ Milestone: Full record/playback cycle with UI
```

### Phase 3: Production Ready (Tutorials 06-08)
**Goal**: Scale to full game requirements
```
Week 1, Day 5 - Week 2, Day 1:
□ Tutorial 06: Multi-Arena Support
□ Tutorial 07: Visual Polish
□ Tutorial 08: Performance Optimization
→ Milestone: 320 ghosts with visual polish
```

## System Architecture

```
┌─────────────────────────────────────────────┐
│              User Input Layer               │
│         (Keyboard, Recording State)         │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│           Recording System                  │
│   (State Machine, Movement/Ability Capture) │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│           Timeline Storage                  │
│    (DraftTimeline → PublishTimeline)        │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│           Playback System                   │
│    (Ghost Entities, Timeline Replay)        │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│         Optimization Layer                  │
│   (Compression, LOD, Spatial Indexing)      │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│          Visual Feedback                    │
│    (UI, Effects, Audio, Indicators)         │
└─────────────────────────────────────────────┘
```

## Key Design Principles

### 1. **Components First**
Every piece of state is a component. Single-purpose, single-value components enable efficient queries and clear ownership.

### 2. **Events for Communication**  
Systems communicate through events, never direct mutation. This ensures loose coupling and traceable state changes.

### 3. **Marker Components**
Zero-sized marker components (`Recording`, `Ghost`, `Replaying`) efficiently categorize entities for queries.

### 4. **Change Detection**
Use `Changed<T>` and `Added<T>` to process only what needs updating, crucial for performance with many entities.

### 5. **Single Responsibility Systems**
Each system does one job well. Systems under 50 lines are easier to understand, test, and maintain.

## Architectural Decisions Made

### What We Keep Simple (Jon-Approved)
- **State Machine**: 4-state enum with event transitions
- **Interpolation**: Linear lerp with intent replay
- **Compression**: Basic RLE, advanced topics in "Further Reading"
- **Spatial**: HashMap grid for 320 entities
- **UI**: Bevy UI with Changed<T> reactivity

### Where We Optimize (Production-Focused)
- **Change Detection**: Changed<T> throughout, reactive systems
- **LOD System**: UpdateFrequency component per ghost
- **iter_many_mut**: Batch processing with cache locality
- **Arc<[T]>**: Zero-copy timeline sharing
- **Binary Search**: O(log n) operations on sorted data
- **VecDeque**: O(1) trail buffer management
- **Material Mutation**: Update in-place, no new handles

### What We Don't Do
- **Collision**: Ghosts are ethereal by design
- **Prediction**: We record actual positions
- **Streaming**: All arenas fit in memory
- **Shaders**: Material changes suffice
- **Particles**: UI feedback doesn't need VFX

## Testing Strategy

### Unit Tests
Each tutorial includes unit tests for core logic:
- Timeline event sorting and storage
- State machine transitions
- Compression algorithms
- Spatial indexing

### Integration Tests
Test complete flows:
- Record → Commit → Playback cycle
- Multi-arena timeline synchronization
- Dialog state management

### Performance Tests
Stress test with many ghosts:
- Spawn 100+ test ghosts (F12 in game)
- Monitor FPS and memory usage
- Verify auto-quality adjustment

## Common Patterns

### Grid to World Conversion
Grid units: 1 tile = 1.0 world units
Origin: (0,0) at arena bottom-left
Helper: `fn grid_to_world(grid: GridPos) -> Vec3`

### Timeline Event Pattern
```rust
TimelineEvent {
    timestamp: f32,        // 0.0 to 120.0 seconds
    event_type: EventType, // Transform, Ability, or Death
}
```

### Recording Flow Pattern
```
Idle → StartRecording → Countdown → Recording → StopRecording → Dialog → Commit/Clear → Idle
```

### Ghost Update Pattern
```rust
// Get current time from arena clock
// Interpolate position from timeline  
// Apply to transform
// Trigger abilities at timestamps
```

### Logging Guidelines

| Level | Usage |
|-------|-------|
| trace! | Per-frame updates |
| debug! | State transitions |
| info! | User actions |
| warn! | Performance issues |
| error! | Critical failures |

## Troubleshooting Guide

### Issue: Ghosts not moving
- Check ArenaTimer is updating
- Verify PublishTimeline has events
- Ensure Replaying component is present

### Issue: Poor performance with many ghosts
- Enable timeline compression
- Check update frequencies are working
- Verify LOD system is active
- Use batch processing

### Issue: Recording not starting
- Check RecordingState is Idle
- Verify character is not a Ghost
- Ensure Active component present

### Issue: Dialog not appearing
- Check DialogState resource
- Verify event is being sent
- Ensure UI camera exists

## Extension Points

After completing all tutorials, consider these enhancements:

1. **Death Recording**: Capture and replay character deaths
2. **Network Sync**: Share recordings between players
3. **Timeline Editing**: Trim or adjust recordings post-capture
4. **Advanced Ghosts**: Ghosts that adapt to current game state
5. **Save System**: Persist timelines between sessions

## Resources

- [Bevy Book](https://bevyengine.org/learn/book/introduction/)
- [Bevy ECS Guide](https://bevyengine.org/learn/book/getting-started/ecs/)
- [Bevy Performance Tips](https://github.com/bevyengine/bevy/blob/main/docs/profiling.md)

## Conclusion

This tutorial series provides a complete, production-ready recording system. By following the incremental approach, you'll build a robust system that scales from simple single-character recording to managing hundreds of ghosts across multiple arenas.

The modular design ensures each component can be understood, tested, and modified independently. The performance optimizations guarantee smooth gameplay even under heavy load.

Most importantly, the system is built following Bevy best practices and idiomatic Rust patterns, ensuring maintainability and extensibility for future development.

## Final Architectural Notes

### Why These Tutorials Succeed
1. **Clear Progression**: Each tutorial builds on the last
2. **Focused Scope**: One concept per tutorial
3. **Practical Code**: Everything compiles and runs
4. **Right-Sized Solutions**: Not under or over-engineered
5. **Testable**: Unit tests validate core logic

### For Production Use
If taking this to production, consider:
- Network synchronization for multiplayer
- Save/load system for persistence
- Timeline validation for anti-cheat
- Advanced visual effects for polish
- Analytics for player behavior

But none of these belong in tutorials. Learn the fundamentals first, then extend.

### Critical Test: Wraparound Ability Replay
Test ability replay across the ~119.98s → 0.02s boundary:
1. Record an ability at 119.5 seconds
2. Let timeline wrap to 0 seconds
3. Verify ability triggers correctly at wraparound
4. Check no duplicate or missed triggers

This tests the most complex edge case in the system.

Happy recording!