use sfml::window::*;

use crate::HexEvent;
use crate::UnitType;
use crate::{ControlActionsFn, OrderKeyboardConfig, UNIT_DESELECTION_TRIGGER};

pub fn init_unit_order(order_config: OrderKeyboardConfig) -> ControlActionsFn {
    Box::new(move |state, _graphics| {
        if let Some(id) = state.unit_selected {
            let unit = state.units.iter().find(|u| u.id == id).unwrap();

            state.events.iter().find_map(|event| {
                match event {
                    Event::KeyPressed { code, .. } => {
                        if *code == order_config.build_city && unit.unit_type == UnitType::Settler {
                            Some(vec![HexEvent::new_from_unit("BUILD_CITY", unit), HexEvent::new_from_unit(UNIT_DESELECTION_TRIGGER, unit)])
                        } else if *code == order_config.build_farm_field && unit.unit_type == UnitType::Settler {
                            Some(vec![HexEvent::new_from_unit("BUILD_FARM_FIELD", unit)])
                        } else {
                            None
                        }
                    },
                    _ => None
                }
            })
        } else {
            None
        }
    })
}
