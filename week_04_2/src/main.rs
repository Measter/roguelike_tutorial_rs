extern crate num;
extern crate rand;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

extern crate tcod;
use tcod::RootConsole;
use tcod::{Console};
use tcod::console::{FontLayout, FontType, Offscreen};
use tcod::input::{Key, KeyCode};

mod traits;
use traits::{Renderable, Movable, Position};

mod point;
use point::Point;

mod rectangle;

mod units;
mod map;

const SCREEN_WIDTH: u8 = 80;
const SCREEN_HEIGHT: u8 = 50;
const PANEL_HEIGHT: u8 = 5;

const FOV_RADIUS: u8 = 10;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn to_rel_point(self) -> Point<i16> {
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
    NewGame,
    Other,
}

fn key_type(key: &Key) -> KeyType {
    match key.code {
        KeyCode::Right  =>  KeyType::Movement(Direction::Right),
        KeyCode::Left   =>  KeyType::Movement(Direction::Left),
        KeyCode::Up     =>  KeyType::Movement(Direction::Up),
        KeyCode::Down   =>  KeyType::Movement(Direction::Down),
        KeyCode::Escape =>  KeyType::Exit,
        KeyCode::F1     =>  KeyType::NewGame,
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

    let mut buffer_console = Offscreen::new(map::MAP_MAX_WIDTH as i32, map::MAP_MAX_HEIGHT as i32);
    root.set_default_foreground(tcod::colors::WHITE);

    let unit_types = units::load_unit_types();
    let (mut map, start_coord) = map::Map::init(&unit_types);

    let mut player = units::Unit::new(start_coord, units::UnitType::new("Player", '@', tcod::colors::WHITE));

    map.update_fov(player.get_position(), FOV_RADIUS);

    while !root.window_closed() {
        buffer_console.clear();
        root.clear();

        // With the scrolling map, we need to try to centre the player on the screen
        // without going past the bounds of the buffer.
        let (map_width, map_height) = map.get_map_size();
        let draw_left = player.get_x() - SCREEN_WIDTH as i16 / 2;
        let draw_top = player.get_y() - SCREEN_HEIGHT as i16 / 2;

        let mut view_port = rectangle::Rectangle::new(Point{x: draw_left, y: draw_top}, (SCREEN_WIDTH, SCREEN_HEIGHT - PANEL_HEIGHT));
        view_port.clamp_to((0,0), (map_width as i16, map_height as i16));

        println!("{:?}", view_port);

        map.render_map(&mut buffer_console);
        map.render_npcs(&mut buffer_console);

        player.render(&mut buffer_console);

        tcod::console::blit(&buffer_console, (view_port.top_left.x as i32, view_port.top_left.y as i32), (SCREEN_WIDTH as i32, (SCREEN_HEIGHT - PANEL_HEIGHT) as i32), &mut root, (0,0), 1.0, 1.0);
        root.flush();

        let key = root.wait_for_keypress(true);

        match key_type(&key) {
            KeyType::Movement(dir)  => {
                let pos = player.get_position();
                let new_pos = pos + dir.to_rel_point();

                if let Ok(tile) = map.get_tile_type(new_pos) {
                    if !tile.blocks_move() {
                        player.move_to(new_pos);
                        map.update_fov(player.get_position(), FOV_RADIUS);
                    }
                }
            },
            KeyType::Exit           => break,
            KeyType::NewGame        => {
                let (new_map, start_coord) = map::Map::init(&unit_types);
                map = new_map;
                player.move_to(start_coord);
                map.update_fov(start_coord, FOV_RADIUS);
            }
            KeyType::Other          => println!("{:?}", key),
        }
    }
}
