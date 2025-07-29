# ***ARENIC: Comprehensive Rulebook (v6.0)***

## ***1\. Core Concept & Vision***

- *Solo 40-Man Raid Feel*  
  You simulate large-scale MMORPG-style raids—but as a *solo* player—via the *Record & Replay* system. Each arena can feature up to *40 characters*, and there are *8 arenas* total (320 characters max).  
    
- *Asynchronous 2-Minute Cycles*  
  Each arena has its own *2-minute timer* for recordings and replays. You can manage or pause them independently.  
    
- *Overall Gameplay Flow*  
  Over multiple “recording” sessions, you layer each hero’s actions as “ghosts” in an arena. Eventually, each arena can have a full 40-person raid playing out simultaneously. Movement, abilities, boss battles, and environment interactions all occur on a *grid* and are replayed exactly as recorded each cycle.

---

## ***2\. Arenas & Grid Fundamentals***

1. *Arena Setup*  
     
   - *8 distinct arenas* with different bosses, themes, and mechanics.  
   - Each arena is a *320×180* grid (size may be adjusted during development).  
   - Timer per arena: *2 minutes*, fully independent of other arenas.  
   - Multiple arenas can run simultaneously in real time; players can also zoom out of an arena to navigate between them.

   

2. *Movement & Collision*  
     
   - *Grid-Based Movement*:  
     - Use *WASD* or *Arrow Keys* to move the selected character up/down/left/right on the arena’s grid.  
     - Each move advances one grid tile at a time.  
     - Movement includes a *small cooldown* or *slight animation* so it feels deliberate rather than twitch-based.  
   - *Collision*:  
     - Multiple characters can *occupy the same grid cell* simultaneously. Overlapped “ghosts” might show a visual “multiply blend.”  
   - *Visual & UX*:  
     - Subtle highlight on the currently selected tile and/or selected character.

   

3. *Global Pause/Resume*  
     
   - Press *Enter/Return* to pause the entire game and open a menu (e.g., for roster management).  
   - Individual arenas can also be paused, leaving others running.

   

4. *Arena Navigation & Viewing* (Design Document §§5, 17\)  
     
   - *Q / E*: Paginate through arenas.  
   - *W*: Zoom out of an arena to view the overworld or zoom back in to confirm arena selection.  
   - Each arena persists in real time for both active (recording) and idle replays, including boss fights and environment interactions.  
   - *Design*:  
     - Show metadata such as arena name and how many active rotations (ghosts) are running.  
     - Indicator icons if a boss is defeated, or if the arena is in an “idle gathering” state, etc.

---

## ***3\. Characters & Classes***

1. *Roster & Classes*  
     
   - You can manage up to *40 characters* per arena—so *320* total if all 8 arenas are fully staffed.  
   - There are *9 classes* (Hunter, Alchemist, Sprinter, Gatherer, Thief, Tank, Cardinal, Merchant, Bard).  
   - Classes have unique abilities; each hero can equip *up to 4* abilities at a time.

   

2. *Primary & Additional Abilities*  
     
   - Basic or “core” abilities are tied to each class, as documented in the separate *Arenic Class Abilities* file.  
   - Some abilities involve multi-tap, tap-and-hold, or positional usage.  
   - While cooldowns and resource costs exist, they do *not* appear on the timeline.  
   - Cast times *do* appear on the timeline, as do movement inputs and the exact grid squares for AoE or targeted spells.

   

3. *Character Selection & Switching* (Design Document §2)  
     
   - *Tab*: Cycle through available characters in the current arena.  
   - *Shift+Tab*: Reverse the selection cycle.  
   - (Optional) *Mouse Click* in the UI (e.g., on character portraits) to select heroes.  
   - Design suggests showing the active character’s portrait and health bar for clarity, especially if you have a large roster.

   

4. *Death & De-Leveling*  
     
   - HP \= 0 → immediate death; that hero is removed from the raid and *loses 1 level*.  
   - Hero must be removed from (or become inactive in) the current rotation.  
   - To rejoin, the hero must either be revived during that same timeline (if recorded) or wait until the next cycle.  
   - Death also triggers a level-down (one level lost).

   

5. *Idle Progression*  
     
   - Even if not controlled, characters continue gathering XP or resources in the background.  
   - Offscreen heroes can also die from hazards or boss AoEs. If they do, they lose 1 level.  
   - Over time, the idle system simulates continuous raids, leveling, and resource collection.

   

6. *Guild House & Roster Management* (Design Document §§8, 9\)  
     
   - Access from the main menu (Enter/Return) or from a physical “Guild House” location.  
   - Manage up to 320 heroes: recruit, dismiss, sort, equip gear, and allocate abilities.  
     - Gacha-like recruiting is possible.  
     - Death causes de-leveling.  
   - You may kick heroes if you reach max capacity.  
   - (Design Doc) “Optional ‘Guild House’ as a physical space” where you can walk around, but you can also open roster screens from anywhere outside an active recording.

---

## ***4\. Record & Replay System***

1. *Recording Sessions*  
     
   - Press *R* to begin recording a 2-minute timeline for the selected character in the active arena.  
   - A *3-second* countdown may appear before the clock starts.  
   - Press *F* to finalize the recording, turning it into a “ghost.”

   

2. *Cancels & Partial Use*  
     
   - If you cancel before finalizing, new data is discarded, and the arena’s timer resets to *0*.  
   - If you finalize early (say at 1:00), the hero idles from 1:00 to 2:00 in that timeline.

   

3. *2-Minute Duration & Idle*  
     
   - The maximum recording length is *2 minutes*.  
   - If the player stops input after 1 minute but still finalizes, the timeline includes 1 minute of action \+ 1 minute of idle.

   

4. *Layered Timelines*  
     
   - Each completed recording is a “ghost” that replays every 2-minute cycle.  
   - You can layer up to *40* hero recordings in one arena, all in parallel.  
   - Over time, you build massive raids by synchronizing multiple ghosts.

   

5. *Death & Revival in the Timeline*  
     
   - If a hero dies at a certain timestamp in their recorded run, that death recurs in replays (unless you record a new run to alter that outcome).  
   - Revival spells must target a specific grid cell/time. If the hero’s corpse is there during that cast, they get revived.

   

6. *No Rewind*  
     
   - Once you finalize a recorded timeline, it’s immutable.  
   - For a different outcome, record a new session.

---

## ***5\. Determinism & Future RNG***

1. *Purely Deterministic (Current)*  
     
   - Boss attacks, damage, healing, and environment events are time- and grid-based with zero randomness.  
   - Perfect reproducibility ensures that ghost replays line up every cycle.

   

2. *Planned RNG (Post-Launch)*  
     
   - Eventually, random crits, random boss patterns, or random recruit rarity might be introduced after the deterministic core is stable.

   

3. *Simultaneous Actions*  
     
   - Multiple events in the exact same frame are processed in parallel.  
   - Resource or ability conflicts are resolved systematically (e.g., first-come-first-serve if needed).

---

## ***6\. Death, Revival & Travel***

1. *Death*  
     
   - HP=0 → immediate removal from that arena’s rotation \+ lose 1 level.  
   - Timelines store the exact death moment for replays.

   

2. *Revival*  
     
   - Revival spells target a grid cell and time. If a dead hero is at that cell/time, they come back to life.  
   - Otherwise, the attempt fails (a “revive miss”).

   

3. *Travel & Offscreen Combat*  
     
   - Heroes traveling between arenas (or from the Guild House) can die offscreen if attacked.  
   - Death offscreen also triggers a de-level and reverts them to the Guild House.  
   - No mini-instance or separate recording for travel; it’s background-simulated.

---

## ***7\. Boss Battles & Mechanics***

1. *2-Minute Boss Timeline*  
     
   - Bosses also operate on a *2-minute* cycle.  
   - They have a rotation of attacks (often 30s–2min in pattern length) that’s fully deterministic.

   

2. *Mechanics & Telegraphs* (Design Document §§11, 19\)  
     
   - Bosses and the arena can have shifting terrain, traps, or hazards telegraphed on the grid.  
   - Large telegraphs or “winds-ups” for big attacks so players can record dodges or counters.  
   - Environmental hazards (lava tiles, elemental storms, etc.) also appear with pre-warnings or hazard timers.

   

3. *No Boss Enrage*  
     
   - There is no forced “enrage timer” at 2:00. The boss simply resets each cycle.

   

4. *Pattern Recognition*  
     
   - Anticipating the boss’s repeated patterns is key to success.  
   - Audio/visual cues often precede major attacks.

---

## ***8\. Gacha-Like Recruitment & Loot***

1. *Loot & Currency* (Design Document §§12, 18\)  
     
   - Defeated enemies can drop loot; use *Space* to pick it up.  
   - Rarity is color-coded. Gear upgrades, ability enhancements, or crafting materials can be found.  
   - Currency can be spent at in-raid shops or an *in-raid auction house* (if implemented).

   

2. *Inventory & Equipment Management* (Design Document §13)  
     
   - Equip heroes with gear that boosts stats or modifies abilities.  
   - No strict limit on the total gear or consumables you can carry.  
   - *Loadouts* for quick gear swapping are recommended.  
   - The UI can highlight gear upgrades vs. downgrades for quick decisions.

   

3. *Gacha & Recruitment*  
     
   - Each arena can grant a “gacha roll” after a successful 2-minute cycle. The class offered typically matches that arena’s theme (e.g., a Thief from the Thief’s arena).  
   - No immediate “pity timer,” though the design might evolve to include incremental improvements over time.  
   - Recruits appear at the Guild House; you can accept or deny them.

   

4. *Gacha Buff Stacking*  
     
   - Some “global buff consumables” come from gacha. They stack or overwrite each other based on buff tags/levels.  
   - Buff timers run in parallel, displayed in a single global HUD, and affect *all* arenas simultaneously.

---

## ***9\. Offline Idle Progression***

1. *Snapshot Approach*  
     
   - When you exit the game, each arena’s state is saved. On restart, the game calculates how many *full 2-minute cycles* occurred during your absence (using floor).  
   - That number of cycles’ worth of XP/loot is awarded if your heroes were actively raiding.

   

2. *Deaths in Offline*  
     
   - Heroes can also die in these offline cycles. Deaths appear in a global chat log upon return.  
   - If many offline events occur, the chat log uses a rolling capacity (FIFO) so it never overflows.

   

3. *Minimal Chat Spam*  
     
   - Because it’s just a “delta” calculation, you won’t see a blow-by-blow account of each fight.  
   - Instead, you see summarized results in the global chat or notifications.

---

## ***10\. User Interface & Controls***

Below is a consolidated control scheme merging both the original ruleset and the Design Document specifics:

| *Key/Control* | *Action* |
| :---- | :---- |
| *WASD / Arrows* | Move the selected hero on the grid, one tile at a time |
| *1,2,3,4* | Activate the selected hero’s abilities (offensive, defensive, utility, healing, etc.) |
| *R* | Start/Stop *recording* the active hero’s 2-minute timeline |
| *F* | *Finalize* a recording, turning it into a ghost |
| *Q / E* | Paginate between the 8 arenas |
| *W* | Zoom out/in of the current arena (toggle to overworld view or confirm selection) |
| *Tab* | Cycle through available heroes in the current arena |
| *Shift+Tab* | Reverse-cycle through heroes |
| *Space* | Interact with the environment (open loot chests, pick up items, talk to NPCs, etc.) |
| *Enter / Return* | Pause/unpause \+ open main menu or Guild UI (if not currently recording) |
| Mouse (Optional) | Select heroes via UI or click on icons; possibly confirm gear/roster actions in menus |

Additional UI/UX Elements:

- *Visual Timelines*: A bar or timeline UI showing the 2-minute recording length.  
- *Health Bars*: Above each hero, along with a “low-health” indicator.  
- *Arena Overlays*: Hazard warnings, boss telegraphs, or buff tile indicators.  
- *Global Status Overlay*: Summarizes each arena’s progress, boss health, or “raid readiness.”  
- *Guild House Menus*: Manage equipment, recruit new heroes, check Gacha rolls, assign tasks, etc.

---

## ***11\. Progression, Rewards & Buffs***

1. *Leveling & De-Leveling* (Design Document §§9, 10\)  
     
   - Heroes gain levels through repeated success, both active and idle.  
   - Each death causes a hero to lose one level.  
   - Visual or audio cues indicate level-ups/downs.  
   - The hero’s historical level changes might be displayed in a “level history.”

   

2. *Permanent Upgrades*  
     
   - Boss kills in Normal/Heroic/Mythic can grant better gear or account-wide perks.  
   - These do not invalidate older replays because there is no boss scaling.

   

3. *Global Buff Timers*  
     
   - Buff stacking is allowed. If the *same* buff is applied at a higher tier, it overwrites the previous one.  
   - *Different* buffs can stack in parallel.  
   - Timers are displayed in a *single global HUD*.

---

## ***12\. Saving & Persistence***

1. *Autosaves*  
     
   - Trigger on key events (recording start/end, boss kills, new loot).  
   - The game also auto-saves continuously in the background.

   

2. *No Partial Recording Data*  
     
   - If the game closes mid-recording, that recording is discarded.  
   - The hero reverts to idle/ghost status if relevant.

   

3. *Immutable Timelines*  
     
   - Once a timeline is finalized, you cannot edit it.  
   - Determinism ensures older ghost replays remain valid, unaffected by new ones.

---

## ***13\. Developer Tools & Future Features***

1. *Dev Debug Workflow*  
     
   - A *Timeline Viewer* plus *console commands* to jump frames or load states is planned.  
   - Snapshots can be stored to quickly replay bug scenarios or test large-scale fights.

   

2. *In-Game Debug (Player-Facing)*  
     
   - A simplified “replay” or “highlight reel” might be offered to players as a *P1* (post-core) feature.

   

3. *Potential RNG & Co-Op*  
     
   - Small random elements (e.g., crits, random boss phases) may be introduced after launch.  
   - Co-Op or synchronous multiplayer remains outside initial scope.

   

4. *Removed/Optional Concepts*  
     
   - *Boss enrage timers* are removed from the final design.  
   - Advanced multi-tap edge cases, boss scaling, or other expansions remain optional.

---

## ***14\. The Echo Guild Commentary*** **(Design Document §16)**

- *Description*:  
  - Simulated chatter from your heroes or NPCs, reacting to events and achievements in real time.  
  - They celebrate wins or lament setbacks, providing extra flavor.  
- *Keyboard Controls*:  
  - None. This is all automatic.  
- *Design*:  
  - Could appear as chat bubbles or in a global chat log, with context-based remarks.

---

## ***15\. The Existential Narrative Integration*** **(Design Document §20)**

- *Description*:  
  - You play as the “Architect,” gradually uncovering existential or humorous storyline elements.  
  - Vignettes or codex entries unlock at certain milestones, such as boss defeats or difficulty jumps.  
- *Implementation*:  
  - Dialogue triggers can appear mid-raid (as text bubbles) or in the chat log.  
  - No direct input to advance the story aside from general gameplay progress.

---

## ***WHAT WAS MISSING / NEWLY CLARIFIED***

Below are items or details that were *not explicitly* stated in the original v5.0 ruleset but appear in the *Design Document* (now integrated in v6.0):

1. *Slight Movement Cooldown/Animation*  
     
   - Original ruleset mentioned grid-based movement but did not specify the *small cooldown* or *animation* to prevent twitchy movement.

   

2. *Explicit “Shift+Tab” for Reverse Character Selection*  
     
   - The original ruleset mentioned *Tab* for cycling but did not include “Shift+Tab” as a reverse option.

   

3. *Mouse Controls for Character Selection*  
     
   - The original ruleset’s official control table did not list *mouse* interactions for switching heroes. Now clarified as optional.

   

4. *Zoom Mechanic (W)*  
     
   - The original ruleset said you can “rotate between arenas” but did not mention explicitly *W* to zoom in/out between the arena and overworld.

   

5. *Environmental Hazards & Buff Tiles*  
     
   - The original ruleset had references to traps and hazards, but the Design Document provided more emphasis on timed hazards, buff tiles, or environmental objects. This has been made more explicit in v6.0.

   

6. *Idle & Automated Progression*  
     
   - The original ruleset included offline progression but did not emphasize an active “idle system” for resource gathering or tactical roles. The design doc clarifies that heroes can gather resources or keep raiding in real time, even when not player-controlled.

   

7. *Guild House as a Physical Space*  
     
   - The original ruleset mentioned a “Guild House” for management but did not highlight it as an optional physical location you could walk around. Now clarified.

   

8. *Healer Roles & Health Systems*  
     
   - The original ruleset was high-level about health. The design doc specifically mentions *Healer* roles, with healing abilities playing a big part in preventing de-leveling.

   

9. *Space Bar for Interactions*  
     
   - The original ruleset had “Space to interact,” but the design doc clarifies it can be used for opening chests, picking up loot, and confirming dialogues.

   

10. *Use of a Global Chat / Banter*  
      
- The design doc emphasizes “Echo Guild commentary” and chatter systems for flavor text, which expands on the original mention of a chat log primarily for offline death reporting.  
11. *Multi-Tap or Hold Abilities*  
- The original ruleset mentioned multi-tap/hold briefly, but the design doc clarifies it as a standard input type for certain skills.  
12. *Contextual UI Cues*  
- The design doc places more emphasis on tooltips, highlights, and prompts near loot or interactive objects, plus subtle color changes for rare drops.  
13. *Arena Pagination vs. Zoom*  
- We had *Q/E* to switch arenas, but the design doc also introduced the concept of zooming out to an overworld map using *W*. This is now clarified as well.  
14. *Total Roster Management & Gacha Rolls*  
- The design doc restates that each arena can only produce recruits from that arena’s themed class. This was not overtly stated in the original ruleset.

---

# **All Follow Up Questions**

- *Gacha Buff Stacking*  
    
  - Since global buffs come from gacha and can be used from the guild menu, do we allow multiple buffs to be queued up simultaneously, or is there a “one active buff” limit?  
    - I think buffs should stack but there could be rule sets set out later when gameplay is more flushed out. Stacking buffs will be more fun but could make game imbalanced. This system needs a prototype before commiting


- *Gear & Inventory Limits*  
    
  - Are there any constraints on how much gear or consumables a player can hold at once? (e.g., shared stash vs. character-bound)  
    - No constraints. And there shouldn't be any problems with memory either so don't worry about that.


- *Chat Log & Reporting*  
    
  - For high-volume offline events, do we need pagination or archiving to avoid overloading the chat log?  
    - During offline the battles will not literally continue, it will just be a delta between the last snapshot recorded and the current time as a herustic. Therefore there will not be any pagination or archiving because there shouldn't be any load. That said, chat logs should have some kind of capacity and when that capcity is full than the logs delete oldest first.


- *Multiple Global Buff Timers*  
    
  - If multiple global buffs from gacha are activated back-to-back, do their timers run in parallel, or must one fully expire before another can be used?  
    - Their timers run in parallel


- *In-Game Debug Options for Players*  
    
  - Aside from dev-only tools, might we offer a simplified “replay” or “highlight reel” for players who want to watch a key moment from a past cycle?  
    - That would be ideal but a P1


1. *Dev Debug Workflow*  
     
   - Do we plan a dedicated “Timeline Viewer” in-engine, or just console commands to jump to specific frames?  
     - Let's plan for both and we'll build incrementally  
   - Any constraints on memory usage for storing multiple dev snapshots?  
     - We will store  snapshots as persistant files that can be loaded in and out

   

2. *Timeline Idle Behavior*  
     
   - When a character finalizes a recording after only 1 minute of input, they idle for the remaining minute. Should the UI clearly show that they are now “idling” from timestamp 1:00 to 2:00?  
     - Yes

   

3. *Global Buff Overlaps*  
     
   - For clarity, do global buffs show a separate timer UI in each arena? Or is it centralized in a single global HUD?  
     - Centralized in a single global HUD

   

4. *Offline Simulation Deaths*  
     
   - If multiple characters die during offline sim, is that explicitly displayed in a “Offline Report” upon returning to the game, so the player knows who died?  
     - It will be captured in the global chat logs and you can have a filter in the chat to review like a report

   

5. *Large-Scale Lore*  
     
   - If we generate “thousands of lines” of codex text, do we want to store them all in local text files, or in a database for easier patch updates?  
     - Just a local file for now. we don't need a database yet.

   

6. *UI Complexity*  
     
   - Are there special controls or quick slots to manage global buffs, or do we treat them like normal abilities cast by a “Guild Commander” or a specific hero?  
     - I think all global buffs come from gacha's and they can be used from the menu in the guild house like item consumables.

   

7. *Collision Rules*  
     
   - Do we definitively allow multiple characters to occupy the same grid cell, or must they remain blocked? This drastically changes how pathfinding and layering of “ghosts” works.  
     - We allow multiple characters to occupy the same grid cell. This will not drastically change anything. Ghosts have a multiply blending mode so it will just make those cells a bit darker in color

   

8. *Global Buff Timers*  
     
   - Are we definitely restricting all buffs to end in sync across all arenas, or can a global buff expire in one arena while continuing in another?  
     - Global buffs could have their own independant timers that affect a each arena and ability indepantly

   

9. *Boss Enrage*  
     
   - Is the “enrage at 2:00” a confirmed design feature for all bosses, or an optional concept only for some?  
     - no longer a design

   

10. *Travel Attacks*  
      
    - If traveling heroes get attacked while en route, do we record those battles as a mini-instance or is it purely background simulation?  
      - It's a background simulation for that one hero and shouldn't have any real barring on the game other than if the hero dies than they return back to guild house and have to run again.

    

11. *Recording Durations*  
      
    - Confirm that *all* recordings are strictly 2 minutes. If a player stops inputs after 1 minute, does the timeline just idle for the remainder?  
      - if the player commits to a recording at 1 min than yes that character will remain idle for the remainder. If the player stoped the recording and doesn't commit than that character goes back intot that background simulation

    

12. *Future Random Elements*  
      
    - Are there short-term plans to add any small RNG (e.g., critical hits) to test how it impacts determinism, or is that strictly post-launch?  
      - Yes there are plans but I haven't figure out how to implement it yet and won't do so until the base game is finished

    

13. *Offline Calculations*  
      
    - Do you want partial cycles for offline progression? E.g., if the user is gone 5 minutes, that’s 2 full cycles plus 1 minute leftover, or do we simply floor/round it?  
      - We do a floor

    

14. *Timeline Editing or Developer Debug*  
      
    - Are you considering any *internal* (dev-only) debugging tools to step through frames or test large sets of ghost data quickly?  
      - Yes this will most likely be required to be productive as a small indie game studio

    

15. *Multiplayer / Co-Op*  
      
    - If we add co-op, do we attempt to synchronize each player’s “ghost recordings” across a network, or does each player just record locally in the same instance?  
      - We will not add co-op in initial release.  
- *Global Buff Timer Overwrites*  
  - If multiple global buffs are triggered (e.g., different sources), do they stack or overwrite each other?  
    - Both. Buffs will have associated tags and levels. If the same buff is applied at a higher level, it overwrites the previous buff with the stronger version. However, if a different buff is added, its effects will stack with existing buffs.  
- *Travel Encounters*  
  - Could we have random encounters or mini-bosses in the overworld that can engage traveling heroes?  
    - Yes and no. There will be no random encounters, but there can be mini-bosses. Everything will still need to fit into the same two-minute timer and remain deterministic. That said, there could be more than one boss at a time—multiple bosses, mini-bosses, or even creeps could all be part of the arena fight.  
- *UI Complexity*  
  - Will we offer players an advanced “chat filter” to selectively see only boss kill messages, recruit notifications, or lore reveals?  
    - Yes


1. *Global Buff Mechanics*  
     
   - If we add global buffs, do they run on a shared clock or separate 2-minute cycle?  
     - They would run on a independant timer but affect all rotation abilities e.g. some global damage multiplier  
   - How do we handle a scenario where a buff ends in one arena but not in another?  
     - This cannot happen

   

2. *In-Game Dialogue Priority*  
     
   - If multiple dialogues trigger at once (e.g., multiple bosses defeated across arenas), do we queue or stack message bubbles?  
     - Dialogs will only show in active arenas. All other's come up as notifications in the global chat box. And you can filter these chat logs based on notifications

   

3. *Overworld Travel*  
     
   - If a traveling hero is “caught” in an arena event halfway, do they automatically join, or is there a “transition zone” to finalize movement?  
     - if caught the character can take damage and die; however, rotation and joining an arena isn't captured until the player initates a recording

   

4. *Lore Frequency*  
     
   - Will large volumes of new codex entries spam players, or do we plan an incremental reveal system (e.g., X entries per boss kill or milestone)?  
     - enteries aren't captured until the player goes into guild house and opens the loot box/gacha

   

5. *Pity Systems*  
     
   - Should we add a “guaranteed higher-rarity recruit” after a set number of common recruits, or do we rely solely on static probability?  
     - There should be static probability but some kind of teir system to incrementally increase base stats and abilities as teh game progresses. This incrementatlity will be considered as we build the prototype

   

6. *Multi-Arena Abilities*  
     
   - Could certain abilities potentially affect more than one arena at a time (e.g., global buffs)? Currently, all abilities are arena-locked. Confirm that this remains the intended design.  
     - We should consider global buffs as a design opportunity.

   

7. *Story Progression Checkpoints*  
     
   - Should story cutscenes or dialogues appear mid-raid if a milestone is reached, or only once the 2-minute cycle ends?  
     - We have no plans for story cutscenes, but in-game dialogues should appear in real time as message bubbles on characters. A “global chat” UI could also display these conversations.

   

8. *AI-Generated Codex or Lore*  
     
   - Any technical constraints on generating thousands of lines of randomized in-game lore? How will we handle performance or memory usage for large-scale text generation?  
     - We could store all generated lore in persistent document storage, such as a text file.

   

9. *Gacha Rarity Mechanics*  
     
   - Confirm whether we need a “pity timer” or guaranteed higher-rarity hero after a set number of lower-rarity recruits.  
     - At the end of a two minute rotation, each arena should have a Gacha roll. Each arena is tied to a specific character class (e.g., a thief's arena). If an arena has an active battle, it triggers the Gacha roll, and only heroes of that arena’s class can be obtained. Essentially, a thief’s arena can only yield thieves, and so on. These Gacha are opened and review when pushing "Enter" and opening the Guild house.

   

10. *Death Loop vs. Ghost Replay*  
      
    - When a character *dies* in the recorded timeline, does their recorded ghost always die at the same moment in future replays, or can subsequent changes (like healing earlier) alter that outcome?  
      - Each arena recording is two minutes long, capturing all actions (including deaths and revivals) within that timeframe. If a character dies at, say, 1:00 and is revived at 1:30, the timeline will reflect death at 1:00, then jump to 1:30 when they are revived, continuing from there with the character alive again. Future replays of that exact recording will mirror those events. However, if you create a new recording where the character receives earlier healing (thus avoiding death), the outcome in that new timeline could differ. Essentially, each recording is a self-contained snapshot of the arena loop.

    

11. *Ability Overlaps & Input Conflicts*  
      
    - If multiple abilities are triggered at the exact same timestamp (e.g., a character’s multi-tap action overlaps a hold-release), how do we prioritize them in the timeline engine?  
      - While it’s rare for multiple abilities to occur at the exact same timestamp, the system supports parallel execution. If two or more actions do overlap precisely, they will all be processed simultaneously rather than queued.

    

12. *Performance & Tick Rate*  
      
    - Are we running all 8 arenas at the same fixed tick rate (e.g., 60 FPS), or do inactive arenas run at a lower simulation rate to save resources?  
      - All arenas run at the same simulation rate (e.g., 60 FPS) for consistency. However, inactive arenas do not render visuals, which helps reduce GPU usage and other resource costs.

    

13. *Guild House Mechanics*  
      
    - Is the physical guild house mandatory for all management interactions, or can players open a management UI from anywhere?  
      - Players can open the management UI from any arena, except during certain events like active recordings, when it might be temporarily locked. Nonetheless, there will be a physical guild house in the game world, and new recruits may need to travel on foot from the guild house to their chosen arena.  
    - If physical, do we track the character’s travel time to and from the guild house?  
      - Travel is not formally recorded as part of the timeline. However, bosses and replays remain active during this travel period. The character continues participating in battles in the background but must manually move between arenas for story or strategic reasons.

    

14. *Arena Synchronization*  
      
    - If each arena has its own 2-minute timer, do they all start simultaneously, or can they be out of sync?  
      - Arenas can be out of sync. If you choose to record a specific arena, that arena’s timer resets to zero and begins a short countdown (e.g., 3 seconds) before recording starts, independent of other arenas.  
    - How do we handle a scenario where Arena A is paused while Arena B continues running?  
      - Recordings and playbacks are decoupled per arena. If Arena A is paused, Arena B can continue uninterrupted (or also be paused). Each arena’s state is managed independently.

    

15. *Offline Idle Progression*  
      
    - How exactly is offline progression calculated? Do we simulate minutes/hours offline, awarding scaled resources/XP accordingly?  
      - Upon exiting, the game captures snapshots of each arena’s state. When you return, the system checks the latest snapshot’s timestamp, compares it to the current time to determine how long you were away, and then divides that duration by two to estimate how many rotations would have happened. It uses the count of active raiding characters to calculate total experience and loot boxes awarded for that offline period.

    

16. *Boss Scaling*  
      
    - Does a boss scale if more total characters are active? Or is it purely the same difficulty regardless of how many ghosts are fighting?  
      - Boss difficulty does not scale with the number of characters. Scaling could invalidate older replays or create inconsistent player experiences, so we keep boss difficulty static to preserve deterministic outcomes across recordings.

    

17. *Network or Shared Instances*  
      
    - While this is primarily a solo experience, is there any plan or placeholder for optional multiplayer or user-created raids?  
      - There is an internal plan for user-created raids (mostly for testing boss mechanics), but it’s considered a secondary priority (labeled P2) until the core single-player experience is solid. Multiplayer is also a potential future feature, but likewise a P2 until the base game is complete.

    

18. *Saving & Persistence*  
      
    - How do we handle partial progress if the user closes mid-recording?  
      - If the game closes during a recording, that recording is discarded, and the character is removed from the arena but remains flagged as active at the start of an arena. They’ll still participate in ongoing battles (and potentially die), but no new recording data is saved until a fresh recording is initiated.  
    - Is there an autosave at the moment the player starts or ends a recording?  
      - Yes. The game will autosave upon starting or ending a recording. A prompt allows players to confirm and commit the new rotation or re-record it if desired.

    

19. *Narrative Triggers*  
      
- Are there specific triggers (e.g., defeating certain bosses or reaching certain hero levels) that cause the story to advance, or is it purely time/gate-based?  
  - Boss defeats can act as narrative triggers to advance the storyline. Additionally, arenas can be upgraded to higher difficulty tiers—Normal, Heroic, or Mythic—once certain progression milestones are met, providing ongoing goals that fit the overall theme of the game.

### **1\. Collision & Overlap**

- *Have we confirmed whether multiple characters can share the same grid cell or if collisions block movement?*  
  Characters can occupy the same grid space; however, certain power-ups might provide damage multipliers if the characters are *not* sharing that space.  
    
- *Should an attempted move be queued/delayed if the target cell is occupied?*  
  No, characters cannot share a space.

---

### **2\. Boss Timeline vs. Hand-Coding**

- *Do we want to unify boss logic fully under the same replay system now, or do we foresee temporarily mixing hand-coded patterns with partial timeline-based logic?*  
  Bosses should share the same logic as the replay system.  
    
- *If hand-coded, how do we ensure it won’t conflict with the overall deterministic approach?*  
  They are shared now.

---

### **3\. Data Structures for Inputs**

- *Are we storing frame-by-frame input events, or do we store an “action start” and “action end” with an associated duration?*  
  We store an action start and an action end, each with an associated duration.  
    
- *How will this data structure scale if we add more complex input types?*  
  The data structure should be robust enough to handle multiple, overlapping actions. If certain interactions do not fit, we will consider removing them.

---

### **4\. Time Manipulation & Debugging**

- *If we add a replay feature (beyond just the “loop”), do we need a fast-forward or step-through mode for debugging?*  
  Not required.  
    
- *Could there be a developer-only console command for these features?*  
  Not required.

---

### **5\. Recording Storage & Memory**

- *Each character’s 2-minute timeline can grow large if storing frame-by-frame input; how do we plan to handle memory usage for potentially hundreds of characters across multiple arenas?*  
  We will only capture action starts and ends.  
    
- *Do we need compression or serialization strategies?*  
  I’m fairly certain the timeline data, even if stored frame by frame, will use minimal memory.

---

### **6\. UI for Timeline Management**

- *Will we have a developer-mode timeline editor with drag-and-drop events or purely in-code data editing?*  
  Not required.  
    
- *Are there concerns about how players (not just devs) might manually edit or reorder these events?*  
  No, this isn’t a competitive or security-sensitive scenario.

---

### **7\. Arena Hotswapping**

- *When moving a character from one arena to another, do we discard the previous timeline, or does the character keep its last recorded timeline paused until returning?*  
  The character keeps its timeline.  
    
- *Do transitions consume in-game time or reset the 2-minute window?*  
  The window resets when the record button is clicked.

### **8\. Timeline Control**

*How does the player manage the timeline (e.g., pausing, rewinding, or stepping through frames) during development or debugging of these recordings?*

- *Pausing:* Pressing Enter/Return opens a menu, which is the only way to pause the game.  
- *Rewinding:* There is no rewind function. However, while recording, the player can cancel and restart the rotation, resetting the timer to 0 for that arena.  
- *Stepping through frames:* Currently, there is no way to step through frames. However, there should be a way to replay recordings if the player wants to review an interesting event. This feature is considered lower priority.

*Are recorded timelines per character locked to the same 2-minute window, or can some recordings be shorter/longer?*

- Recorded timelines are per character and locked to the same 2-minute window.  
- Timelines cannot be longer or shorter. However, a character may stop performing actions before time is up, effectively simulating a shorter timeline.

---

### **9\. Multi-Arena Interaction**

*Is there any interaction or shared resource across arenas, or are all arenas completely isolated aside from the player’s attention?*

- Currently, there are no plans for shared interactions between arenas, aside from the possibility of stopping a character in one arena and moving them to another.  
- Other than that, arenas remain entirely isolated.

---

### **10\. Character Classes & Abilities**

*Do the 8 classes share any abilities, or are all 4 abilities per class unique?*

- All eight classes have their own unique pool of abilities.  
- The abilities and their attributes will be randomized based on the game’s current progression and RNG.  
- Each character can have up to four abilities—sometimes fewer, but never more than four.

*Are there cooldowns or resource costs for abilities that need to be tracked in the timeline?*

- Cooldowns and resource costs do not need to be tracked in the timeline because they serve as “gating mechanics.”  
- However, certain details—like cast times and the placement of abilities after they’re cast—are “execution parameters” that must be tracked in the timeline.  
- In summary, while cooldowns and costs must be recorded, they do not belong in the timeline itself; data such as cast times and ability placement does need to be included in the timeline.

---

### **11\. Deterministic Mechanics**

*Besides grid-based location checks, are there any random elements (e.g., critical hits, random damage) or is everything purely deterministic?*

- While I do want to include non-deterministic elements in the future, the current game is entirely deterministic until I can build out some levels, see how chaotic it can get, handle edge cases, and ultimately find the fun.

*How do we handle simultaneous actions in terms of priority (e.g., two characters trying to occupy the same grid at once)?*

- I have not yet decided whether two characters can occupy the same grid space—it could go either way.  
- Whether simultaneous actions are possible will ultimately depend on what is most fun for the gameplay.

---

### **12\. Death & Revival Mechanics**

*How exactly is the moment of revival determined? By a recorded action (e.g., a recorded “Revive” ability) or by some external event?*

- All actions have certain “execution parameters,” one of which is the grid square on which the ability is cast.  
- Revive is not cast on a character directly but rather on a grid square. If a character is present on that grid square when the cast is executed *and* they are dead, they will be revived.  
- However, if no character is present or the character is not dead, it counts as a “revive miss,” and the timeline continues normally.

*Do revived characters resume from the exact point they left off in their timeline or restart their loop?*

- Revived characters continue from where they would be in the current timeline. For example, if a character dies at 1:00 and is revived at 1:30, they disappear at 1:00 and reappear at 1:30, then resume their rotation from that 1:30 point.

---

### **13\. Boss Battles**

*Are boss battles also integrated into the replay system (i.e., do bosses record & replay actions), or are they simply repeating patterns coded separately?*

- As a software engineer, I plan for bosses to have different abilities but still follow the same 2-minute recording and replay system.  
- However, it might be simpler initially to hand-code boss timelines to avoid over-engineering. That said, unifying all entities under the same system could make level design much easier in the long run.

*How do these deterministic boss patterns handle interactions with multiple ghost replays?*

- Bosses follow the same 2-minute timeline.  
- Their abilities have cast times, cooldowns, and costs, and they target the grid as execution parameters.  
- These abilities and movements live within the same 2-minute rotation that the player records for their characters.

---

### **14\. Implementation of Input Types**

*How do we plan to store complex inputs (e.g., click-and-hold duration)?*

- I want to use a musical staff metaphor, where we have notation or data representing each ability.  
- If an ability has a click-and-hold duration of 3 seconds (or 180 frames), the timeline will represent those 180 frames.  
- This could be handled via a data structure like fireball(held: 180frames, grid\_destination(10,10)), or by recording all keystrokes frame by frame.

*Are multi-tap or click-and-hold actions broken into sub-events with timestamps or tracked as single actions with an associated duration?*

- Click-and-hold actions will be captured with two events: *keydown* and *keyup*.  
- Multi-tap actions will be captured with *N* tap/click events. However, the state of each tap belongs to the character, so the timeline must communicate with that character appropriately.

---

### **15\. Scalability & Performance**

*How do we handle performance with up to 320 characters replaying simultaneously?*

- With 320 characters, 8 bosses, and potentially more mobs, this is still a relatively small computational load.  
- Because the game is deterministic, I can control the speed of gameplay.  
- When focusing on one arena, I can pause or stop rendering other arenas to reduce computational overhead.

*Do we need any culling or optimization strategies for characters/arenas that are out of view?*

- We will likely avoid rendering characters that are off-screen.

---

### **16\. UI & UX**

*Will there be a visual timeline editor for players (or devs) to see, edit, and manage recorded actions?*

- Having a visual way to see actions would provide helpful feedback.  
- I need to figure out a minimalistic approach to avoid cluttering the UI.

*Are there any constraints on how often a new “recording session” can start or how many sessions can be stored?*

- Each character can only have one active recording at a time.  
- However, having a persistent history of recordings for replaying when something goes wrong could be a useful strategic feature.

