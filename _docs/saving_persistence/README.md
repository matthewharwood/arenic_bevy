## ***12. Saving & Persistence***

1. *Autosaves*

    - Trigger on key events (recording start/end, boss kills, new loot).
    - The game also auto-saves continuously in the background.


2. *No Partial Recording Data*

    - If the game closes mid-recording, that recording is discarded.
    - The hero reverts to idle/ghost status if relevant.


3. *Immutable Timelines*

    - Once a timeline is finalized, you cannot edit it.
    - Determinism ensures older ghost replays remain valid, unaffected by new ones.