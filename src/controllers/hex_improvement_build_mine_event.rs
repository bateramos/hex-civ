use crate::ControlEventFn;
use crate::{HexImprovement, HexImprovementType, HexagonCategory};

pub fn init_hex_improvement_build_mine_event<'a>() -> ControlEventFn<'a> {
    ControlEventFn {
        func: Box::new(|mut state, _graphics, event| {
            let position = event.position.unwrap();
            if let None = state.hex_improvements.iter().find(|improvement| improvement.position == position) {
                let hexagon = state.get_hex_with_position(position.x, position.y);
                if hexagon.category == HexagonCategory::Hill || hexagon.category == HexagonCategory::HillWithTrees || hexagon.category == HexagonCategory::Mountain {
                    state.add_hex_improvement(HexImprovement::new_with_type(position, HexImprovementType::MINE));
                }
            }

            state
        }),
        event: "BUILD_MINE"
    }
}

