# Week 04

## Week 04.2 - Preparing for Combat

This ended up being way harder than I expected. I'm no longer sure if I like the NPCs being owned by the map. It might be better to have the map generate them, but then have the init function return them.

Adding weighted selection of monsters was a right pain in the backside! I did try to use the `WeightedChoice` type provided by the `rand` crate, but it required a *mutable* reference to the weight objects, which meant that I ran into lifetime and borrow check issues with the references.

After fiddling with it for way too long, I just gave up and implemented my own weighted selection function, that *doesn't* require a mutable reference. Then re-implemented it because I'm an idiot and got it wrong.