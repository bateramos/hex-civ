use crate::ControlFn;

pub const UNIT_CREATE_UNIT : &str = "UNIT_CREATE_UNIT";

pub fn init_unit_placement() -> ControlFn {
    Box::new(|state, _graphics| {
        if let Some(event) = state.dispatched_events.iter().find(|e| e.name == UNIT_CREATE_UNIT) {
        }

        state
    })
}
