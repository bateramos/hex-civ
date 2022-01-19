use sfml::window::*;

use crate::ControlFn;
use crate::entities::HexagonCategory;

pub fn init_unit_movement() -> ControlFn {
    Box::new(|mut state, _graphics| {
        if let Some(mut unit) = state.unit_selected {
            state.events.iter().for_each(|event| {
                match event {
                    Event::KeyPressed { code: Key::NUMPAD9, .. } => {
                        let x = unit.grid_position.0;
                        let y = unit.grid_position.1;
                        unit.category = HexagonCategory::FIELD;
                        let unit_type = state.units[y][x];
                        state.units[y][x] = None;

                        let x = if y % 2 == 0 {
                            unit.grid_position.0
                        } else {
                            unit.grid_position.0 + 1
                        };

                        let y = y - 1;
                        state.units[y][x] = unit_type;
                        state.unit_selected = Some(state.hexagons[y][x]);
                    },
                    Event::KeyPressed { code: Key::NUMPAD7, .. } => {
                        let x = unit.grid_position.0;
                        let y = unit.grid_position.1;
                        unit.category = HexagonCategory::FIELD;
                        let unit_type = state.units[y][x];
                        state.units[y][x] = None;

                        let x = if y % 2 == 0 {
                            unit.grid_position.0 - 1
                        } else {
                            unit.grid_position.0
                        };

                        let y = y - 1;
                        state.units[y][x] = unit_type;
                        state.unit_selected = Some(state.hexagons[y][x]);
                    },
                    Event::KeyPressed { code: Key::NUMPAD4, .. } => {
                        let x = unit.grid_position.0;
                        let y = unit.grid_position.1;
                        unit.category = HexagonCategory::FIELD;
                        let unit_type = state.units[y][x];
                        state.units[y][x] = None;

                        let x = if y % 2 == 0 {
                            unit.grid_position.0 - 1
                        } else {
                            unit.grid_position.0
                        };

                        let y = y + 1;
                        state.units[y][x] = unit_type;
                        state.unit_selected = Some(state.hexagons[y][x]);
                    },
                    Event::KeyPressed { code: Key::NUMPAD6, .. } => {
                        let x = unit.grid_position.0;
                        let y = unit.grid_position.1;
                        unit.category = HexagonCategory::FIELD;
                        let unit_type = state.units[y][x];
                        state.units[y][x] = None;

                        let x = if y % 2 == 0 {
                            unit.grid_position.0
                        } else {
                            unit.grid_position.0 + 1
                        };

                        let y = y + 1;
                        state.units[y][x] = unit_type;
                        state.unit_selected = Some(state.hexagons[y][x]);
                    },
                    Event::KeyPressed { code: Key::NUMPAD8, .. } => {
                        let x = unit.grid_position.0;
                        let y = unit.grid_position.1;
                        unit.category = HexagonCategory::FIELD;
                        let unit_type = state.units[y][x];
                        state.units[y][x] = None;

                        let y = unit.grid_position.1 - 2;
                        state.units[y][x] = unit_type;

                        state.unit_selected = Some(state.hexagons[y][x]);
                    },
                    Event::KeyPressed { code: Key::NUMPAD5, .. } => {
                        let x = unit.grid_position.0;
                        let y = unit.grid_position.1;
                        unit.category = HexagonCategory::FIELD;
                        let unit_type = state.units[y][x];
                        state.units[y][x] = None;

                        let y = unit.grid_position.1 + 2;
                        state.units[y][x] = unit_type;

                        state.unit_selected = Some(state.hexagons[y][x]);
                    },
                    _ => {}
                }
            });
        }
        state
    })
}
