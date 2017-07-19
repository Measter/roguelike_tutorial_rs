# Week 05

## Week 05.1 - Going Berserk!

The first thing I did here was to clean up the main loop. It was getting a bit clunky, so I pulled out the rendering and input handling to their own functions. Probably a bit overdue.

When I started implementing the AI, I ran into issues with the borrow checker. The AI routine needed to borrow the map, but couldn't do so because it was already borrowed elsewhere. I ended up doing what I exected I'd need to a couple weeks ago, and now the Map no longer owns the list of NPCs.

I will probably implement the A\* pathfinding for the AI after the next step. I really don't like that they can't go round walls. Even the dumbest of animals can manage that!

I decided to split out the dumb items like corpses from the NPCs. It didn't make sense for them to be shared like that. Once I'd decided on that, it made less sense to implement the fighter and AI components as the tutorial did. I also decided to add the unit stats to the unit_types data file.