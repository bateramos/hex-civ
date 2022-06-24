use sfml::window::*;

use crate::HexEvent;
use crate::{ControlActionFn, MapKeyboardConfig};

pub fn init_turn_event(map_config: MapKeyboardConfig) -> ControlActionFn {
    Box::new(move |state, _graphics| {
        state.events.iter().find_map(|event| {
            match event {
                Event::KeyPressed { code, .. } => {
                    if *code == map_config.new_turn {
                        Some(HexEvent::new("NEW_TURN"))
                    } else {
                        None
                    }
                },
                _ => None
            }
        })
    })
}

