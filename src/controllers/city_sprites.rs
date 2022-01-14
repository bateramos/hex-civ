use sfml::graphics::*;

use crate::State;

pub fn init_city_sprites<'a>() -> Box<dyn Fn(State<'a>, &'a Sprite) -> State<'a>> {
    Box::new(|mut state, city_sprite| {
        let mut cities = Vec::new();

        let mut city = city_sprite.clone();
        let hexagon = state.hexagons[3][3];
        city.set_position(hexagon.sprite_position);

        cities.push(city);

        state.cities = cities;

        state
    })
}

