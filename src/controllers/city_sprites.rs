use sfml::{graphics::*, system::*, window::*};

use crate::{Hexagon, HexagonCategory, HexagonColumn};

pub fn init_city_sprites<'a>() -> Box<dyn Fn(&'a HexagonColumn, &'a Sprite) -> Vec<Sprite<'a>>> {

    Box::new(|hexagons, city_sprite| {
        let mut cities = Vec::new();

        let mut city = city_sprite.clone();
        let mut hexagon = hexagons[6][7];
        city.set_scale(Vector2f {x: 2.8, y: 2.4 });
        city.set_position(Vector2f { x: hexagon.position.x, y: hexagon.position.y - 42.});

        cities.push(city);

        cities.to_vec()
    })
}

