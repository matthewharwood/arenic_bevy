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
        timeline.add_event(TimelineEvent {
            timestamp: TimeStamp::new(5.0),
            event_type: EventType::Movement(GridPos::new(1, 0)),
        });

        // Event 2: timestamp 2.0, Ability AUTO_SHOT with no target
        timeline.add_event(TimelineEvent {
            timestamp: TimeStamp::new(2.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        });
        // Event 3: timestamp 10.0, Death event
        timeline.add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Death,
        });

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
        draft_timeline.add_event(TimelineEvent {
            timestamp: TimeStamp::new(5.0),
            event_type: EventType::Death,
        });

        // Event 2: timestamp 2.0, Ability AUTO_SHOT with no target
        draft_timeline.add_event(TimelineEvent {
            timestamp: TimeStamp::new(2.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        });
        draft_timeline.add_event(TimelineEvent {
            timestamp: TimeStamp::new(3.0),
            event_type: EventType::Movement(GridPos::new(1, 0)),
        });
        let publish_timeline = PublishTimeline::from_draft(draft_timeline);
        let event_range: Vec<_> = publish_timeline
            .events_in_range(TimeStamp::new(1.0), TimeStamp::new(3.5))
            .collect();

        assert_eq!(event_range.len(), 2);
        assert_eq!(event_range[0].timestamp, TimeStamp::new(2.0));
        assert_eq!(event_range[1].timestamp, TimeStamp::new(3.0));
    }
    #[test]
    fn test_next_event_after_edge_cases() {
        let mut draft = DraftTimeline::new();

        // Add events at specific timestamps
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Movement(GridPos::new(0, 0)),
        });
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(20.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        });
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(30.0),
            event_type: EventType::Movement(GridPos::new(1, 0)),
        });

        let published = PublishTimeline::from_draft(draft);

        // Test: Find next event after a timestamp with no exact match
        let next = published.next_event_after(TimeStamp::new(15.0));
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(20.0));

        // Test: Find next event when timestamp matches exactly
        let next = published.next_event_after(TimeStamp::new(20.0));
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(30.0));

        // Test: No next event when at or past last event
        let next = published.next_event_after(TimeStamp::new(30.0));
        assert!(next.is_none());

        let next = published.next_event_after(TimeStamp::new(35.0));
        assert!(next.is_none());

        // Test: Find first event when timestamp is before all events
        let next = published.next_event_after(TimeStamp::new(5.0));
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(10.0));
    }
}
