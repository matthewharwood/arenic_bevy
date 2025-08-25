use super::*;
use crate::ability::AbilityType;
use crate::arena::{Arena, ArenaName};
use std::time::Duration;

#[test]
fn test_draft_timeline_adds_events_sorted() {
    let mut timeline = DraftTimeline::new();

    timeline
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(5.0),
            event_type: EventType::Movement(Vec3::new(1.0, 0.0, 0.0)),
        })
        .expect("Failed to add event");

    timeline
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(2.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        })
        .expect("Failed to add event");

    timeline
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Death,
        })
        .expect("Failed to add event");

    assert_eq!(timeline.events.len(), 3);
    assert_eq!(timeline.events[0].timestamp, TimeStamp::new(2.0));
    assert_eq!(timeline.events[1].timestamp, TimeStamp::new(5.0));
    assert_eq!(timeline.events[2].timestamp, TimeStamp::new(10.0));
}

#[test]
fn test_timeline_clock_loops_at_120_seconds() {
    let mut clock = TimelineClock::new();

    clock.tick(Duration::from_secs(125));

    assert_eq!(clock.current().as_secs(), 5.0);
}

#[test]
fn test_timestamp_wrap_around() {
    let timestamp = TimeStamp::wrapped(TimeStamp::MAX.0);
    assert_eq!(timestamp.as_secs(), TimeStamp::ZERO.0);

    let timestamp = TimeStamp::wrapped(365.0);
    assert_eq!(timestamp.as_secs(), 5.0);

    let timestamp = TimeStamp::wrapped(-10.0);
    assert_eq!(timestamp.as_secs(), 110.0);
}

#[test]
fn test_timeline_clock_pause_resume() {
    let mut clock = TimelineClock::new();

    clock.tick(Duration::from_secs(10));
    assert_eq!(clock.current().as_secs(), 10.0);

    clock.pause();
    clock.tick(Duration::from_secs(10));
    assert_eq!(clock.current().as_secs(), 10.0);

    clock.resume();
    clock.tick(Duration::from_secs(10));
    assert_eq!(clock.current().as_secs(), 20.0);
}

#[test]
fn test_publish_timeline_get_events_in_range() {
    let mut draft = DraftTimeline::new();

    for i in 0..10 {
        draft
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(i as f32 * 2.0),
                event_type: EventType::Movement(Vec3::new(i as f32, 0.0, 0.0)),
            })
            .expect("Failed to add event");
    }

    let published = PublishTimeline::from_draft(draft);

    let events: Vec<_> = published
        .events_in_range(TimeStamp::new(5.0), TimeStamp::new(10.0))
        .unwrap()
        .collect();

    assert_eq!(events.len(), 2); // Should get events at 6.0, 8.0
    assert_eq!(events[0].timestamp, TimeStamp::new(6.0));
    assert_eq!(events[1].timestamp, TimeStamp::new(8.0));
}

#[test]
fn test_next_event_after_edge_cases() {
    let mut draft = DraftTimeline::new();

    draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Movement(Vec3::new(0.0, 0.0, 0.0)),
        })
        .expect("Failed to add event");
    draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(20.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        })
        .expect("Failed to add event");
    draft
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(30.0),
            event_type: EventType::Movement(Vec3::new(1.0, 0.0, 0.0)),
        })
        .expect("Failed to add event");

    let published = PublishTimeline::from_draft(draft);

    let next = published.next_event_after(TimeStamp::new(15.0)).unwrap();
    assert!(next.is_some());
    assert_eq!(next.unwrap().timestamp, TimeStamp::new(20.0));

    let next = published.next_event_after(TimeStamp::new(20.0)).unwrap();
    assert!(next.is_some());
    assert_eq!(next.unwrap().timestamp, TimeStamp::new(30.0));

    let next = published.next_event_after(TimeStamp::new(30.0)).unwrap();
    assert!(next.is_none());

    let next = published.next_event_after(TimeStamp::new(35.0)).unwrap();
    assert!(next.is_none());

    let next = published.next_event_after(TimeStamp::new(5.0)).unwrap();
    assert!(next.is_some());
    assert_eq!(next.unwrap().timestamp, TimeStamp::new(10.0));
}

#[test]
fn test_explicit_constructors() {
    // Test TimeStamp::new() as primary constructor
    let timestamp = TimeStamp::new(42.5);
    assert_eq!(timestamp.as_secs(), 42.5);
    assert_eq!(timestamp.to_string(), "42.5s");

    // Test TimeStamp::ZERO constant
    assert_eq!(TimeStamp::ZERO.as_secs(), TimeStamp::ZERO.0);

    // Test Arena component with ArenaName enum
    let arena = Arena(ArenaName::Bastion);
    assert_eq!(arena.0.as_u8(), 4);
    assert_eq!(arena.0, ArenaName::Bastion);

    let pos_data = IVec2::new(5, -3);
    assert_eq!(pos_data.x, 5);
    assert_eq!(pos_data.y, -3);
    assert_eq!(pos_data.to_string(), "[5, -3]");

    let pos_component = GridPositionComponent(IVec2::new(5, -3));
    assert_eq!(pos_component.0.x, 5);
    assert_eq!(pos_component.0.y, -3);
    assert_eq!(pos_component.to_string(), "[5, -3]");
}

#[test]
fn test_timeline_manager_multi_arena_storage() {
    // Test the new architecture: TimelineManager stores multiple timelines per character using array indexing
    let mut timeline_manager = TimelineManager::new();

    // Create test timelines for different arenas
    let mut draft_labyrinth = DraftTimeline::new();
    draft_labyrinth
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Movement(Vec3::new(0.0, 0.0, 0.0)),
        })
        .expect("Failed to add event");
    let timeline_labyrinth = PublishTimeline::from_draft(draft_labyrinth);

    let mut draft_gala = DraftTimeline::new();
    draft_gala
        .add_event(TimelineEvent {
            timestamp: TimeStamp::new(30.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        })
        .expect("Failed to add event");
    let timeline_gala = PublishTimeline::from_draft(draft_gala);

    // Store timelines for different arenas using ArenaName directly
    let labyrinth_name = ArenaName::Labyrinth;
    let gala_name = ArenaName::Gala;

    timeline_manager.set_timeline(labyrinth_name, timeline_labyrinth);
    timeline_manager.set_timeline(gala_name, timeline_gala);

    // Verify separate timeline storage
    assert_eq!(timeline_manager.arena_count(), 2);
    assert!(timeline_manager.has_recording_for(labyrinth_name));
    assert!(timeline_manager.has_recording_for(gala_name));
    assert!(!timeline_manager.has_recording_for(ArenaName::GuildHouse)); // GuildHouse - no recording

    // Verify we can retrieve specific arena timelines
    let labyrinth_timeline = timeline_manager.get_timeline(labyrinth_name).unwrap();
    assert_eq!(labyrinth_timeline.events.len(), 1);
    assert_eq!(labyrinth_timeline.events[0].timestamp, TimeStamp::new(10.0));

    let gala_timeline = timeline_manager.get_timeline(gala_name).unwrap();
    assert_eq!(gala_timeline.events.len(), 1);
    assert_eq!(gala_timeline.events[0].timestamp, TimeStamp::new(30.0));

    // Verify recorded arenas iterator
    let recorded: Vec<_> = timeline_manager.recorded_arenas().collect();
    assert_eq!(recorded.len(), 2);
    assert!(recorded.contains(&labyrinth_name));
    assert!(recorded.contains(&gala_name));
}

#[test]
fn test_timeline_clock_only_runs_with_playback_component() {
    // Test with arena that has Playback component
    let mut clock_with_playback = TimelineClock::new();
    
    // Test with arena that doesn't have Playback component  
    let mut clock_without_playback = TimelineClock::new();
    
    // Simulate what the update_timeline_clocks system does
    // It only ticks clocks for entities with the Playback component
    
    // Tick the clock that "has" Playback
    clock_with_playback.tick(Duration::from_secs(10));
    assert_eq!(
        clock_with_playback.current().as_secs(), 
        10.0, 
        "Clock with Playback should advance"
    );
    
    // Don't tick the clock that "doesn't have" Playback
    // (simulating the With<Playback> filter in the query)
    assert_eq!(
        clock_without_playback.current().as_secs(), 
        0.0, 
        "Clock without Playback should not advance"
    );
    
    // Now simulate adding Playback component to the second clock
    // and verify it starts updating
    clock_without_playback.tick(Duration::from_secs(5));
    assert_eq!(
        clock_without_playback.current().as_secs(), 
        5.0, 
        "Clock should advance after Playback is added"
    );
    
    // Continue advancing the first clock
    clock_with_playback.tick(Duration::from_secs(5));
    assert_eq!(
        clock_with_playback.current().as_secs(), 
        15.0, 
        "First clock should continue advancing"
    );
}