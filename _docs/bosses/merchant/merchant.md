# The Gambler - Merchant Boss

## Overview
- **Theme**: Probability manipulator who weaponizes chance
- **Difficulty**: RNG management and risk assessment
- **Arena**: Casino floor with slot machines and roulette wheels
- **Unique Verb**: GAMBLE - Creates cascading probability events

## Phase Structure (2 minutes total)

### Phase 1: Opening Bets (0:00-0:30)
**Core Mechanic**: Introduction to probability manipulation

**Boss Abilities**:
- **Dice Roll** (every 8s): Random damage between 50-500
- **Lucky Strike** (every 10s): 50% chance to crit or miss completely
- **House Edge** (at 0:20): Boss gains 10% crit chance per player death

**Environmental**:
- Slot machines activate randomly for buffs/debuffs
- Roulette wheel spins, landing zones for damage
- Cards scatter providing random effects

**Counter-play**:
- Minimize RNG exposure
- Stack defensive stats
- Control slot machine triggers

### Phase 2: Raising Stakes (0:30-1:00)
**Core Mechanic**: Risk/reward escalation

**Boss Abilities**:
- **All In** (every 12s): Next attack either heals or devastates
- **Probability Storm** (every 9s): All attacks become random chance
- **Double or Nothing** (at 0:45): All damage/healing doubled but 50% miss chance

**Environmental**:
- Casino chips grant stacking random buffs
- Jackpot zones appear with high risk/reward
- Probability fields alter hit chances

**Counter-play**:
- Calculate risk vs reward
- Use guaranteed effects
- Manipulate probability in your favor

### Phase 3: House Advantage (1:00-1:30)
**Core Mechanic**: Stacked odds system

**Boss Abilities**:
- **Loaded Dice** (continuous): Boss crits increase by 1% per second
- **Bankruptcy** (every 10s): Player with most buffs loses them all
- **Fortune's Wheel** (at 1:15): Spins wheel, random raid-wide effect

**Environmental**:
- Lucky streaks create damage chains
- Unlucky streaks cause spreading failure
- Probability increasingly favors boss

**Counter-play**:
- Distribute buffs evenly
- Break luck chains quickly
- Create your own probability

### Phase 4: Final Gambit (1:30-2:00)
**Core Mechanic**: Pure chaos gambling

**Boss Abilities**:
- **Quantum Lottery** (at 1:30): All abilities have random effects
- **Russian Roulette** (every 6s): One random player takes 90% damage
- **Winner Takes All** (at 1:50): Coin flip - boss or raid dies

**Environmental**:
- Complete RNG chaos
- Every action has random outcome
- Luck becomes tangible resource

**Counter-play**:
- Stack survival for roulette
- Manipulate coin flip odds
- Embrace calculated chaos

## Orthogonal Design Analysis

### Unique Mechanics
- **Probability Cascades**: RNG creating more RNG
- **Risk Management**: Gambling with health/resources
- **Luck Manipulation**: Changing probability itself

### Taxonomy Mapping
- **Verb**: GAMBLE (probability manipulation)
- **Modifier**: RNG cascades, risk/reward
- **Cost**: Uncertainty tax, risk management

### OC Score: 0.26
- Lowest overlap with: Forager (0.18) - chance vs terrain
- Highest overlap with: Thief (0.30) - both involve uncertainty

### Strategic Niche
Creates a unique risk management experience where players must navigate cascading probability events and make calculated gambles with their resources.

## Component Architecture

```rust
// Probability Cascade Entity
commands.spawn((
    ProbabilityCascade,
    InitialChance(0.5),
    CascadeDepth(3),
    SuccessMultiplier(2.0),
    FailureMultiplier(0.5),
));

// Gambling Effect Entity
commands.spawn((
    GamblingEffect,
    WinChance(0.5),
    WinHealing(500.0),  // Separate win outcome
    LoseDamage(500.0),  // Separate lose outcome
    DoubleOrNothing(true),
    Duration(5.0),
));

// Luck Tracker
commands.spawn((
    LuckLevel(0.0),
    LuckModifier(0.01),
    MaxLuck(0.5),
    LuckDecay(0.05),
));
```