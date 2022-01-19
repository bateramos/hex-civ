use sfml::graphics::*;
use sfml::system::*;

use crate::ControlFn;

pub fn init_unit_sprite(scale: f32) -> ControlFn {
    Box::new(move |mut state, graphics| {
        let mut units : Vec<Sprite> = Vec::new();

        for y in 0..state.units.len() {
            for x in 0..state.units[y].len() {
                if let Some(_unit) = state.units[y][x] {
                    let mut sprite = graphics.textures.pikeman.clone();

                    sprite.set_position(state.hexagons[y][x].sprite_position);
                    sprite.move_(Vector2f { y: 0., x: 6. * scale });
                    units.push(sprite);
                }
            }
        }

        state.units_sprites = units;

        state
    })
}
