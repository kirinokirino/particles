use macroquad::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Object {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            mass: 0.0,
        }
    }
}
