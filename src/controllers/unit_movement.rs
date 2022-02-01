use sfml::{window::*, system::Vector2i};

use crate::entities::MoveKeyboardConfig;
use crate::ControlFn;

pub fn init_unit_movement(move_config: MoveKeyboardConfig) -> ControlFn {
    Box::new(move |mut state, _graphics| {
        if let Some(id) = state.unit_selected {
            let unit = state.units.iter_mut().find(|u| u.id == id).unwrap();
            let mut x = unit.position.x;
            let mut y = unit.position.y;

            let mut event_triggered = false;

            state.events.iter().for_each(|event| {
                match event {
                    Event::KeyPressed { code, .. } => {
                        if *code == move_config.top {
                            event_triggered = true;

                            y = y - 2;
                        } else if *code == move_config.top_right {
                            event_triggered = true;

                            x = if y % 2 == 0 {
                                x
                            } else {
                                x + 1
                            };

                            y = y - 1;
                        } else if *code == move_config.top_left {
                            event_triggered = true;

                            x = if y % 2 == 0 {
                                x - 1
                            } else {
                                x
                            };

                            y = y - 1;
                        } else if *code == move_config.bottom_right {
                            event_triggered = true;

                            x = if y % 2 == 0 {
                                x
                            } else {
                                x + 1
                            };

                            y = y + 1;
                        } else if *code == move_config.bottom {
                            event_triggered = true;

                            y = y + 2;
                        } else if *code == move_config.bottom_left {
                            event_triggered = true;

                            x = if y % 2 == 0 {
                                x - 1
                            } else {
                                x
                            };

                            y = y + 1;
                        }
                    },
                    _ => {}
                }
            });

            if event_triggered {
                unit.position = Vector2i { x: x as i32, y: y as i32 };
            }
        }
        state
    })
}
