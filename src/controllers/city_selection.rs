use sfml::window::*;

use crate::utils::find_with_location;
use crate::entities::Hexagon;
use crate::ControlFn;

pub fn init_city_selection(scale: f32) -> ControlFn {
    Box::new(move |mut state, graphics| {
        let mut closest : Option<Hexagon> = state.selected_city;
        let center = graphics.view_center;

        state.events.iter().for_each(|event| {
            match event {
                Event::MouseButtonPressed { button, .. } => {
                    if mouse::Button::RIGHT == *button {
                        closest = None
                    } else if mouse::Button::LEFT == *button {
                        if let Some(_) = state.selected_city {
                            return
                        }

                        closest = find_with_location(center, scale, &state.hexagons);

                        if let Some(c) = closest {
                            if let None = state.cities[c.grid_position.1][c.grid_position.0] {
                                closest.take();
                            }
                        }
                    }
                },
                _ => {}
            }
        });

        state.selected_city = closest;
        state
    })
}
