use sfml::system::*;

use crate::entities::{Hexagon, HexagonColumn};

pub fn find_with_location(center: Vector2f, scale: f32, hexagons: &HexagonColumn) -> Option<Hexagon> {
    let mut closest = None;

    let mut x_index = (center.x / (40. * scale)) as usize;
    let mut y_index = (center.y / (10. * scale)) as usize;

    if x_index <= 0 {
        x_index = 1;
    }
    if y_index <= 0 {
        y_index = 1;
    }

    let mut candidates = Vec::new();

    if let Some(line) = hexagons.get(y_index + 1) {
        if let Some(hex) = line.get(x_index - 1) {
            candidates.push(hex);
        }
        if let Some(hex) = line.get(x_index) {
            candidates.push(hex);
        }
        if let Some(hex) = line.get(x_index + 1) {
            candidates.push(hex);
        }
    }
    if let Some(line) = hexagons.get(y_index) {
        if let Some(hex) = line.get(x_index) {
            candidates.push(hex);
        }
    }
    if let Some(line) = hexagons.get(y_index - 1) {
        if let Some(hex) = line.get(x_index - 1) {
            candidates.push(hex);
        }
        if let Some(hex) = line.get(x_index) {
            candidates.push(hex);
        }
        if let Some(hex) = line.get(x_index + 1) {
            candidates.push(hex);
        }
    }

    let mut closest_distance = f32::MAX;
    candidates.iter().for_each(|hex| {
        let distance = (
            (hex.center.x - center.x as f32).powf(2.) +
            (hex.center.y - center.y as f32).powf(2.)
        ).sqrt();

        if distance < closest_distance {
            closest = Some(**hex);
            closest_distance = distance;
        }
    });

    closest
}
