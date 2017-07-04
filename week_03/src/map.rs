use std::ops::Range;

use rand;
use rand::Rng;

use tcod::console::Console;
use tcod::Color;
use tcod::colors;

use traits::{Position, Renderable};
use units::Unit;
use {SCREEN_HEIGHT, SCREEN_WIDTH, PANEL_HEIGHT};

use point::Point;
use rectangle::Rectangle;

const MAP_WIDTH: i8 = SCREEN_WIDTH;
const MAP_HEIGHT: i8 = SCREEN_HEIGHT - PANEL_HEIGHT;

const ROOM_MAX_SIZE: u8 = 10;
const ROOM_MIN_SIZE: u8 = 6;
const ROOM_MAX_COUNT: u8 = 30;

const ERR_MSG_TUNNEL: &str = "Failed to create tunnel.";
const ERR_MSG_ROOM: &str = "Failed to create room.";
const ERR_MSG_WINDOW: &str = "Should have two rooms.";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

impl TileType {
    fn get_color(self) -> Color {
        match self {
            TileType::Floor => Color::new(50, 50, 150),
            TileType::Wall => Color::new(240, 240, 240),
        }
    }

    fn get_glyph(self) -> char {
        match self {
            TileType::Floor => '.',
            TileType::Wall => '#',
        }
    }

    pub fn blocks_move(self) -> bool {
        match self {
            TileType::Floor => false,
            TileType::Wall => true,
        }
    }

    pub fn blocks_sight(self) -> bool {
        match self {
            TileType::Floor => false,
            TileType::Wall => true,
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    position: Point<i8>,
    tile_type: TileType,
}

impl Position for Tile {
    fn get_x(&self) -> i8 {
        self.position.x
    }

    fn get_y(&self) -> i8 {
        self.position.y
    }
}

impl Renderable for Tile {
    fn get_color(&self) -> Color {
        self.tile_type.get_color()
    }
    fn get_glyph(&self) -> char {
        self.tile_type.get_glyph()
    }
}

impl Tile {
    pub fn new(pos: Point<i8>, tile_type: TileType) -> Tile {
        Tile{
            position: pos,
            tile_type: tile_type
        }
    }
}


#[derive(Debug)]
pub struct Map {
    tile_map: Vec<Tile>,
    npcs: Vec<Unit>,
}

impl Map {
    pub fn init() -> (Map, Point<i8>) {
        let mut map = vec![];
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                map.push( Tile::new(Point{x: x, y: y}, TileType::Wall) );
            }
        }

        let mut map = Map {
            tile_map: map,
            npcs: vec![Unit::new(Point{x: 52, y: 18}, '@', colors::YELLOW)],
        };

        let mut rng = rand::thread_rng();
        let mut rooms = vec![];

        let mut player_start = Point{x:0, y:0};

        for _ in 0..ROOM_MAX_COUNT {
            let width = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
            let height = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);

            let room = Rectangle::new(
                Point {
                    x: rng.gen_range(0, MAP_WIDTH - width as i8 - 1),
                    y: rng.gen_range(0, MAP_HEIGHT - height as i8 - 1)
                },
                (width, height)
            );

            if !rooms.iter().any(|r: &Rectangle| r.is_intersecting(&room)) {
                map.create_room(&room).expect(ERR_MSG_ROOM);

                if rooms.len() == 0 {
                    player_start = room.centre();
                }

                rooms.push(room);
            }
        }

        for pair in rooms.windows(2) {
            let r1_centre = pair.get(0).expect(ERR_MSG_WINDOW).centre();
            let r2_centre = pair.get(1).expect(ERR_MSG_WINDOW).centre();

            // Decide whether to first tunnel horizontally or vertically.
            if rng.gen_weighted_bool(2) {
                // Horizontal
                map.create_h_tunnel(r1_centre.x..r2_centre.x, r1_centre.y).expect(ERR_MSG_TUNNEL);
                map.create_v_tunnel(r2_centre.x, r1_centre.y..r2_centre.y).expect(ERR_MSG_TUNNEL);
            } else {
                // Vertical
                map.create_v_tunnel(r1_centre.x, r1_centre.y..r2_centre.y).expect(ERR_MSG_TUNNEL);
                map.create_h_tunnel(r1_centre.x..r2_centre.x, r2_centre.y).expect(ERR_MSG_TUNNEL);
            }
        }

        (map, player_start)
    }

    pub fn render_map<T: Console>(&self, cons: &mut T) {
        for tile in self.tile_map.iter() {
            tile.render(cons);
        }
    }

    pub fn render_npcs<T: Console>(&self, cons: &mut T) {
        for unit in self.npcs.iter() {
            unit.render(cons);
        }
    }

    pub fn point_in_map(&self, Point{x,y}: Point<i8>) -> bool {
        x >= 0 && x < MAP_WIDTH && y >= 0 && y < MAP_HEIGHT
    }

    pub fn get_tile_type(&self, pos: Point<i8>) -> Result<TileType,()> {
        if !self.point_in_map(pos) {
            Err(())
        } else {
            let Point{x, y} = pos;
            Ok(self.tile_map[y as usize * MAP_WIDTH as usize + x as usize].tile_type)
        }
    }

    fn create_v_tunnel(&mut self, x: i8, y_r: Range<i8>) -> Result<(),()> {
        // Need to handle the case where start > end.
        // I decided to do it here to make it easier to call the function.
        // Because this is a half-open range, we need to adjust the start
        // and end to account for that.
        let y_r = if y_r.start > y_r.end {
            y_r.end+1 .. y_r.start+1
        } else {
            y_r
        };
        
        for y in y_r {
            let pos = Point{x:x, y:y};
            self.set_tile_type(pos, TileType::Floor)?;
        }

        Ok(())
    }

    fn create_h_tunnel(&mut self, x_r: Range<i8>, y: i8) -> Result<(),()> {
        // Need to handle the case where start > end.
        // I decided to do it here to make it easier to call the function.
        // Because this is a half-open range, we need to adjust the start
        // and end to account for that.
        let x_r = if x_r.start > x_r.end {
            x_r.end+1 .. x_r.start+1
        } else {
            x_r
        };
        
        for x in x_r {
            let pos = Point{x:x, y:y};
            self.set_tile_type(pos, TileType::Floor)?;
        }

        Ok(())
    }

    fn create_room(&mut self, rect: &Rectangle) -> Result<(),()> {
        for y in (rect.top_left.y+1)..rect.bottom_right.y {
            for x in (rect.top_left.x+1)..rect.bottom_right.x {
                let pos = Point{x:x, y:y};
                self.set_tile_type(pos, TileType::Floor)?;
            }
        }

        Ok(())
    }

    fn set_tile_type(&mut self, pos: Point<i8>, new_tile: TileType) -> Result<(),()> {
        if !self.point_in_map(pos) {
            Err(())
        } else {
            let Point{x,y} = pos;
            self.tile_map[y as usize * MAP_WIDTH as usize + x as usize].tile_type = new_tile;

            Ok(())
        }
    }
}