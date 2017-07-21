use tcod;
use tcod::{BackgroundFlag, TextAlignment};
use tcod::console::{Console, Offscreen};
use tcod::colors::Color;

use traits::{Position, Renderable};
use point::Point;

pub struct UI {
    position: Point<i16>,
    width: i32,
    height: i32,
    panel: Offscreen,
    bar_hp: Bar,
}

impl UI {
    pub fn new(pos: Point<i16>, panel_width: i32, panel_height: i32, max_hp: i16) -> UI {
        UI {
            position: pos,
            width: panel_width,
            height: panel_height,
            panel: Offscreen::new(panel_width, panel_height),
            bar_hp: Bar::new(Point{x: 0, y: 0}, 20, "HP", max_hp, tcod::colors::DARKER_RED, tcod::colors::LIGHT_RED)
        }
    }

    pub fn update_hp_val(&mut self, new_val: i16) {
        self.bar_hp.set_value(new_val);
    }

    pub fn render<T: Console>(&mut self, cons: &mut T) {
        self.bar_hp.render(&mut self.panel);

        tcod::console::blit(&self.panel, (0,0), (self.width, self.height), cons, (self.position.x as i32, self.position.y as i32), 1.0, 1.0);
    }
}


#[derive(Debug)]
pub struct Bar {
    position: Point<i16>,
    width: i16,
    name: String,
    value_max: i16,
    value_cur: i16,
    color_bar: Color,
    color_background: Color,
}

impl Bar {
    pub fn new(pos: Point<i16>, width: i16, name: &str, max_val: i16, col_back: Color, col_bar: Color) -> Bar {
        Bar {
            position: pos,
            width: width,
            name: name.into(),
            value_max: max_val,
            value_cur: max_val,
            color_bar: col_bar,
            color_background: col_back,
        }
    }

    pub fn set_value(&mut self, new_val: i16) {
        assert!(new_val <= self.value_max);
        self.value_cur = new_val;
    }
}

impl Position for Bar {
    fn get_x(&self) -> i16 {
        self.position.x
    }
    
    fn get_y(&self) -> i16 {
        self.position.y
    }
}

impl Renderable for Bar {
    fn get_color(&self) -> Color {
        self.color_bar
    }

    fn get_glyph(&self) -> char {
        '-'
    }

    fn render<T: Console>(&self, cons: &mut T) {
        let bar_width = (self.value_cur * self.width)/self.value_max;

        cons.set_default_background(self.color_background);
        cons.rect(self.position.x as i32, self.position.y as i32, self.width as i32, 1, false, BackgroundFlag::Set );

        if bar_width > 0 {
            cons.set_default_background(self.color_bar);
            cons.rect(self.position.x as i32, self.position.y as i32, bar_width as i32, 1, false, BackgroundFlag::Set );
        }

        cons.set_default_foreground(tcod::colors::WHITE);
        cons.print_ex((self.position.x + self.width/2) as i32, self.position.y as i32, BackgroundFlag::None, TextAlignment::Center, format!("{}: {}/{}", self.name, self.value_cur, self.value_max));
    }
}