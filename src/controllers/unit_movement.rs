use crate::{ControlEventFn, UNIT_MOVEMENT_EVENT};

pub fn init_unit_movement<'a>() -> ControlEventFn<'a> {
    ControlEventFn {
        event: UNIT_MOVEMENT_EVENT,
        func: Box::new(|mut state, _graphics, event| {
            let unit = state.units.iter_mut().find(|u| u.id == event.unit_id.unwrap()).unwrap();

            unit.position = event.position.unwrap();

            state
        })
    }
}
