use sfml::{window::*, system::Vector2i};

use crate::entities::MoveKeyboardConfig;
use crate::{ControlActionFn, HexEvent};

pub const UNIT_MOVEMENT_EVENT : &str = "UNIT_MOVEMENT_EVENT";

pub fn init_unit_movement_event(move_config: MoveKeyboardConfig) -> ControlActionFn {
    Box::new(move |state, _graphics| {
        if let Some(id) = state.unit_selected {
            let unit = state.units.iter().find(|u| u.id == id).unwrap();
            let mut x = unit.position.x;
            let mut y = unit.position.y;

            let mut event_triggered = false;

            state.events.iter().for_each(|event| {
                match event {
                    Event::KeyPressed { code, .. } => {
                        if *code == move_config.top {
                            event_triggered = true;

                            y = y - 1;
                        } else if *code == move_config.top_right {
                            event_triggered = true;

                            if x % 2 == 0 {
                                y = y - 1;
                            }
                            x = x + 1;
                        } else if *code == move_config.top_left {
                            event_triggered = true;

                            if x % 2 == 0 {
                                y = y - 1;
                            }
                            x = x - 1;
                        } else if *code == move_config.bottom_right {
                            event_triggered = true;

                            if x % 2 != 0 {
                                y = y + 1;
                            }
                            x = x + 1;
                        } else if *code == move_config.bottom {
                            event_triggered = true;

                            y = y + 1;
                        } else if *code == move_config.bottom_left {
                            event_triggered = true;

                            if x % 2 != 0 {
                                y = y + 1;
                            }
                            x = x - 1;
                        }
                    },
                    _ => {}
                }
            });

            if event_triggered {
                let to_position = Vector2i { x: x as i32, y: y as i32 };

                Some(HexEvent::new_from_unit_position(UNIT_MOVEMENT_EVENT, unit, to_position))
            } else {
                None
            }
        } else {
            None
        }
    })
}
