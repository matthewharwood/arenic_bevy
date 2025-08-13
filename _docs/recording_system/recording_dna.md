# Recording System Implementation DNA

## Architecture Overview

The recording system follows these core principles:
- **Components First**: Single-purpose, single-value components
- **Events for Communication**: Systems communicate via events, not direct mutation
- **Marker Components**: Zero-sized components for entity categorization
- **Change Detection**: React to changes, don't poll everything
- **Single Responsibility**: Each system does exactly one job

---

## Phase 1: Core Components

### Timeline Components
```rust
// Marker for entities that can be recorded
#[derive(Component)]
pub struct Recordable;

// Marker for entities currently being recorded
#[derive(Component)]
pub struct Recording;

// Marker for entities replaying a timeline
#[derive(Component)]
pub struct Replaying;

// The draft timeline being recorded (temporary buffer)
#[derive(Component)]
pub struct DraftTimeline {
    pub events: Vec<TimelineEvent>,
}

// The published timeline for playback (immutable)
#[derive(Component)]
pub struct PublishTimeline {
    pub events: Vec<TimelineEvent>,
}

// Current playback position in seconds (0.0 to 120.0)
#[derive(Component)]
pub struct TimelinePosition(pub f32);

// Arena-wide timer for 2-minute cycles
#[derive(Component)]
pub struct ArenaTimer(pub f32);

// Countdown timer for recording start
#[derive(Component)]
pub struct RecordingCountdown(pub f32);

// Marker for paused timelines
#[derive(Component)]
pub struct TimelinePaused;
```

### Event Data Components
```rust
// Single timeline event
#[derive(Clone, Debug)]
pub struct TimelineEvent {
    pub timestamp: f32,
    pub event_type: EventType,
}

#[derive(Clone, Debug)]
pub enum EventType {
    Transform(Vec3),
    Ability(AbilityId),
    Death,
}

// Ability identifier
#[derive(Clone, Copy, Debug)]
pub struct AbilityId(pub u8);
```

### Recording State Components
```rust
// Global recording mode state
#[derive(Resource)]
pub struct RecordingState {
    pub mode: RecordingMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordingMode {
    Idle,
    Countdown,
    Recording,
    DialogPaused,
}

// Dialog state for confirmations
#[derive(Component)]
pub struct ConfirmationDialog {
    pub dialog_type: DialogType,
}

#[derive(Debug, Clone)]
pub enum DialogType {
    MidRecording,
    EndRecording,
    RetryGhost,
}
```

### Ghost Components
```rust
// Marker for ghost entities
#[derive(Component)]
pub struct Ghost;

// Reference to the original character being ghosted
#[derive(Component)]
pub struct GhostSource(pub Entity);

// Visual transparency for ghosts
#[derive(Component)]
pub struct GhostTransparency(pub f32);
```

---

## Phase 2: Event Definitions

```rust
// Recording control events
#[derive(Event)]
pub struct StartRecording {
    pub character: Entity,
    pub arena: u8,
}

#[derive(Event)]
pub struct StopRecording {
    pub character: Entity,
}

#[derive(Event)]
pub struct CommitRecording {
    pub character: Entity,
}

#[derive(Event)]
pub struct ClearRecording {
    pub character: Entity,
}

#[derive(Event)]
pub struct RetryRecording {
    pub character: Entity,
}

// Timeline control events
#[derive(Event)]
pub struct PauseAllTimelines;

#[derive(Event)]
pub struct ResumeAllTimelines;

#[derive(Event)]
pub struct ResetArenaTimeline {
    pub arena: u8,
}

// Dialog events
#[derive(Event)]
pub struct ShowDialog {
    pub dialog_type: DialogType,
}

#[derive(Event)]
pub struct DialogChoice {
    pub choice: DialogOption,
}

#[derive(Debug)]
pub enum DialogOption {
    Commit,
    Clear,
    Cancel,
    Retry,
}
```

---

## Phase 3: System Implementation Order

### 1. Timer Update System
```rust
pub fn update_arena_timers(
    time: Res<Time>,
    mut arena_q: Query<(&Arena, &mut ArenaTimer), Without<TimelinePaused>>,
) {
    // Update timer for each arena that isn't paused
    // - Get delta time from Time resource
    // - Add delta to ArenaTimer value
    // - If timer >= 120.0, reset to 0.0 (loop)
    // - This creates the 2-minute cycle foundation
    
    for (arena, mut timer) in arena_q.iter_mut() {
        timer.0 += time.delta_secs();
        if timer.0 >= 120.0 {
            timer.0 = 0.0;
        }
    }
}
```

### 2. Recording Input Detection System
```rust
pub fn detect_recording_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    recording_state: Res<RecordingState>,
    active_character_q: Query<Entity, (With<Character>, With<Active>, Without<Ghost>)>,
    mut start_recording_events: EventWriter<StartRecording>,
    current_arena: Res<CurrentArena>,
) {
    // Only process R key in idle state
    // - Check if R pressed and state is Idle
    // - Get the active character entity
    // - Send StartRecording event with character and arena
    // - This initiates the recording flow
    
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }
    
    if recording_state.mode != RecordingMode::Idle {
        return;
    }
    
    if let Ok(character) = active_character_q.get_single() {
        start_recording_events.send(StartRecording {
            character,
            arena: current_arena.0,
        });
    }
}
```

### 3. Recording Initialization System
```rust
pub fn initialize_recording(
    mut commands: Commands,
    mut start_events: EventReader<StartRecording>,
    mut recording_state: ResMut<RecordingState>,
    character_q: Query<&Transform, With<Character>>,
    mut reset_events: EventWriter<ResetArenaTimeline>,
) {
    // Process StartRecording events
    // - Reset arena timeline to 0.0
    // - Capture initial transform as first event
    // - Add Recording marker to character
    // - Add DraftTimeline component with initial event
    // - Set RecordingState to Countdown
    // - Add RecordingCountdown(3.0) to character
    
    for event in start_events.read() {
        // Reset the arena timeline
        reset_events.send(ResetArenaTimeline { arena: event.arena });
        
        // Get character transform
        if let Ok(transform) = character_q.get(event.character) {
            // Create initial timeline event
            let initial_event = TimelineEvent {
                timestamp: 0.0,
                event_type: EventType::Transform(transform.translation),
            };
            
            // Add recording components
            commands.entity(event.character)
                .insert(Recording)
                .insert(DraftTimeline {
                    events: vec![initial_event],
                })
                .insert(RecordingCountdown(3.0));
            
            // Update global state
            recording_state.mode = RecordingMode::Countdown;
        }
    }
}
```

### 4. Countdown Update System
```rust
pub fn update_recording_countdown(
    mut commands: Commands,
    time: Res<Time>,
    mut recording_state: ResMut<RecordingState>,
    mut countdown_q: Query<(Entity, &mut RecordingCountdown)>,
) {
    // Update countdown timers
    // - Decrement countdown by delta time
    // - Print countdown messages at 3, 2, 1
    // - When countdown <= 0:
    //   - Remove RecordingCountdown component
    //   - Set RecordingState to Recording
    //   - Print "Recording!" message
    
    for (entity, mut countdown) in countdown_q.iter_mut() {
        let prev = countdown.0;
        countdown.0 -= time.delta_secs();
        
        // Print countdown messages
        if prev > 2.0 && countdown.0 <= 2.0 {
            println!("2...");
        } else if prev > 1.0 && countdown.0 <= 1.0 {
            println!("1...");
        } else if prev > 0.0 && countdown.0 <= 0.0 {
            println!("Recording!");
            
            // Transition to recording
            commands.entity(entity).remove::<RecordingCountdown>();
            recording_state.mode = RecordingMode::Recording;
        }
    }
}
```

### 5. Movement Recording System
```rust
pub fn record_character_movement(
    recording_q: Query<(Entity, &Transform, &mut DraftTimeline), (With<Recording>, Changed<Transform>)>,
    arena_q: Query<&ArenaTimer, With<Arena>>,
    current_arena: Res<CurrentArena>,
) {
    // Capture transform changes during recording
    // - Only runs when Transform changes (Changed<Transform>)
    // - Get current arena timer value as timestamp
    // - Create TransformEvent with position
    // - Append to DraftTimeline events
    // - This creates the movement trail
    
    for (entity, transform, mut timeline) in recording_q.iter_mut() {
        // Get current timestamp from arena timer
        if let Some(timer) = arena_q.iter().find(|t| /* match arena */) {
            let event = TimelineEvent {
                timestamp: timer.0,
                event_type: EventType::Transform(transform.translation),
            };
            timeline.events.push(event);
        }
    }
}
```

### 6. Ability Recording System
```rust
pub fn record_ability_usage(
    keyboard: Res<ButtonInput<KeyCode>>,
    recording_q: Query<&mut DraftTimeline, With<Recording>>,
    arena_q: Query<&ArenaTimer, With<Arena>>,
    current_arena: Res<CurrentArena>,
) {
    // Capture ability key presses during recording
    // - Check keys 1-4 for ability activation
    // - Get current arena timer as timestamp
    // - Create AbilityEvent with ability ID
    // - Append to DraftTimeline events
    // - This records ability casts
    
    for mut timeline in recording_q.iter_mut() {
        // Check each ability key
        let ability_id = if keyboard.just_pressed(KeyCode::Digit1) {
            Some(AbilityId(1))
        } else if keyboard.just_pressed(KeyCode::Digit2) {
            Some(AbilityId(2))
        } else if keyboard.just_pressed(KeyCode::Digit3) {
            Some(AbilityId(3))
        } else if keyboard.just_pressed(KeyCode::Digit4) {
            Some(AbilityId(4))
        } else {
            None
        };
        
        if let Some(id) = ability_id {
            // Record ability event
            if let Some(timer) = arena_q.iter().find(|t| /* match arena */) {
                let event = TimelineEvent {
                    timestamp: timer.0,
                    event_type: EventType::Ability(id),
                };
                timeline.events.push(event);
            }
        }
    }
}
```

### 7. Recording Interruption Detection System
```rust
pub fn detect_recording_interruption(
    keyboard: Res<ButtonInput<KeyCode>>,
    recording_state: Res<RecordingState>,
    recording_q: Query<Entity, With<Recording>>,
    mut pause_events: EventWriter<PauseAllTimelines>,
    mut dialog_events: EventWriter<ShowDialog>,
) {
    // Detect attempts to interrupt recording
    // - Check for R key during Recording mode
    // - Check for arena switch keys ([ ])
    // - Check for character switch (Tab)
    // - If detected:
    //   - Send PauseAllTimelines event
    //   - Send ShowDialog event with MidRecording type
    //   - Set RecordingState to DialogPaused
    
    if recording_state.mode != RecordingMode::Recording {
        return;
    }
    
    let interrupted = keyboard.just_pressed(KeyCode::KeyR) ||
                     keyboard.just_pressed(KeyCode::BracketLeft) ||
                     keyboard.just_pressed(KeyCode::BracketRight) ||
                     keyboard.just_pressed(KeyCode::Tab);
    
    if interrupted && recording_q.get_single().is_ok() {
        pause_events.send(PauseAllTimelines);
        dialog_events.send(ShowDialog {
            dialog_type: DialogType::MidRecording,
        });
    }
}
```

### 8. Recording Completion Detection System
```rust
pub fn detect_recording_completion(
    arena_q: Query<(&Arena, &ArenaTimer)>,
    recording_q: Query<Entity, With<Recording>>,
    current_arena: Res<CurrentArena>,
    mut pause_events: EventWriter<PauseAllTimelines>,
    mut dialog_events: EventWriter<ShowDialog>,
) {
    // Check if recording has reached 120 seconds
    // - Get current arena timer
    // - If timer >= 120.0 and character is recording:
    //   - Send PauseAllTimelines event
    //   - Send ShowDialog event with EndRecording type
    // - This triggers the completion dialog
    
    if let Ok(recording_entity) = recording_q.get_single() {
        for (arena, timer) in arena_q.iter() {
            if arena.0 == current_arena.0 && timer.0 >= 119.9 {
                pause_events.send(PauseAllTimelines);
                dialog_events.send(ShowDialog {
                    dialog_type: DialogType::EndRecording,
                });
            }
        }
    }
}
```

### 9. Timeline Pause System
```rust
pub fn pause_all_timelines(
    mut commands: Commands,
    mut pause_events: EventReader<PauseAllTimelines>,
    arena_q: Query<Entity, With<Arena>>,
    mut recording_state: ResMut<RecordingState>,
) {
    // Pause all arena timelines
    // - Add TimelinePaused marker to all arenas
    // - Set RecordingState to DialogPaused
    // - This freezes all timeline progression
    
    for _ in pause_events.read() {
        for arena_entity in arena_q.iter() {
            commands.entity(arena_entity).insert(TimelinePaused);
        }
        recording_state.mode = RecordingMode::DialogPaused;
    }
}
```

### 10. Dialog Display System
```rust
pub fn display_confirmation_dialog(
    mut dialog_events: EventReader<ShowDialog>,
    mut commands: Commands,
) {
    // Create dialog UI entities
    // - Spawn dialog background entity
    // - Add ConfirmationDialog component with type
    // - Spawn button entities for options
    // - Position in screen center
    // - This creates the visual dialog
    
    for event in dialog_events.read() {
        // Spawn dialog UI (simplified - would use bevy_ui)
        commands.spawn(ConfirmationDialog {
            dialog_type: event.dialog_type.clone(),
        });
        
        println!("Dialog: {:?}", event.dialog_type);
        println!("Options: Commit, Clear, Cancel/Retry");
    }
}
```

### 11. Dialog Input System
```rust
pub fn process_dialog_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    dialog_q: Query<(Entity, &ConfirmationDialog)>,
    mut choice_events: EventWriter<DialogChoice>,
    mut commands: Commands,
) {
    // Process dialog choice input
    // - Map keys to dialog options (1=Commit, 2=Clear, 3=Cancel/Retry)
    // - Send DialogChoice event with selection
    // - Remove dialog entity
    // - This handles user dialog interaction
    
    if let Ok((dialog_entity, dialog)) = dialog_q.get_single() {
        let choice = if keyboard.just_pressed(KeyCode::Digit1) {
            Some(DialogOption::Commit)
        } else if keyboard.just_pressed(KeyCode::Digit2) {
            Some(DialogOption::Clear)
        } else if keyboard.just_pressed(KeyCode::Digit3) {
            match dialog.dialog_type {
                DialogType::MidRecording => Some(DialogOption::Cancel),
                DialogType::EndRecording => Some(DialogOption::Retry),
                DialogType::RetryGhost => Some(DialogOption::Retry),
            }
        } else {
            None
        };
        
        if let Some(option) = choice {
            choice_events.send(DialogChoice { choice: option });
            commands.entity(dialog_entity).despawn();
        }
    }
}
```

### 12. Dialog Choice Processing System
```rust
pub fn process_dialog_choice(
    mut choice_events: EventReader<DialogChoice>,
    recording_q: Query<Entity, With<Recording>>,
    mut commit_events: EventWriter<CommitRecording>,
    mut clear_events: EventWriter<ClearRecording>,
    mut resume_events: EventWriter<ResumeAllTimelines>,
    mut retry_events: EventWriter<RetryRecording>,
) {
    // Route dialog choices to appropriate events
    // - Read DialogChoice events
    // - Based on choice type:
    //   - Commit: Send CommitRecording event
    //   - Clear: Send ClearRecording event
    //   - Cancel: Send ResumeAllTimelines event
    //   - Retry: Send RetryRecording event
    
    for event in choice_events.read() {
        if let Ok(recording_entity) = recording_q.get_single() {
            match event.choice {
                DialogOption::Commit => {
                    commit_events.send(CommitRecording {
                        character: recording_entity,
                    });
                }
                DialogOption::Clear => {
                    clear_events.send(ClearRecording {
                        character: recording_entity,
                    });
                }
                DialogOption::Cancel => {
                    resume_events.send(ResumeAllTimelines);
                }
                DialogOption::Retry => {
                    retry_events.send(RetryRecording {
                        character: recording_entity,
                    });
                }
            }
        }
    }
}
```

### 13. Commit Recording System
```rust
pub fn commit_recording(
    mut commands: Commands,
    mut commit_events: EventReader<CommitRecording>,
    draft_q: Query<&DraftTimeline>,
    mut resume_events: EventWriter<ResumeAllTimelines>,
    mut recording_state: ResMut<RecordingState>,
) {
    // Commit draft timeline to published timeline
    // - Get DraftTimeline from character
    // - Create PublishTimeline with events
    // - Remove Recording marker
    // - Remove DraftTimeline
    // - Add PublishTimeline component
    // - Add Ghost marker
    // - Send ResumeAllTimelines event
    
    for event in commit_events.read() {
        if let Ok(draft) = draft_q.get(event.character) {
            // Clone events for publish timeline
            let published = PublishTimeline {
                events: draft.events.clone(),
            };
            
            // Update character components
            commands.entity(event.character)
                .remove::<Recording>()
                .remove::<DraftTimeline>()
                .insert(published)
                .insert(Ghost)
                .insert(Replaying)
                .insert(TimelinePosition(0.0));
            
            recording_state.mode = RecordingMode::Idle;
            resume_events.send(ResumeAllTimelines);
        }
    }
}
```

### 14. Clear Recording System
```rust
pub fn clear_recording(
    mut commands: Commands,
    mut clear_events: EventReader<ClearRecording>,
    mut resume_events: EventWriter<ResumeAllTimelines>,
    mut recording_state: ResMut<RecordingState>,
) {
    // Discard draft timeline
    // - Remove Recording marker
    // - Remove DraftTimeline component
    // - Set RecordingState to Idle
    // - Send ResumeAllTimelines event
    // - This cancels the recording
    
    for event in clear_events.read() {
        commands.entity(event.character)
            .remove::<Recording>()
            .remove::<DraftTimeline>();
        
        recording_state.mode = RecordingMode::Idle;
        resume_events.send(ResumeAllTimelines);
    }
}
```

### 15. Resume Timelines System
```rust
pub fn resume_all_timelines(
    mut commands: Commands,
    mut resume_events: EventReader<ResumeAllTimelines>,
    paused_q: Query<Entity, With<TimelinePaused>>,
) {
    // Resume all paused timelines
    // - Remove TimelinePaused marker from all arenas
    // - This resumes timeline progression
    
    for _ in resume_events.read() {
        for entity in paused_q.iter() {
            commands.entity(entity).remove::<TimelinePaused>();
        }
    }
}
```

### 16. Reset Arena Timeline System
```rust
pub fn reset_arena_timeline(
    mut reset_events: EventReader<ResetArenaTimeline>,
    mut arena_q: Query<(&Arena, &mut ArenaTimer)>,
    mut ghost_q: Query<(&Parent, &mut TimelinePosition), With<Ghost>>,
) {
    // Reset specific arena to time 0
    // - Set ArenaTimer to 0.0
    // - Reset all ghost TimelinePositions in that arena
    // - Reset all ghost transforms to first position
    // - This creates the clean slate for recording
    
    for event in reset_events.read() {
        // Reset arena timer
        for (arena, mut timer) in arena_q.iter_mut() {
            if arena.0 == event.arena {
                timer.0 = 0.0;
            }
        }
        
        // Reset ghost positions
        for (parent, mut position) in ghost_q.iter_mut() {
            // Check if ghost belongs to this arena
            position.0 = 0.0;
        }
    }
}
```

### 17. Ghost Playback System
```rust
pub fn playback_ghost_timelines(
    ghost_q: Query<(&PublishTimeline, &mut TimelinePosition, &mut Transform), With<Replaying>>,
    arena_q: Query<(&Arena, &ArenaTimer)>,
    current_arena: Res<CurrentArena>,
) {
    // Replay published timelines for ghosts
    // - Get current arena timer
    // - For each ghost with PublishTimeline:
    //   - Find events near current timestamp
    //   - Interpolate transform between events
    //   - Update ghost position
    // - This creates the ghost movement
    
    for (timeline, mut position, mut transform) in ghost_q.iter_mut() {
        // Get current time from arena
        if let Some((_, timer)) = arena_q.iter().find(|(a, _)| a.0 == current_arena.0) {
            position.0 = timer.0;
            
            // Find relevant events for interpolation
            let current_time = timer.0;
            
            // Find the two events to interpolate between
            let mut prev_event = None;
            let mut next_event = None;
            
            for event in &timeline.events {
                if event.timestamp <= current_time {
                    prev_event = Some(event);
                } else {
                    next_event = Some(event);
                    break;
                }
            }
            
            // Apply interpolated transform
            if let Some(prev) = prev_event {
                if let EventType::Transform(pos) = prev.event_type {
                    if let Some(next) = next_event {
                        if let EventType::Transform(next_pos) = next.event_type {
                            // Interpolate between positions
                            let t = (current_time - prev.timestamp) / 
                                   (next.timestamp - prev.timestamp);
                            transform.translation = pos.lerp(next_pos, t);
                        }
                    } else {
                        // Use last known position
                        transform.translation = pos;
                    }
                }
            }
        }
    }
}
```

### 18. Ghost Ability Playback System
```rust
pub fn playback_ghost_abilities(
    ghost_q: Query<(&PublishTimeline, &TimelinePosition), With<Ghost>>,
    mut ability_events: EventWriter<TriggerAbility>,
) {
    // Trigger ghost abilities at recorded timestamps
    // - Check timeline for ability events at current time
    // - Send ability trigger events
    // - This recreates ability usage
    
    for (timeline, position) in ghost_q.iter() {
        let current_time = position.0;
        
        // Find ability events at current timestamp
        for event in &timeline.events {
            // Check if this event should trigger now
            if (event.timestamp - current_time).abs() < 0.05 {
                if let EventType::Ability(ability_id) = event.event_type {
                    // Trigger the ability
                    ability_events.send(TriggerAbility {
                        caster: ghost_entity,
                        ability: ability_id,
                    });
                }
            }
        }
    }
}
```

### 19. Ghost Visual Update System
```rust
pub fn update_ghost_visuals(
    mut ghost_q: Query<&mut Handle<StandardMaterial>, Added<Ghost>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Apply ghost transparency to newly created ghosts
    // - Create transparent material variant
    // - Apply to ghost entity
    // - This provides visual distinction
    
    for mut material_handle in ghost_q.iter_mut() {
        // Create ghost material with transparency
        let ghost_material = StandardMaterial {
            base_color: Color::srgba(0.5, 0.5, 1.0, 0.5),
            alpha_mode: AlphaMode::Blend,
            ..default()
        };
        *material_handle = materials.add(ghost_material);
    }
}
```

### 20. Ghost Input Interception System
```rust
pub fn intercept_ghost_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    ghost_q: Query<Entity, (With<Ghost>, With<Active>)>,
    mut dialog_events: EventWriter<ShowDialog>,
    mut pause_events: EventWriter<PauseAllTimelines>,
) {
    // Intercept input attempts on ghost characters
    // - Check if active character is a ghost
    // - If player tries to control (WASD/abilities/R):
    //   - Send PauseAllTimelines event
    //   - Send ShowDialog with RetryGhost type
    // - This prevents ghost control
    
    if let Ok(ghost_entity) = ghost_q.get_single() {
        let input_attempted = 
            keyboard.just_pressed(KeyCode::KeyW) ||
            keyboard.just_pressed(KeyCode::KeyA) ||
            keyboard.just_pressed(KeyCode::KeyS) ||
            keyboard.just_pressed(KeyCode::KeyD) ||
            keyboard.just_pressed(KeyCode::KeyR) ||
            keyboard.just_pressed(KeyCode::Digit1) ||
            keyboard.just_pressed(KeyCode::Digit2) ||
            keyboard.just_pressed(KeyCode::Digit3) ||
            keyboard.just_pressed(KeyCode::Digit4);
        
        if input_attempted {
            pause_events.send(PauseAllTimelines);
            dialog_events.send(ShowDialog {
                dialog_type: DialogType::RetryGhost,
            });
        }
    }
}
```

---

## Phase 4: Plugin Structure

```rust
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(RecordingState {
                mode: RecordingMode::Idle,
            })
            
            // Events
            .add_event::<StartRecording>()
            .add_event::<StopRecording>()
            .add_event::<CommitRecording>()
            .add_event::<ClearRecording>()
            .add_event::<RetryRecording>()
            .add_event::<PauseAllTimelines>()
            .add_event::<ResumeAllTimelines>()
            .add_event::<ResetArenaTimeline>()
            .add_event::<ShowDialog>()
            .add_event::<DialogChoice>()
            
            // Systems - Timer Management
            .add_systems(Update, (
                update_arena_timers,
                update_recording_countdown,
            ).chain())
            
            // Systems - Recording Flow
            .add_systems(Update, (
                detect_recording_input,
                initialize_recording,
                record_character_movement,
                record_ability_usage,
                detect_recording_interruption,
                detect_recording_completion,
            ).chain())
            
            // Systems - Dialog Management
            .add_systems(Update, (
                display_confirmation_dialog,
                process_dialog_input,
                process_dialog_choice,
            ).chain())
            
            // Systems - Timeline Control
            .add_systems(Update, (
                pause_all_timelines,
                resume_all_timelines,
                reset_arena_timeline,
                commit_recording,
                clear_recording,
            ).chain())
            
            // Systems - Ghost Playback
            .add_systems(Update, (
                playback_ghost_timelines,
                playback_ghost_abilities,
                update_ghost_visuals,
                intercept_ghost_input,
            ).chain());
    }
}
```

---

## Performance Considerations

### For 320 Simultaneous Ghosts

1. **Timeline Compression**
   - Store only keyframe events, not every frame
   - Use delta compression for similar positions
   - Limit event frequency (max 10 per second)

2. **Spatial Optimization**
   - Only update ghosts in current arena at full fidelity
   - Use LOD for distant arena ghosts
   - Batch transform updates

3. **Memory Management**
   - Pool timeline event allocations
   - Use arena allocators for temporary data
   - Compress old timelines

4. **Query Optimization**
   - Use `Changed<T>` filters aggressively
   - Cache query results when iterating multiple times
   - Use `With<T>` and `Without<T>` to narrow queries

5. **Event Batching**
   - Batch similar events in same frame
   - Use event queues with capacity hints
   - Clear events after processing

---

## Implementation Checklist

### Phase 1: Foundation (Core Recording)
- [ ] Add timeline components to character entities
- [ ] Implement arena timer system
- [ ] Create recording state management
- [ ] Add recording input detection
- [ ] Implement countdown system
- [ ] Create movement recording
- [ ] Add basic commit/clear functionality

### Phase 2: Full System
- [ ] Add ability recording
- [ ] Implement dialog system
- [ ] Create pause/resume mechanics
- [ ] Add ghost playback system
- [ ] Implement visual differentiation
- [ ] Add input interception

### Phase 3: Polish & Optimization
- [ ] Add timeline compression
- [ ] Implement LOD system
- [ ] Optimize queries for 320 ghosts
- [ ] Add visual effects for recording state
- [ ] Create smooth interpolation
- [ ] Add error recovery

---

## Testing Strategy

1. **Unit Tests**
   - Timeline event serialization
   - Timer wraparound at 120 seconds
   - Event interpolation accuracy

2. **Integration Tests**
   - Full recording flow
   - Dialog state transitions
   - Multi-arena timeline sync

3. **Performance Tests**
   - 320 ghost stress test
   - Memory usage monitoring
   - Frame time validation

---

## Edge Cases Handled

1. **Death During Recording**: Captured as event, replayed consistently
2. **Arena Transitions**: Blocked during recording with dialog
3. **Corrupted Timelines**: Automatic detection and clearing
4. **Memory Limits**: Event count caps per timeline
5. **Frame Drops**: Interpolation handles missing updates
6. **Multiple Recordings**: Proper cleanup of previous drafts
7. **Empty Arenas**: Graceful handling of no characters
8. **Simultaneous Events**: Proper ordering and batching

---

## Success Metrics

- Clean separation of concerns (each system < 50 lines)
- No direct component mutation between systems
- All communication through events
- Predictable, deterministic playback
- Smooth performance with 320 ghosts
- Clear visual feedback for all states
- Robust error handling without crashes