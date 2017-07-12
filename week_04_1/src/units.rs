use tcod::colors::{Color};

use traits::{Renderable, Movable, Position};
use Direction;
use point::Point;

#[derive(Debug)]
pub struct Unit {
    position: Point<i8>,
    glyph: char,
    color: Color,
}

impl Unit {
    pub fn new(pos: Point<i8>, glyph: char, color: Color) -> Unit {
        Unit {
            position: pos,
            glyph: glyph,
            color: color,
        }
    }
}

impl Position for Unit {
    fn get_x(&self) -> i8 {
        self.position.x
    }

    fn get_y(&self) -> i8 {
        self.position.y
    }
}

impl Renderable for Unit {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_glyph(&self) -> char {
        self.glyph
    }
}

impl Movable for Unit {
    fn move_to(&mut self, pos: Point<i8>){
        self.position = pos;
    }
    fn nudge(&mut self, dir: Direction){
        self.position = self.position + dir.to_rel_point();
    }
}