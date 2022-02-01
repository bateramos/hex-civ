use sfml::graphics::*;

use crate::ControlFn;

pub fn init_city_sprites<'a>() -> ControlFn {
    Box::new(|mut state, graphics| {
        state.cities.iter_mut().for_each(|city| {
            if let None = city.sprite {
                let mut new_sprite = graphics.textures.city.clone();
                let hex = state.hexagons[city.position.y as usize][city.position.x as usize];
                new_sprite.set_position(hex.sprite_position);

                city.sprite = Some(new_sprite);
            };
        });

        state
    })
}
