use tcod::colors::{Color};

use rand;
use rand::Rng;
use rand::distributions::{Weighted};

use serde_yaml;

use std::fs::File;
use std::path::Path;

const ERR_UNIT_LOAD: &str = "Unable to load unit type data.";

#[derive(Debug, PartialEq)]
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_glyph(&self) -> char {
        self.glyph
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_max_hp(&self) -> u8 {
        self.max_hp
    }

    pub fn get_defence(&self) -> u8 {
        self.defence
    }

    pub fn get_attack(&self) -> u8 {
        self.attack
    }

    pub fn get_is_blocking(&self) -> bool {
        self.is_blocking
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