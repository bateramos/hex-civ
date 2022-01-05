use sfml::{graphics::*, system::*, window::*};

use crate::{HexagonColumn, HexagonCategory, Hexagon};

pub fn init_city_interface() -> Box<dyn for<'a> Fn(Option<Hexagon>, Vec<RectangleShape<'a>>, Vec<Text<'a>>, &'a Font) -> (Vec<RectangleShape<'a>>, Vec<Text<'a>>)> {
    Box::new(|selected_city, mut city_ui_components, mut city_ui_text, font| {
        match selected_city {
            Some(selected_city) => {
                if city_ui_components.is_empty() {
                    let center = selected_city.center;
                    let mut panel = RectangleShape::with_size(Vector2f { x: 400., y: 70. });
                    panel.set_position(Vector2f { x: center.x - 200., y: center.y - 100. });
                    panel.set_fill_color(Color::rgba(100, 16, 58, 91));
                    city_ui_components.push(panel);

                    let mut text = Text::new(&format!("SUPER COOL CITY"), &font, 30);
                    text.set_position(Vector2f { x: center.x - 170., y: center.y - 80. });
                    city_ui_text.push(text);
                }
            },
            None => {
                city_ui_components.clear();
                city_ui_text.clear();
            }
        }

        (city_ui_components, city_ui_text)
    })
}
