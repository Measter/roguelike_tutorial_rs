use std;
use std::ops::Range;
use std::collections::HashSet;

use rand;
use rand::Rng;

use tcod;
use tcod::console::Console;
use tcod::Color;

use traits::{Position, Renderable};
use units::Unit;

use point::Point;
use rectangle::Rectangle;

use SCREEN_WIDTH;
use SCREEN_HEIGHT;

const MAP_MIN_WIDTH: u8 = SCREEN_WIDTH;
const MAP_MIN_HEIGHT: u8 = SCREEN_HEIGHT;
pub const MAP_MAX_WIDTH: u8 = std::u8::MAX;
pub const MAP_MAX_HEIGHT: u8 = std::u8::MAX;

const ROOM_MAX_SIZE: u8 = 10;
const ROOM_MIN_SIZE: u8 = 6;
// This value determines the maximum number of rooms for a given map size.
// This was calculated by the previous map size (80 x 45) divided by the
// previous max room count of 30.
// This should provide a similar room densiter for each map.
const ROOM_PER_TILE: u8 = 120;

const ERR_MSG_TUNNEL: &str = "Failed to create tunnel.";
const ERR_MSG_ROOM: &str = "Failed to create room.";
const ERR_MSG_ROOM_CMP: &str = "Error comparing rooms.";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

impl TileType {
    fn get_color_not_visible(self) -> Color {
        match self {
            TileType::Floor => Color::new(50, 50, 150),
            TileType::Wall => Color::new(0, 0, 100),
        }
    }

    fn get_color_visible(self) -> Color {
        match self {
            TileType::Floor => Color::new(200, 180, 50),
            TileType::Wall => Color::new(130, 110, 50),
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
    position: Point<i16>,
    tile_type: TileType,
    is_explored: bool,
    is_visible: bool,
}

impl Position for Tile {
    fn get_x(&self) -> i16 {
        self.position.x
    }

    fn get_y(&self) -> i16 {
        self.position.y
    }
}

impl Renderable for Tile {
    fn get_color(&self) -> Color {
        if !self.is_explored {
            tcod::colors::BLACK
        } else {        
            match self.is_visible {
                true => self.tile_type.get_color_visible(),
                false => self.tile_type.get_color_not_visible(),
            }
        }
    }
    fn get_glyph(&self) -> char {
        self.tile_type.get_glyph()
    }
}

impl Tile {
    pub fn new(pos: Point<i16>, tile_type: TileType) -> Tile {
        Tile{
            position: pos,
            tile_type: tile_type,
            is_explored: false,
            is_visible: false,
        }
    }
}


pub struct Map {
    width: u8,
    height: u8,
    tile_map: Vec<Tile>,
    fov_map: tcod::map::Map,
    npcs: Vec<Unit>,
}

impl Map {
    pub fn init() -> (Map, Point<i16>) {
        let mut rng = rand::thread_rng();

        let map_width = rng.gen_range(MAP_MIN_WIDTH, MAP_MAX_WIDTH);
        let map_height = rng.gen_range(MAP_MIN_HEIGHT, MAP_MAX_HEIGHT);

        let mut map = vec![];
        for y in 0..map_height {
            for x in 0..map_width {
                map.push( Tile::new(Point{x: x as i16, y: y as i16}, TileType::Wall) );
            }
        }

        let mut map = Map {
            width: map_width,
            height: map_height,
            tile_map: map,
            fov_map: tcod::map::Map::new(map_width as i32, map_height as i32),
            npcs: vec![],
        };

        // Set all tiles to be not walkable, not transparent.
        // We'll be setting these in the build_rooms and build_coridoors methods.
        map.fov_map.clear(false, false);

        let (rooms, player_start) = map.build_rooms(&mut rng);
        map.build_coridoors(&rooms, &mut rng);

        (map, player_start)
    }

    fn build_rooms(&mut self, rng: &mut rand::ThreadRng) -> (Vec<Rectangle>, Point<i16>) {
        let mut rooms = vec![];
        let mut player_start = Point{x:0, y:0};

        // Casts are to avoid overflow.
        let max_rooms = self.width as u16 * self.height as u16 / ROOM_PER_TILE as u16;

        for _ in 0..max_rooms {
            let width = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
            let height = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);

            let room = Rectangle::new(
                Point {
                    x: rng.gen_range(0, self.width as i16 - width as i16 - 1),
                    y: rng.gen_range(0, self.height as i16 - height as i16 - 1)
                },
                (width, height)
            );

            if !rooms.iter().any(|r: &Rectangle| r.is_intersecting(&room)) {
                self.create_room(&room).expect(ERR_MSG_ROOM);

                if rooms.len() == 0 {
                    player_start = room.centre();
                }

                rooms.push(room);
            }
        }

        (rooms, player_start)
    }

    fn build_coridoors(&mut self, rooms: &Vec<Rectangle>, rng: &mut rand::ThreadRng) {
        let mut sorted_rooms: Vec<_> = rooms.iter().map(|r| r).collect();
        let mut connected_rooms = HashSet::new();
        

        for room in rooms {
            sorted_rooms.sort_by(|a,b| (room.centre() - a.centre()).sqr_radius().partial_cmp(&(room.centre() - b.centre()).sqr_radius()).expect(ERR_MSG_ROOM_CMP) );

            for next in sorted_rooms.iter().skip(1).take(3) {
                if connected_rooms.contains(&(*next, room)) {
                    continue;
                }

                connected_rooms.insert((room, *next));

                let r1_centre = room.centre();
                let r2_centre = next.centre();

                // Decide whether to first tunnel horizontally or vertically.
                if rng.gen_weighted_bool(2) {
                    // Horizontal
                    self.create_h_tunnel(r1_centre.x..r2_centre.x, r1_centre.y).expect(ERR_MSG_TUNNEL);
                    self.create_v_tunnel(r2_centre.x, r1_centre.y..r2_centre.y).expect(ERR_MSG_TUNNEL);
                } else {
                    // Vertical
                    self.create_v_tunnel(r1_centre.x, r1_centre.y..r2_centre.y).expect(ERR_MSG_TUNNEL);
                    self.create_h_tunnel(r1_centre.x..r2_centre.x, r2_centre.y).expect(ERR_MSG_TUNNEL);
                }
            }
        }
    }

    pub fn render_map<T: Console>(&self, cons: &mut T) {
        for tile in self.tile_map.iter() {
            tile.render(cons);
        }
    }

    pub fn render_npcs<T: Console>(&self, cons: &mut T) {
        for unit in self.npcs.iter() {
            if self.fov_map.is_in_fov(unit.get_x() as i32, unit.get_y() as i32) {
                unit.render(cons);
            }
        }
    }

    pub fn point_in_map(&self, Point{x,y}: Point<i16>) -> bool {
        x >= 0 && x < self.width as i16 && y >= 0 && y < self.height as i16
    }

    pub fn get_tile_type(&self, pos: Point<i16>) -> Result<TileType,()> {
        if !self.point_in_map(pos) {
            Err(())
        } else {
            let Point{x, y} = pos;
            Ok(self.tile_map[y as usize * self.width as usize + x as usize].tile_type)
        }
    }

    pub fn update_fov(&mut self, Point{x, y}: Point<i16>, light_radius: u8) {
        self.fov_map.compute_fov(x as i32, y as i32, light_radius as i32, true, tcod::map::FovAlgorithm::Permissive0);

        // I've opted to update the tile map here, because it doesn't make sense that
        // a function for rendering should need to mutate the object.
        // It does make this function more expensive to run, but it won't be run
        // too frequently.
        for tile in self.tile_map.iter_mut() {
            tile.is_visible = self.fov_map.is_in_fov(tile.get_x() as i32, tile.get_y() as i32);

            if tile.is_visible {
                tile.is_explored = true;
            }
        }
    }

    pub fn get_map_size(&self) -> (u8, u8) {
        (self.width, self.height)
    }

    fn create_v_tunnel(&mut self, x: i16, y_r: Range<i16>) -> Result<(),()> {
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
            self.fov_map.set(x as i32, y as i32, true, true);
        }

        Ok(())
    }

    fn create_h_tunnel(&mut self, x_r: Range<i16>, y: i16) -> Result<(),()> {
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
            self.fov_map.set(x as i32, y as i32, true, true);
        }

        Ok(())
    }

    fn create_room(&mut self, rect: &Rectangle) -> Result<(),()> {
        for y in (rect.top_left.y+1)..rect.bottom_right.y {
            for x in (rect.top_left.x+1)..rect.bottom_right.x {
                let pos = Point{x:x, y:y};
                self.set_tile_type(pos, TileType::Floor)?;
                self.fov_map.set(x as i32, y as i32, true, true);
            }
        }

        Ok(())
    }

    fn set_tile_type(&mut self, pos: Point<i16>, new_tile: TileType) -> Result<(),()> {
        if !self.point_in_map(pos) {
            Err(())
        } else {
            let Point{x,y} = pos;
            self.tile_map[y as usize * self.width as usize + x as usize].tile_type = new_tile;

            Ok(())
        }
    }
}