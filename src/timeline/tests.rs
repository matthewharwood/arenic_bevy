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
}
