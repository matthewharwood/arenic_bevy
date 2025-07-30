@ -0,0 +1,1846 @@

# Building a Sophisticated Movement, Recording, and Playback System in Bevy ECS

*A comprehensive tutorial for intermediate Rust developers new to Bevy ECS*

**Estimated Reading Time:** 35-40 minutes  
**Prerequisites:** Intermediate Rust (ownership, traits, generics), basic async concepts  
**Learning Level:** Intermediate → Advanced ECS Patterns

---

## Why This Tutorial Matters

You've mastered Rust's ownership system and built solid applications, but Entity-Component-System architecture feels
alien. Traditional object-oriented patterns don't translate cleanly, and Bevy's documentation assumes ECS fluency you
don't yet have.

This tutorial bridges that gap by teaching production-grade ECS patterns through a sophisticated movement and recording
system that manages **360 characters across 9 arenas** using only **10.8MB of memory**. You'll learn architectural
patterns that scale to real games, not toy examples.

## The Central Mental Model: ECS as a Database

Before diving into code, establish this foundational analogy that will anchor every concept:

**Bevy's ECS is like a specialized database:**

- **Components** = Table columns (Position, Velocity, Health)
- **Entities** = Row IDs (invisible primary keys)
- **Systems** = Stored procedures (queries that process data)
- **Resources** = Global configuration tables
- **Events** = Message queues between systems

This isn't just a metaphor—it's how ECS actually works. Systems query component data efficiently, just like SQL queries.
This mental model will make every pattern we explore feel natural rather than foreign.

**🧠 Active Recall Checkpoint:** Before continuing, explain to yourself why ECS separates data (Components) from
behavior (Systems). How does this differ from traditional OOP classes?

---

## Chapter 1: Foundation Components and Basic Queries

*Reading Time: 7 minutes*

### Understanding Component Design

Components in ECS are pure data containers—no methods, just state. Here's how our character system structures data:

```rust
// Core identity component
#[derive(Component, Debug, Clone)]
pub struct Character {
    pub name: String,
}

// Location tracking - which arena (0-8) is this character in?
#[derive(Component, Debug, Clone)]
pub struct CurrentArena(pub u8);

// Selection state - marker component pattern
#[derive(Component, Debug, Clone)]
pub struct CharacterSelected;
```

**Key Design Decision:** `CharacterSelected` is a *marker component*—it has no data, only presence. This is more
efficient than storing `selected: bool` in the Character component because:

1. Queries for selected characters are faster (sparse iteration)
2. Automatic cleanup when entity is despawned
3. No risk of forgetting to update boolean flags

### Basic System Patterns

Systems are functions that query component data and apply logic:

```rust
// Query for characters in a specific arena
fn characters_in_arena_system(
    query: Query<&Character, With<CurrentArena>>,
    arena_query: Query<&CurrentArena>,
) {
    for character in &query {
        println!("Character: {}", character.name);
    }
}
```

**Database Analogy:** This is like `SELECT name FROM characters WHERE arena_id IS NOT NULL`.

### **Hands-On Exercise 1:** Basic Component Query

Create a new Bevy project and implement this basic character system:

```rust
use bevy::prelude::*;

#[derive(Component, Debug)]
struct Character {
    name: String,
    health: f32,
}

#[derive(Component)]
struct Selected;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_characters)
        .add_systems(Update, display_selected_characters)
        .run();
}

fn setup_characters(mut commands: Commands) {
    // Create 3 characters, select the first one
    commands.spawn((
        Character { name: "Alice".to_string(), health: 100.0 },
        Selected,
    ));

    commands.spawn(Character { name: "Bob".to_string(), health: 85.0 });
    commands.spawn(Character { name: "Charlie".to_string(), health: 92.0 });
}

fn display_selected_characters(
    query: Query<&Character, With<Selected>>,
) {
    for character in &query {
        println!("Selected: {} ({}HP)", character.name, character.health);
    }
}
```

**Verification Step:** Run this code. You should see "Selected: Alice (100HP)" printed continuously. The query only
processes entities that have BOTH components.

**🧠 What's the Output?** Before running, predict: How many characters will be displayed? Why doesn't it show Bob or
Charlie?

---

## Chapter 2: Event-Driven Architecture Benefits

*Reading Time: 8 minutes*

### The Problem with Direct Function Calls

Traditional game architectures tightly couple input handling to game logic:

```rust
// Tightly coupled - hard to extend
fn handle_input(mut characters: Query<&mut Transform, With<Selected>>) {
    if keyboard.pressed(KeyCode::W) {
        for mut transform in &mut characters {
            transform.translation.y += 1.0; // Direct modification
        }
    }
}
```

This creates problems:

1. **Testing Difficulty:** Can't simulate movement without keyboard input
2. **Replay Systems:** No way to "record" movements
3. **AI Integration:** AI can't generate the same movements
4. **Network Sync:** Can't reproduce movements on other clients

### Event-Driven Solution

Events decouple "what happened" from "how to respond":

```rust
#[derive(Event, Debug, Clone)]
pub struct CharacterMoveEvent {
    pub entity: Entity,
    pub direction: MovementDirection,
}

#[derive(Debug, Clone, Copy)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

// System 1: Input → Events (one source of truth)
fn handle_movement_input(
    selected_query: Query<Entity, With<CharacterSelected>>,
    input: Res<ButtonInput<KeyCode>>,
    mut move_events: EventWriter<CharacterMoveEvent>,
) {
    if let Ok(entity) = selected_query.get_single() {
        if input.just_pressed(KeyCode::KeyW) {
            move_events.send(CharacterMoveEvent {
                entity,
                direction: MovementDirection::Up,
            });
        }
        // ... other directions
    }
}

// System 2: Events → World Changes (unified processing)
fn process_movement_events(
    mut move_events: EventReader<CharacterMoveEvent>,
    mut character_query: Query<&mut Transform, With<Character>>,
) {
    for event in move_events.read() {
        if let Ok(mut transform) = character_query.get_mut(event.entity) {
            match event.direction {
                MovementDirection::Up => transform.translation.y += 1.0,
                MovementDirection::Down => transform.translation.y -= 1.0,
                MovementDirection::Left => transform.translation.x -= 1.0,
                MovementDirection::Right => transform.translation.x += 1.0,
            }
        }
    }
}
```

**Architectural Benefit:** Now ANY system can generate `CharacterMoveEvent`—user input, AI, replay systems, network
messages. They're all processed identically by `process_movement_events`.

### **Hands-On Exercise 2:** Event-Driven Movement

Extend your previous example with event-driven movement:

```rust
use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
struct MoveEvent {
    entity: Entity,
    direction: Vec3,
}

#[derive(Component, Debug)]
struct Character {
    name: String,
}

#[derive(Component)]
struct Selected;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<MoveEvent>()  // Register the event type
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            process_movement_events,
        ).chain()) // Ensure input runs before processing
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Character { name: "Player".to_string() },
        Selected,
        Transform::default(),
    ));
}

fn handle_input(
    query: Query<Entity, With<Selected>>,
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<MoveEvent>,
) {
    if let Ok(entity) = query.get_single() {
        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::KeyW) { direction.y += 1.0; }
        if input.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
        if input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if input.pressed(KeyCode::KeyD) { direction.x += 1.0; }

        if direction != Vec3::ZERO {
            events.send(MoveEvent { entity, direction });
        }
    }
}

fn process_movement_events(
    mut events: EventReader<MoveEvent>,
    mut query: Query<&mut Transform>,
) {
    for event in events.read() {
        if let Ok(mut transform) = query.get_mut(event.entity) {
            transform.translation += event.direction * 0.1;
            println!("Moved entity {:?} by {:?}", event.entity, event.direction);
        }
    }
}
```

**Verification Step:** Run this code and use WASD keys to move. You should see movement logged to console. The character
moves because events are processed every frame.

**🧠 Active Recall:** Why do we use `.chain()` for the systems? What would happen if `process_movement_events` ran before
`handle_input`?

**Answer:** Events are sent and read within the same frame. Without `.chain()`, system ordering is undefined,
potentially causing one-frame delays in movement.

---

## Chapter 3: Complex State Management with Sessions

*Reading Time: 10 minutes*

### The Draft/Saved Session Pattern

One of the most sophisticated patterns in this system is the **draft/saved session architecture** for recording
management. This pattern prevents data loss while enabling experimentation:

```rust
#[derive(Component, Debug, Clone)]
pub struct RecordedActions {
    /// Single draft recording (working copy) - cleared on Tab or arena transitions
    pub draft_recording: Option<RecordingSession>,
    /// Saved recording sessions for revert capability (one per arena)
    pub saved_sessions: [Option<RecordingSession>; 9],
}

#[derive(Debug, Clone)]
pub struct RecordingSession {
    pub actions: Vec<ActionEvent>,
    pub start_time: f64,
    pub end_time: Option<f64>,
    pub arena: u8,
}

#[derive(Debug, Clone)]
pub enum ActionEvent {
    Position { x: f32, y: f32, timestamp: f64 },
    Move { direction: MovementDirection, timestamp: f64 },
}
```

### Understanding the Pattern

**Draft Recording:**

- Only one active draft per character at any time
- Automatically cleared when character is deselected (Tab key)
- Can be saved to persistent storage or discarded
- Think of it as an "unsaved document" in a text editor

**Saved Sessions:**

- One saved session per arena per character (9 total)
- Provides "undo" functionality—revert to last saved state
- Persists across character selection changes
- Think of it as "saved files" you can reload

### Memory Analysis

Let's understand the memory implications:

```rust
impl RecordedActions {
    // Memory usage calculation for capacity planning
    pub fn estimate_memory_usage() -> usize {
        // Each ActionEvent is approximately 24 bytes (worst case)
        // 2-minute recording at 60fps = 7200 actions maximum
        let max_actions_per_session = 7200;
        let bytes_per_action = 24;
        let bytes_per_session = max_actions_per_session * bytes_per_action; // ~172KB

        // Per character: 1 draft + 9 saved sessions
        let max_sessions_per_character = 10;
        let bytes_per_character = bytes_per_session * max_sessions_per_character; // ~1.7MB

        // System supports 360 characters (40 per arena × 9 arenas)
        let max_characters = 360;
        max_characters * bytes_per_character // ~612MB theoretical maximum
    }
}
```

**Design Trade-off:** This seems like a lot of memory, but in practice:

1. Most characters won't have recordings in all arenas
2. Recording sessions are much shorter than 2 minutes typically
3. Actions are sparse (only stored when movement occurs)
4. Real usage is closer to 10-20MB for typical gameplay

### Session Management Methods

```rust
impl RecordedActions {
    pub fn start_recording(&mut self, arena_index: u8, start_time: f64) -> bool {
        if self.draft_recording.is_some() {
            return false; // Already recording
        }

        self.draft_recording = Some(RecordingSession {
            actions: Vec::new(),
            start_time,
            end_time: None,
            arena: arena_index,
        });
        true
    }

    pub fn record_action(&mut self, action: ActionEvent) -> bool {
        if let Some(ref mut session) = self.draft_recording {
            session.actions.push(action);
            true
        } else {
            false // Not recording
        }
    }

    pub fn save_current_session(&mut self, arena_index: u8) -> bool {
        if let Some(session) = self.draft_recording.take() {
            self.saved_sessions[arena_index as usize] = Some(session);
            true
        } else {
            false // No draft to save
        }
    }

    pub fn clear_draft(&mut self) {
        self.draft_recording = None;
    }

    pub fn revert_to_saved(&mut self, arena_index: u8) -> bool {
        if let Some(saved) = &self.saved_sessions[arena_index as usize] {
            self.draft_recording = Some(saved.clone());
            true
        } else {
            false // No saved session to revert to
        }
    }
}
```

### **Hands-On Exercise 3:** Basic Recording System

Implement a simplified version of the recording pattern:

```rust
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Event, Debug, Clone)]
struct MoveEvent {
    entity: Entity,
    direction: Vec2,
    timestamp: f64,
}

#[derive(Component, Debug)]
struct Character {
    name: String,
}

#[derive(Component)]
struct Selected;

#[derive(Component, Debug, Default)]
struct SimpleRecorder {
    draft_actions: VecDeque<MoveEvent>,
    saved_actions: Option<Vec<MoveEvent>>,
    is_recording: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<MoveEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            record_movements,
            display_recording_status,
        ).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Character { name: "Player".to_string() },
        Selected,
        Transform::default(),
        SimpleRecorder::default(),
    ));
}

fn handle_input(
    query: Query<Entity, With<Selected>>,
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<MoveEvent>,
    time: Res<Time>,
    mut recorder_query: Query<&mut SimpleRecorder, With<Selected>>,
) {
    if let Ok(entity) = query.get_single() {
        // Movement input
        let mut direction = Vec2::ZERO;
        if input.pressed(KeyCode::KeyW) { direction.y += 1.0; }
        if input.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
        if input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if input.pressed(KeyCode::KeyD) { direction.x += 1.0; }

        if direction != Vec2::ZERO {
            events.send(MoveEvent {
                entity,
                direction,
                timestamp: time.elapsed_seconds_f64(),
            });
        }

        // Recording controls
        if let Ok(mut recorder) = recorder_query.get_single_mut() {
            if input.just_pressed(KeyCode::KeyR) {
                recorder.is_recording = !recorder.is_recording;
                if recorder.is_recording {
                    recorder.draft_actions.clear();
                    println!("Started recording");
                } else {
                    println!("Stopped recording ({} actions)", recorder.draft_actions.len());
                }
            }

            if input.just_pressed(KeyCode::KeyP) {
                // Save current draft
                recorder.saved_actions = Some(recorder.draft_actions.iter().cloned().collect());
                recorder.draft_actions.clear();
                println!("Saved recording!");
            }
        }
    }
}

fn record_movements(
    mut events: EventReader<MoveEvent>,
    mut recorder_query: Query<&mut SimpleRecorder>,
) {
    for event in events.read() {
        if let Ok(mut recorder) = recorder_query.get_mut(event.entity) {
            if recorder.is_recording {
                recorder.draft_actions.push_back(event.clone());

                // Keep only last 100 actions to prevent memory issues
                if recorder.draft_actions.len() > 100 {
                    recorder.draft_actions.pop_front();
                }
            }
        }
    }
}

fn display_recording_status(
    recorder_query: Query<&SimpleRecorder, With<Selected>>,
) {
    if let Ok(recorder) = recorder_query.get_single() {
        if recorder.is_recording {
            println!("Recording: {} actions", recorder.draft_actions.len());
        }
    }
}
```

**Verification Steps:**

1. Run the code and move with WASD
2. Press 'R' to start recording—you should see action counts
3. Press 'R' again to stop recording
4. Press 'P' to save the recording
5. Start recording again—the draft should be empty

**🧠 Active Recall:** Explain why we use `VecDeque` for draft actions but `Vec` for saved actions. What's the performance
difference?

**Answer:** `VecDeque` allows efficient removal from the front (for bounded recording), while `Vec` is more
memory-efficient for saved sessions that won't be modified.

---

## Chapter 4: Multi-Arena Timer Coordination

*Reading Time: 9 minutes*

### The Challenge of Coordinated State

Managing multiple game areas simultaneously requires careful coordination. Each arena needs:

1. Independent timer state (paused, recording, playback)
2. Synchronized character behavior within each arena
3. Isolation between arenas (events in Arena 1 don't affect Arena 2)

### Arena Status State Machine

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaStatus {
    Paused,    // Default - timer paused, no activity
    Recording, // Timer active, selected character recording  
    Playback,  // Timer active, all characters replaying
}

#[derive(Component, Debug, Clone)]
pub struct ArenaTimer {
    pub timer: Timer,
    pub arena: ArenaName,
    pub status: ArenaStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaName {
    Arena0,
    Arena1,
    Arena2,
    Arena3,
    Arena4,
    Arena5,
    Arena6,
    Arena7,
    Arena8,
}
```

### State Transition Logic

The arena timer manages its own state and automatically controls the underlying timer:

```rust
impl ArenaTimer {
    pub fn new(arena: ArenaName) -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            arena,
            status: ArenaStatus::Paused, // Start paused
        }
    }

    pub fn set_status(&mut self, status: ArenaStatus) {
        let previous_status = self.status;
        self.status = status;

        match status {
            ArenaStatus::Paused => {
                self.timer.pause();
            }
            ArenaStatus::Recording | ArenaStatus::Playback => {
                if self.timer.paused() {
                    self.timer.unpause();
                }
                // Reset timer when transitioning to playback for synchronization
                if status == ArenaStatus::Playback && previous_status != ArenaStatus::Playback {
                    self.timer.reset();
                }
            }
        }
    }

    pub fn elapsed_seconds(&self) -> f64 {
        self.timer.elapsed_seconds_f64()
    }
}
```

**Key Design Decision:** Automatically controlling the underlying timer prevents desynchronization between arena status
and timer state. You can't accidentally have a "Recording" arena with a paused timer.

### Arena-Specific System Processing

Systems need to query and process only entities belonging to specific arenas:

```rust
// Process recording only for the current arena
fn update_recording_timers(
    mut timer_query: Query<&mut ArenaTimer>,
    mut character_query: Query<(&mut CharacterTimer, &CurrentArena), With<CharacterSelected>>,
    time: Res<Time>,
) {
    // Find the arena that's currently recording
    let recording_arena = timer_query
        .iter()
        .find(|timer| timer.status == ArenaStatus::Recording)
        .map(|timer| timer.arena);

    if let Some(arena) = recording_arena {
        // Update only characters in the recording arena
        for (mut char_timer, current_arena) in &mut character_query {
            if ArenaName::from_index(current_arena.0) == arena && char_timer.is_recording {
                char_timer.timer.tick(time.delta());
            }
        }
    }
}
```

### **Hands-On Exercise 4:** Multi-Arena Coordination

Build a simplified multi-arena system:

```rust
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArenaStatus {
    Paused,
    Active,
}

#[derive(Component, Debug)]
struct ArenaTimer {
    id: u8,
    timer: Timer,
    status: ArenaStatus,
}

#[derive(Component, Debug)]
struct Character {
    name: String,
    arena_id: u8,
}

#[derive(Resource)]
struct CurrentArena(u8);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CurrentArena(0))
        .add_systems(Startup, setup_arenas)
        .add_systems(Update, (
            handle_arena_switching,
            update_active_arena_timer,
            display_arena_status,
        ))
        .run();
}

fn setup_arenas(mut commands: Commands) {
    // Create 3 arenas
    for i in 0..3 {
        commands.spawn(ArenaTimer {
            id: i,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            status: if i == 0 { ArenaStatus::Active } else { ArenaStatus::Paused },
        });

        // Add some characters to each arena
        commands.spawn(Character {
            name: format!("Character{}", i * 2),
            arena_id: i,
        });
        commands.spawn(Character {
            name: format!("Character{}", i * 2 + 1),
            arena_id: i,
        });
    }
}

fn handle_arena_switching(
    input: Res<ButtonInput<KeyCode>>,
    mut current_arena: ResMut<CurrentArena>,
    mut timer_query: Query<&mut ArenaTimer>,
) {
    let mut new_arena = None;

    if input.just_pressed(KeyCode::Digit1) { new_arena = Some(0); }
    if input.just_pressed(KeyCode::Digit2) { new_arena = Some(1); }
    if input.just_pressed(KeyCode::Digit3) { new_arena = Some(2); }

    if let Some(arena_id) = new_arena {
        current_arena.0 = arena_id;

        // Update arena statuses
        for mut timer in &mut timer_query {
            timer.status = if timer.id == arena_id {
                ArenaStatus::Active
            } else {
                ArenaStatus::Paused
            };
        }

        println!("Switched to Arena {}", arena_id);
    }
}

fn update_active_arena_timer(
    mut timer_query: Query<&mut ArenaTimer>,
    time: Res<Time>,
) {
    for mut arena_timer in &mut timer_query {
        if arena_timer.status == ArenaStatus::Active {
            arena_timer.timer.tick(time.delta());

            if arena_timer.timer.just_finished() {
                println!("Arena {} timer tick: {:.1}s",
                         arena_timer.id,
                         arena_timer.timer.elapsed_seconds());
            }
        }
    }
}

fn display_arena_status(
    current_arena: Res<CurrentArena>,
    timer_query: Query<&ArenaTimer>,
    character_query: Query<&Character>,
) {
    if current_arena.is_changed() {
        println!("=== Arena {} Status ===", current_arena.0);

        // Show arena timer status
        for timer in &timer_query {
            if timer.id == current_arena.0 {
                println!("Timer: {:.1}s ({:?})",
                         timer.timer.elapsed_seconds(),
                         timer.status);
            }
        }

        // Show characters in current arena
        let characters: Vec<_> = character_query
            .iter()
            .filter(|c| c.arena_id == current_arena.0)
            .collect();

        println!("Characters: {}",
                 characters.iter()
                     .map(|c| c.name.as_str())
                     .collect::<Vec<_>>()
                     .join(", "));
    }
}
```

**Verification Steps:**

1. Run the code—you should see Arena 0 is active initially
2. Press keys 1, 2, 3 to switch arenas
3. Only the active arena's timer should tick
4. Each arena shows its own characters

**🧠 Active Recall:** Why do we pause inactive arena timers instead of just ignoring them? What problems would arise if
all timers ran simultaneously?

**Answer:** Resource efficiency and state consistency. Running unnecessary timers wastes CPU, and you couldn't have
arena-specific recording/playback timing if all timers advanced together.

---

## Chapter 5: Time-Synchronized Recording and Playback

*Reading Time: 8 minutes*

### The Synchronization Challenge

For accurate playback, recorded actions must be synchronized with arena timers. Consider these timing challenges:

1. **Recording Drift:** If actions are recorded with system time but played back with arena time, they'll be out of sync
2. **Variable Frame Rates:** Recording at 30fps and playing at 60fps changes timing
3. **Pause/Resume:** Pausing during recording should pause the action timeline

### Timestamp-Based Recording

The solution is to use arena-relative timestamps:

```rust
#[derive(Debug, Clone)]
pub enum ActionEvent {
    Position { x: f32, y: f32, timestamp: f64 },
    Move { direction: MovementDirection, timestamp: f64 },
}

// Recording system - uses arena timer, not system time
fn record_movement_events(
    mut move_events: EventReader<CharacterMoveEvent>,
    mut character_query: Query<(&mut RecordedActions, &CurrentArena), With<CharacterSelected>>,
    arena_timer_query: Query<&ArenaTimer>,
) {
    for event in move_events.read() {
        if let Ok((mut recorded, current_arena)) = character_query.get_mut(event.entity) {
            // Find the arena timer for accurate timestamps
            let arena_time = arena_timer_query
                .iter()
                .find(|timer| timer.arena == ArenaName::from_index(current_arena.0))
                .map(|timer| timer.elapsed_seconds())
                .unwrap_or(0.0);

            let action = ActionEvent::Move {
                direction: event.direction,
                timestamp: arena_time, // Arena-relative time
            };

            recorded.record_action(action);
        }
    }
}
```

### Playback System Architecture

Playback reads recorded actions and emits the same events that recording captured:

```rust
#[derive(Component, Debug, Clone)]
pub struct PlaybackState {
    pub recording: RecordingSession,
    pub current_action_index: usize,
}

fn process_playback_actions(
    mut playback_query: Query<(Entity, &mut PlaybackState, &CurrentArena)>,
    arena_timer_query: Query<&ArenaTimer>,
    mut move_events: EventWriter<CharacterMoveEvent>,
) {
    for (entity, mut playback, current_arena) in &mut playback_query {
        // Get current arena time
        let arena = ArenaName::from_index(current_arena.0);
        let current_time = arena_timer_query
            .iter()
            .find(|timer| timer.arena == arena && timer.status == ArenaStatus::Playback)
            .map(|timer| timer.elapsed_seconds())
            .unwrap_or(0.0);

        // Process all actions that should have happened by now
        while playback.current_action_index < playback.recording.actions.len() {
            let action = &playback.recording.actions[playback.current_action_index];

            match action {
                ActionEvent::Move { direction, timestamp } => {
                    if *timestamp <= current_time {
                        // Time for this action - emit the same event recording captured
                        move_events.send(CharacterMoveEvent {
                            entity,
                            direction: *direction,
                        });
                        playback.current_action_index += 1;
                    } else {
                        break; // Future actions wait for their time
                    }
                }
                ActionEvent::Position { x, y, timestamp } => {
                    if *timestamp <= current_time {
                        // Handle position-based actions...
                        playback.current_action_index += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}
```

**Key Insight:** Playback generates the exact same `CharacterMoveEvent` objects that recording captured. The movement
processing system can't tell the difference between user input and playback—they're processed identically.

### Playback Control Systems

Starting and stopping playback requires careful state management:

```rust
fn start_playback_for_arena(
    arena_timer_query: Query<&ArenaTimer>,
    mut character_query: Query<(Entity, &mut RecordedActions, &CurrentArena), Without<PlaybackState>>,
    mut commands: Commands,
) {
    // Find arena entering playback mode
    let playback_arena = arena_timer_query
        .iter()
        .find(|timer| timer.status == ArenaStatus::Playback)
        .map(|timer| timer.arena);

    if let Some(arena) = playback_arena {
        let arena_index = arena.to_index();

        // Start playback for all characters with saved sessions in this arena
        for (entity, recorded_actions, current_arena) in &mut character_query {
            if current_arena.0 == arena_index {
                if let Some(session) = &recorded_actions.saved_sessions[arena_index as usize] {
                    commands.entity(entity).insert(PlaybackState {
                        recording: session.clone(),
                        current_action_index: 0,
                    });
                }
            }
        }
    }
}

fn stop_playback_when_complete(
    mut playback_query: Query<(Entity, &PlaybackState)>,
    mut commands: Commands,
) {
    for (entity, playback) in &mut playback_query {
        if playback.current_action_index >= playback.recording.actions.len() {
            // Playback finished - remove component
            commands.entity(entity).remove::<PlaybackState>();
        }
    }
}
```

### **Hands-On Exercise 5:** Basic Playback System

Create a simple recording and playback system:

```rust
use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
struct MoveEvent {
    entity: Entity,
    direction: Vec2,
    timestamp: f64,
}

#[derive(Component, Debug)]
struct Character {
    name: String
}

#[derive(Component)]
struct Selected;

#[derive(Component, Debug, Default)]
struct Recorder {
    actions: Vec<MoveEvent>,
    is_recording: bool,
}

#[derive(Component, Debug)]
struct Playback {
    actions: Vec<MoveEvent>,
    current_index: usize,
    start_time: f64,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<MoveEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            record_events,
            process_movement,
            playback_events,
        ).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Character { name: "Player".to_string() },
        Selected,
        Transform::default(),
        Recorder::default(),
    ));
}

fn handle_input(
    query: Query<Entity, With<Selected>>,
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<MoveEvent>,
    time: Res<Time>,
    mut recorder_query: Query<&mut Recorder, With<Selected>>,
    mut commands: Commands,
) {
    if let Ok(entity) = query.get_single() {
        let current_time = time.elapsed_seconds_f64();

        // Movement
        let mut direction = Vec2::ZERO;
        if input.pressed(KeyCode::KeyW) { direction.y += 1.0; }
        if input.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
        if input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if input.pressed(KeyCode::KeyD) { direction.x += 1.0; }

        if direction != Vec2::ZERO {
            events.send(MoveEvent {
                entity,
                direction,
                timestamp: current_time,
            });
        }

        // Recording controls
        if let Ok(mut recorder) = recorder_query.get_single_mut() {
            if input.just_pressed(KeyCode::KeyR) {
                recorder.is_recording = !recorder.is_recording;
                if recorder.is_recording {
                    recorder.actions.clear();
                    println!("Started recording");
                } else {
                    println!("Stopped recording");
                }
            }

            if input.just_pressed(KeyCode::KeyP) && !recorder.actions.is_empty() {
                // Start playback
                commands.entity(entity).insert(Playback {
                    actions: recorder.actions.clone(),
                    current_index: 0,
                    start_time: current_time,
                });
                println!("Started playback of {} actions", recorder.actions.len());
            }
        }
    }
}

fn record_events(
    mut events: EventReader<MoveEvent>,
    mut recorder_query: Query<&mut Recorder>,
) {
    for event in events.read() {
        if let Ok(mut recorder) = recorder_query.get_mut(event.entity) {
            if recorder.is_recording {
                recorder.actions.push(event.clone());
            }
        }
    }
}

fn playback_events(
    mut playback_query: Query<(Entity, &mut Playback)>,
    mut events: EventWriter<MoveEvent>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let current_time = time.elapsed_seconds_f64();

    for (entity, mut playback) in &mut playback_query {
        let playback_time = current_time - playback.start_time;

        // Process actions whose time has come
        while playback.current_index < playback.actions.len() {
            let action = &playback.actions[playback.current_index];
            let relative_time = action.timestamp - playback.actions[0].timestamp;

            if relative_time <= playback_time {
                events.send(MoveEvent {
                    entity,
                    direction: action.direction,
                    timestamp: current_time,
                });
                playback.current_index += 1;
            } else {
                break;
            }
        }

        // Remove playback when complete
        if playback.current_index >= playback.actions.len() {
            commands.entity(entity).remove::<Playback>();
            println!("Playback complete");
        }
    }
}

fn process_movement(
    mut events: EventReader<MoveEvent>,
    mut query: Query<&mut Transform>,
) {
    for event in events.read() {
        if let Ok(mut transform) = query.get_mut(event.entity) {
            transform.translation += (event.direction * 0.1).extend(0.0);
        }
    }
}
```

**Verification Steps:**

1. Run the code and move with WASD
2. Press 'R' to start recording, move around, press 'R' to stop
3. Press 'P' to start playback—the character should replay your movements
4. The timing should match your original movements

**🧠 Active Recall:** Why do we calculate `relative_time` in the playback system? What would happen if we used absolute
timestamps?

**Answer:** Absolute timestamps would only work if playback started at the exact same time as recording. Relative
timestamps let us replay the sequence starting from any point in time.

---

## Chapter 6: Memory Optimization and Bounded Storage

*Reading Time: 6 minutes*

### Understanding Memory Constraints

Real-time recording systems must balance functionality with memory usage. Our system handles:

- **360 characters** (40 per arena × 9 arenas)
- **Up to 10 recording sessions per character** (1 draft + 9 saved)
- **2-minute maximum recording duration** per session

### Memory Usage Analysis

Let's calculate the theoretical maximum memory usage:

```rust
pub struct MemoryAnalysis;

impl MemoryAnalysis {
    pub fn calculate_system_capacity() -> MemoryReport {
        // ActionEvent size analysis
        let position_event_size = std::mem::size_of::<f32>() * 2 + std::mem::size_of::<f64>(); // 16 bytes
        let move_event_size = std::mem::size_of::<u8>() + std::mem::size_of::<f64>(); // 9 bytes
        let avg_event_size = (position_event_size + move_event_size) / 2; // ~12.5 bytes

        // Recording session overhead
        let session_overhead = std::mem::size_of::<RecordingSession>(); // ~48 bytes

        // Maximum actions per session (2 minutes at 30 actions/second)
        let max_actions_per_session = 2 * 60 * 30; // 3600 actions
        let session_data_size = max_actions_per_session * avg_event_size; // ~45KB
        let total_session_size = session_data_size + session_overhead; // ~45KB

        // Per character maximum (1 draft + 9 saved sessions)
        let max_sessions_per_character = 10;
        let character_max_memory = max_sessions_per_character * total_session_size; // ~450KB

        // System total (360 characters)
        let max_characters = 360;
        let system_max_memory = max_characters * character_max_memory; // ~162MB

        MemoryReport {
            avg_event_size,
            max_actions_per_session,
            character_max_memory,
            system_max_memory,
            practical_usage_estimate: system_max_memory / 10, // ~16MB realistic
        }
    }
}

#[derive(Debug)]
pub struct MemoryReport {
    pub avg_event_size: usize,
    pub max_actions_per_session: usize,
    pub character_max_memory: usize,
    pub system_max_memory: usize,
    pub practical_usage_estimate: usize,
}
```

### Bounded Storage Implementation

To prevent unbounded memory growth, implement automatic limits:

```rust
impl RecordedActions {
    const MAX_ACTIONS_PER_SESSION: usize = 3600; // 2 minutes at 30 actions/sec
    const MAX_SESSION_DURATION: f64 = 120.0; // 2 minutes in seconds

    pub fn record_action(&mut self, action: ActionEvent) -> bool {
        if let Some(ref mut session) = self.draft_recording {
            // Check duration limit
            if let Some(end_time) = session.end_time {
                if end_time - session.start_time >= Self::MAX_SESSION_DURATION {
                    return false; // Session too long
                }
            }

            // Check action count limit
            if session.actions.len() >= Self::MAX_ACTIONS_PER_SESSION {
                return false; // Too many actions
            }

            session.actions.push(action);
            true
        } else {
            false
        }
    }

    pub fn estimate_memory_usage(&self) -> usize {
        let mut total = 0;

        if let Some(ref draft) = self.draft_recording {
            total += draft.actions.len() * 16; // Estimate 16 bytes per action
        }

        for session in &self.saved_sessions {
            if let Some(ref session) = session {
                total += session.actions.len() * 16;
            }
        }

        total
    }
}
```

### Memory Monitoring System

Add runtime monitoring to track actual usage:

```rust
#[derive(Resource, Default)]
pub struct MemoryMonitor {
    pub total_characters: usize,
    pub active_recordings: usize,
    pub estimated_memory_usage: usize,
}

fn update_memory_monitor(
    character_query: Query<&RecordedActions>,
    mut monitor: ResMut<MemoryMonitor>,
) {
    let mut total_memory = 0;
    let mut active_recordings = 0;
    let character_count = character_query.iter().count();

    for recorded_actions in &character_query {
        total_memory += recorded_actions.estimate_memory_usage();

        if recorded_actions.draft_recording.is_some() {
            active_recordings += 1;
        }
    }

    monitor.total_characters = character_count;
    monitor.active_recordings = active_recordings;
    monitor.estimated_memory_usage = total_memory;

    // Log if memory usage is high
    if total_memory > 50_000_000 { // 50MB warning threshold
        warn!("High memory usage: {}MB across {} characters", 
              total_memory / 1_000_000, character_count);
    }
}
```

### **Quick Exercise:** Memory Estimation

Calculate the memory usage for a more realistic scenario:

```rust
// Realistic usage scenario:
// - 50 active characters (not 360)
// - Average 30-second recordings (not 2 minutes)  
// - 3 saved sessions per character (not 9)
// - 5 actions per second (not 30)

fn calculate_realistic_memory() -> usize {
    let characters = 50;
    let avg_recording_duration = 30.0; // seconds
    let actions_per_second = 5;
    let saved_sessions_per_character = 3;
    let bytes_per_action = 16;

    let actions_per_session = (avg_recording_duration * actions_per_second as f64) as usize;
    let session_memory = actions_per_session * bytes_per_action;
    let character_memory = session_memory * (saved_sessions_per_character + 1); // +1 for draft
    let total_memory = characters * character_memory;

    println!("Realistic memory usage: {}KB", total_memory / 1024);
    total_memory
}
```

**🧠 Active Recall:** Why do we limit both recording duration AND action count? What scenarios would hit each limit?

**Answer:** Duration limits prevent very long recordings, while action count limits prevent rapid-fire movement from
consuming memory. A player holding down a movement key could hit the action limit before the duration limit.

---

## Chapter 7: Plugin Architecture and System Coordination

*Reading Time: 7 minutes*

### Plugin Architecture Principles

Well-designed Bevy plugins follow the single responsibility principle while coordinating through well-defined
interfaces:

```rust
// Each plugin owns a specific domain
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CharacterMoveEvent>()
            .add_systems(Update, (
                handle_movement_input,
                process_movement_events,
            ).chain());
    }
}

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            // Playback runs BEFORE input to inject events first
            handle_playback_toggle,
            start_playback_for_arena,
            process_playback_actions,
        ).chain().before(MovementPlugin::handle_movement_input))
            .add_systems(Update, (
                // Recording runs AFTER movement to capture events
                record_movement_events,
                update_character_timers,
            ).chain().after(MovementPlugin::process_movement_events));
    }
}
```

### Critical System Ordering

The order systems run determines correctness. Our system requires:

1. **Playback systems** → Generate events from recordings
2. **Input systems** → Generate events from user input
3. **Movement systems** → Process all events uniformly
4. **Recording systems** → Capture events for future playback

```rust
app.add_systems(Update, (
// Phase 1: Event Generation
(
process_playback_actions,
handle_movement_input,
).chain(),
// Phase 2: Event Processing  
process_movement_events,
// Phase 3: Event Recording
record_movement_events,
).chain());
```

**Why This Order Matters:**

- If recording ran before movement processing, it would miss events
- If input ran before playback, playback events might be processed later
- If movement processing ran multiple times, events would be duplicated

### Inter-Plugin Communication

Plugins communicate through shared components and events, not direct dependencies:

```rust
// Shared data structures (in a common module)
#[derive(Event, Debug, Clone)]
pub struct CharacterMoveEvent {
    pub entity: Entity,
    pub direction: MovementDirection,
}

#[derive(Component, Debug, Clone)]
pub struct CurrentArena(pub u8);

// MovementPlugin produces and consumes CharacterMoveEvent
// RecordingPlugin consumes CharacterMoveEvent to record it
// RecordingPlugin produces CharacterMoveEvent during playback
// No plugin directly calls another plugin's functions
```

### Plugin Configuration Pattern

Use configuration structs for flexible plugin setup:

```rust
pub struct RecordingPluginConfig {
    pub max_recording_duration: f64,
    pub max_actions_per_session: usize,
    pub memory_warning_threshold: usize,
}

impl Default for RecordingPluginConfig {
    fn default() -> Self {
        Self {
            max_recording_duration: 120.0, // 2 minutes
            max_actions_per_session: 3600,
            memory_warning_threshold: 50_000_000, // 50MB
        }
    }
}

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        let config = app.world()
            .get_resource::<RecordingPluginConfig>()
            .cloned()
            .unwrap_or_default();

        app.insert_resource(config)
            .add_systems(/* ... systems using config ... */);
    }
}
```

### Error Handling and Debugging

Production plugins need robust error handling:

```rust
fn record_movement_events(
    mut move_events: EventReader<CharacterMoveEvent>,
    mut character_query: Query<(&mut RecordedActions, &CurrentArena), With<CharacterSelected>>,
    arena_timer_query: Query<&ArenaTimer>,
) {
    for event in move_events.read() {
        match character_query.get_mut(event.entity) {
            Ok((mut recorded, current_arena)) => {
                // Find arena timer for accurate timestamps
                let arena_time = arena_timer_query
                    .iter()
                    .find(|timer| timer.arena == ArenaName::from_index(current_arena.0))
                    .map(|timer| timer.elapsed_seconds())
                    .unwrap_or_else(|| {
                        warn!("No arena timer found for arena {}", current_arena.0);
                        0.0
                    });

                let action = ActionEvent::Move {
                    direction: event.direction,
                    timestamp: arena_time,
                };

                if !recorded.record_action(action) {
                    debug!("Failed to record action for entity {:?} - session may be full", event.entity);
                }
            }
            Err(_) => {
                // Entity doesn't have required components - this is normal
                // Not every entity that moves needs to record actions
            }
        }
    }
}
```

### **Final Exercise:** Complete Plugin Integration

Build a complete plugin that demonstrates all patterns:

```rust
use bevy::prelude::*;

// Shared types
#[derive(Event, Debug, Clone)]
pub struct MoveEvent {
    pub entity: Entity,
    pub direction: Vec2,
}

#[derive(Component, Debug)]
pub struct Character {
    pub name: String
}

#[derive(Component)]
pub struct Selected;

// Movement Plugin
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveEvent>()
            .add_systems(Update, (
                handle_input,
                process_movement,
            ).chain());
    }
}

fn handle_input(
    query: Query<Entity, With<Selected>>,
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<MoveEvent>,
) {
    if let Ok(entity) = query.get_single() {
        let mut direction = Vec2::ZERO;
        if input.pressed(KeyCode::KeyW) { direction.y += 1.0; }
        if input.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
        if input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if input.pressed(KeyCode::KeyD) { direction.x += 1.0; }

        if direction != Vec2::ZERO {
            events.send(MoveEvent { entity, direction });
        }
    }
}

fn process_movement(
    mut events: EventReader<MoveEvent>,
    mut query: Query<&mut Transform>,
) {
    for event in events.read() {
        if let Ok(mut transform) = query.get_mut(event.entity) {
            transform.translation += (event.direction * 0.1).extend(0.0);
        }
    }
}

// Debug Plugin
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, log_movement_events.after(process_movement));
    }
}

fn log_movement_events(
    mut events: EventReader<MoveEvent>,
) {
    for event in events.read() {
        println!("Movement: {:?} moved by {:?}", event.entity, event.direction);
    }
}

// Main app
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((MovementPlugin, DebugPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Character { name: "Test Player".to_string() },
        Selected,
        Transform::default(),
    ));
}
```

**Verification:** Run this code and verify:

1. WASD keys move the character
2. Movement events are logged to console
3. The debug plugin runs after movement processing
4. Plugins are completely independent but coordinate through events

**🧠 Final Active Recall:** Explain why plugin order matters in the main app, but system order is defined within each
plugin. What would happen if we reversed the plugin order?

**Answer:** Plugin order determines when each plugin's systems are registered, but the actual system execution order is
determined by the system ordering constraints (`.before()`, `.after()`, `.chain()`). Reversing plugin order wouldn't
change system execution order because the constraints are explicit.

---

## Chapter 8: Production Considerations and Scaling

*Reading Time: 5 minutes*

### Performance Optimization Strategies

Real production systems need optimization beyond basic functionality:

```rust
// Use change detection to avoid unnecessary work
fn update_recording_status(
    mut character_query: Query<(&mut RecordedActions, &CharacterTimer), Changed<CharacterTimer>>,
) {
    // Only process characters whose timers changed
    for (mut recorded, timer) in &mut character_query {
        if timer.is_recording && recorded.draft_recording.is_none() {
            // Start recording if timer indicates recording but no draft exists
            recorded.start_recording(0, timer.timer.elapsed_seconds_f64());
        }
    }
}

// Batch operations for efficiency
fn cleanup_completed_playbacks(
    playback_query: Query<(Entity, &PlaybackState)>,
    mut commands: Commands,
) {
    let completed_entities: Vec<Entity> = playback_query
        .iter()
        .filter_map(|(entity, playback)| {
            if playback.current_action_index >= playback.recording.actions.len() {
                Some(entity)
            } else {
                None
            }
        })
        .collect();

    // Batch remove components
    for entity in completed_entities {
        commands.entity(entity).remove::<PlaybackState>();
    }
}
```

### Memory Pool Pattern

For high-frequency allocations, consider memory pools:

```rust
#[derive(Resource)]
pub struct ActionEventPool {
    pool: Vec<Vec<ActionEvent>>,
}

impl ActionEventPool {
    pub fn rent(&mut self) -> Vec<ActionEvent> {
        self.pool.pop().unwrap_or_else(Vec::new)
    }

    pub fn return_vec(&mut self, mut vec: Vec<ActionEvent>) {
        vec.clear();
        if vec.capacity() > 1000 {
            vec.shrink_to(1000); // Prevent unbounded growth
        }
        self.pool.push(vec);
    }
}
```

### Error Recovery Patterns

Robust systems handle corrupt or invalid data gracefully:

```rust
impl RecordedActions {
    pub fn validate_and_repair(&mut self) -> bool {
        let mut repaired = false;

        // Validate draft recording
        if let Some(ref mut draft) = self.draft_recording {
            let original_len = draft.actions.len();
            draft.actions.retain(|action| match action {
                ActionEvent::Move { timestamp, .. } => *timestamp >= 0.0,
                ActionEvent::Position { x, y, timestamp } => {
                    timestamp >= &0.0 && x.is_finite() && y.is_finite()
                }
            });

            if draft.actions.len() != original_len {
                warn!("Removed {} invalid actions from draft recording", 
                      original_len - draft.actions.len());
                repaired = true;
            }
        }

        // Validate saved sessions
        for (index, session) in self.saved_sessions.iter_mut().enumerate() {
            if let Some(ref mut session) = session {
                let original_len = session.actions.len();
                session.actions.retain(|action| {
                    // Same validation logic
                    true // Simplified for example
                });

                if session.actions.len() != original_len {
                    warn!("Repaired saved session {} for arena {}", 
                          original_len - session.actions.len(), index);
                    repaired = true;
                }
            }
        }

        repaired
    }
}
```

### Scalability Considerations

Design decisions that affect system scalability:

1. **Component Sparsity:** Using marker components (`CharacterSelected`) is more efficient than boolean flags when most
   entities don't have the property

2. **Query Efficiency:** Specific queries (`Query<&Transform, (With<Character>, With<Selected>)>`) are faster than broad
   queries with filtering

3. **Event Cleanup:** Bevy automatically clears events each frame, preventing memory leaks

4. **System Parallelization:** Systems without conflicting queries can run in parallel:

```rust
app.add_systems(Update, (
// These can run in parallel - no shared mutable access
(
update_arena_timers,
process_ui_input,
update_camera_position,
),
// This must run after the parallel systems
process_movement_events,
).chain());
```

### Production Checklist

✅ **Memory Management**

- Bounded storage prevents unbounded growth
- Memory monitoring catches leaks early
- Resource pooling reduces allocation overhead

✅ **Error Handling**

- Invalid data doesn't crash the system
- Logging provides debugging information
- Graceful degradation when components are missing

✅ **Performance**

- Change detection avoids unnecessary processing
- Efficient queries minimize iteration overhead
- Batch operations reduce system call overhead

✅ **Maintainability**

- Plugin architecture enables feature isolation
- Clear system ordering prevents race conditions
- Configuration resources allow runtime customization

---

## Conclusion: From Patterns to Production

You've learned sophisticated ECS patterns that handle real production challenges:

### **🧠 Knowledge Check - Test Your Understanding**

1. **Component Design:** Why use marker components instead of boolean flags?
2. **Event Architecture:** How does event-driven design enable replay systems?
3. **State Management:** What problems does the draft/saved session pattern solve?
4. **Memory Optimization:** How do bounded storage limits prevent memory leaks?
5. **Plugin Architecture:** Why is system ordering critical for correctness?

### **Key Mental Models to Remember**

- **ECS as Database:** Components are columns, entities are rows, systems are queries
- **Events as Message Queues:** Decouple producers from consumers for flexibility
- **Draft/Saved Pattern:** Separate working state from persistent state
- **Arena Isolation:** Independent timers enable multi-context coordination
- **Plugin Boundaries:** Clear interfaces prevent tight coupling

### **Learning Artifacts for Long-Term Retention**

**Reference Implementation Checklist:**

```rust
// Essential components for movement/recording system
struct Character {
    name: String
}
struct CurrentArena(u8);
struct CharacterSelected; // Marker component

// Event-driven movement
struct CharacterMoveEvent {
    entity: Entity,
    direction: MovementDirection
}

// Recording state management  
struct RecordedActions {
    draft_recording: Option<RecordingSession>,
    saved_sessions: [Option<RecordingSession>; 9],
}

// Time synchronization
struct ArenaTimer {
    timer: Timer,
    status: ArenaStatus
}

// Plugin architecture
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CharacterMoveEvent>()
            .add_systems(Update, (input_handler, movement_processor).chain());
    }
}
```

**Memory Usage Quick Reference:**

- 16 bytes average per ActionEvent
- 45KB per 2-minute recording session
- 450KB maximum per character (10 sessions)
- 162MB theoretical system maximum (360 characters)
- 16MB realistic usage estimate

**System Ordering Template:**

```rust
app.add_systems(Update, (
// 1. Generate events (playback, input)
(playback_systems, input_systems).chain(),
// 2. Process events (movement, actions)  
event_processing_systems,
// 3. Record events (capture for replay)
recording_systems,
).chain());
```

### **Next Steps: Advanced Patterns**

To continue your ECS mastery, explore:

1. **State Machines with Enums:** More complex state transitions using Rust enums
2. **Custom Queries:** Building reusable query types for complex filtering
3. **Asset Loading Integration:** Loading and saving recordings to disk
4. **Network Synchronization:** Replicating events across multiple clients
5. **Performance Profiling:** Using Bevy's built-in diagnostics for optimization

### **Community Resources**

- **Bevy Documentation:** https://bevy-cheatbook.github.io/
- **ECS Patterns Guide:** https://github.com/SanderMertens/ecs-faq
- **Performance Optimization:** Bevy's official performance guide
- **Advanced Examples:** Bevy's GitHub repository examples folder

You now have the architectural foundation to build sophisticated, scalable game systems using Bevy ECS. These patterns
will serve you well in production environments where performance, maintainability, and correctness are critical.

Remember: Great ECS code isn't just about making it work—it's about making it work efficiently, maintainably, and
correctly under all conditions. You're now equipped with the knowledge to achieve all three.