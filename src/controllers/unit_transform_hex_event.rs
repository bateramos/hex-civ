use crate::{ControlEventFn, HexagonCategory};

pub fn init_unit_transform_hex_event<'a>() -> ControlEventFn<'a> {
    ControlEventFn {
        event: "UNIT_TRANSFORM_HEX",
        func: Box::new(|mut state, _graphics, event| {
            let position = event.position.unwrap();
            let mut hex = state.get_hex_with_position_mut(position.x, position.y);

            let new_category = match hex.category {
                HexagonCategory::DenseForest => HexagonCategory::Forest,
                HexagonCategory::Forest => HexagonCategory::Field,
                _ => hex.category,
            };

            hex.category = new_category;

            state
        })
    }
}
