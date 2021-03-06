use sfml::{system::*, window::*};

use crate::ControlGraphicsFn;

pub fn init_map_navigation(center: Vector2f) -> ControlGraphicsFn {
    Box::new(move |mut view, state, _graphics| {
        if state.city_selected.is_none() {
            state.events.iter().for_each(|event| {
                match event {
                    Event::KeyPressed { code: Key::UP, .. } => view.move_(Vector2f { x: 0., y: -10. }),
                    Event::KeyPressed { code: Key::DOWN, .. } => view.move_(Vector2f { x: 0., y: 10. }),
                    Event::KeyPressed { code: Key::RIGHT, .. } => view.move_(Vector2f { x: 10., y: 0. }),
                    Event::KeyPressed { code: Key::LEFT, .. } => view.move_(Vector2f { x: -10., y: 0. }),
                    Event::MouseButtonPressed { x, y, button } => {
                        if mouse::Button::LEFT == *button {
                            view.move_(Vector2f { x: (center.x - *x as f32) * -1., y: (center.y - *y as f32) * -1. });
                        }
                    },
                    _ => {}
                }
            });
        }

        view
    })
}
