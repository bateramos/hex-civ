use crate::ControlEventFn;
use crate::{City, HexagonCategory};

pub fn init_city_build_event<'a>() -> ControlEventFn<'a> {
    ControlEventFn {
        func: Box::new(|mut state, _graphics, event| {
            let position = event.position.unwrap();
            let unit_id = event.unit_id.unwrap();

            if state.get_hex_with_position(position.x, position.y).category != HexagonCategory::City {
                state.units.retain(|u| u.id != unit_id);
                state.cities.push(City::new(position));
                let mut hex = state.get_hex_with_position_mut(position.x, position.y);
                hex.category = HexagonCategory::City;
            }

            state
        }),
        event: "BUILD_CITY"
    }
}
