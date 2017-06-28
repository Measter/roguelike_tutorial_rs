use tcod::console::Console;
use tcod::Color;
use tcod::colors;

use traits::{Position, Renderable};
use units::Unit;
use {SCREEN_HEIGHT, SCREEN_WIDTH, PANEL_HEIGHT};

const MAP_WIDTH: u8 = SCREEN_WIDTH;
const MAP_HEIGHT: u8 = SCREEN_HEIGHT - PANEL_HEIGHT;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

impl TileType {
    fn get_color(self) -> Color {
        match self {
            TileType::Floor => Color::new(50, 50, 150),
            TileType::Wall => Color::new(0, 0, 100),
        }
    }

    fn get_glyph(self) -> char {
        match self {
            TileType::Floor => '.',
            TileType::Wall => '#',
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    x: u8,
    y: u8,
    tile_type: TileType,
}

impl Position for Tile {
    fn get_x(&self) -> u8 {
        self.x
    }

    fn get_y(&self) -> u8 {
        self.y
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
    pub fn new((x, y): (u8, u8), tile_type: TileType) -> Tile {
        Tile{
            x: x,
            y: y,
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
                map.push( Tile::new((x,y), TileType::Floor) );
            }
        }
        
        Map {
            tile_map: map,
            npcs: vec![Unit::new(5, 5, '@', colors::YELLOW)],
        }
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

    pub fn get_tile_type(&self, (x,y): (u8, u8)) -> Result<TileType,()> {
        if x >= MAP_WIDTH || y >= MAP_HEIGHT {
            Err(())
        } else {
            Ok(self.tile_map[y as usize * MAP_HEIGHT as usize + x as usize].tile_type)
        }
    }
}