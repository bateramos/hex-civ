use sfml::window::*;

pub fn init_key_handler() -> Box<dyn Fn(&Vec<Event>)> {
    Box::new(|events| {
        events.iter().for_each(|event| {
            match event {
                Event::Closed | Event::KeyPressed {
                    code: Key::ESCAPE | Key::Q, ..
                } => {
                    std::process::exit(0);
                },
                _ => {}
            };
        });
    })
}
