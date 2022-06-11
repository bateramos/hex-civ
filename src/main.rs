use sfml::{graphics::*, system::*, window::*};
use sfml::SfBox;

use rand;

mod controllers;
mod entities;
mod utils;

use entities::*;

use controllers::*;
use controllers::Drawable;

pub struct GraphicsContext <'a> {
    pub textures: &'a LoadedTextures<'a>,
    pub font: &'a Font,
    pub view_center: Vector2f,
    pub view_size: Vector2f,
}

pub struct EventStateFn <'a> {
    pub func: Box<dyn Fn(State<'a>, &GraphicsContext<'a>, HexEvent) -> State<'a>>,
    pub event: &'static str,
}

pub struct EventGraphicFn <'a> {
    pub func: Box<dyn Fn(SfBox<View>, &State<'a>, &GraphicsContext<'a>, HexEvent) -> SfBox<View>>,
    pub event: &'static str,
}

type ControlEventFn <'a> = EventStateFn<'a>;
type ControlEventGraphicFn <'a> = EventGraphicFn<'a>;
type ControlFn = Box<dyn for<'a> Fn(State<'a>, &GraphicsContext<'a>) -> State<'a>>;
type ControlActionFn = Box<dyn for<'a> Fn(&State<'a>, &GraphicsContext<'a>) -> Option<HexEvent>>;
type ControlActionsFn = Box<dyn for<'a> Fn(&State<'a>, &GraphicsContext<'a>) -> Option<Vec<HexEvent>>>;
type ControlGraphicsFn = Box<dyn for<'a> Fn(SfBox<View>, &State<'a>, &GraphicsContext<'a>) -> SfBox<View>>;

type Resolution = (u32, u32, f32);
type GridSize = (usize, usize);

fn resolutions() -> Vec<Resolution> {
    vec![
        (2048, 1536, 5.),
        (800, 600, 3.),
    ]
}

fn move_configs() -> Vec<MoveKeyboardConfig> {
    vec![
        MoveKeyboardConfig {
            top_left: Key::U, top: Key::I, top_right: Key::O,
            bottom_left: Key::J, bottom: Key::K, bottom_right: Key::L
        },
        MoveKeyboardConfig {
            top_left: Key::NUMPAD7, top: Key::NUMPAD8, top_right: Key::NUMPAD9,
            bottom_left: Key::NUMPAD4, bottom: Key::NUMPAD5, bottom_right: Key::NUMPAD6
        },
    ]
}

fn main() {
    let res_index = &std::env::args().collect::<Vec<String>>()[1];
    let resolution : Resolution = resolutions()[res_index.parse::<usize>().unwrap_or(0)];
    let unit_control_index = &std::env::args().collect::<Vec<String>>()[2];
    let unit_controls : MoveKeyboardConfig = move_configs().remove(unit_control_index.parse::<usize>().unwrap_or(0));
    let order_controls : OrderKeyboardConfig = OrderKeyboardConfig {
        build_city: Key::B, build_farm_field: Key::I, transform: Key::T,
    };
    let scale = resolution.2;
    let grid_size : GridSize = (30, 20);
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
    let mut new_view = View::from_rect(&FloatRect::new(0.,0., resolution.0 as f32, resolution.1 as f32));
    window.set_view(&new_view);

    let font = Font::from_file("res/fonts/Seagram tfb.ttf").unwrap();
    let texture = Texture::from_file("res/textures/main.png").unwrap();
    let texture_pillar = Texture::from_file("res/textures/pillar.png").unwrap();
    let texture_pikeman = Texture::from_file("res/textures/pikeman.png").unwrap();
    let texture_peasant = Texture::from_file("res/textures/peasant.png").unwrap();

    let textures = init_textures(scale, &texture, &texture_pillar, &texture_pikeman, &texture_peasant);

    let control_fns = vec![
        init_key_handler(),
        init_city_selection(),
        init_city_sprites(),
        init_city_unit_construction(),
        init_unit_selection(),
        init_unit_sprite(scale),
        init_unit_selection_effect(),
        init_unit_deselection_effect(),
        init_unit_placement(),
        init_hex_improvement_sprite(scale),
    ];

    let control_graphic_fns = vec![
        init_map_navigation(new_view.center()),
    ];

    let control_event_fns = vec![
        init_map_sprite_start_event(),
        init_mouse_button_handler(),
        init_city_interface(),
        init_city_mouse_right_click(),
        init_unit_deselection(),
        init_unit_movement_event(unit_controls),
    ];

    let control_events_fns = vec![
        init_unit_order(order_controls),
    ];

    let control_hex_event_functions = vec![
        init_map_sprite_allocation(scale),
        init_unit_deselection_handler(),
        init_unit_transform_hex_event(),
        init_city_build_event(),
        init_city_interface_creation(scale),
        init_city_exit_handler(),
        init_unit_movement(grid_size.clone()),
        init_hex_improvement_build_event(),
    ];

    let control_hex_event_graphic_functions = vec![
        init_map_unit_follow(scale),
    ];

    let mut graphics = GraphicsContext {
        textures: &textures,
        font: &font,
        view_center: Vector2f{ x: 0., y: 0. },
        view_size: new_view.size(),
    };

    //let (_hexagons, _sprites, _background_grid) = init_map_creation(scale, seed, &textures, &grid_size);
    let hexagons = create_map_hex(scale, seed, &grid_size);

    //let selected_city = hexagons[3][3];

    let mut state = State::new(hexagons, grid_size);
    state.units.push(Unit::new_with_type(Vector2i { x: 5, y: 5 }, UnitType::Settler));
    state.cities.push(City::new(Vector2i { x: 2, y: 10 }));
    //state.city_selected = Some(state.get_city_on_hex(&selected_city).unwrap().id);
    state.unit_selected = Some(state.units[0].id);
    state.hex_improvements.push(HexImprovement::new(Vector2i { x: 4, y: 5 }));

    state.dispatched_events.push(HexEvent::new("INIT_MAP_HEX_SPRITES"));

    let mut clock = Clock::start();

    loop {
        clock.restart();

        let mut events = Vec::new();
        while let Some(event) = window.poll_event() {
            events.push(event);
        }

        state.events = events;

        state = control_fns.iter().fold(state, |state, fun| {
            (fun)(state, &graphics)
        });

        new_view = control_graphic_fns.iter().fold(new_view, |new_view, fun| {
            (fun)(new_view, &state, &graphics)
        });

        graphics.view_center = new_view.center();

        window.set_view(&new_view);
        window.clear(Color::BLACK);

        let mut improve_iter = state.hex_improvements.iter();
        let mut hex_improvement : Option<&HexImprovement> = improve_iter.next();
        let mut improvement_to_render : Vec<&HexImprovement> = Vec::new();
        let mut last_y = 0;
        state.map_sprites.iter().enumerate().for_each(|(index, sprite)| {
            let y = index / grid_size.0;
            let x = index % grid_size.0;

            if last_y != y {
                improvement_to_render.retain(|imp| {
                    window.draw(imp.sprite.as_ref().unwrap());
                    false
                });
                last_y = y;
            }

            if let Some(improvement) = &hex_improvement {
                let x_found = improvement.position.x as usize == x;
                let y_found = improvement.position.y as usize == y;
                if x_found && y_found {
                    improvement_to_render.push(improvement);
                    hex_improvement = improve_iter.next();
                }
            }

            window.draw(sprite);
        });
        improvement_to_render.iter().for_each(|imp|
            window.draw(imp.sprite.as_ref().unwrap())
        );

        //background_grid.iter().for_each(|shape| window.draw(shape));
        state.cities.iter().filter(|city| city.sprite.is_some()).for_each(|city| window.draw(city.sprite.as_ref().unwrap()));
        state.units.iter().filter(|unit| unit.sprite.is_some()).for_each(|unit| window.draw(unit.sprite.as_ref().unwrap()));

        state.hexagons.iter().for_each(|line| {
            line.iter().for_each(|hex| {
                let mut text = Text::new(&format!("{:?}", hex.grid_position), &font, (4. * scale) as u32);
                text.set_fill_color(Color::RED);
                text.set_position(hex.center);
                window.draw(&text);
            });
        });

        if !state.map_sprites.is_empty() {
            state.dispatched_events.clear();
        }

        if let Some(interface) = &state.city_interface {
            interface.draw(&mut window);
            state.events.iter().for_each(|event| {
                match event {
                    Event::MouseButtonPressed { button, .. } => {
                        if mouse::Button::LEFT == *button {
                            let mouse_position = window.map_pixel_to_coords_current_view(window.mouse_position());
                            if let Some(action) = interface.action_on(mouse_position) {
                                state.dispatched_events.push(action);
                            }
                        }
                    },
                    _ => {},
                }
            });
        }

        control_event_fns.iter().for_each(|fun| {
            if let Some(event) = (fun)(&state, &graphics) {
                state.dispatched_events.push(event);
            }
        });

        control_events_fns.iter().for_each(|fun| {
            if let Some(events) = (fun)(&state, &graphics) {
                state.dispatched_events.extend(events);
            }
        });

        if !state.dispatched_events.is_empty() {
            println!("{:?}", state.dispatched_events);
            state = control_hex_event_functions.iter().fold(state, |state, controller| {
                if let Some(event) = state.has_event_triggered(controller.event) {
                    let event = event.clone();
                    (controller.func)(state, &graphics, event)
                } else {
                    state
                }
            });
            new_view = control_hex_event_graphic_functions.iter().fold(new_view, |new_view, controller| {
                if let Some(event) = state.has_event_triggered(controller.event) {
                    let event = event.clone();
                    (controller.func)(new_view, &state, &graphics, event)
                } else {
                    new_view
                }
            });
        }

        state.tick_time = clock.elapsed_time().as_milliseconds() as f32;

        window.display();
    }
}
