use sfml::{graphics::*, system::*};

use crate::HexagonColumn;

pub fn init_city_sprites<'a>(y_offset: f32) -> Box<dyn Fn(&'a HexagonColumn, &'a Sprite) -> Vec<Sprite<'a>>> {
    Box::new(move |hexagons, city_sprite| {
        let mut cities = Vec::new();

        let mut city = city_sprite.clone();
        let hexagon = hexagons[3][3];
        city.set_position(Vector2f { x: hexagon.position.x, y: hexagon.position.y - y_offset});

        cities.push(city);

        cities.to_vec()
    })
}

