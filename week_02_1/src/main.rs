extern crate tcod;
use tcod::RootConsole;
use tcod::{Console};
use tcod::console::{FontLayout, FontType};
use tcod::input::{Key, KeyCode};

mod traits;
use traits::{Renderable, Movable, Position};

mod units;

const SCREEN_WIDTH: u8 = 80;
const SCREEN_HEIGHT: u8 = 50;

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

    root.set_default_foreground(tcod::colors::WHITE);

    let mut player = units::Unit::new(1, 1, '@');

    while !root.window_closed() {
        root.clear();
        player.render(&mut root);
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
