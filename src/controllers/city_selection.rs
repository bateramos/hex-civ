use sfml::window::*;

use crate::utils::find_with_location;
use crate::ControlFn;

pub fn init_city_selection(scale: f32) -> ControlFn {
    Box::new(move |mut state, graphics| {
        let mut selected : Option<i32> = state.city_selected;
        let center = graphics.view_center;

        state.events.iter().for_each(|event| {
            match event {
                Event::MouseButtonPressed { button, .. } => {
                    if mouse::Button::LEFT == *button {
                        if let Some(_) = state.city_selected {
                            return
                        }

                        let hex = find_with_location(center, scale, &state.hexagons);

                        if let Some(c) = hex {
                            if let Some(city) = state.get_city_on_hex(&c) {
                                selected = Some(city.id);
                            }
                        }
                    }
                },
                _ => {}
            }
        });

        state.city_selected = selected;
        state
    })
}
