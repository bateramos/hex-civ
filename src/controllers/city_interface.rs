use sfml::{graphics::*, system::*};

use crate::{Hexagon, State};

struct Button <'a> {
    panel: RectangleShape<'a>,
    text: Text<'a>,
    on_click: Box<dyn Fn() -> ()>,
}

impl <'a> Button<'a> {
    pub fn draw(&self, render_target: &mut dyn RenderTarget) {
        render_target.draw(&self.panel);
        render_target.draw(&self.text);
    }
}

pub struct CityInterface <'a> {
    panel: RectangleShape<'a>,
    text: Text<'a>,
    left_pillar: Sprite<'a>,
    right_pillar: Sprite<'a>,
    exit_button: Button<'a>,
}

impl <'a> CityInterface<'a> {
    pub fn draw(&self, render_target: &mut dyn RenderTarget) {
        render_target.draw(&self.panel);
        render_target.draw(&self.text);
        render_target.draw(&self.left_pillar);
        render_target.draw(&self.right_pillar);

        self.exit_button.draw(render_target);
    }
}

pub fn init_city_interface(scale: f32, screen_size: Vector2f) -> Box<dyn for<'a, 'b> Fn(&'b Font, &'b Sprite, &View, State<'b>) -> State<'b>> {
    Box::new(move |font, pillar_sprite, view, mut state| {
        match state.selected_city {
            Some(_selected_city) => {
                if state.city_interface.is_none() {
                    let x0 = view.center().x - view.size().x / 2.;
                    let y0 = view.center().y - view.size().y / 2.;

                    let mut panel = RectangleShape::with_size(Vector2f { x: screen_size.x, y: 34. * scale });
                    panel.set_position(Vector2f { x: x0, y: y0 + 2. * scale });
                    panel.set_fill_color(Color::rgba(100, 16, 58, 91));

                    let mut text = Text::new(&format!("SUPER COOL CITY"), &font, (10. * scale) as u32);
                    text.set_position(Vector2f { x: x0 + (screen_size.x / 2.) - (text.global_bounds().width / 2.), y: y0 + 16. * scale });

                    let mut right_pillar = pillar_sprite.clone();
                    let mut left_pillar = pillar_sprite.clone();
                    right_pillar.set_position(Vector2f {x: x0 - 6., y: y0});
                    left_pillar.set_position(Vector2f {x: x0 + view.size().x - 40. * scale, y: y0});

                    let exit_button = {
                        let padding = 5. * scale;
                        let mut button_text = Text::new(&format!("EXIT"), &font, (10. * scale) as u32);
                        button_text.set_position(Vector2f { x: x0 + 90. * scale, y: y0 + screen_size.y - 30. * scale });

                        let mut button_panel = RectangleShape::with_size(Vector2f { x: button_text.global_bounds().width + padding * 4., y: button_text.global_bounds().height + padding * 2. });
                        button_panel.set_position(Vector2f { x: button_text.global_bounds().left - padding * 2., y: button_text.global_bounds().top - padding });
                        button_panel.set_fill_color(Color::rgb(60, 38, 49));

                        let on_click = || {
                        };

                        Button { panel: button_panel, text: button_text, on_click: Box::new(on_click) }
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
