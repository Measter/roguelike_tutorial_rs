# Week 05

## Week 05.2 - The GUI

Adding the GUI was relatively easy. I decided to encapsulate it all in one object. I don't really like how I'm passing it around to the NPCs as they take their turns. Perhaps have some sort of return value for `take_turn()` that indicates what happened. That way the NPC won't need to know about the UI.

I do think I need to do a general clean up of the project. Parts of it are pretty messy, especially around the game state.