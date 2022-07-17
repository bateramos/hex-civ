use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use sfml::{graphics::*, system::*};

use crate::{GridSize, EventStateFn, ControlActionFn, State};
use crate::entities::{HexagonLine, HexagonColumn, Hexagon, HexagonCategory, HexEvent, HexImprovement};

pub struct HexagonSprites <'a> {
    pub id: u32,
    pub sprites: Vec<Sprite<'a>>,
}

fn generate_random<H>(args: Vec<H>, seed: u64) -> u32 where H: Hash {
    let mut hasher = DefaultHasher::new();
    let hash = args.iter().fold(1, |acc, item| {
        item.hash(&mut hasher);
        let hash = hasher.finish();
        acc + (hash % 10000)
    });

    (hash * seed) as u32 % 100
}

pub fn create_map_hex<'a>(scale: f32, seed: u64, grid_size: &GridSize) -> HexagonColumn {
    let mut hexagons = HexagonColumn::new();

    let padding = 5. * scale;

    for y_i in 0..grid_size.1 {
        let mut line = HexagonLine::new();
        let top = 20. * scale * y_i as f32;
        let y = y_i as f32 * scale + top;


        for x_i in 0..grid_size.0 {
            let bottom = -10. * scale * x_i as f32;
            let x = x_i as f32 * 30. * scale + padding + bottom;

            let y = if x_i % 2 == 0 {
                y - 10. * scale
            } else {
                y
            };

            let center = Vector2f { x: x + 15. * scale, y: y + 10. * scale };

            let category = if y_i == 1 || y_i == 0 {
                match generate_random(vec![y_i, x_i], seed) {
                    00 ..= 10 => HexagonCategory::Mountain,
                    19 ..= 40 => HexagonCategory::SnowWithTrees,
                    _ => HexagonCategory::Snow,
                }
            } else {
                match generate_random(vec![y_i, x_i], seed) {
                    0 ..= 8 => HexagonCategory::DenseForest,
                    21 ..= 25 => HexagonCategory::Hill,
                    26 ..= 27 => HexagonCategory::Mountain,
                    29 ..= 32 => HexagonCategory::HillWithTrees,
                    80 ..= 98 => HexagonCategory::Forest,
                    _ => HexagonCategory::Field,
                }
            };

            let hexagon = Hexagon {
                id: rand::random::<u32>(), category, grid_position: (x_i, y_i),
                scale, position: Vector2f { x, y }, center, sprite_position: None,
                fill_color: Color::TRANSPARENT, outline_color: Color::rgba(86, 84, 85, 51), thickness: 1.,
                improvements: vec![],
            };

            /*
            let mut shape = CustomShape::new(Box::new(hexagon));
            shape.set_fill_color(Color::TRANSPARENT);
            shape.set_outline_color(Color::rgba(86, 84, 85, 51));
            shape.set_outline_thickness(1.);
            */

            line.push(hexagon);
        }

        hexagons.push(line);
    }

    hexagons
}

pub fn init_map_sprite_start_event() -> ControlActionFn {
    Box::new(|state, _| {
        if state.map_sprites.is_empty() {
            Some(HexEvent::new("INIT_MAP_HEX_SPRITES"))
        } else {
            None
        }
    })
}

pub fn init_map_sprite_allocation<'a>(scale: f32) -> EventStateFn<'a> {
    EventStateFn {
        event: "INIT_MAP_HEX_SPRITES",
        func: Box::new(move |mut state, graphic, _event| {
            state.map_sprites.clear();
            let State { mut map_sprites, .. } = state;

            state.hexagons.iter_mut().for_each(|lines| {
                lines.iter_mut().for_each(|mut hex| {
                    let x = hex.position.x;
                    let y = hex.position.y;
                    let category = hex.category;
                    let textures = graphic.textures;
                    let mut sprite_position = Vector2f {x, y: y - 15.8 * scale};

                    let mut sprite = match category {
                        HexagonCategory::SnowWithTrees => {
                            sprite_position = Vector2f {x, y: y - 13. * scale};
                            textures.snow_with_tress.clone()
                        },
                        HexagonCategory::Snow => {
                            sprite_position = Vector2f {x, y: y - 12. * scale};
                            textures.snow.clone()
                        },
                        HexagonCategory::DenseForest => textures.dense_forest.clone(),
                        HexagonCategory::Hill => textures.hill.clone(),
                        HexagonCategory::HillWithTrees => textures.hill_with_trees.clone(),
                        HexagonCategory::Mountain => textures.mountain.clone(),
                        HexagonCategory::Forest => textures.forest.clone(),
                        HexagonCategory::Field => textures.green_field.clone(),
                        HexagonCategory::City => textures.city.clone(),
                    };
                    sprite.set_position(sprite_position);

                    let mut hex_sprite = HexagonSprites {
                        id: hex.id,
                        sprites: vec![sprite.clone()],
                    };

                    hex.sprite_position = Some(sprite_position);

                    hex.improvements.iter()
                        .filter_map(|id_imp| state.hex_improvements.iter().find(|hex_imp| hex_imp.id == *id_imp))
                        .collect::<Vec<&HexImprovement>>()
                        .iter()
                        .for_each(|imp| {
                            if let Some(sprite) = &imp.sprite {
                                hex_sprite.sprites.push(sprite.clone());
                            }
                        });

                    if hex.grid_position.0 % 2 == 0 && map_sprites.len() > 0 {
                        map_sprites.insert(map_sprites.len() - 1, hex_sprite);
                    } else {
                        map_sprites.insert(map_sprites.len(), hex_sprite);
                    }
                });
            });

            state.map_sprites = map_sprites;
            state
        })
    }
}
