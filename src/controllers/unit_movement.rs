use crate::{ControlEventFn, GridSize, UNIT_MOVEMENT_EVENT};

pub fn init_unit_movement<'a>(grid_size: GridSize) -> ControlEventFn<'a> {
    ControlEventFn {
        event: UNIT_MOVEMENT_EVENT,
        func: Box::new(move |mut state, _graphics, event| {
            let unit = state.units.iter_mut().find(|u| u.id == event.unit_id.unwrap()).unwrap();

            let within_map : bool = if let Some(position) = event.position {
                if position.y >= grid_size.1 {
                    false
                } else if position.x >= grid_size.0 {
                    false
                } else {
                    true
                }
            } else {
                false
            };

            if within_map {
                unit.position = event.position.unwrap();
            }

            state
        })
    }
}
