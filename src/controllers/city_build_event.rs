use crate::{ControlEventFn, EventFn};
use crate::City;

pub fn init_city_build_event<'a>() -> ControlEventFn<'a> {
    EventFn {
        func: Box::new(|mut state, _graphics| {
            if let Some(unit_id) = state.unit_selected {
                let unit = state.units.iter().find(|u| u.id == unit_id).unwrap();

                state.cities.push(City::new(unit.position));
                state.units.retain(|u| u.id != unit_id);
                state.unit_selected = None;
            }
            state
        }),
        event: "BUILD_CITY"
    }
}
