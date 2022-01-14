use sfml::{graphics::*, window::*};

use crate::{HexagonCategory, Hexagon, State};

pub fn init_city_selection(scale: f32) -> Box<dyn for<'a> Fn(&View, &Vec<Event>, State<'a>) -> State<'a>> {
    Box::new(move |view, events, mut state| {
        let mut closest : Option<Hexagon> = state.selected_city;
        let center = view.center();

        events.iter().for_each(|event| {
            match event {
                Event::MouseButtonPressed { button, .. } => {
                    if mouse::Button::RIGHT == *button {
                        closest = None
                    } else if mouse::Button::LEFT == *button {
                        if let Some(_) = state.selected_city {
                            return
                        }
                        let mut x_index = (center.x / (40. * scale)) as usize;
                        let mut y_index = (center.y / (10. * scale)) as usize;

                        if x_index <= 0 {
                            x_index = 1;
                        }
                        if y_index <= 0 {
                            y_index = 1;
                        }

                        let mut candidates = Vec::new();

                        if let Some(line) = state.hexagons.get(y_index + 1) {
                            if let Some(hex) = line.get(x_index - 1) {
                                candidates.push(hex);
                            }
                            if let Some(hex) = line.get(x_index) {
                                candidates.push(hex);
                            }
                            if let Some(hex) = line.get(x_index + 1) {
                                candidates.push(hex);
                            }
                        }
                        if let Some(line) = state.hexagons.get(y_index) {
                            if let Some(hex) = line.get(x_index) {
                                candidates.push(hex);
                            }
                        }
                        if let Some(line) = state.hexagons.get(y_index - 1) {
                            if let Some(hex) = line.get(x_index - 1) {
                                candidates.push(hex);
                            }
                            if let Some(hex) = line.get(x_index) {
                                candidates.push(hex);
                            }
                            if let Some(hex) = line.get(x_index + 1) {
                                candidates.push(hex);
                            }
                        }

                        let mut closest_distance = f32::MAX;
                        candidates.iter().for_each(|hex| {
                            match hex.category {
                                HexagonCategory::CITY => {
                                    let distance = (
                                        (hex.center.x - center.x as f32).powf(2.) +
                                        (hex.center.y - center.y as f32).powf(2.)
                                    ).sqrt();

                                    if distance < closest_distance {
                                        closest = Some(**hex);
                                        closest_distance = distance;
                                    }
                                    println!("{} - {:?}", closest_distance, closest.unwrap().category);
                                },
                                _ => {},
                            }
                        });
                    }
                },
                _ => {}
            }
        });

        state.selected_city = closest;
        state
    })
}
