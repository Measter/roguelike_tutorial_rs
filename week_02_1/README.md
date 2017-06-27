# Week 02

Well, it seems that `tcod` has an issue with the MSVC linker, so I had to [switch](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-rust-on-windows) to the GNU version of Rust.

I've split this week up corresponding to the tutorial parts. It seemed to make more sense than putting them both together.

## Week 02.1 - Graphics and Movement

I originally intended to use the default font `terminal.png`, hence the original push, but ultimately decided to stay with the tutorial's use of `arial10x10.png'.

I decided here to make use of the trait system to define position, rendering and movement. It didn't feel right just having random variables and direct drawing for that.

The way I'm handling movement at the moment feels a bit awkward, but I don't think I want to be passing in objects to collision-check to the unit when moving. That feels like it would get out of hand very quickly.