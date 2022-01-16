use sfml::{graphics::*, system::*, window::*};

use rand;

mod controllers;
mod entities;

use entities::*;

use controllers::{
    init_map_navigation,
    init_map_creation,
    init_key_handler,
    init_city_placement,
    init_city_selection,
    init_city_interface,
    init_city_sprites,
    init_textures,
    CityInterface,
};

type Resolution = (u32, u32, f32);

fn resolutions() -> Vec<Resolution> {
    vec![
        (2048, 1536, 5.),
        (800, 600, 3.),
    ]
}

pub struct State <'a> {
    city_interface: Option<CityInterface<'a>>,
    hexagons: HexagonColumn,
    selected_city: Option<Hexagon>,

    dispatched_events: Vec<String>,
    on_clicks: Vec<(Rect<f32>, Box<dyn Fn() -> State<'a>>)>,
    cities: Vec<Sprite<'a>>,
}

impl <'a> State <'a> {
    pub fn dispatch_event(&mut self, event: String) {
        self.dispatched_events.insert(0, event);
    }

    pub fn drain_one_event(&mut self) {
        if let Some(last) = self.dispatched_events.last() {
            if last == "empty" {
                return
            }
        }

        self.dispatched_events.pop();

        if self.dispatched_events.is_empty() {
            self.dispatched_events.push("empty".to_string());
        }
    }
}

fn main() {
    let res_index = &std::env::args().collect::<Vec<String>>()[1];
    let resolution : Resolution = resolutions()[res_index.parse::<usize>().unwrap_or(0)];
    let scale = resolution.2;
    let _grid_size = (30, 20);
    let seed = rand::random::<u64>() % 10000;

    println!("Resolution: {:?}; Seed: {};", resolution, seed);

    let mut window = RenderWindow::new(
        (resolution.0, resolution.1),
        "Hex Civ",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_vertical_sync_enabled(true);
    window.set_position(Vector2i { x: 200, y: 200 });
    let mut new_view = View::from_rect(&FloatRect::new(0., 0., resolution.0 as f32, resolution.1 as f32));
    window.set_view(&new_view);

    let font = Font::from_file("res/fonts/Seagram tfb.ttf").unwrap();
    let texture = Texture::from_file("res/textures/main.png").unwrap();
    let texture_pillar = Texture::from_file("res/textures/pillar.png").unwrap();

    let textures = init_textures(scale, &texture, &texture_pillar);

    let (mut hexagons, sprites, background_grid)  = init_map_creation(scale, seed, &textures);
    hexagons = init_city_placement(hexagons);

    println!("{:?}", new_view.size());

    let map_navigation_ticker = init_map_navigation(new_view.center());
    let key_handler_ticker = init_key_handler();
    let city_selection_ticker = init_city_selection(scale);
    let city_interface_ticker = init_city_interface(scale, new_view.size());
    let city_sprites_ticker = init_city_sprites();

    let mut state = State {
        city_interface: None,
        selected_city: Some(hexagons[3][3]),
        hexagons,
        dispatched_events: Vec::new(),

        on_clicks: Vec::new(),
        cities: Vec::new(),
    };

    loop {
        let mut events = Vec::new();
        while let Some(event) = window.poll_event() {
            events.push(event);
        }

        state = map_navigation_ticker(&mut new_view, &events, state);
        state = key_handler_ticker(&events, state);
        state = city_selection_ticker(&new_view, &events, state);
        state = city_interface_ticker(&font, &textures, &new_view, state);
        state = city_sprites_ticker(state, &textures);

        window.set_view(&new_view);
        window.clear(Color::BLACK);

        sprites.iter().for_each(|sprite: &Sprite| window.draw(sprite));
        background_grid.iter().for_each(|shape| window.draw(shape));
        state.cities.iter().for_each(|shape| window.draw(shape));
        //texts.iter().for_each(|text| window.draw(text));
        if let Some(interface) = &state.city_interface {
            interface.draw(&mut window);
        }

        state.drain_one_event();

        window.display();
    }
}
