use sfml::system::Vector2i;
use sfml::graphics::Sprite;

#[derive(Clone, Copy, Debug)]
pub enum UnitType {
    Pikeman,
}

#[derive(Clone, Debug)]
pub struct Unit <'a> {
    pub unit_type: UnitType,
    pub position: Vector2i,
    pub sprite: Option<Sprite<'a>>,
}
