# Instructions

Next we need to have a playback mode:

1. A user will hit `r` to start recording, the FIRST event will be the active characters transform inside the
   CurrentArena.
2. After a user hits `r` -> now recording, casts abilities and movements -> presses `f` and commits that draft to
   publish timeline.
2. For that arena, the a state called `PlaybackCountdown`  counts from 3 to 0.
3. During the `PlaybackCountdown` the FIRST event, ALL characters of that arena with a
   TimelineManager[Some(PublishTimeline)] in that arena's index, will be emitted to set the Character back to their
   starting transform e.g. where they started intital during their recording.
4. When the countdown reaches 0, the mode state transitions to `Playback`.
5. In playback mode, ALL characters of that arena with a TimelineManager[Some(PublishTimeline)] in that arena's index
   will play back the timeline's event in time to the arena's Clock.
6. In Playback mode, When a character's Movement Event gets read it will move them, when an ability event gets read it
   will cast it.
7. In playback mode, when the timer hits 120second the area will reset to 0, the `PlaybackCountdown` state will trigger
   again. and therefore the FIRST event, ALL characters of that arena with a TimelineManager[Some(PublishTimeline)] in
   that arena's index, will be emitted to set the Character back to their starting transform e.g. where they started
   intital during their recording.
8. Repeat.