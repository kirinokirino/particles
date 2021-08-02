use macroquad::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub pos: Vec2,
}

impl Default for Position {
    fn default() -> Self {
        Self { pos: Vec2::ZERO }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub vel: Vec2,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { vel: Vec2::ZERO }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Acceleration {
    pub acc: Vec2,
}

impl Default for Acceleration {
    fn default() -> Self {
        Self { acc: Vec2::ZERO }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mass {
    pub mass: f32,
}

impl Default for Mass {
    fn default() -> Self {
        Self { mass: 1.0 }
    }
}
