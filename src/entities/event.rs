use sfml::system::*;

#[derive(Debug)]
pub struct HexEvent {
    pub position: Option<Vector2i>,
    pub name: String,
}

impl HexEvent {
    pub fn new(name: &str) -> HexEvent {
        HexEvent { position: None, name: name.to_owned() }
    }
}
