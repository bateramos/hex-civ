use sfml::{system::*, graphics::*};

pub type HexagonLine = Vec<Hexagon>;
pub type HexagonColumn = Vec<HexagonLine>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HexagonCategory {
    Field,
    City,
    Snow,
    SnowWithTrees,
    Mountain,
    DenseForest,
    Hill,
    HillWithTrees,
    Forest,
}

#[derive(Clone, Copy, Debug)]
pub struct Hexagon {
    pub id: u32,
    pub scale: f32,
    pub position: Vector2f,
    pub fill_color: Color,
    pub outline_color: Color,
    pub thickness: f32,
    pub center: Vector2f,
    pub sprite_position: Vector2f,
    pub category: HexagonCategory,
    pub grid_position: (usize, usize),
}

impl Hexagon {
    pub fn create_point(&self, value_x: f32, value_y: f32) -> Vector2f {
        Vector2f {
            x: self.scale * value_x + self.position.x,
            y: self.scale * value_y + self.position.y,
        }
    }
}

impl CustomShapePoints for Hexagon {
    fn point_count(&self) -> u32 {
        6
    }

    fn point(&self, point: u32) -> Vector2f {
        match point {
            0 => self.create_point(10., 0.),
            1 => self.create_point(20., 0.),
            2 => self.create_point(30., 10.),
            3 => self.create_point(20., 20.),
            4 => self.create_point(10., 20.),
            5 => self.create_point(0., 10.),
            p => panic!("Something wrong with point: {}", p),
        }
    }
}
