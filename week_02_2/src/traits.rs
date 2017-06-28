use tcod::console::{Console, BackgroundFlag};
use tcod::{Color};

use Direction;

pub trait Position {
    fn get_x(&self) -> u8;
    fn get_y(&self) -> u8;
    fn get_position(&self) -> (u8, u8) {
        (self.get_x(), self.get_y())
    }
}

pub trait Renderable: Position {
    fn get_color(&self) -> Color;
    fn get_glyph(&self) -> char;
    fn render<T: Console>(&self, cons: &mut T) {
        cons.set_default_foreground(self.get_color());
        cons.put_char(self.get_x() as i32, self.get_y() as i32, self.get_glyph(), BackgroundFlag::None);
    }
}

pub trait Movable: Position {
    fn move_to(&mut self, x: u8, y: u8);
    fn nudge(&mut self, dir: Direction);
}
