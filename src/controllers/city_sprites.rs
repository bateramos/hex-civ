use sfml::graphics::*;

use crate::State;
use crate::controllers::LoadedTextures;

pub fn init_city_sprites<'a>() -> Box<dyn Fn(State<'a>, &'a LoadedTextures) -> State<'a>> {
    Box::new(|mut state, textures| {
        let mut cities = Vec::new();

        let mut city = textures.city.clone();
        let hexagon = state.hexagons[3][3];
        city.set_position(hexagon.sprite_position);

        cities.push(city);

        state.cities = cities;

        state
    })
}

