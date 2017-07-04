# Week 03 - Dungeon Generation

This week isn't really much different to the tutorial. Due to my decision to use ranges for defining tunnels, I did have to do a bit of fiddling to make sure it was going the right way, but at least that's hidden behind the function itself.

I decided to draw the tunnels after generating all the rooms. It is another iteration, but I felt it was cleaner than keeping track of the previous room as I went. Besides, it's <=30 rooms, so it's not a huge deal.

I'm not greatly happy with the result of the generation of the tunnels. It feels messy. I'm considering changing it so that each room connects to the closest 2-4 rooms, and also where in the room it connects the tunnel.