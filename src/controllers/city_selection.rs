use sfml::{graphics::*, window::*};

use crate::utils::find_with_location;
use crate::entities::{State, Hexagon, HexagonCategory};

pub fn init_city_selection(scale: f32) -> Box<dyn for<'a> Fn(&View, State<'a>) -> State<'a>> {
    Box::new(move |view, mut state| {
        let mut closest : Option<Hexagon> = state.selected_city;
        let center = view.center();

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
                            if c.category != HexagonCategory::CITY {
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
