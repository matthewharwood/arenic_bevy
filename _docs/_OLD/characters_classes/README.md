## ***3. Characters & Classes***

1. *Roster & Classes*

    - You can manage up to *40 characters* per arena—so *320* total if all 8 arenas are fully staffed.
    - There are *8 classes* (Hunter, Alchemist, Forager, Thief, Tank, Cardinal, Merchant, Bard).
    - Classes have unique abilities; each hero can equip *up to 4* abilities at a time.
    - There will be an inventory of a total of 400 characters (360 or 40 per arena and an overflow of 40 for end game management).

2. *Primary & Additional Abilities*

    - Basic or "core" abilities are tied to each class, as documented in the separate *Arenic Class Abilities* file.
    - Some abilities involve multi-tap, tap-and-hold, or positional usage.
    - While cooldowns and resource costs exist, they do *not* appear on the timeline.
    - Cast times *do* appear on the timeline, as do movement inputs and the exact grid squares for AoE or targeted
      spells.


3. *Character Selection & Switching* (Design Document §2)

    - *Tab*: Cycle through available characters in the current arena.
    - *Shift+Tab*: Reverse the selection cycle.
    - (Optional) *Mouse Click* in the UI (e.g., on character portraits) to select heroes.
    - Design suggests showing the active character's portrait and health bar for clarity, especially if you have a large
      roster.


4. *Death & De-Leveling*

    - HP \= 0 → immediate death; that hero is removed from the raid and *loses 1 level*.
    - Hero must be removed from (or become inactive in) the current rotation.
    - To rejoin, the hero must either be revived during that same timeline (if recorded) or wait until the next cycle.
    - Death also triggers a level-down (one level lost).


5. *Idle Progression*

    - Even if not controlled, characters continue gathering XP or resources in the background.
    - Offscreen heroes can also die from hazards or boss AoEs. If they do, they lose 1 level.
    - Over time, the idle system simulates continuous raids, leveling, and resource collection.


6. *Guild House & Roster Management* (Design Document §§8, 9)

    - Access from the main menu (Enter/Return) or from a physical "Guild House" location.
    - Manage up to 320 heroes: recruit, dismiss, sort, equip gear, and allocate abilities.
        - Gacha-like recruiting is possible.
        - Death causes de-leveling.
    - You may kick heroes if you reach max capacity.
    - (Design Doc) "Optional 'Guild House' as a physical space" where you can walk around, but you can also open roster
      screens from anywhere outside an active recording.