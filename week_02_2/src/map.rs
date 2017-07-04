use tcod::console::Console;
use tcod::Color;
use tcod::colors;

use traits::{Position, Renderable};
use units::Unit;
use {SCREEN_HEIGHT, SCREEN_WIDTH, PANEL_HEIGHT};

use point::Point;

const MAP_WIDTH: i8 = SCREEN_WIDTH;
const MAP_HEIGHT: i8 = SCREEN_HEIGHT - PANEL_HEIGHT;

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
    pub fn init() -> Map {
        let mut map = vec![];
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                map.push( Tile::new(Point{x: x, y: y}, TileType::Floor) );
            }
        }

        let mut map = Map {
            tile_map: map,
            npcs: vec![Unit::new(Point{x: 5, y: 5}, '@', colors::YELLOW)],
        };

        map.set_tile_type(Point{x: 5, y: 3}, TileType::Wall).expect("Failed to set tile.");

        map
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