use crate::ControlEventFn;
use crate::City;

pub fn init_city_build_event<'a>() -> ControlEventFn<'a> {
    ControlEventFn {
        func: Box::new(|mut state, _graphics, event| {
            let unit_id = event.unit_id.unwrap();

            state.units.retain(|u| u.id != unit_id);
            state.cities.push(City::new(event.position.unwrap()));

            state
        }),
        event: "BUILD_CITY"
    }
}
