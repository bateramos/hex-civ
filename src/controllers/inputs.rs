use sfml::window::*;

use crate::ControlActionFn;
use crate::entities::HexEvent;

pub const MOUSE_CLICK_RIGHT : &str = "MOUSE_CLICK_RIGHT";

pub fn init_mouse_button_handler() -> ControlActionFn {
    Box::new(|state, _graphics| {
        let mut hex_event : Option<HexEvent> = None;
        state.events.iter().for_each(|event| {
            match event {
                Event::MouseButtonPressed { button, .. } => {
                    hex_event  = if mouse::Button::RIGHT == *button {
                        Some(HexEvent::new(MOUSE_CLICK_RIGHT))
                    } else {
                        None
                    }
                },
                _ => {},
            }
        });
        hex_event
    })
}
