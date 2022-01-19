use sfml::window::*;

use crate::ControlFn;

pub fn init_key_handler() -> ControlFn {
    Box::new(|state, _graphics| {
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
