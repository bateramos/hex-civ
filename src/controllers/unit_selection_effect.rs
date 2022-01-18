use sfml::graphics::*;

use crate::entities::State;

pub fn init_unit_selection_effect<'a>() -> Box<dyn Fn(State<'a>) -> State<'a>> {
    Box::new(|mut state| {
        if let Some(_unit) = state.unit_selected {
            let mut timer = state.unit_selection_effect_timer;
            timer += state.tick_time;

            let mut color = state.units_sprites[0].color();
            let alpha = color.alpha_mut();

            if timer < 500. {
                *alpha = 100;
            } else if timer < 700. {
                state.units_sprites[0].set_color(Color::TRANSPARENT);
            } else {
                timer = 0.;
            }

            state.unit_selection_effect_timer = timer;
        }

        state
    })
}
