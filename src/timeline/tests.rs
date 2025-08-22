#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::ability::AbilityType;
    // Import everything from the parent module

    #[test]
    fn test_draft_timeline_adds_events_sorted() {
        // TODO(human): Create a new DraftTimeline
        // Hint: DraftTimeline has a new() method
        let mut timeline = DraftTimeline::new();

        // TODO(human): Add 3 events OUT OF ORDER to test sorting
        // Event 1: timestamp 5.0, Movement to position (1, 0)
        timeline
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(5.0),
                event_type: EventType::Movement(IVec3::new(1, 0, 0)),
            })
            .expect("Failed to add movement event");

        // Event 2: timestamp 2.0, Ability AUTO_SHOT with no target
        timeline
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(2.0),
                event_type: EventType::Ability(AbilityType::AutoShot, None),
            })
            .expect("Failed to add ability event");

        // Event 3: timestamp 10.0, Death event
        timeline
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(10.0),
                event_type: EventType::Death,
            })
            .expect("Failed to add death event");

        // TODO(human): Check that the events were added correctly
        // 1. timeline.events.len() should equal 3
        // 2. Events should be sorted by timestamp:
        assert_eq!(timeline.events[0].timestamp, TimeStamp::new(2.0));
        assert_eq!(timeline.events[1].timestamp, TimeStamp::new(5.0));
        assert_eq!(timeline.events[2].timestamp, TimeStamp::new(10.0));
        assert_eq!(timeline.events.len(), 3);
    }
    #[test]
    fn test_timeline_clock_loops_at_120_seconds() {
        let mut clock = TimelineClock::default();
        clock.tick(Duration::from_secs(125));
        assert_eq!(clock.current().as_secs(), 5.0);
    }

    #[test]
    fn test_timestamp_wrap_around_edge_cases() {
        // Test exact boundary
        let timestamp = TimeStamp::wrapped(TimeStamp::MAX.0);
        assert_eq!(timestamp.as_secs(), TimeStamp::ZERO.0);

        // Test multiple wraps
        let timestamp = TimeStamp::wrapped(365.0); // 365 = 3*120 + 5
        assert_eq!(timestamp.as_secs(), 5.0);

        // Test negative wrapping
        let timestamp = TimeStamp::wrapped(-10.0);
        assert_eq!(timestamp.as_secs(), 110.0); // -10 + 120 = 110
    }
    #[test]
    fn test_timeline_clock_pause_resume() {
        let mut clock = TimelineClock::default();
        clock.tick(Duration::from_secs(10));
        assert_eq!(clock.current().as_secs(), 10.0);
        clock.pause();
        clock.tick(Duration::from_secs(15));
        assert_eq!(clock.current().as_secs(), 10.0);
        clock.resume();
        clock.tick(Duration::from_secs(20));
        assert_eq!(clock.current().as_secs(), 30.0);
    }
    #[test]
    fn test_publish_timeline_get_events_in_range() {
        let mut draft_timeline = DraftTimeline::new();
        draft_timeline
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(5.0),
                event_type: EventType::Death,
            })
            .expect("Failed to add death event");

        // Event 2: timestamp 2.0, Ability AUTO_SHOT with no target
        draft_timeline
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(2.0),
                event_type: EventType::Ability(AbilityType::AutoShot, None),
            })
            .expect("Failed to add ability event");

        draft_timeline
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(3.0),
                event_type: EventType::Movement(IVec3::new(1, 0, 0)),
            })
            .expect("Failed to add movement event");

        let publish_timeline = PublishTimeline::from_draft(draft_timeline)
            .expect("Failed to create timeline from draft");
        let event_range: Vec<_> = publish_timeline
            .events_in_range(TimeStamp::new(1.0), TimeStamp::new(3.5))
            .expect("Failed to get events in range")
            .collect();

        assert_eq!(event_range.len(), 2);
        assert_eq!(event_range[0].timestamp, TimeStamp::new(2.0));
        assert_eq!(event_range[1].timestamp, TimeStamp::new(3.0));
    }
    #[test]
    fn test_next_event_after_edge_cases() {
        let mut draft = DraftTimeline::new();

        // Add events at specific timestamps
        draft
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(10.0),
                event_type: EventType::Movement(IVec3::new(0, 0, 0)),
            })
            .expect("Failed to add movement event");

        draft
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(20.0),
                event_type: EventType::Ability(AbilityType::AutoShot, None),
            })
            .expect("Failed to add ability event");

        draft
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(30.0),
                event_type: EventType::Movement(IVec3::new(1, 0, 0)),
            })
            .expect("Failed to add movement event");

        let published =
            PublishTimeline::from_draft(draft).expect("Failed to create timeline from draft");

        // Test: Find the next event after a timestamp with no exact match
        let next = published
            .next_event_after(TimeStamp::new(15.0))
            .expect("Failed to get next event");
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(20.0));

        // Test: Find next event when timestamp matches exactly
        let next = published
            .next_event_after(TimeStamp::new(20.0))
            .expect("Failed to get next event");
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(30.0));

        // Test: No next event when at or past last event
        let next = published
            .next_event_after(TimeStamp::new(30.0))
            .expect("Failed to get next event");
        assert!(next.is_none());

        let next = published
            .next_event_after(TimeStamp::new(35.0))
            .expect("Failed to get next event");
        assert!(next.is_none());

        // Test: Find first event when timestamp is before all events
        let next = published
            .next_event_after(TimeStamp::new(5.0))
            .expect("Failed to get next event");
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(10.0));
    }

    #[test]
    fn test_empty_timeline_error() {
        let empty_draft = DraftTimeline::new();
        let result = PublishTimeline::from_draft(empty_draft);

        assert!(result.is_err());
        match result {
            Err(crate::timeline::TimelineError::EmptyTimeline) => {
                // Expected error case
            }
            Err(other) => panic!("Unexpected error: {:?}", other),
            Ok(_) => panic!("Expected error for empty timeline"),
        }
    }

    #[test]
    fn test_arena_validation() {
        use crate::arena::Arena;

        // Safe index always succeeds
        let safe = Arena(ArenaName::from_index_safe(15));
        assert_eq!(safe.0.as_u8(), 8); // Should clamp to max valid arena
    }

    #[test]
    fn test_error_context_and_display() {
        let empty_error = crate::timeline::TimelineError::EmptyTimeline;
        assert_eq!(empty_error.to_string(), "Timeline is empty");
    }

    #[test]
    fn test_character_timelines_multi_arena_storage() {
        // Test the critical architectural fix: TimelineManager stores multiple timelines per character
        let mut timeline_manager = TimelineManager::new();
        
        // Create test timelines for different arenas
        let mut draft_labyrinth = DraftTimeline::new();
        draft_labyrinth.add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Movement(IVec3::new(0, 0, 0)),
        }).expect("Failed to add event");
        let timeline_labyrinth = PublishTimeline::from_draft(draft_labyrinth).expect("Failed to create timeline");
        
        let mut draft_gala = DraftTimeline::new();
        draft_gala.add_event(TimelineEvent {
            timestamp: TimeStamp::new(30.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        }).expect("Failed to add event");
        let timeline_gala = PublishTimeline::from_draft(draft_gala).expect("Failed to create timeline");
        
        // Store timelines for different arenas
        let labyrinth_id = ArenaName::from_index_safe(0); // Labyrinth
        let gala_id = ArenaName::from_index_safe(8);      // Gala
        
        timeline_manager.set_timeline(labyrinth_id, timeline_labyrinth);
        timeline_manager.set_timeline(gala_id, timeline_gala);
        
        // Verify separate timeline storage
        assert_eq!(timeline_manager.arena_count(), 2);
        assert!(timeline_manager.has_recording_for(labyrinth_id));
        assert!(timeline_manager.has_recording_for(gala_id));
        assert!(!timeline_manager.has_recording_for(ArenaName::from_index_safe(1))); // GuildHouse - no recording
        
        // Verify we can retrieve specific arena timelines
        let labyrinth_timeline = timeline_manager.get_timeline(labyrinth_id).unwrap();
        assert_eq!(labyrinth_timeline.events.len(), 1);
        assert_eq!(labyrinth_timeline.events[0].timestamp, TimeStamp::new(10.0));
        
        let gala_timeline = timeline_manager.get_timeline(gala_id).unwrap();
        assert_eq!(gala_timeline.events.len(), 1);
        assert_eq!(gala_timeline.events[0].timestamp, TimeStamp::new(30.0));
        
        // Verify recorded arenas iterator
        let recorded: Vec<_> = timeline_manager.recorded_arenas().collect();
        assert_eq!(recorded.len(), 2);
        assert!(recorded.contains(&labyrinth_id));
        assert!(recorded.contains(&gala_id));
    }
}
