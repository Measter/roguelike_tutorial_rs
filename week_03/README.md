# Week 02

Well, it seems that `tcod` has an issue with the MSVC linker, so I had to [switch](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-rust-on-windows) to the GNU version of Rust.

I've split this week up corresponding to the tutorial parts. It seemed to make more sense than putting them both together.

## Week 02.2 - Map

Apparently I got a bit ahead of myself in the previous section, and generalised the unit too early, so this is one is mostly just the map. I did add the colour to the unit object, which I had missed when doing the last section.

I opted, for now at least, to keep the player unit out of the NPC list. If I put it in the list, I wouldn't be able to hold a constant reference, and would have to specifically fetch the player unit in each iteration of the loop. Depending on how I handle AI in the future, I may be able to offload the AI handling routine to a separate object that's injected into the Unit, and handle player input with that. In that case I may be able to just shove the player unit into the NPC array and handle it with the rest. We'll see.

I decided to add the functions `get_color` and `get_glyph` to the `Renderable` trait, so that I could turn `render` into a provided method. That'll mean I don't need to keep rewriting the same function in the same way for anything simple that just puts a character on screen.

For the map, instead of just storing an array, I decided to build a full map structure. That way, the implementation details are hidden from the rest of the program. I also opted to place the NPC list into the map, so it will handle rendering and updating them. Given NPCs will be *on* a map, it seemed to make sense to have them owned by the map.

### Update

I realised just before starting the week 3 section that I had forgotten to handle wall collisions. While doing that, I decided to re-write how I handled coordinates, opting for a Point type that implements the `Add` and `Sub` traits. I also added a function to the `Direction` enum to convert it to a relative `Point`. That simplified the movement handling greatly, as I can just sum the relative point with the player's current position, and use that to check the map for blocking tiles.