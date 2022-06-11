use crate::{ControlEventFn, ControlActionFn, HexagonCategory, HexEvent};

pub fn init_unit_transform_hex_event<'a>() -> ControlEventFn<'a> {
    ControlEventFn {
        event: "UNIT_TRANSFORM_HEX",
        func: Box::new(|mut state, _graphics, event| {
            let position = event.position.unwrap();
            let mut hex = state.get_hex_with_position_mut(position.x, position.y);

            let new_category = match hex.category {
                HexagonCategory::DenseForest => HexagonCategory::Forest,
                HexagonCategory::Forest => HexagonCategory::Field,
                HexagonCategory::HillWithTrees => HexagonCategory::Hill,
                _ => hex.category,
            };

            hex.category = new_category;

            state
        })
    }
}

pub fn init_unit_transform_hex_event_refresh_map<'a>() -> ControlActionFn {
    Box::new(|state, _graphics| {
        if let Some(_event) = state.dispatched_events.iter().find(|event| event.name == "UNIT_TRANSFORM_HEX") {
            Some(HexEvent::new("INIT_MAP_HEX_SPRITES"))
        } else {
            None
        }
    })
}
