use sfml::graphics::*;

use crate::ControlFn;
use crate::entities::Unit;

pub fn init_unit_selection_effect() -> ControlFn {
    Box::new(|mut state, _graphics| {
        if let Some(hex) = state.unit_selected {
            let mut timer = state.unit_selection_effect_timer;
            timer += state.tick_time;

            let unit = state.get_unit_on_hex_mut(&hex).unwrap();
            let Unit { sprite, .. } = unit;
            let sprite = sprite.as_mut().unwrap();

            if timer < 500. {
                sprite.set_color(Color::WHITE);
            } else if timer < 700. {
                sprite.set_color(Color::TRANSPARENT);
            } else {
                timer = 0.;
            }

            state.unit_selection_effect_timer = timer;
        }

        state
    })
}
