use crate::entities::{State, HexagonCategory};

pub fn init_unit_placement() -> Box<dyn Fn(State) -> State> {
    Box::new(|mut state| {
        for y in 0..state.units.len() {
            for x in 0..state.units[y].len() {
                if let Some(_unit) = state.units[y][x] {
                    state.hexagons[y][x].category = HexagonCategory::UNIT;
                }
            }
        }
        state
    })
}
