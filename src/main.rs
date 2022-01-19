use sfml::{graphics::*, system::*, window::*};
use sfml::SfBox;

use rand;

mod controllers;
mod entities;
mod utils;

use entities::*;

use controllers::*;
use controllers::{Drawable, Actionable};

pub struct GraphicsContext <'a> {
    pub textures: &'a LoadedTextures<'a>,
    pub font: &'a Font,
    pub view_center: Vector2f,
    pub view_size: Vector2f,
}

type ControlFn = Box<dyn for<'a> Fn(State<'a>, &GraphicsContext<'a>) -> State<'a>>;
type ControlGraphicsFn = Box<dyn for<'a> Fn(SfBox<View>, &State<'a>, &GraphicsContext<'a>) -> SfBox<View>>;

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

    let control_fns = vec![
        init_key_handler(),
        init_city_selection(scale),
        init_city_sprites(),
        init_city_interface(scale),
        init_unit_selection(scale),
        init_unit_sprite(scale),
        init_unit_selection_effect(),
        init_unit_movement(),
    ];

    let control_graphic_fns = vec![
        init_map_navigation(new_view.center()),
    ];

    let mut graphics = GraphicsContext {
        textures: &textures,
        font: &font,
        view_center: Vector2f{ x: 0., y: 0. },
        view_size: new_view.size(),
    };
    let mut state = State::new(hexagons, grid_size);
    state.units[4][4] = Some(UnitType::Pikeman);

    let mut clock = Clock::start();

    loop {
        clock.restart();

        let mut events = Vec::new();
        while let Some(event) = window.poll_event() {
            events.push(event);
        }

        state.events = events;

        new_view = control_graphic_fns.iter().fold(new_view, |new_view, fun| {
            (fun)(new_view, &state, &graphics)
        });

        graphics.view_center = new_view.center();

        state = control_fns.iter().fold(state, |state, fun| {
            (fun)(state, &graphics)
        });

        window.set_view(&new_view);
        window.clear(Color::BLACK);

        sprites.iter().for_each(|sprite: &Sprite| window.draw(sprite));
        background_grid.iter().for_each(|shape| window.draw(shape));
        state.cities.iter().for_each(|shape| window.draw(shape));
        state.units_sprites.iter().for_each(|shape| window.draw(shape));

        /*
        state.hexagons.iter().for_each(|line| {
            line.iter().for_each(|hex| {
                let mut text = Text::new(&format!("{:?}", hex.grid_position), &font, (4. * scale) as u32);
                text.set_fill_color(Color::RED);
                text.set_position(hex.center);
                window.draw(&text);
            });
        });
        */

        state.dispatched_events.clear();

        if let Some(interface) = &state.city_interface {
            interface.draw(&mut window);
            state.events.iter().for_each(|event| {
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
