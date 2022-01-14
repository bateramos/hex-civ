use sfml::{graphics::*, system::*, window::*};

use crate::State;

pub fn init_map_navigation(center: Vector2f) -> Box<dyn for<'a> Fn(&mut View, &Vec<Event>, State<'a>) -> State<'a>> {
    Box::new(move |view, events, mut state| {
        events.iter().for_each(|event| {
            match event {
                Event::KeyPressed { code: Key::UP, .. } => view.move_(Vector2f { x: 0., y: -10. }),
                Event::KeyPressed { code: Key::DOWN, .. } => view.move_(Vector2f { x: 0., y: 10. }),
                Event::KeyPressed { code: Key::RIGHT, .. } => view.move_(Vector2f { x: 10., y: 0. }),
                Event::KeyPressed { code: Key::LEFT, .. } => view.move_(Vector2f { x: -10., y: 0. }),
                Event::MouseButtonPressed { x, y, button } => {
                    if mouse::Button::LEFT == *button {
                        view.move_(Vector2f { x: (center.x - *x as f32) * -1., y: (center.y - *y as f32) * -1. });
                        state.dispatch_event("move_to_position".to_string());
                    }
                },
                _ => {}
            }
        });

        state
    })
}
