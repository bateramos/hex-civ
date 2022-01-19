use sfml::graphics::*;
use sfml::system::*;

use crate::State;
use crate::controllers::LoadedTextures;

pub fn init_unit_sprite<'a>(scale: f32) -> Box<dyn Fn(State<'a>, &'a LoadedTextures) -> State<'a>> {
    Box::new(move |mut state, textures| {
        let mut units : Vec<Sprite> = Vec::new();

        for y in 0..state.units.len() {
            for x in 0..state.units[y].len() {
                if let Some(_unit) = state.units[y][x] {
                    let mut sprite = textures.pikeman.clone();

                    sprite.set_position(state.hexagons[y][x].sprite_position);
                    sprite.move_(Vector2f { y: 0., x: 6. * scale });
                    units.push(sprite);
                }
            }
        }
        if units.is_empty() {
            println!("Empty");
        }

        state.units_sprites = units;

        state
    })
}
