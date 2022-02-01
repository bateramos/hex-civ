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
    pub dispatched_events: Vec<HexEvent>,
    pub cities_sprites: Vec<Sprite<'a>>,
    pub cities: Vec<Vec<Option<City>>>,

    pub units: Vec<Unit<'a>>,
    pub units_sprites: Vec<Sprite<'a>>,
    pub unit_selected: Option<i32>,
    pub unit_selection_effect_timer: f32,
}

impl <'a> State <'a> {
    pub fn new(hexagons: HexagonColumn, grid_size: (i32, i32)) -> State<'static> {
        let cities = vec![vec![None; grid_size.1 as usize]; grid_size.0 as usize];

        State {
            tick_time: 0.0,

            city_interface: None,
            selected_city: None,
            hexagons,

            events: Vec::new(),
            dispatched_events: Vec::new(),

            cities_sprites: Vec::new(),
            cities,
            units: Vec::new(),
            units_sprites: Vec::new(),
            unit_selected: None,
            unit_selection_effect_timer: 0.0,
        }
    }

    pub fn get_unit_on_hex(&self, hex: &Hexagon) -> Option<&Unit<'a>> {
        let x = hex.grid_position.0 as i32;
        let y = hex.grid_position.1 as i32;

        self.units.iter().find(|u| u.position.x == x && u.position.y == y)
    }

    pub fn get_unit_on_hex_mut(&mut self, hex: &Hexagon) -> Option<&mut Unit<'a>> {
        let x = hex.grid_position.0 as i32;
        let y = hex.grid_position.1 as i32;

        self.units.iter_mut().find(|u| u.position.x == x && u.position.y == y)
    }

    pub fn has_event_triggered(&self, event: &str) -> Option<&HexEvent> {
        self.dispatched_events.iter().find(|e| e.name == event)
    }
}
