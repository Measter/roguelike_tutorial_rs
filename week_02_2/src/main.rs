extern crate tcod;
use tcod::RootConsole;
use tcod::{Console};
use tcod::console::{FontLayout, FontType, Offscreen};
use tcod::input::{Key, KeyCode};

mod traits;
use traits::{Renderable, Movable, Position};

mod units;
mod map;

const SCREEN_WIDTH: u8 = 80;
const SCREEN_HEIGHT: u8 = 50;
const PANEL_HEIGHT: u8 = 5;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
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

    let mut player = units::Unit::new(1, 1, '@', tcod::colors::WHITE);

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
                let (x,y) = player.get_position();
                match dir {
                    Direction::Up    if y > 0                 => player.nudge(dir),
                    Direction::Down  if y < SCREEN_HEIGHT-1   => player.nudge(dir),
                    Direction::Left  if x > 0                 => player.nudge(dir),
                    Direction::Right if x < SCREEN_WIDTH-1    => player.nudge(dir),
                    _ => {}
                }
            },
            KeyType::Exit           => break,
            KeyType::Other          => println!("{:?}", key),
        }
    }
}
