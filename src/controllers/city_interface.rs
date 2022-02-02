use sfml::{graphics::*, system::*};

use crate::controllers::MOUSE_CLICK_RIGHT;
use crate::utils::find_with_location;
use crate::entities::{HexEvent, Unit};
use crate::{ControlFn, ControlActionFn};

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
    city_hex_position: Vector2i,
    pub exit_button: Button<'a>,
    pub build_unit_button: Button<'a>,
}

impl <'a> Drawable for CityInterface<'a> {
    fn draw(&self, render_target: &mut dyn RenderTarget) {
        render_target.draw(&self.panel);
        render_target.draw(&self.text);
        render_target.draw(&self.left_pillar);
        render_target.draw(&self.right_pillar);
        self.exit_button.draw(render_target);
        self.build_unit_button.draw(render_target);
    }
}

impl <'a> CityInterface <'a> {
    pub fn action_on(&self, mouse_position: Vector2f) -> Option<HexEvent> {
        if self.exit_button.bounds().contains(mouse_position) {
            Some(HexEvent::new(&self.exit_button.on_click))
        } else if self.build_unit_button.bounds().contains(mouse_position) {
            Some(HexEvent { position: Some(self.city_hex_position), name: self.build_unit_button.on_click.to_owned() })
        } else {
            None
        }
    }
}

pub const CITY_INTERFACE_INIT_EVENT : &str = "CITY_INTERFACE_INIT_EVENT";
pub const CITY_INTERFACE_EXIT_EVENT : &str = "city_interface_exit";
pub const CITY_INTERFACE_BUILD_UNIT_EVENT : &str = "city_interface_build_unit";

pub fn init_city_unit_construction() -> ControlFn {
    Box::new(|mut state, _graphics| {
        if let Some(event) = state.has_event_triggered(CITY_INTERFACE_BUILD_UNIT_EVENT) {
            if let Some(position) = event.position {
                state.units.push(Unit::new(position));
            }
        }
        state
    })
}

pub fn init_city_interface_creation(scale: f32) -> ControlFn {
    Box::new(move |mut state, graphics| {
        if let Some(_) = state.has_event_triggered(CITY_INTERFACE_INIT_EVENT) {
            let center = graphics.view_center;
            let hex = find_with_location(center, scale, &state.hexagons).unwrap();

            let city_hex_position = Vector2i {
                x: hex.grid_position.0 as i32, y: hex.grid_position.1 as i32
            };
            let view_size = graphics.view_size;
            let view_center = graphics.view_center;

            let x0 = view_center.x - view_size.x / 2.;
            let y0 = view_center.y - view_size.y / 2.;

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

            let exit_button = {
                let padding = 5. * scale;
                let mut button_text = Text::new(&format!("EXIT"), &graphics.font, (10. * scale) as u32);
                button_text.set_position(Vector2f { x: x0 + 90. * scale, y: y0 + view_size.y - 30. * scale });

                let mut button_panel = RectangleShape::with_size(Vector2f { x: button_text.global_bounds().width + padding * 4., y: button_text.global_bounds().height + padding * 2. });
                button_panel.set_position(Vector2f { x: button_text.global_bounds().left - padding * 2., y: button_text.global_bounds().top - padding });
                button_panel.set_fill_color(Color::rgb(60, 38, 49));

                Button { panel: button_panel, text: button_text, on_click: CITY_INTERFACE_EXIT_EVENT.to_owned() }
            };

            let build_unit_button = {
                let padding = 5. * scale;
                let mut button_text = Text::new(&format!("BUILD UNIT"), &graphics.font, (10. * scale) as u32);
                button_text.set_position(Vector2f { x: x0 + 30. * scale, y: y0 + 90. * scale });

                let mut button_panel = RectangleShape::with_size(Vector2f { x: button_text.global_bounds().width + padding * 4., y: button_text.global_bounds().height + padding * 2. });
                button_panel.set_position(Vector2f { x: button_text.global_bounds().left - padding * 2., y: button_text.global_bounds().top - padding });
                button_panel.set_fill_color(Color::rgb(60, 38, 49));

                Button { panel: button_panel, text: button_text, on_click: CITY_INTERFACE_BUILD_UNIT_EVENT.to_owned() }
            };

            state.city_interface.replace(CityInterface { city_hex_position, panel, text, right_pillar, left_pillar, exit_button, build_unit_button });
        }
        state
    })
}

pub fn init_city_exit_handler() -> ControlFn {
    Box::new(|mut state, _graphics| {
        if let Some(_event) = state.has_event_triggered(CITY_INTERFACE_EXIT_EVENT) {
            state.city_selected.take();
            state.city_interface.take();
        }
        state
    })
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
