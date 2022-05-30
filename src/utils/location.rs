use sfml::system::*;

use crate::entities::{Hexagon, HexagonColumn};

fn calculate_distance(hex: &Hexagon, position: Vector2f) -> f32 {
    (
        (hex.center.x - position.x as f32).powf(2.) +
        (hex.center.y - position.y as f32).powf(2.)
    ).sqrt()
}

pub fn find_with_location(center: Vector2f, hexagons: &HexagonColumn) -> Option<Hexagon> {
    let mut closest_distance = f32::MAX;

    hexagons.iter().fold(None, |acc, line| {
        line.iter().fold(acc, |acc, hex| {
            let distance = calculate_distance(hex, center);
            if closest_distance > distance {
                closest_distance = distance;
                Some(*hex)
            } else {
                acc
            }
        })
    })
}
