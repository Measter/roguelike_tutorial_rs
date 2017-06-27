use tcod::console::Console;

use Direction;

pub trait Position {
    fn get_x(&self) -> u8;
    fn get_y(&self) -> u8;
    fn get_position(&self) -> (u8, u8) {
        (self.get_x(), self.get_y())
    }
}

pub trait Renderable: Position {
    fn render<T: Console>(&self, cons: &mut T);
}

pub trait Movable: Position {
    fn move_to(&mut self, x: u8, y: u8);
    fn nudge(&mut self, dir: Direction);
}
