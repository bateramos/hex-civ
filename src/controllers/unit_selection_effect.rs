use sfml::graphics::*;

use crate::ControlFn;
use crate::entities::Unit;
use crate::controllers::UNIT_DESELECTION_TRIGGER;

pub fn init_unit_deselection_effect() -> ControlFn {
    Box::new(|mut state, _graphics| {
        if let Some(_event) = state.has_event_triggered(UNIT_DESELECTION_TRIGGER) {
            state.units.iter_mut().for_each(|unit| {
                let Unit { sprite, .. } = unit;
                let sprite = sprite.as_mut().unwrap();
                sprite.set_color(Color::WHITE);
            });
        }
        state
    })
}

pub fn init_unit_selection_effect() -> ControlFn {
    Box::new(|mut state, _graphics| {
        if let Some(id) = state.unit_selected {
            let unit = state.units.iter_mut().find(|u| u.id == id).unwrap();
            let mut timer = state.unit_selection_effect_timer;
            timer += state.tick_time;

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
