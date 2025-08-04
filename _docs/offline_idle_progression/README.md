## ***9. Offline Idle Progression***

1. *Snapshot Approach*

    - When you exit the game, each arena's state is saved. On restart, the game calculates how many *full 2-minute
      cycles* occurred during your absence (using floor).
    - That number of cycles' worth of XP/loot is awarded if your heroes were actively raiding.


2. *Deaths in Offline*

    - Heroes can also die in these offline cycles. Deaths appear in a global chat log upon return.
    - If many offline events occur, the chat log uses a rolling capacity (FIFO) so it never overflows.


3. *Minimal Chat Spam*

    - Because it's just a "delta" calculation, you won't see a blow-by-blow account of each fight.
    - Instead, you see summarized results in the global chat or notifications.