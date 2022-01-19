use sfml::window::*;

use crate::ControlFn;

pub fn init_unit_movement() -> ControlFn {
    Box::new(|mut state, _graphics| {
        if let Some(unit) = state.unit_selected {
            let mut x = unit.grid_position.0;
            let mut y = unit.grid_position.1;
            let unit_type = state.units[y][x];

            let mut event_triggered = false;

            state.events.iter().for_each(|event| {
                match event {
                    Event::KeyPressed { code: Key::NUMPAD9, .. } => {
                        event_triggered = true;
                        state.units[y][x] = None;

                        x = if y % 2 == 0 {
                            unit.grid_position.0
                        } else {
                            unit.grid_position.0 + 1
                        };

                        y = y - 1;
                    },
                    Event::KeyPressed { code: Key::NUMPAD7, .. } => {
                        event_triggered = true;
                        state.units[y][x] = None;

                        x = if y % 2 == 0 {
                            unit.grid_position.0 - 1
                        } else {
                            unit.grid_position.0
                        };

                        y = y - 1;
                    },
                    Event::KeyPressed { code: Key::NUMPAD4, .. } => {
                        event_triggered = true;
                        state.units[y][x] = None;

                        x = if y % 2 == 0 {
                            unit.grid_position.0 - 1
                        } else {
                            unit.grid_position.0
                        };

                        y = y + 1;
                    },
                    Event::KeyPressed { code: Key::NUMPAD6, .. } => {
                        event_triggered = true;
                        state.units[y][x] = None;

                        x = if y % 2 == 0 {
                            unit.grid_position.0
                        } else {
                            unit.grid_position.0 + 1
                        };

                        y = y + 1;
                    },
                    Event::KeyPressed { code: Key::NUMPAD8, .. } => {
                        event_triggered = true;
                        state.units[y][x] = None;

                        y = y - 2;
                    },
                    Event::KeyPressed { code: Key::NUMPAD5, .. } => {
                        event_triggered = true;
                        state.units[y][x] = None;

                        y = y + 2;
                    },
                    _ => {}
                }

                if event_triggered {
                    state.units[y][x] = unit_type;
                    state.unit_selected = Some(state.hexagons[y][x]);
                }
            });
        }
        state
    })
}
