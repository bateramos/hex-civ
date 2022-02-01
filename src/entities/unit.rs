use sfml::system::Vector2i;
use sfml::graphics::Sprite;

use rand;

#[derive(Clone, Copy, Debug)]
pub enum UnitType {
    Pikeman,
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
}
