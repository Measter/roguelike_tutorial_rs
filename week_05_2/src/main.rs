extern crate num;
extern crate rand;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

extern crate textwrap;

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
mod item;
mod units;
mod unit_type;
mod map;
mod ui;

use std::collections::VecDeque;

const SCREEN_WIDTH: u8 = 80;
const SCREEN_HEIGHT: u8 = 50;
const PANEL_HEIGHT: u8 = 5;
const PANEL_Y: u8 = SCREEN_HEIGHT - PANEL_HEIGHT;

const FOV_RADIUS: u8 = 10;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum GameState {
    Playing,
    Dead,
    NewMap,
    Menu,
    Exit,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum PlayerAction {
    Moved,
    Turn,
    NoTurn,
}

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

fn render_all<'a>(root: &mut RootConsole, buffer_console: &mut Offscreen, ui: &mut ui::UI, map: &map::Map, npcs: &VecDeque<units::Unit<'a>>, player: &units::Unit) {
    buffer_console.clear();
    root.clear();

    // With the scrolling map, we need to try to centre the player on the screen
    // without going past the bounds of the buffer.
    let (map_width, map_height) = map.get_map_size();
    let draw_left = player.get_x() - SCREEN_WIDTH as i16 / 2;
    let draw_top = player.get_y() - SCREEN_HEIGHT as i16 / 2;

    let mut view_port = rectangle::Rectangle::new(Point{x: draw_left, y: draw_top}, (SCREEN_WIDTH, SCREEN_HEIGHT - PANEL_HEIGHT));
    view_port.clamp_to((0,0), (map_width as i16, map_height as i16));

    map.render_map(buffer_console);
    
    for unit in npcs.iter() {
        if map.point_in_fov(unit.get_position()) {
            unit.render(buffer_console);
        }
    }

    // If the player has died, a corpse will have been created.
    if player.get_hp() > 0 {
        player.render(buffer_console);
    }

    tcod::console::blit(buffer_console, (view_port.top_left.x as i32, view_port.top_left.y as i32), (SCREEN_WIDTH as i32, (SCREEN_HEIGHT - PANEL_HEIGHT) as i32), root, (0,0), 1.0, 1.0);

    ui.render(root);

    root.flush();
}

fn handle_input<'a>(root: &mut RootConsole, cur_game_state: GameState, map: &map::Map, ui: &mut ui::UI, npcs: &mut VecDeque<units::Unit<'a>>, player: &mut units::Unit) -> (PlayerAction, GameState) {
    let key = root.wait_for_keypress(true);

    let mut player_action: PlayerAction = PlayerAction::NoTurn;
    let mut new_game_state: GameState = cur_game_state;

    match key_type(&key) {
        KeyType::Movement(dir) if cur_game_state == GameState::Playing => {
            let pos = player.get_position();
            let new_pos = pos + dir.to_rel_point();

            player_action = match map.can_move_to(new_pos) {
                map::CanMoveResponse::Open => {
                    if let Some(enemy) = npcs.iter_mut().filter(|n| n.get_position() == new_pos).next() {
                        player.attack(enemy, ui);
                        PlayerAction::Turn
                    } else {
                        player.move_to(new_pos);
                        PlayerAction::Moved
                    }
                },
                map::CanMoveResponse::Scenery => {
                    PlayerAction::NoTurn
                } // Nothing to do.
            };
        },
        KeyType::Movement(_) if cur_game_state == GameState::Menu => {} // Will likely be used for menus
        KeyType::Movement(_) => {},

        KeyType::Exit           => {
            new_game_state = GameState::Exit;
        },
        KeyType::NewGame        => {
            new_game_state = GameState::NewMap;
        }
        KeyType::Other          => println!("{:?}", key),
    }

    (player_action, new_game_state)
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

    let unit_types = unit_type::load_unit_types();
    let (mut map, mut npcs, start_coord) = map::Map::init(&unit_types);

    let player_type = unit_type::UnitType::new("Player", '@', tcod::colors::WHITE);
    let mut player = units::Unit::new(start_coord, &player_type);

    let mut ui = ui::UI::new(Point{x: 0, y: PANEL_Y as i16}, SCREEN_WIDTH as i32, PANEL_HEIGHT as i32, player_type.get_max_hp() as i16);

    ui.add_message("Welcome stranger! Prepare to perish in the Tombs of the Ancient Kings.", tcod::colors::RED);

    map.update_fov(player.get_position(), FOV_RADIUS);

    let mut game_state = GameState::Playing;

    while !root.window_closed() {
        ui.update_hp_val(player.get_hp() as i16);

        render_all(&mut root, &mut buffer_console, &mut ui, &map, &npcs, &player);

        let (player_action, new_game_state) = handle_input(&mut root, game_state, &map, &mut ui, &mut npcs, &mut player);
        game_state = new_game_state;

        if player_action == PlayerAction::Moved {
            map.update_fov(player.get_position(), FOV_RADIUS);
        }

        match (game_state, player_action) {
            (GameState::Exit, _) => break,
            (GameState::NewMap, _) => {
                let (new_map, units, start_coord) = map::Map::init(&unit_types);
                map = new_map;
                npcs = units;
                player.move_to(start_coord);
                player.heal(255); // Just max health, whatever that is.
                map.update_fov(start_coord, FOV_RADIUS);
                game_state = GameState::Playing;
            }
            (GameState::Playing, PlayerAction::Moved) | (GameState::Playing, PlayerAction::Turn) => {
                // We have to work around the borrow checker here.
                // Because the compiler enforces that only a single mutable reference
                // exists at any one time, we can't mutably borrow the NPC *and* pass
                // in a list of all NPCs to it's take_turn.
                // Therefore we must remove the NPC from the collection before 
                // taking its turn.

                for _ in 0..npcs.len() {
                    let mut enemy = npcs.pop_front().expect("Failed to deque NPC");
                    
                    if enemy.get_hp() > 0 {
                        enemy.take_turn(&map, &mut ui, &npcs, &mut player);

                        if player.get_hp() == 0 {
                            ui.add_message("You died!", tcod::colors::LIGHT_RED);
                            let corpse = item::Item::new(&player.get_name(), player.get_glyph(), tcod::colors::DARK_RED, player.get_position());
                            map.place_item(corpse);
                            game_state = GameState::Dead;
                            break;
                        }

                        // Must re-add the enemy to the NPC list, or it'll be lost.
                        npcs.push_back(enemy);
                    } else {
                        let corpse = item::Item::new(&enemy.get_name(), enemy.get_glyph(), tcod::colors::DARK_RED, enemy.get_position());

                        map.place_item(corpse);
                        ui.add_message(&format!("{} is dead!", enemy.get_name()), tcod::colors::WHITE);
                    }
                }
            }
            _ => {} // Don't update AI.
        }
    }
}
