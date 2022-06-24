use crate::{ControlEventFn, City};

fn increase_value<'t>(city: &mut City<'t>, resource: &'t str, amount: i32) {
    let original_amount = city.storage.get(&resource).unwrap_or(&0).clone();
    city.storage.insert(&resource, original_amount + amount);
}

pub fn init_city_storage_manager<'a>() -> ControlEventFn<'a> {
    ControlEventFn {
        func: Box::new(|mut state, _graphics, _event| {
            state.cities.iter_mut().for_each(|city| {
                let hexes = vec![
                    state.hexagons[(city.position.y) as usize][(city.position.x + 1) as usize],
                    state.hexagons[(city.position.y + 1) as usize][(city.position.x) as usize],
                    state.hexagons[(city.position.y) as usize][(city.position.x - 1) as usize],
                    state.hexagons[(city.position.y - 1) as usize][(city.position.x + 1) as usize],
                    state.hexagons[(city.position.y - 1) as usize][(city.position.x) as usize],
                    state.hexagons[(city.position.y - 1) as usize][(city.position.x - 1) as usize],
                ];

                println!("{:?} ", city.position);
                hexes.iter().for_each(|hex| {
                    println!("{:?} {:?}", hex.category, hex.grid_position);
                    increase_value(city, "gold", 1);
                });
            });

            state
        }),
        event: "NEW_TURN"
    }
}
