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


fn main() {
    let resolution = resolutions()[0];
    let scale = resolution.2;
    let grid_size = (30, 20);
    let seed = rand::random::<u64>() % 10000;
    let y_sprite_offset = 14. * scale;
    println!("Seed: {};", seed);

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

            let mut sprite = green_field.clone();
            if y_i == 1 || y_i == 0 {
                sprite = match generate_random(vec![y_i, x_i], seed) {
                    00 ..= 10 => mountain.clone(),
                    19 ..= 40 => snow_with_tress.clone(),
                    _ => snow.clone(),
                };

                sprite.set_scale(Vector2f {x: 0.9 * scale, y: 0.9 * scale });
            } else {
                sprite = match generate_random(vec![y_i, x_i], seed) {
                    0 ..= 8 => dense_forest.clone(),
                    21 ..= 25 => hill.clone(),
                    26 ..= 27 => mountain.clone(),
                    29 ..= 32 => hill_with_trees.clone(),
                    80 ..= 98 => forest.clone(),
                    _ => green_field.clone(),
                };

                sprite.set_scale(Vector2f {x: 0.9 * scale, y: 0.8 * scale });
            }

            sprite.set_position(Vector2f {x, y: y - y_sprite_offset});

            let center = Vector2f { x: x + 15. * scale, y: y + 10. * scale };
            let hexagon = Hexagon {
                id: rand::random::<u32>(), category: HexagonCategory::FIELD,
                scale, position: Vector2f { x, y }, center,
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

    let map_navigation_ticker = init_map_navigation(new_view.center());
    let key_handler_ticker = init_key_handler();
    let city_selection_ticker = init_city_selection(scale);
    let city_interface_ticker = init_city_interface();
    let city_sprites_ticker = init_city_sprites(y_sprite_offset + (scale * 5.));

    let mut selected_city = None;
    let mut city_interface = None;

    loop {
        let mut events = Vec::new();
        while let Some(event) = window.poll_event() {
            events.push(event);
        }

        map_navigation_ticker(&mut *new_view, &events);
        key_handler_ticker(&events);
        selected_city = city_selection_ticker(&new_view, &events, &hexagons, selected_city);
        city_interface = city_interface_ticker(selected_city, city_interface, &font);
        let cities = city_sprites_ticker(&hexagons, &city);

        window.set_view(&new_view);
        window.clear(Color::BLACK);

        sprites.iter().for_each(|sprite| window.draw(sprite));
        background_grid.iter().for_each(|shape| window.draw(shape));
        cities.iter().for_each(|shape| window.draw(shape));
        //texts.iter().for_each(|text| window.draw(text));
        if let Some(interface) = &city_interface {
            interface.draw(&mut window);
        }

        window.display();
    }
}
