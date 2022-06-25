use std::collections::HashMap;

use sfml::system::Vector2u;
use sfml::graphics::Sprite;

use rand;

#[derive(Clone)]
pub struct City <'a> {
    pub id: i32,
    pub position: Vector2u,
    pub sprite: Option<Sprite<'a>>,
    pub storage: HashMap<&'a str, i32>,
}

impl <'a> City <'a> {
    pub fn new(position: Vector2u) -> City<'a> {
        City {
            id: rand::random::<i32>(),
            position,
            sprite: None,
            storage: HashMap::new(),
        }
    }
}
