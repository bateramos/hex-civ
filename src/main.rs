use sfml::{graphics::*, system::*, window::*};

use rand;

mod controllers;
mod entities;
mod utils;

use entities::*;

use controllers::*;
use controllers::{Drawable, Actionable};

type Resolution = (u32, u32, f32);

fn resolutions() -> Vec<Resolution> {
    vec![
        (2048, 1536, 5.),
        (800, 600, 3.),
    ]
}

fn main() {
    let res_index = &std::env::args().collect::<Vec<String>>()[1];
    let resolution : Resolution = resolutions()[res_index.parse::<usize>().unwrap_or(0)];
    let scale = resolution.2;
    let grid_size = (30, 20);
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
    let texture_pikeman = Texture::from_file("res/textures/pikeman.png").unwrap();

    let textures = init_textures(scale, &texture, &texture_pillar, &texture_pikeman);

    let (mut hexagons, sprites, background_grid) = init_map_creation(scale, seed, &textures);
    hexagons = init_city_placement(hexagons);

    println!("{:?}", new_view.size());

    let map_navigation_ticker = init_map_navigation(new_view.center());
    let key_handler_ticker = init_key_handler();
    let city_selection_ticker = init_city_selection(scale);
    let city_interface_ticker = init_city_interface(scale, new_view.size());
    let city_sprites_ticker = init_city_sprites();
    let unit_placement = init_unit_placement();
    let unit_sprite = init_unit_sprite(scale);
    let unit_selection = init_unit_selection(scale);
    let unit_selection_effect = init_unit_selection_effect();

    let mut state = State::new(hexagons, grid_size);
    state.units[4][4] = Some(UnitType::Pikeman);

    let mut clock = Clock::start();

    loop {
        clock.restart();
        let mut events = Vec::new();
        while let Some(event) = window.poll_event() {
            events.push(event);
        }

        state = map_navigation_ticker(&mut new_view, &events, state);
        state = key_handler_ticker(&events, state);
        state = city_selection_ticker(&new_view, &events, state);
        state = city_interface_ticker(&font, &textures, &new_view, state);
        state = city_sprites_ticker(state, &textures);
        state = unit_placement(state);
        state = unit_sprite(state, &textures);
        state = unit_selection(state, &events, &new_view);
        state = unit_selection_effect(state);

        window.set_view(&new_view);
        window.clear(Color::BLACK);

        sprites.iter().for_each(|sprite: &Sprite| window.draw(sprite));
        background_grid.iter().for_each(|shape| window.draw(shape));
        state.cities.iter().for_each(|shape| window.draw(shape));
        state.units_sprites.iter().for_each(|shape| window.draw(shape));

        state.dispatched_events.clear();

        if let Some(interface) = &state.city_interface {
            interface.draw(&mut window);
            events.iter().for_each(|event| {
                match event {
                    Event::MouseButtonPressed { button, .. } => {
                        if mouse::Button::LEFT == *button {
                            let mouse_position = window.map_pixel_to_coords_current_view(window.mouse_position());
                            if interface.exit_button.bounds().contains(mouse_position) {
                                state.dispatched_events.push(interface.exit_button.on_action());
                            }
                        }
                    },
                    _ => {},
                }
            });
        }

        state.tick_time = clock.elapsed_time().as_milliseconds() as f32;

        window.display();
    }
}
