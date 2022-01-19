use sfml::window::*;

use crate::utils::find_with_location;
use crate::entities::Hexagon;
use crate::ControlFn;

pub fn init_unit_selection<'a>(scale: f32) -> ControlFn {
    Box::new(move |mut state, graphics| {
        let mut closest : Option<Hexagon> = state.unit_selected;
        let center = graphics.view_center;

        state.events.iter().for_each(|event| {
            match event {
                Event::MouseButtonPressed { button, .. } => {
                    if mouse::Button::RIGHT == *button {
                        closest = None
                    } else if mouse::Button::LEFT == *button {
                        if let Some(_) = state.unit_selected {
                            return
                        }

                        closest = find_with_location(center, scale, &state.hexagons);

                        if let Some(c) = closest {
                            if let None = state.units[c.grid_position.1][c.grid_position.0] {
                                closest.take();
                            }
                        }
                    }
                },
                _ => {},
            }
        });

        state.unit_selected = closest;
        state
    })
}
