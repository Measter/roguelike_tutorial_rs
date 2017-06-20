# Week 01 - The Setup

The first, obvious, step, is to install Rust. You can obtain it [from the Rust website](https://www.rust-lang.org/en-US/install.html). The website also contains a couple documents which will give you decent guide to using Rust.

The compiler version used on this step is rustc 1.18.

The crate I'll be using for this is called [tcod](https://crates.io/crates/tcod). Unfortunately, this only provides bindings for libtcod 1.5.2, so I'll be a little out of date. The crate [repository](https://github.com/tomassedovic/tcod-rs) has instructions for building the crate, so you should follow those.

The final step is to add the dependency to your Cargo.toml file and compile using `cargo build`. The first compilation may take a bit longer, bcause Rust will build all your dependencies the first time, but these are cached in the project unless cleaned or the compiler is updated.