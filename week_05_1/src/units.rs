use tcod::colors::{Color};

use traits::{Renderable, Movable, Position};
use Direction;
use point::Point;
use map;
use map::Map;

use std::fs::File;
use std::path::Path;
use std::cmp::min;

use rand;
use rand::Rng;
use rand::distributions::{Weighted};

use serde_yaml;

const ERR_UNIT_LOAD: &str = "Unable to load unit type data.";

#[derive(Debug)]
pub struct UnitType {
    name: String,
    glyph: char,
    color: Color,
    is_blocking: bool,
    max_hp: u8,
    defence: u8,
    attack: u8,
}

impl UnitType {
    // Only used for the player for now.
    pub fn new(name: &str, glyph: char, color: Color) -> UnitType {
        UnitType {
            name: name.into(),
            glyph: glyph,
            color: color,
            is_blocking: true,
            // Following based on standard human in the data files.
            max_hp: 30,
            defence: 3,
            attack: 5,
        }
    }
}

impl<'a> From<&'a UnitTypeRaw> for UnitType {
    fn from(raw: &'a UnitTypeRaw) -> UnitType {
        UnitType {
            name: raw.name.clone(),
            glyph: raw.glyph,
            color: Color::new(raw.color[0], raw.color[1], raw.color[2]),
            is_blocking: raw.is_blocking,
            max_hp: raw.max_hp,
            defence: raw.defence,
            attack: raw.attack,
        }
    }
}

// The Color type in tcod isn't deserializable, so have this to make reading
// the data file easier.
#[derive(Debug, Deserialize)]
struct UnitTypeRaw {
    name: String,
    glyph: char,
    color: [u8; 3],
    chance: u32,
    is_blocking: bool,
    max_hp: u8,
    defence: u8,
    attack: u8,
}

pub struct UnitTypeLists {
    pub types: Vec<UnitType>,
    weights: Vec<Weighted<usize>>,
    max_weight: u32,
}

impl UnitTypeLists {
    pub fn get_random_type(&self, rng: &mut rand::ThreadRng) -> &UnitType {
        let mut val = rng.gen_range(0, self.max_weight);

        for weight in self.weights.iter() {
            if weight.weight > val {
                return &self.types[weight.item];
            }

            val -= weight.weight;
        }

        unreachable!()
    }
}

pub fn load_unit_types() -> UnitTypeLists{
    let path = Path::new("data").join("unit_types.yaml");
    let data_file = File::open(&path).expect(ERR_UNIT_LOAD);
    let raw_units: Vec<UnitTypeRaw> = serde_yaml::from_reader(&data_file).expect(ERR_UNIT_LOAD);

    let mut types = vec![];
    let mut weights = vec![];

    let mut running_total = 0;
    for (i, raw_unit) in raw_units.iter().enumerate() {
        types.push(raw_unit.into());
        running_total += raw_unit.chance;

        weights.push( Weighted{ weight: raw_unit.chance, item: i });
    }

    UnitTypeLists {
        types: types,
        weights: weights,
        max_weight: running_total,
    }
}


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
        let cur_hp = unit_type.max_hp;
        Unit {
            position: pos,
            unit_type: unit_type,
            cur_hp: cur_hp,
        }
    }

    pub fn is_blocking(&self) -> bool {
        self.unit_type.is_blocking
    }

    pub fn get_name(&self) -> &str {
        &self.unit_type.name
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
        if let Some(damage) = self.unit_type.attack.checked_sub(target.unit_type.defence) {
            println!("{} attacks {} for {} damage.", self.get_name(), target.get_name(), damage);
            target.take_damage(damage)
        } else {
            println!("{} attacks {}, but it has no effect!", self.get_name(), target.get_name());
            AttackResult::NoEffect
        }
    }

    pub fn heal(&mut self, amount: u8) {
        self.cur_hp = min(amount, self.unit_type.max_hp);
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
        self.unit_type.color
    }

    fn get_glyph(&self) -> char {
        self.unit_type.glyph
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