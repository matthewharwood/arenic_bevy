

* After each loop, a **progress score** is calculated per arena. This score is based on two main factors:

    1. **How many characters survived** in the arena.
    2. **How much damage the boss took** during the battle.

* The **subtotal score** for each arena is calculated as:
  `duration_survived_in_seconds * average_damage_per_second (DPS) dealt to the boss`.

* These subtotals are then used to **seed the random selection** of **three loot options**, which the player can pick from.

### Example:

A player has three characters in the **Hunter’s Arena**. They survive for **30 seconds**, and the boss takes **1000 DPS** during that time.
This gives the arena a subtotal score of `30 * 1000 = 30,000`.

Because this score came from the **Hunter’s Arena**, the loot options will be **Hunter-themed**, and drawn from a **score tier ≤ 30,000**.

### Loot Categories:

Loot can be one of the following:

* **Ability Upgrade**
* **Stat Upgrade**
* **New Character**

You will **only receive loot** from the **arenas that contributed a score** during that loop.

### Multi-Arena Example:

If two arenas are active:

* **Hunter’s Arena** scores 30,000
* **Bard’s Arena** scores 40,000

Then the 3 loot options will be randomly chosen from the **Hunter and Bard loot tables**, within their respective score ranges (≤30,000 and ≤40,000).


1. *Loot & Currency* (Design Document 12, 18)

    - Rarity is color-coded. Gear upgrades, ability enhancements, or crafting materials can be found.
    - Currency can be spent at in-raid shops or an *in-raid auction house* (if implemented).


2. *Inventory & Equipment Management* (Design Document §13)

    - Equip heroes with gear that boosts stats or modifies abilities.
    - No strict limit on the total gear or consumables you can carry.
    - *Loadouts* for quick gear swapping are recommended.
    - The UI can highlight gear upgrades vs. downgrades for quick decisions.


3. *Gacha & Recruitment*

    - Each arena can grant a “gacha roll” after a successful 2-minute cycle. The class offered typically matches that
      arena’s theme (e.g., a Thief from the Thief’s arena).
    - New Hero Recruits appear at the Guild House; you can accept or deny them.


4. *Gacha Buff Stacking*

    - Some “global buff consumables” come from gacha. They stack or overwrite each other based on buff tags/levels.
    - Buff timers run in parallel, displayed in a single global HUD, and affect *all* arenas simultaneously.