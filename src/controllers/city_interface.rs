use sfml::{graphics::*, system::*};

use crate::Hexagon;

pub struct CityInterface <'a> {
    panel: RectangleShape<'a>,
    text: Text<'a>,
}

impl <'a> CityInterface<'a> {
    pub fn draw(&self, render_target: &mut dyn RenderTarget) {
        render_target.draw(&self.panel);
        render_target.draw(&self.text);
    }
}

pub fn init_city_interface() -> Box<dyn for<'a> Fn(Option<Hexagon>, Option<CityInterface<'a>>, &'a Font) -> Option<CityInterface<'a>>> {
    Box::new(|selected_city, mut city_interface, font| {
        match selected_city {
            Some(selected_city) => {
                if city_interface.is_none() {
                    let center = selected_city.center;
                    let mut panel = RectangleShape::with_size(Vector2f { x: 400., y: 70. });
                    panel.set_position(Vector2f { x: center.x - 200., y: center.y - 100. });
                    panel.set_fill_color(Color::rgba(100, 16, 58, 91));

                    let mut text = Text::new(&format!("SUPER COOL CITY"), &font, 30);
                    text.set_position(Vector2f { x: center.x - 170., y: center.y - 80. });

                    city_interface.replace(CityInterface { panel, text });
                }
            },
            None => {
                city_interface.take();
            }
        }
        city_interface
    })
}
