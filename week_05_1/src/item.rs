use tcod::colors::{Color};

use traits::{Renderable, Movable, Position};
use point::Point;

#[derive(Debug)]
pub struct Item {
    name: String,
    glyph: char,
    color: Color,
    position: Point<i16>,
}

impl Item {
    pub fn new(name: &str, glyph: char, color: Color, pos: Point<i16>) -> Item {
        Item {
            name: name.into(),
            glyph: glyph,
            color: color,
            position: pos,
        }
    }
}

impl Position for Item {
    fn get_x(&self) -> i16 {
        self.position.x
    }

    fn get_y(&self) -> i16 {
        self.position.y
    }
}

impl Renderable for Item {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_glyph(&self) -> char {
        self.glyph
    }
}