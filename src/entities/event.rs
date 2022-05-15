use sfml::system::*;

use crate::Unit;

#[derive(Debug)]
pub struct HexEvent {
    pub position: Option<Vector2i>,
    pub name: String,
    pub unit_id: Option<i32>,
}

impl HexEvent {
    pub fn new(name: &str) -> HexEvent {
        HexEvent { position: None, name: name.to_owned(), unit_id: None }
    }

    pub fn new_from_position(name: &str, position: Vector2i) -> HexEvent {
        HexEvent { position: Some(position), name: name.to_owned(), unit_id: None }
    }

    pub fn new_from_unit(name: &str, unit: &Unit) -> HexEvent {
        HexEvent { position: Some(unit.position), name: name.to_owned(), unit_id: Some(unit.id) }
    }
}
