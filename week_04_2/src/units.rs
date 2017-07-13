use tcod::colors::{Color};

use traits::{Renderable, Movable, Position};
use Direction;
use point::Point;

use std::fs::File;
use std::path::Path;

use rand::distributions::Weighted;

use serde_yaml;

const ERR_UNIT_LOAD: &str = "Unable to load unit type data.";

#[derive(Debug)]
pub struct UnitType {
    name: String,
    glyph: char,
    color: Color,
}

impl UnitType {
    // Only used for the player for now.
    pub fn new(name: &str, glyph: char, color: Color) -> UnitType {
        UnitType {
            name: name.into(),
            glyph: glyph,
            color: color,
        }
    }
}

impl<'a> From<&'a UnitTypeRaw> for UnitType {
    fn from(raw: &'a UnitTypeRaw) -> UnitType {
        UnitType {
            name: raw.name.clone(),
            glyph: raw.glyph,
            color: Color::new(raw.color[0], raw.color[1], raw.color[2]),
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
}

pub struct UnitTypeLists {
    pub types: Vec<UnitType>,
    pub weights: Vec<Weighted<usize>>,
}

pub fn load_unit_types() -> UnitTypeLists {
    let path = Path::new("data").join("unit_types.yaml");
    let data_file = File::open(&path).expect(ERR_UNIT_LOAD);
    let raw_units: Vec<UnitTypeRaw> = serde_yaml::from_reader(&data_file).expect(ERR_UNIT_LOAD);

    let mut unit_list = UnitTypeLists {
        types: vec![],
        weights: vec![],
    };

    for (i, raw_unit) in raw_units.iter().enumerate() {
        unit_list.types.push(raw_unit.into());
        unit_list.weights.push( Weighted{ weight: raw_unit.chance, item: i });
    }

    unit_list
}

#[derive(Debug)]
pub struct Unit {
    position: Point<i16>,
    unit_type: UnitType,
}

impl Unit {
    pub fn new(pos: Point<i16>, unit_type: UnitType) -> Unit {
        Unit {
            position: pos,
            unit_type: unit_type,
        }
    }
}

impl Position for Unit {
    fn get_x(&self) -> i16 {
        self.position.x
    }

    fn get_y(&self) -> i16 {
        self.position.y
    }
}

impl Renderable for Unit {
    fn get_color(&self) -> Color {
        self.unit_type.color
    }

    fn get_glyph(&self) -> char {
        self.unit_type.glyph
    }
}

impl Movable for Unit {
    fn move_to(&mut self, pos: Point<i16>){
        self.position = pos;
    }
    fn nudge(&mut self, dir: Direction){
        self.position = self.position + dir.to_rel_point();
    }
}