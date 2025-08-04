## ***2. Arenas & Grid Fundamentals***

1. *Arena Setup*

    - *9 distinct arenas* with different bosses, themes, and mechanics.
    - Each arena is a *66×31* grid (size may be adjusted during development).
    - Timer per arena: *2 minutes*, fully independent of other arenas.
    - Multiple arenas can run simultaneously in real time; players can also zoom out of an arena to navigate between
      them.
    - Each Arena has 1 unique boss which belongs to that particular arena e.g. Hunter belongs to the arena the Labyrinth.


2. *Movement & Collision*

    - *Grid-Based Movement*:
        - Use *WASD* or *Arrow Keys* to move the selected character up/down/left/right on the arena's grid.
        - Each move advances one grid tile at a time.
        - Movement includes a *small cooldown* or *slight animation* so it feels deliberate rather than twitch-based.
    - *Collision*:
        - Multiple characters can *occupy the same grid cell* simultaneously. Overlapped "ghosts" might show a visual
          "multiply blend."
    - *Visual & UX*:
        - Subtle highlight on the currently selected tile and/or selected character.



4. *Arena Navigation & Viewing* (Design Document §§5, 17)

    - *`[` / `]`*: Paginate through arenas.
    - `P`: Toggles the `arena_camera` from `scale: 1` to `scale: 3` and back. At `scale 3` the Player can see all 9 arenas in a 3x3 layout. At `scale: 1` the `selected` arena is centered in view.
    - While there is a global timer in bevy, each arena's 2min timers are independant of another. Meaning that a timer in the Hunter's `Labyrinth` could be at 90s of the 120s time while the Bard's `Gala` timer could be at 30s of the 120s. All these arena timers on game load start in sync; however, as the Player starts recordings of a character in a particular arena, the arena's timer will be restarted from `0` where other arenas will be continuing to play through.
    - Once an Arena timer has hit it's 2min timer it will loop. E.g. timer reset to 0 for that arena, and all characters with recording will start playing back again.