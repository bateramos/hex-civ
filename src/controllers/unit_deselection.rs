use sfml::window::*;

use crate::{ControlEventFn, ControlActionFn};
use crate::entities::HexEvent;

pub const UNIT_DESELECTION_TRIGGER : &str = "UNIT_DESELECTION_TRIGGER";

pub fn init_unit_deselection_handler<'a>() -> ControlEventFn<'a> {
    ControlEventFn {
        func: Box::new(|mut state, _graphics, _event| {
            state.unit_selected = None;

            state
        }),
        event: UNIT_DESELECTION_TRIGGER
    }
}

pub fn init_unit_deselection() -> ControlActionFn {
    Box::new(|state, _graphics| {
        let mut hex_event : Option<HexEvent> = None;
        state.events.iter().for_each(|event| {
            match event {
                Event::MouseButtonPressed { button, .. } => {
                    hex_event = if mouse::Button::RIGHT == *button {
                        Some(HexEvent::new(UNIT_DESELECTION_TRIGGER))
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

