use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use sfml::{graphics::*, system::*, window::*};
use rand;

mod controllers;

use controllers::{
    init_map_navigation,
    init_key_handler,
    init_city_placement,
    init_city_selection,
    init_city_interface,
    init_city_sprites,
    CityInterface,
};

#[derive(Clone, Copy, Debug)]
pub enum HexagonCategory {
    FIELD,
    CITY,
}

#[derive(Clone, Copy, Debug)]
pub struct Hexagon {
    id: u32,
    scale: f32,
    position: Vector2f,
    fill_color: Color,
    outline_color: Color,
    thickness: f32,
    center: Vector2f,
    sprite_position: Vector2f,
    category: HexagonCategory,
}

pub type HexagonLine = Vec<Hexagon>;
pub type HexagonColumn = Vec<HexagonLine>;

impl Hexagon {
    fn create_point(&self, value_x: f32, value_y: f32) -> Vector2f {
        Vector2f {
            x: self.scale * value_x + self.position.x,
            y: self.scale * value_y + self.position.y,
        }
    }
}

impl CustomShapePoints for Hexagon {
    fn point_count(&self) -> u32 {
        6
    }

    fn point(&self, point: u32) -> Vector2f {
        match point {
            0 => self.create_point(10., 0.),
            1 => self.create_point(20., 0.),
            2 => self.create_point(30., 10.),
            3 => self.create_point(20., 20.),
            4 => self.create_point(10., 20.),
            5 => self.create_point(0., 10.),
            p => panic!("Something wrong with point: {}", p),
        }
    }
}

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
    let grid_size = (30, 20);
    let seed = rand::random::<u64>() % 10000;
    let y_sprite_offset = 14. * scale;

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

    let texture = Texture::from_file("res/textures/pillar.png").unwrap();
    let mut pillar = Sprite::with_texture_and_rect(&texture, &IntRect::new(0, 0, 50, 160));
    pillar.set_scale(Vector2f {x: scale, y: 1.9 * scale});

    let sprite_x_padding = 32;
    let sprite_y_padding = 50;
    let texture = Texture::from_file("res/textures/main.png").unwrap();
    let green_field = Sprite::with_texture_and_rect(&texture, &IntRect::new(0 * sprite_x_padding, 0, 32, 50));
    let forest = Sprite::with_texture_and_rect(&texture, &IntRect::new(1 * sprite_x_padding, 0, 32, 50));
    let dense_forest = Sprite::with_texture_and_rect(&texture, &IntRect::new(2 * sprite_x_padding, 0, 32, 50));
    let hill = Sprite::with_texture_and_rect(&texture, &IntRect::new(3 * sprite_x_padding, 0, 32, 50));
    let hill_with_trees = Sprite::with_texture_and_rect(&texture, &IntRect::new(4 * sprite_x_padding, 0, 32, 50));
    let mountain = Sprite::with_texture_and_rect(&texture, &IntRect::new(5 * sprite_x_padding, 0, 32, 50));
    let mut city = Sprite::with_texture_and_rect(&texture, &IntRect::new(0 * sprite_x_padding, 1 * sprite_y_padding, 32, 50));
    let snow = Sprite::with_texture_and_rect(&texture, &IntRect::new(0 * sprite_x_padding, 2 * sprite_y_padding, 32, 50));
    let snow_with_tress = Sprite::with_texture_and_rect(&texture, &IntRect::new(1 * sprite_x_padding, 2 * sprite_y_padding, 32, 50));

    city.set_scale(Vector2f {x: 0.9 * scale, y: 0.9 * scale });

    let mut sprites = Vec::new();
    let mut background_grid = Vec::new();
    let mut hexagons = HexagonColumn::new();
    let mut texts = Vec::new();

    fn generate_random<H>(args: Vec<H>, seed: u64) -> u32 where H: Hash {
        let mut hasher = DefaultHasher::new();
        let hash = args.iter().fold(1, |acc, item| {
            item.hash(&mut hasher);
            let hash = hasher.finish();
            acc + (hash % 10000)
        });

        (hash * seed) as u32 % 100
    }

    for y_i in 0..grid_size.0 {
        let mut line = HexagonLine::new();
        let top = 9. * scale * y_i as f32;
        let y = y_i as f32 * scale + top;

        let padding = if y_i % 2 == 0 {
            -15. * scale
        } else {
            5. * scale
        };

        for x_i in 0..grid_size.1 {
            let bottom = 10. * scale * x_i as f32;
            let x = x_i as f32 * 30. * scale + bottom + padding;

            let mut sprite = if y_i == 1 || y_i == 0 {
                let mut sprite = match generate_random(vec![y_i, x_i], seed) {
                    00 ..= 10 => mountain.clone(),
                    19 ..= 40 => snow_with_tress.clone(),
                    _ => snow.clone(),
                };

                sprite.set_scale(Vector2f {x: 0.9 * scale, y: 0.9 * scale });
                sprite
            } else {
                let mut sprite = match generate_random(vec![y_i, x_i], seed) {
                    0 ..= 8 => dense_forest.clone(),
                    21 ..= 25 => hill.clone(),
                    26 ..= 27 => mountain.clone(),
                    29 ..= 32 => hill_with_trees.clone(),
                    80 ..= 98 => forest.clone(),
                    _ => green_field.clone(),
                };

                sprite.set_scale(Vector2f {x: 0.9 * scale, y: 0.8 * scale });
                sprite
            };

            let sprite_position = Vector2f {x, y: y - y_sprite_offset};
            let center = Vector2f { x: x + 15. * scale, y: y + 10. * scale };

            sprite.set_position(sprite_position);

            let hexagon = Hexagon {
                id: rand::random::<u32>(), category: HexagonCategory::FIELD,
                scale, position: Vector2f { x, y }, center, sprite_position,
                fill_color: Color::TRANSPARENT, outline_color: Color::rgba(86, 84, 85, 51), thickness: 1.,
            };

            let mut shape = CustomShape::new(Box::new(hexagon));
            shape.set_fill_color(hexagon.fill_color);
            shape.set_outline_color(hexagon.outline_color);
            shape.set_outline_thickness(hexagon.thickness);

            let mut text = Text::new(&format!("{},{}", x_i, y_i), &font, 14);
            text.set_position(center);
            texts.push(text);

            background_grid.push(shape);
            sprites.push(sprite);
            line.push(hexagon);
        }

        hexagons.push(line);
    }

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
        state = city_interface_ticker(&font, &pillar, &new_view, state);
        state = city_sprites_ticker(state, &city);

        window.set_view(&new_view);
        window.clear(Color::BLACK);

        sprites.iter().for_each(|sprite| window.draw(sprite));
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
