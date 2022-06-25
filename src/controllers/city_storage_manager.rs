use crate::{ControlEventFn, City, HexagonColumn, Hexagon, Vector2u};

enum PositionAnchor {
    Top, TopRight, TopLeft,
    Bottom, BottomRight, BottomLeft,
}

fn increase_value<'t>(city: &mut City<'t>, resource: &'t str, amount: i32) {
    let original_amount = city.storage.get(&resource).unwrap_or(&0).clone();
    city.storage.insert(&resource, original_amount + amount);
}

fn hex_on<'a>(hexagons: &'a HexagonColumn, center: Vector2u, position: PositionAnchor) -> &'a Hexagon {
    let is_odd_add = if center.x % 2 != 0 { 1 } else { 0 };
    let is_even_subtract = if center.x % 2 == 0 { 1 } else { 0 };

    let target = match position {
        PositionAnchor::Top => (center.x, center.y - 1),
        PositionAnchor::TopRight => (center.x + 1, center.y - 1 + is_odd_add),
        PositionAnchor::TopLeft => (center.x - 1, center.y - 1 + is_odd_add),
        PositionAnchor::Bottom => (center.x, center.y + 1),
        PositionAnchor::BottomRight => (center.x + 1, center.y + 1 - is_even_subtract),
        PositionAnchor::BottomLeft => (center.x - 1, center.y + 1 - is_even_subtract),
    };

    &hexagons[target.1 as usize][target.0 as usize]
}

pub fn init_city_storage_manager<'a>() -> ControlEventFn<'a> {
    ControlEventFn {
        func: Box::new(|mut state, _graphics, _event| {
            state.cities.iter_mut().for_each(|city| {
                let hexes = vec![
                    hex_on(&state.hexagons, city.position, PositionAnchor::Top),
                    hex_on(&state.hexagons, city.position, PositionAnchor::TopRight),
                    hex_on(&state.hexagons, city.position, PositionAnchor::TopLeft),
                    hex_on(&state.hexagons, city.position, PositionAnchor::Bottom),
                    hex_on(&state.hexagons, city.position, PositionAnchor::BottomRight),
                    hex_on(&state.hexagons, city.position, PositionAnchor::BottomLeft),
                ];

                hexes.iter().for_each(|_hex| {
                    //println!("{:?} {:?}", hex.category, hex.grid_position);
                    increase_value(city, "gold", 1);
                });
            });

            state
        }),
        event: "NEW_TURN"
    }
}
