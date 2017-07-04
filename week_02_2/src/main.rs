extern crate num;

extern crate tcod;
use tcod::RootConsole;
use tcod::{Console};
use tcod::console::{FontLayout, FontType, Offscreen};
use tcod::input::{Key, KeyCode};

mod traits;
use traits::{Renderable, Movable, Position};

mod point;
use point::Point;

mod units;
mod map;

const SCREEN_WIDTH: i8 = 80;
const SCREEN_HEIGHT: i8 = 50;
const PANEL_HEIGHT: i8 = 5;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn to_rel_point(self) -> Point<i8> {
        match self {
            Direction::Up       => Point::new(0, -1),
            Direction::Down     => Point::new(0, 1),
            Direction::Left     => Point::new(-1, 0),
            Direction::Right    => Point::new(1, 0),
        }
    }
}


#[derive(Debug)]
enum KeyType {
    Movement(Direction),
    Exit,
    Other,
}

fn key_type(key: &Key) -> KeyType {
    match key.code {
        KeyCode::Right  =>  KeyType::Movement(Direction::Right),
        KeyCode::Left   =>  KeyType::Movement(Direction::Left),
        KeyCode::Up     =>  KeyType::Movement(Direction::Up),
        KeyCode::Down   =>  KeyType::Movement(Direction::Down),
        KeyCode::Escape =>  KeyType::Exit,
        _ => KeyType::Other,
    }
}

fn main() {
    let mut root = RootConsole::initializer()
                    .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
                    .title("Roguelike Tutorial")
                    .fullscreen(false)
                    .font("arial10x10.png", FontLayout::Tcod)
                    .font_type(FontType::Greyscale)
                    .init();

    let mut buffer_console = Offscreen::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);

    root.set_default_foreground(tcod::colors::WHITE);

    let mut player = units::Unit::new(Point{x: 1, y: 1}, '@', tcod::colors::WHITE);

    let map = map::Map::init();

    while !root.window_closed() {
        buffer_console.clear();

        map.render_map(&mut buffer_console);
        map.render_npcs(&mut buffer_console);

        player.render(&mut buffer_console);

        tcod::console::blit(&buffer_console, (0,0), (0,0), &mut root, (0,0), 1.0, 1.0);
        root.flush();

        let key = root.wait_for_keypress(true);

        match key_type(&key) {
            KeyType::Movement(dir)  => {
                let pos = player.get_position();
                let new_pos = pos + dir.to_rel_point();

                if let Ok(tile) = map.get_tile_type(new_pos) {
                    if !tile.blocks_move() {
                        player.move_to(new_pos);
                    }
                }
            },
            KeyType::Exit           => break,
            KeyType::Other          => println!("{:?}", key),
        }
    }
}
