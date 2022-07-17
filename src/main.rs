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
type GridSize = (u32, u32);

fn resolutions(index: usize) -> Resolution {
    vec![
        (2048, 1536, 5.),
        (800, 600, 3.),
    ].remove(index)
}

fn move_configs(index: usize) -> MoveKeyboardConfig {
    vec![
        MoveKeyboardConfig {
            top_left: Key::U, top: Key::I, top_right: Key::O,
            bottom_left: Key::J, bottom: Key::K, bottom_right: Key::L
        },
        MoveKeyboardConfig {
            top_left: Key::NUMPAD7, top: Key::NUMPAD8, top_right: Key::NUMPAD9,
            bottom_left: Key::NUMPAD4, bottom: Key::NUMPAD5, bottom_right: Key::NUMPAD6
        },
    ].remove(index)
}

fn order_configs(index: usize) -> OrderKeyboardConfig {
    vec![
        OrderKeyboardConfig {
            build_city: Key::B, build_farm_field: Key::F, build_mine: Key::M, transform: Key::T,
        },
        OrderKeyboardConfig {
            build_city: Key::B, build_farm_field: Key::I, build_mine: Key::M, transform: Key::T,
        },
    ].remove(index)
}

fn main() {
    let args = &std::env::args().collect::<Vec<String>>();
    let res_index = args[1].parse::<usize>().unwrap_or(0);
    let control_index = args[2].parse::<usize>().unwrap_or(0);

    let resolution : Resolution = resolutions(res_index);
    let unit_controls : MoveKeyboardConfig = move_configs(control_index);
    let order_controls : OrderKeyboardConfig = order_configs(control_index);
    let map_config : MapKeyboardConfig = MapKeyboardConfig { new_turn: Key::SPACE };
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
    let texture_mine = Texture::from_file("res/textures/mine.png").unwrap();
    let texture_pillar = Texture::from_file("res/textures/pillar.png").unwrap();
    let texture_pikeman = Texture::from_file("res/textures/pikeman.png").unwrap();
    let texture_peasant = Texture::from_file("res/textures/peasant.png").unwrap();

    let textures = init_textures(scale, &texture, &texture_mine, &texture_pillar, &texture_pikeman, &texture_peasant);

    let control_graphic_fns : Vec<ControlGraphicsFn> = vec![
        init_map_navigation(new_view.center()),
    ];

    let control_fns : Vec<ControlFn> = vec![
        init_key_handler(),
        init_city_selection(),
        init_city_sprites(),
        init_city_unit_construction(),
        init_unit_selection(),
        init_unit_sprite(scale),
        init_unit_selection_effect(),
        init_unit_deselection_effect(),
        init_unit_placement(),
    ];

    // Events Cleared

    let control_events_fns : Vec<ControlActionsFn> = vec![
        init_unit_order(order_controls),
        init_hex_improvement_sprite_reload(),
    ];

    let control_event_fns : Vec<ControlActionFn> = vec![
        init_turn_event(map_config),
        init_map_sprite_start_event(),
        init_mouse_button_handler(),
        init_city_interface(),
        init_city_mouse_right_click(),
        init_unit_deselection(),
        init_unit_movement_event(unit_controls),
        init_unit_transform_hex_event_refresh_map(),
    ];

    let control_hex_event_functions : Vec<EventStateFn> = vec![
        init_unit_deselection_handler(),
        init_unit_transform_hex_event(),
        init_unit_movement(grid_size.clone()),
        init_city_build_event(),
        init_city_storage_manager(),
        init_city_interface_creation(scale),
        init_city_exit_handler(),
        init_hex_improvement_build_farm_event(),
        init_hex_improvement_build_mine_event(),
        init_hex_improvement_sprite(scale),
        init_map_sprite_allocation(scale),
    ];

    let control_hex_event_graphic_functions : Vec<EventGraphicFn> = vec![
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
    state.units.push(Unit::new_with_type(Vector2u { x: 5, y: 5 }, UnitType::Settler));
    let city = City::new(Vector2u { x: 3, y: 9 });
    state.cities.push(city);
    //state.city_selected = Some(state.get_city_on_hex(&selected_city).unwrap().id);
    state.unit_selected = Some(state.units[0].id);

    state.dispatched_events.push(HexEvent::new("INIT_MAP_HEX_SPRITES"));

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

        state.map_sprites.iter().for_each(|hex_sprite| {
            hex_sprite.sprites.iter().for_each(|sprite| window.draw(sprite));
        });

        //background_grid.iter().for_each(|shape| window.draw(shape));
        state.cities.iter().filter(|city| city.sprite.is_some()).for_each(|city| window.draw(city.sprite.as_ref().unwrap()));
        state.units.iter().filter(|unit| unit.sprite.is_some()).for_each(|unit| window.draw(unit.sprite.as_ref().unwrap()));

        /*state.hexagons.iter().for_each(|line| {
            line.iter().for_each(|hex| {
                let mut text = Text::new(&format!("{:?}", hex.grid_position), &font, (4. * scale) as u32);
                text.set_fill_color(Color::RED);
                text.set_position(hex.center);
                window.draw(&text);
            });
        });*/

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

        control_events_fns.iter().for_each(|fun| {
            if let Some(events) = (fun)(&state, &graphics) {
                state.dispatched_events.extend(events);
            }
        });

        control_event_fns.iter().for_each(|fun| {
            if let Some(event) = (fun)(&state, &graphics) {
                state.dispatched_events.push(event);
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
