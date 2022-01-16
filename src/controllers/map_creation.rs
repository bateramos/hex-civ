use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use sfml::{graphics::*, system::*};

use crate::entities::{HexagonLine, HexagonColumn, Hexagon, HexagonCategory};
use crate::controllers::LoadedTextures;

fn generate_random<H>(args: Vec<H>, seed: u64) -> u32 where H: Hash {
    let mut hasher = DefaultHasher::new();
    let hash = args.iter().fold(1, |acc, item| {
        item.hash(&mut hasher);
        let hash = hasher.finish();
        acc + (hash % 10000)
    });

    (hash * seed) as u32 % 100
}

pub fn init_map_creation<'a>(scale: f32, seed: u64, textures: &'a LoadedTextures) -> (HexagonColumn, Vec<Sprite<'a>>, Vec<CustomShape<'a>>) {
    let grid_size = (30, 20);

    let mut hexagons = HexagonColumn::new();
    let mut background_grid = Vec::new();
    let mut sprites = Vec::new();

    for y_i in 0..grid_size.0 {
        let mut line = HexagonLine::new();
        let top = 9. * scale * y_i as f32;
        let y = y_i as f32 * scale + top;

        let padding = if y_i % 2 == 0 {
            -15. * scale
        } else {
            5. * scale
        };

        for x_i in 0..grid_size.1 {
            let bottom = 10. * scale * x_i as f32;
            let x = x_i as f32 * 30. * scale + bottom + padding;

            let mut sprite = if y_i == 1 || y_i == 0 {
                let mut sprite = match generate_random(vec![y_i, x_i], seed) {
                    00 ..= 10 => textures.mountain.clone(),
                    19 ..= 40 => textures.snow_with_tress.clone(),
                    _ => textures.snow.clone(),
                };

                sprite.set_scale(Vector2f {x: 0.9 * scale, y: 0.9 * scale });
                sprite
            } else {
                let mut sprite = match generate_random(vec![y_i, x_i], seed) {
                    0 ..= 8 => textures.dense_forest.clone(),
                    21 ..= 25 => textures.hill.clone(),
                    26 ..= 27 => textures.mountain.clone(),
                    29 ..= 32 => textures.hill_with_trees.clone(),
                    80 ..= 98 => textures.forest.clone(),
                    _ => textures.green_field.clone(),
                };

                sprite.set_scale(Vector2f {x: 0.9 * scale, y: 0.8 * scale });
                sprite
            };

            let sprite_position = Vector2f {x, y: y - 15.8 * scale};
            let center = Vector2f { x: x + 15. * scale, y: y + 10. * scale };

            sprite.set_position(sprite_position);

            let hexagon = Hexagon {
                id: rand::random::<u32>(), category: HexagonCategory::FIELD,
                scale, position: Vector2f { x, y }, center, sprite_position,
                fill_color: Color::TRANSPARENT, outline_color: Color::rgba(86, 84, 85, 51), thickness: 1.,
            };

            let mut shape = CustomShape::new(Box::new(hexagon));
            shape.set_fill_color(hexagon.fill_color);
            shape.set_outline_color(hexagon.outline_color);
            shape.set_outline_thickness(hexagon.thickness);

            background_grid.push(shape);
            sprites.push(sprite);

            line.push(hexagon);
        }

        hexagons.push(line);
    }

    (hexagons, sprites, background_grid)
}
