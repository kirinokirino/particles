use macroquad::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Object {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
}
