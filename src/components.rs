use macroquad::color::Color;
use macroquad::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Object {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
}

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    pub r: f32,
    pub color: Color,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub target: Vec2,
    pub zoom: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time {
    pub elapsed_seconds: f64,
    pub overall_time: f64,
}
