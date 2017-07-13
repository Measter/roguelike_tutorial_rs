use tcod::colors::{Color};

use traits::{Renderable, Movable, Position};
use Direction;
use point::Point;

use std::fs::File;
use std::path::Path;

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
}

impl UnitType {
    // Only used for the player for now.
    pub fn new(name: &str, glyph: char, color: Color) -> UnitType {
        UnitType {
            name: name.into(),
            glyph: glyph,
            color: color,
            is_blocking: true,
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
}

pub struct UnitTypeLists {
    pub types: Vec<UnitType>,
    weights: Vec<Weighted<usize>>,
    max_weight: u32,
}

impl UnitTypeLists {
    pub fn get_random_type(&self, rng: &mut rand::ThreadRng) -> &UnitType {
        let val = rng.gen_range(0, self.max_weight);

        let mut sel_index = self.weights.iter().skip_while(|x| x.weight > val);

        if let Some(i) = sel_index.next() {
            // We didn't get to the end of the list, so just return this one.
            &self.types[i.item]
        } else {
            // Just return the last item.
            &self.types[self.types.len()-1]
        }
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

        weights.push( Weighted{ weight: running_total, item: i });
    }

    UnitTypeLists {
        types: types,
        weights: weights,
        max_weight: running_total,
    }
}

#[derive(Debug)]
pub struct Unit<'a> {
    position: Point<i16>,
    unit_type: &'a UnitType,
}

impl<'a> Unit<'a> {
    pub fn new(pos: Point<i16>, unit_type: &'a UnitType) -> Unit<'a> {
        Unit {
            position: pos,
            unit_type: unit_type,
        }
    }

    pub fn is_blocking(&self) -> bool {
        self.unit_type.is_blocking
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