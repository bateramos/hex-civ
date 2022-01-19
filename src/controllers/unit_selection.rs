use sfml::{window::*, graphics::*};

use crate::utils::find_with_location;
use crate::entities::{State, Hexagon, HexagonCategory};

pub fn init_unit_selection<'a>(scale: f32) -> Box<dyn Fn(State<'a>, &View) -> State<'a>> {
    Box::new(move |mut state, view| {
        let mut closest : Option<Hexagon> = state.unit_selected;
        let center = view.center();

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
                            if c.category != HexagonCategory::UNIT {
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
