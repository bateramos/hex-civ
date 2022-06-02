use sfml::system::Vector2i;
use sfml::graphics::Sprite;

use rand;

pub enum HexImprovementType {
    FARM, MINE
}

pub struct HexImprovement<'a> {
    pub id: i32,
    pub position: Vector2i,
    pub sprite: Option<Sprite<'a>>,
    pub improvement_type: HexImprovementType,
}

impl <'a> HexImprovement <'a> {
    pub fn new(position: Vector2i) -> HexImprovement<'a> {
        HexImprovement {
            id: rand::random::<i32>(),
            position,
            sprite: None,
            improvement_type: HexImprovementType::FARM,
        }
    }
}
