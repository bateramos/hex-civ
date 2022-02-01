use sfml::window::*;

use crate::utils::find_with_location;
use crate::ControlFn;

pub fn init_unit_selection<'a>(scale: f32) -> ControlFn {
    Box::new(move |mut state, graphics| {
        let mut selected : Option<i32> = state.unit_selected;
        let center = graphics.view_center;

        state.events.iter().for_each(|event| {
            match event {
                Event::MouseButtonPressed { button, .. } => {
                    if mouse::Button::LEFT == *button {
                        if let Some(_) = state.unit_selected {
                            return
                        }

                        let closest = find_with_location(center, scale, &state.hexagons);

                        if let Some(c) = closest {
                            if let Some(unit) = state.get_unit_on_hex(&c) {
                                selected = Some(unit.id);
                            } else {
                                selected = None;
                            }
                        }
                    }
                },
                _ => {},
            }
        });

        state.unit_selected = selected;
        state
    })
}
