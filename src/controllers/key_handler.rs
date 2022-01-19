use sfml::window::*;

use crate::State;

pub fn init_key_handler() -> Box<dyn for<'a> Fn(State<'a>) -> State<'a>> {
    Box::new(|state| {
        state.events.iter().for_each(|event| {
            match event {
                Event::Closed | Event::KeyPressed {
                    code: Key::ESCAPE | Key::Q, ..
                } => {
                    std::process::exit(0);
                },
                _ => {}
            };
        });
        state
    })
}
