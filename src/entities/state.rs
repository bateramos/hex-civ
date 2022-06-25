use sfml::graphics::Sprite;
use sfml::window::Event;

use crate::GridSize;
use crate::entities::*;
use crate::controllers::*;

pub struct State <'a> {
    pub tick_time: f32,

    pub city_interface: Option<CityInterface<'a>>,
    pub hexagons: HexagonColumn,
    pub hex_improvements: Vec<HexImprovement<'a>>,

    pub events: Vec<Event>,
    pub dispatched_events: Vec<HexEvent>,

    pub cities: Vec<City<'a>>,
    pub city_selected: Option<i32>,

    pub units: Vec<Unit<'a>>,
    pub map_sprites: Vec<Sprite<'a>>,
    pub units_sprites: Vec<Sprite<'a>>,
    pub unit_selected: Option<i32>,
    pub unit_selection_effect_timer: f32,
}

impl <'a> State <'a> {
    pub fn new(hexagons: HexagonColumn, _grid_size: GridSize) -> State<'static> {
        State {
            tick_time: 0.0,

            city_interface: None,
            city_selected: None,
            hexagons,

            events: Vec::new(),
            dispatched_events: Vec::new(),

            cities: Vec::new(),
            units: Vec::new(),
            map_sprites: Vec::new(),
            units_sprites: Vec::new(),
            unit_selected: None,
            unit_selection_effect_timer: 0.0,
            hex_improvements: Vec::new(),
        }
    }

    pub fn add_hex_improvement(&mut self, hex_improvement: HexImprovement<'a>) {
        self.hex_improvements.push(hex_improvement);
        self.hex_improvements.sort_by_key(|a| format!("{:03} {:03}", a.position.y, a.position.x));
    }

    pub fn get_hex_with_position(&self, x: u32, y: u32) -> &Hexagon {
        &self.hexagons[y as usize][x as usize]
    }

    pub fn get_hex_with_position_mut(&mut self, x: u32, y: u32) -> &mut Hexagon {
        self.hexagons[y as usize].get_mut(x as usize).unwrap()
    }

    pub fn get_city_on_hex(&self, hex: &Hexagon) -> Option<&City<'a>> {
        let x = hex.grid_position.0;
        let y = hex.grid_position.1;

        self.cities.iter().find(|c| c.position.x == x && c.position.y == y)
    }

    pub fn get_unit_on_hex(&self, hex: &Hexagon) -> Option<&Unit<'a>> {
        let x = hex.grid_position.0;
        let y = hex.grid_position.1;

        self.units.iter().find(|u| u.position.x == x && u.position.y == y)
    }

    pub fn get_unit_on_hex_mut(&mut self, hex: &Hexagon) -> Option<&mut Unit<'a>> {
        let x = hex.grid_position.0;
        let y = hex.grid_position.1;

        self.units.iter_mut().find(|u| u.position.x == x && u.position.y == y)
    }

    pub fn has_event_triggered(&self, event: &str) -> Option<&HexEvent> {
        self.dispatched_events.iter().find(|e| e.name == event)
    }
}
