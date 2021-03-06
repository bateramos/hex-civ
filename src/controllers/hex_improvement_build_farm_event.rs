use crate::ControlEventFn;
use crate::{HexImprovement, HexagonCategory};

pub fn init_hex_improvement_build_farm_event<'a>() -> ControlEventFn<'a> {
    ControlEventFn {
        func: Box::new(|mut state, _graphics, event| {
            let position = event.position.unwrap();
            if let None = state.hex_improvements.iter().find(|improvement| improvement.position == position) {
                let hexagon = state.get_hex_with_position(position.x, position.y);
                if hexagon.category == HexagonCategory::Field {
                    state.add_hex_improvement(HexImprovement::new(position));
                }
            }

            state
        }),
        event: "BUILD_FARM_FIELD"
    }
}
