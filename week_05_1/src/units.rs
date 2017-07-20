use tcod::colors::{Color};

use traits::{Renderable, Movable, Position};
use Direction;
use point::Point;
use map;
use map::Map;
use unit_type::UnitType;

use std::cmp::min;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AttackResult {
    Dead,
    Alive,
    NoEffect,
}

#[derive(Debug)]
pub struct Unit<'a> {
    position: Point<i16>,
    unit_type: &'a UnitType,
    cur_hp: u8,
}

impl<'a> Unit<'a> {
    pub fn new(pos: Point<i16>, unit_type: &'a UnitType) -> Unit<'a> {
        let cur_hp = unit_type.get_max_hp();
        Unit {
            position: pos,
            unit_type: unit_type,
            cur_hp: cur_hp,
        }
    }

    pub fn is_blocking(&self) -> bool {
        self.unit_type.get_is_blocking()
    }

    pub fn get_name(&self) -> &str {
        &self.unit_type.get_name()
    }

    pub fn get_hp(&self) -> u8 {
        self.cur_hp
    }

    fn get_step_towards(&mut self, target: Point<i16>) -> Point<i16> {
        let cur_pos = self.get_position();
        let delta = target - cur_pos;
        let dist = delta.radius();

        let new_pos = Point {
            x: (delta.x as f64 / dist).round() as i16,
            y: (delta.y as f64 / dist).round() as i16,
        };
        new_pos + cur_pos
    }

    pub fn take_turn(&mut self, map: &Map, player: &mut Unit) {
        if !map.point_in_fov(self.get_position()) {
            return;
        }

        if (self.get_position() - player.get_position()).radius() >= 2.0 {
            let new_pos = self.get_step_towards(player.get_position());
            if map.can_move_to(new_pos) == map::CanMoveResponse::Open {
                self.move_to(new_pos);
            }
        } else if player.get_hp() > 0 {
            self.attack(player);
        }
    }

    pub fn take_damage(&mut self, damage: u8) -> AttackResult {
        if let Some(new_hp) = self.cur_hp.checked_sub(damage) {
            self.cur_hp = new_hp;
            AttackResult::Alive
        } else {
            self.cur_hp = 0;
            AttackResult::Dead
        }
    }

    pub fn attack(&self, target: &mut Unit) -> AttackResult {
        if let Some(damage) = self.unit_type.get_attack().checked_sub(target.unit_type.get_defence()) {
            println!("{} attacks {} for {} damage.", self.get_name(), target.get_name(), damage);
            target.take_damage(damage)
        } else {
            println!("{} attacks {}, but it has no effect!", self.get_name(), target.get_name());
            AttackResult::NoEffect
        }
    }

    pub fn heal(&mut self, amount: u8) {
        self.cur_hp = min(amount, self.unit_type.get_max_hp());
    }
}

impl<'a> Position for Unit<'a> {
    fn get_x(&self) -> i16 {
        self.position.x
    }

    fn get_y(&self) -> i16 {
        self.position.y
    }
}

impl<'a> Renderable for Unit<'a> {
    fn get_color(&self) -> Color {
        self.unit_type.get_color()
    }

    fn get_glyph(&self) -> char {
        self.unit_type.get_glyph()
    }
}

impl<'a> Movable for Unit<'a> {
    fn move_to(&mut self, pos: Point<i16>){
        self.position = pos;
    }
    fn nudge(&mut self, dir: Direction){
        self.position = self.position + dir.to_rel_point();
    }
}