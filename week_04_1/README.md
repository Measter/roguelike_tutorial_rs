# Week 04

## Week 04.1 - Field of View and Scrolling Map.

Adding the FOV calculation wasn't too different to the Python version, though I did run into the minor issue of *where* to update the FOV state on the tile map. The place I initially thought would be good was when rendering, but then I realised that this would require the `render_map` method to take the map mutably. While that would work, it doesn't feel right that a function for rendering should need to mutate the map.

I eventually settled on putting the tile update in the `update_fov` function. It would make it more expensive to run, given it's now got to iterate the tile map, but the map isn't huge and the function only gets called on a map or player position update.

I decided to implement the scrolling map. The maps are a little boring for it, but at least it demonstrates a method. It took an embarrassingly long time to figure out why the scroll clamping was weird on the bottom and right edges of the map. I had the signs flipped in the clamp function...

### Update

I've adjusted the room generation to use the map size to calculate the maximum number of rooms. That way the density of each level will be roughly the same.