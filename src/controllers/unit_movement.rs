use sfml::{window::*, system::Vector2i};

use crate::entities::MoveKeyboardConfig;
use crate::ControlFn;

pub fn init_unit_movement(move_config: MoveKeyboardConfig) -> ControlFn {
    Box::new(move |mut state, _graphics| {
        if let Some(hex) = state.unit_selected {
            let mut x = hex.grid_position.0;
            let mut y = hex.grid_position.1;

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
                                hex.grid_position.0
                            } else {
                                hex.grid_position.0 + 1
                            };

                            y = y - 1;
                        } else if *code == move_config.top_left {
                            event_triggered = true;

                            x = if y % 2 == 0 {
                                hex.grid_position.0 - 1
                            } else {
                                hex.grid_position.0
                            };

                            y = y - 1;
                        } else if *code == move_config.bottom_right {
                            event_triggered = true;

                            x = if y % 2 == 0 {
                                hex.grid_position.0
                            } else {
                                hex.grid_position.0 + 1
                            };

                            y = y + 1;
                        } else if *code == move_config.bottom {
                            event_triggered = true;

                            y = y + 2;
                        } else if *code == move_config.bottom_left {
                            event_triggered = true;

                            x = if y % 2 == 0 {
                                hex.grid_position.0 - 1
                            } else {
                                hex.grid_position.0
                            };

                            y = y + 1;
                        }
                    },
                    _ => {}
                }
            });

            if event_triggered {
                let unit = state.get_unit_on_hex_mut(&hex).unwrap();
                unit.position = Vector2i { x: x as i32, y: y as i32 };
                state.unit_selected = Some(state.hexagons[y][x]);
            }
        }
        state
    })
}
