use crate::ControlEventFn;
use crate::{HexImprovement, HexagonCategory};

pub fn init_hex_improvement_build_event<'a>() -> ControlEventFn<'a> {
    ControlEventFn {
        func: Box::new(|mut state, _graphics, event| {
            let position = event.position.unwrap();
            if let None = state.hex_improvements.iter().find(|improvement| improvement.position == position) {
                let hexagon = state.get_hex_with_position(position.x, position.y);
                println!("{:?}", hexagon.category);
                if hexagon.category == HexagonCategory::FIELD {
                    state.hex_improvements.push(HexImprovement::new(position));
                    state.hex_improvements.sort_by_key(|a| format!("{:03} {:03}", a.position.y, a.position.x));
                }
            }

            state
        }),
        event: "BUILD_FARM_FIELD"
    }
}

