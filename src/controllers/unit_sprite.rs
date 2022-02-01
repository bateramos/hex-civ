use sfml::graphics::*;
use sfml::system::*;

use crate::ControlFn;
use crate::entities::Unit;

pub fn init_unit_sprite(scale: f32) -> ControlFn {
    Box::new(move |mut state, graphics| {
        state.units.iter_mut().for_each(|unit| {
            if let None = unit.sprite {
                let new_sprite = graphics.textures.pikeman.clone();
                unit.sprite = Some(new_sprite);
            };

            let Unit { sprite, position, .. } = unit;

            if let Some(sprite) = sprite {
                sprite.set_position(state.hexagons[position.y as usize][position.x as usize].sprite_position);
                sprite.move_(Vector2f { y: 0., x: 6. * scale });
            }
        });

        state
    })
}
