use tcod::console::{Console, BackgroundFlag};

use traits::{Renderable, Movable, Position};
use Direction;

#[derive(Debug)]
pub struct Unit {
    x: u8,
    y: u8,
    glyph: char,
}

impl Unit {
    pub fn new(x: u8, y: u8, glyph: char) -> Unit {
        Unit {
            x: x,
            y: y,
            glyph: glyph,
        }
    }
}

impl Position for Unit {
    fn get_x(&self) -> u8 {
        self.x
    }

    fn get_y(&self) -> u8 {
        self.y
    }
}

impl Renderable for Unit {
    fn render<T: Console>(&self, cons: &mut T) {
        cons.put_char(self.x as i32, self.y as i32, self.glyph, BackgroundFlag::None);
    }
}

impl Movable for Unit {
    fn move_to(&mut self, x: u8, y: u8){
        self.x = x;
        self.y = y;
    }
    fn nudge(&mut self, dir: Direction){
        match dir {
            Direction::Up =>     self.y -= 1,
            Direction::Down =>   self.y += 1,
            Direction::Left =>   self.x -= 1,
            Direction::Right =>  self.x += 1,
        }
    }
}