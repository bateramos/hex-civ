use crate::{ControlActionsFn, HexEvent};

pub fn init_hex_improvement_sprite_reload() -> ControlActionsFn {
    let events = vec![
        "BUILD_FARM_FIELD", "BUILD_MINE",
    ];
    Box::new(move |state, _graphics| {
        if let Some(_) = events.iter().find(|event_name| state.has_event_triggered(event_name).is_some()) {
            Some(vec![HexEvent::new("IMPROVEMENT_REFRESH_SPRITE"), HexEvent::new("INIT_MAP_HEX_SPRITES")])
        } else {
            None
        }
    })
}
