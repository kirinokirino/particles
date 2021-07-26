use macroquad::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub pos: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub vel: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Acceleration {
    pub acc: Vec2,
}

pub struct Mass {
    pub mass: f32,
}
