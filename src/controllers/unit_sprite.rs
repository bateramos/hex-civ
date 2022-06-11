use sfml::graphics::*;
use sfml::system::*;

use crate::ControlFn;
use crate::entities::{Unit, UnitType};

pub fn init_unit_sprite(scale: f32) -> ControlFn {
    Box::new(move |mut state, graphics| {
        state.units.iter_mut().for_each(|unit| {
            if let None = unit.sprite {
                let new_sprite = match unit.unit_type {
                    UnitType::Pikeman => graphics.textures.pikeman.clone(),
                    UnitType::Settler => graphics.textures.peasant.clone()
                };
                unit.sprite = Some(new_sprite);
            };

            let Unit { sprite, position, .. } = unit;

            if let Some(sprite) = sprite {
                if let Some(position) = state.hexagons[position.y as usize][position.x as usize].sprite_position {
                    sprite.set_position(position);
                    sprite.move_(Vector2f { y: 0., x: 6. * scale });
                }
            }
        });

        state
    })
}
