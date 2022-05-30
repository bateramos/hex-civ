use sfml::system::*;

use crate::{ControlEventGraphicFn, UNIT_MOVEMENT_EVENT};

pub fn init_map_unit_follow<'a>(scale: f32) -> ControlEventGraphicFn<'a> {
    ControlEventGraphicFn {
        event: UNIT_MOVEMENT_EVENT,
        func: Box::new(move |mut view, state, _graphics, event| {
            if let Some(_unit_id) = state.unit_selected {
                let position = event.position.unwrap();

                view.set_center(Vector2f { x: position.x as f32 * 20. * scale, y: position.y as f32 * 20. * scale });
            }

            view
        })
    }
}
