use sfml::system::*;

pub struct HexEvent {
    pub position: Option<Vector2i>,
    pub name: String,
}
