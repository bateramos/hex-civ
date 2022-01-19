use sfml::{graphics::*, system::*};

pub struct LoadedTextures <'a> {
    pub pillar: Sprite<'a>,

    pub green_field: Sprite<'a>,
    pub forest: Sprite<'a>,
    pub dense_forest: Sprite<'a>,
    pub hill: Sprite<'a>,
    pub hill_with_trees: Sprite<'a>,
    pub mountain: Sprite<'a>,
    pub city: Sprite<'a>,
    pub snow: Sprite<'a>,
    pub snow_with_tress: Sprite<'a>,

    pub pikeman: Sprite<'a>,
}

pub const SPRITE_X_PADDING : i32 = 32;
pub const SPRITE_Y_PADDING : i32 = 50;

fn load_texture<'a>(texture: &'a Texture, x: i32, y: i32, scale_x: f32, scale_y: f32) -> Sprite {
    let mut sprite = Sprite::with_texture_and_rect(&texture, &IntRect::new(x * SPRITE_X_PADDING , y * SPRITE_Y_PADDING, 32, 50));
    sprite.set_scale(Vector2f { x: scale_x, y: scale_y });

    sprite
}

pub fn init_textures<'a>(scale: f32, texture: &'a Texture, texture_pillar: &'a Texture, texture_pikeman: &'a Texture) -> LoadedTextures<'a> {
    let mut pillar = Sprite::with_texture_and_rect(&texture_pillar, &IntRect::new(0, 0, 50, 160));
    pillar.set_scale(Vector2f {x: scale, y: 1.9 * scale});

    let x_scale = 0.9 * scale;
    let y_scale = 0.8 * scale;

    let green_field = load_texture(&texture, 0, 0, x_scale, y_scale);
    let forest = load_texture(&texture, 1, 0, x_scale, y_scale);
    let dense_forest = load_texture(&texture, 2, 0, x_scale, y_scale);
    let hill = load_texture(&texture, 3, 0, x_scale, y_scale);
    let hill_with_trees = load_texture(&texture, 4, 0, x_scale, y_scale);

    let y_scale = 0.8 * scale;

    let city = load_texture(&texture, 0, 1, x_scale, y_scale);
    let mountain = load_texture(&texture, 5, 0, x_scale, y_scale);
    let snow = load_texture(&texture, 0, 2, x_scale, y_scale);
    let snow_with_tress  = load_texture(&texture, 1, 2, x_scale, y_scale);

    let mut pikeman = Sprite::with_texture_and_rect(&texture_pikeman, &IntRect::new(0, 0, 48, 88));
    pikeman.set_scale(Vector2f {x: 0.4 * scale, y: 0.4 * scale});

    LoadedTextures {
        pillar,

        green_field,
        forest,
        dense_forest,
        hill,
        hill_with_trees,
        mountain,
        city,
        snow,
        snow_with_tress,

        pikeman,
    }
}
