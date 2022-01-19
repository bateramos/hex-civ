use sfml::graphics::Sprite;
use sfml::window::Event;

use crate::entities::*;
use crate::controllers::*;

pub struct State <'a> {
    pub tick_time: f32,

    pub city_interface: Option<CityInterface<'a>>,
    pub hexagons: HexagonColumn,
    pub selected_city: Option<Hexagon>,

    pub events: Vec<Event>,
    pub dispatched_events: Vec<String>,
    pub cities: Vec<Sprite<'a>>,

    pub units: Vec<Vec<Option<UnitType>>>,
    pub units_sprites: Vec<Sprite<'a>>,
    pub unit_selected: Option<Hexagon>,
    pub unit_selection_effect_timer: f32,
}

impl <'a> State <'a> {
    pub fn new(hexagons: HexagonColumn, grid_size: (i32, i32)) -> State<'static> {
        let units = vec![vec![None; grid_size.1 as usize]; grid_size.0 as usize];

        State {
            tick_time: 0.0,

            city_interface: None,
            selected_city: None,
            hexagons,

            events: Vec::new(),
            dispatched_events: Vec::new(),

            cities: Vec::new(),
            units,
            units_sprites: Vec::new(),
            unit_selected: None,
            unit_selection_effect_timer: 0.0,
        }
    }

    pub fn dispatch_event(&mut self, event: String) {
        self.dispatched_events.push(event);
    }
}

