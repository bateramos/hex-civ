use sfml::system::Vector2u;
use sfml::graphics::Sprite;

use rand;

#[derive(Debug, Eq, PartialEq)]
pub enum HexImprovementType {
    FARM, MINE
}

pub struct HexImprovement<'a> {
    pub id: i32,
    pub position: Vector2u,
    pub sprite: Option<Sprite<'a>>,
    pub improvement_type: HexImprovementType,
}

impl <'a> HexImprovement <'a> {
    pub fn new_with_type(position: Vector2u, improvement_type: HexImprovementType) -> HexImprovement<'a> {
        HexImprovement {
            id: rand::random::<i32>(),
            position,
            sprite: None,
            improvement_type,
        }
    }

    pub fn new(position: Vector2u) -> HexImprovement<'a> {
        HexImprovement::new_with_type(position, HexImprovementType::FARM)
    }
}
