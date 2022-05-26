use sfml::system::*;

use crate::{ControlEventGraphicFn, UNIT_MOVEMENT_EVENT};

pub fn init_map_unit_follow<'a>(scale: f32) -> ControlEventGraphicFn<'a> {
    ControlEventGraphicFn {
        event: UNIT_MOVEMENT_EVENT,
        func: Box::new(move |mut view, state, _graphics, _event| {
            if let Some(unit_id) = state.unit_selected {
                let unit = state.units.iter().find(|u| u.id == unit_id).unwrap();
                let x = unit.position.x;
                let y = unit.position.y;

                //view.set_center(Vector2f { x: x as f32 * scale * 40., y: y as f32 * scale * 10. });
                //println!("{:?}", Vector2f { x: x as f32 * scale, y: y as f32 * scale });
            }

            view
        })
    }
}
