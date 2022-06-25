use sfml::{graphics::*, system::*};

use crate::controllers::MOUSE_CLICK_RIGHT;
use crate::utils::find_with_location;
use crate::entities::{HexEvent, Unit, UnitType};
use crate::{ControlFn, ControlActionFn, ControlEventFn};

enum Anchor {
    TOP, BOTTOM
}

pub trait Drawable {
    fn draw(&self, render_target: &mut dyn RenderTarget);
}

pub trait Actionable {
    fn on_action(&self) -> String;
    fn bounds(&self) -> FloatRect;
}

pub trait ActionButton: Actionable + Drawable {}

pub struct Button <'a> {
    panel: RectangleShape<'a>,
    text: Text<'a>,
    on_click: String,
}

impl <'a> ActionButton for Button<'a> {}

impl <'a> Drawable for Button<'a> {
    fn draw(&self, render_target: &mut dyn RenderTarget) {
        render_target.draw(&self.panel);
        render_target.draw(&self.text);
    }
}

impl <'a> Actionable for Button<'a> {
    fn on_action(&self) -> String {
        self.on_click.clone()
    }

    fn bounds(&self) -> FloatRect {
        self.panel.global_bounds()
    }
}

pub struct CityInterface <'a> {
    panel: RectangleShape<'a>,
    text: Text<'a>,
    left_pillar: Sprite<'a>,
    right_pillar: Sprite<'a>,
    city_hex_position: Vector2u,
    pub exit_button: Button<'a>,
    pub build_unit_buttons: Vec<Button<'a>>,
}

impl <'a> Drawable for CityInterface<'a> {
    fn draw(&self, render_target: &mut dyn RenderTarget) {
        render_target.draw(&self.panel);
        render_target.draw(&self.text);
        render_target.draw(&self.left_pillar);
        render_target.draw(&self.right_pillar);
        self.exit_button.draw(render_target);
        self.build_unit_buttons.iter().for_each(|button| button.draw(render_target));
    }
}

impl <'a> CityInterface <'a> {
    pub fn action_on(&self, mouse_position: Vector2f) -> Option<HexEvent> {
        if let Some(button) = self.build_unit_buttons.iter().find(|button| button.bounds().contains(mouse_position)) {
            Some(HexEvent::new_from_position(&button.on_click.to_owned(), self.city_hex_position))
        } else if self.exit_button.bounds().contains(mouse_position) {
            Some(HexEvent::new(&self.exit_button.on_click))
        } else {
            None
        }
    }
}

pub const CITY_INTERFACE_INIT_EVENT : &str = "CITY_INTERFACE_INIT_EVENT";
pub const CITY_INTERFACE_EXIT_EVENT : &str = "CITY_INTERFACE_EXIT";
pub const CITY_INTERFACE_BUILD_UNIT_EVENT : &str = "CITY_INTERFACE_BUILD_UNIT";
pub const CITY_INTERFACE_BUILD_SETTLER_EVENT : &str = "CITY_INTERFACE_BUILD_SETTLER_EVENT";

pub fn init_city_unit_construction() -> ControlFn {
    Box::new(|mut state, _graphics| {
        if let Some(event) = state.has_event_triggered(CITY_INTERFACE_BUILD_UNIT_EVENT) {
            if let Some(position) = event.position {
                state.units.push(Unit::new_with_type(position, UnitType::Pikeman));
            }
        } else if let Some(event) = state.has_event_triggered(CITY_INTERFACE_BUILD_SETTLER_EVENT) {
            if let Some(position) = event.position {
                state.units.push(Unit::new_with_type(position, UnitType::Settler));
            }
        }
        state
    })
}

pub fn init_city_interface_creation<'a>(scale: f32) -> ControlEventFn<'a> {
    ControlEventFn {
        event: CITY_INTERFACE_INIT_EVENT,
        func: Box::new(move |mut state, graphics, _event| {
            let center = graphics.view_center;
            let hex = find_with_location(center, &state.hexagons).unwrap();

            let city_hex_position = Vector2u {
                x: hex.grid_position.0, y: hex.grid_position.1
            };
            let view_size = graphics.view_size;
            let view_center = graphics.view_center;

            let x0 = view_center.x - view_size.x / 2.;
            let y0 = view_center.y - view_size.y / 2.;

            let create_button = |event: &str, text: &str, x: f32, y: f32, anchor: Anchor| {
                let padding = 5. * scale;
                let y_anchor = match anchor {
                    Anchor::TOP => 0.,
                    Anchor::BOTTOM => view_size.y,
                };

                let mut button_text = Text::new(text, &graphics.font, (10. * scale) as u32);
                button_text.set_position(Vector2f { x: x0 + x * scale, y: y0 + y_anchor + y * scale });

                let mut button_panel = RectangleShape::with_size(Vector2f { x: button_text.global_bounds().width + padding * 4., y: button_text.global_bounds().height + padding * 2. });
                button_panel.set_position(Vector2f { x: button_text.global_bounds().left - padding * 2., y: button_text.global_bounds().top - padding });
                button_panel.set_fill_color(Color::rgb(60, 38, 49));

                Button { panel: button_panel, text: button_text, on_click: event.to_owned() }
            };

            let mut panel = RectangleShape::with_size(Vector2f { x: view_size.x, y: 34. * scale });
            panel.set_position(Vector2f { x: x0, y: y0 + 2. * scale });
            panel.set_fill_color(Color::rgba(100, 16, 58, 91));

            let mut text = Text::new(&format!("SUPER COOL CITY"), &graphics.font, (10. * scale) as u32);
            text.set_position(Vector2f { x: x0 + (view_size.x / 2.) - (text.global_bounds().width / 2.), y: y0 + 16. * scale });

            let pillar_sprite = graphics.textures.pillar.clone();

            let mut right_pillar = pillar_sprite.clone();
            let mut left_pillar = pillar_sprite.clone();
            right_pillar.set_position(Vector2f {x: x0 - 6., y: y0});
            left_pillar.set_position(Vector2f {x: x0 + view_size.x - 40. * scale, y: y0});

            let exit_button = create_button(CITY_INTERFACE_EXIT_EVENT, "EXIT", 90., -30., Anchor::BOTTOM);
            let build_unit_button = create_button(CITY_INTERFACE_BUILD_UNIT_EVENT, "BUILD UNIT", 30., 90., Anchor::TOP);
            let build_settler_button = create_button(CITY_INTERFACE_BUILD_SETTLER_EVENT, "BUILD SETTLER", 30., 120., Anchor::TOP);

            let build_unit_buttons = vec![
                build_unit_button,
                build_settler_button,
            ];

            state.city_interface.replace(CityInterface { city_hex_position, panel, text, right_pillar, left_pillar, exit_button, build_unit_buttons });

            state
        })
    }
}

pub fn init_city_exit_handler<'a>() -> ControlEventFn<'a> {
    ControlEventFn { 
        event: CITY_INTERFACE_EXIT_EVENT,
        func: Box::new(|mut state, _graphics, _event| {
            state.city_selected.take();
            state.city_interface.take();
            state
        })
    }
}

pub fn init_city_mouse_right_click() -> ControlActionFn {
    Box::new(|state, _graphics| {
        if state.city_selected.is_some() {
            if let Some(_) = state.has_event_triggered(MOUSE_CLICK_RIGHT) {
                Some(HexEvent::new(CITY_INTERFACE_EXIT_EVENT))
            } else {
                None
            }
        } else {
            None
        }
    })
}

pub fn init_city_interface() -> ControlActionFn {
    Box::new(|state, _graphics| {
        match state.city_selected {
            Some(_) => {
                if state.city_interface.is_none() {
                    Some(HexEvent::new(CITY_INTERFACE_INIT_EVENT))
                } else {
                    None
                }
            },
            _ => None
        }
    })
}
