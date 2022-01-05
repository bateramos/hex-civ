use sfml::{graphics::*, system::*, window::*};

use crate::{Hexagon, HexagonCategory, HexagonColumn};

pub fn init_city_placement(mut hexagons: HexagonColumn) -> HexagonColumn {
    let mut hexagon = hexagons[6].get_mut(7).unwrap();
    hexagon.category = HexagonCategory::CITY;

    hexagons
}
