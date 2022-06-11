use sfml::graphics::*;
use sfml::system::*;

use crate::ControlFn;
use crate::entities::{HexImprovement, HexImprovementType};

pub fn init_hex_improvement_sprite(scale: f32) -> ControlFn {
    Box::new(move |mut state, graphics| {
        state.hex_improvements.iter_mut().for_each(|improvement| {
            if let None = improvement.sprite {
                let new_sprite = match improvement.improvement_type {
                    HexImprovementType::FARM => graphics.textures.farm_field.clone(),
                    HexImprovementType::MINE => graphics.textures.farm_field.clone(),
                };

                improvement.sprite = Some(new_sprite);
            }

            let HexImprovement { sprite, position, .. } = improvement;
            if let Some(sprite) = sprite {
                if let Some(position) = state.hexagons[position.y as usize][position.x as usize].sprite_position {
                    sprite.set_position(position);
                    sprite.move_(Vector2f { y: 2. * scale, x: 1. * scale });
                }
            }
        });

        state
    })
}
