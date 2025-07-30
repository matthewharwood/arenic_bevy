# Creating a Tutorial for the Chracter Create State

## Firstly, I need you all to read and commit to memory the "_docs/RULEBOOK.md", "_docs/game_design_doc.md", "arenic_bevy/src/**/*".

## The goal is to create then write a step-by-step tutorial located at: "_docs/character_create.md".

## Hey Calvin, I want you to take a first pass at scaffolding the the initial character_create.rs picking screen. Here is a starting place for your game design for this screen:

- a grid of 8 cards, one for each character type (matches boss types e.g. hunter, alchemist, etc). This will be your
  starting character that will transition to the Intro scene.
- Each card will have an icon, the boss type on it.
- The background will be red #E3334B and the cards will be #ffffff text, icon, and a thick white border.
- Once the character has been selected, the player will name the character.
- This selected and named character will transition to the Intro state and will spawn in the center of the guild house
  next to the guild master.

## Next, Adam, I want you to very lightly weave in some very subtle narrative to this screen, anything you add shouldn't take longer than 30mins to program into the game so please give detailed directions for Jon to confirm how long and Jon should push back if too difficult. Adam please confirm with Jon on details and find a balance of the narrative design.

## Next Damien, I want you to weave in some very subtle lighting maybe based on where the mouse is on the card? But perhaps talk to Jon first to ensure it's very simple as the engineer hasn't worked with lighting yet and this will be their first time.

## Next Jon, read the "bevy_migration_guide.md" then read what Calvin, Adam, Damien wrote and then write the complete the implementation details for the tutorial step by step.

## Marcus, revise the tutorial and ensure it's of the the highest quality.

## Lastly Jon, take one final pass and make sure all the code is crystal clear and to latest spec

