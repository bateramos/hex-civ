use crate::entities::{HexagonCategory, HexagonColumn};

pub fn init_city_placement(mut hexagons: HexagonColumn) -> HexagonColumn {
    let mut hexagon = hexagons[3].get_mut(3).unwrap();
    hexagon.category = HexagonCategory::City;

    hexagons
}
