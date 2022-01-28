use sfml::graphics::*;

use crate::ControlFn;

pub fn init_city_sprites<'a>() -> ControlFn {
    Box::new(|mut state, graphics| {
        let mut cities = Vec::new();

        let mut city = graphics.textures.city.clone();
        let hexagon = state.hexagons[3][3];
        city.set_position(hexagon.sprite_position);

        cities.push(city);

        state.cities_sprites = cities;

        state
    })
}
