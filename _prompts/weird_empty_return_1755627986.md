# weird_empty_return

**Usecase:** weird empty return types Vec<_>

**Created:** 1755627986

---

```rust
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
```

why do i need `        let event_range: Vec<_> = publish_timeline
            .events_in_range(TimeStamp::new(1.0), TimeStamp::new(3.5))
            .collect();`

Is that the best way to do it? Like being explicit is annoying but prob better? Help me understand the 5Ws of this and
other ways to do it. And if there is soemthing wrong with the underlying api. 

