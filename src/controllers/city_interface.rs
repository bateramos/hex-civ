use sfml::{graphics::*, system::*};

use crate::ControlFn;

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
    pub exit_button: Button<'a>,
}

impl <'a> Drawable for CityInterface<'a> {
    fn draw(&self, render_target: &mut dyn RenderTarget) {
        render_target.draw(&self.panel);
        render_target.draw(&self.text);
        render_target.draw(&self.left_pillar);
        render_target.draw(&self.right_pillar);
        self.exit_button.draw(render_target);
    }
}

pub const CITY_INTERFACE_EXIT_EVENT : &str = "city_interface_exit";

pub fn init_city_interface(scale: f32) -> ControlFn {
    Box::new(move |mut state, graphics| {
        if state.dispatched_events.contains(&CITY_INTERFACE_EXIT_EVENT.to_owned()) {
            state.selected_city.take();
        }

        match state.selected_city {
            Some(_selected_city) => {
                if state.city_interface.is_none() {
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

                    state.city_interface.replace(CityInterface { panel, text, right_pillar, left_pillar, exit_button });
                }
            },
            None => {
                state.city_interface.take();
            }
        }
        state
    })
}
