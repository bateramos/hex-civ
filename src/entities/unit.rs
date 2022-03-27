use sfml::system::Vector2i;
use sfml::graphics::Sprite;

use rand;

#[derive(Clone, Copy, Debug)]
pub enum UnitType {
    Pikeman, Settler
}

#[derive(Clone, Debug)]
pub struct Unit <'a> {
    pub id: i32,
    pub unit_type: UnitType,
    pub position: Vector2i,
    pub sprite: Option<Sprite<'a>>,
}

impl <'a> Unit <'a> {
    pub fn new(position: Vector2i) -> Unit<'a> {
        Unit {
            id: rand::random::<i32>(),
            unit_type: UnitType::Pikeman,
            position,
            sprite: None,
        }
    }
    pub fn new_with_type(position: Vector2i, unit_type: UnitType) -> Unit<'a> {
        let mut unit = Unit::new(position);
        unit.unit_type = unit_type;

        unit
    }
}
