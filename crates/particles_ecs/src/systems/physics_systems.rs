use crate::components::common::*;
use crate::components::physics_components::*;
use crate::components::physics_obj::*;
use legion::{system, Resources, Schedule, World};
use macroquad::math::Vec2;

pub fn resources() -> Resources {
    let mut resources = Resources::default();

    resources
}

pub fn world() -> World {
    World::default()
}

pub fn schedule() -> Schedule {
    // construct a schedule (you should do this on init)
    Schedule::builder()
        .add_system(apply_gravity_system())
        .add_system(update_velocity_system())
        .add_system(apply_air_drag_system())
        .add_system(update_position_system())
        .add_system(drop_acceleration_system())
        .add_system(update_objects_system())
        .build()
}

#[system(for_each)]
#[allow(clippy::cast_possible_truncation)]
fn update_objects(obj: &mut Object, #[resource] time: &Time) {
    let time: f32 = time.elapsed_seconds as f32;
    obj.acc.y += -50.0 * obj.mass * time;
    obj.vel += obj.acc * time;
    obj.vel *= 0.999;
    obj.pos += obj.vel * time;
    obj.acc = Vec2::ZERO;
}

#[system(for_each)]
#[allow(clippy::cast_possible_truncation)]
fn apply_gravity(acc: &mut Acceleration, mass: &Mass, #[resource] time: &Time) {
    acc.acc.y += -50.0 * mass.mass * time.elapsed_seconds as f32;
}

#[system(for_each)]
fn drop_acceleration(acc: &mut Acceleration) {
    acc.acc.y = 0.0;
    acc.acc.x = 0.0;
}

#[system(for_each)]
fn apply_air_drag(vel: &mut Velocity) {
    vel.vel.y *= 0.999;
    vel.vel.x *= 0.999;
}

#[system(for_each)]
#[allow(clippy::cast_possible_truncation, clippy::trivially_copy_pass_by_ref)]
fn update_velocity(vel: &mut Velocity, acc: &Acceleration, #[resource] time: &Time) {
    vel.vel.x += acc.acc.x * time.elapsed_seconds as f32;
    vel.vel.y += acc.acc.y * time.elapsed_seconds as f32;
}

#[system(for_each)]
#[allow(clippy::cast_possible_truncation, clippy::trivially_copy_pass_by_ref)]
fn update_position(pos: &mut Position, vel: &Velocity, #[resource] time: &Time) {
    pos.pos.x += vel.vel.x * time.elapsed_seconds as f32;
    pos.pos.y += vel.vel.y * time.elapsed_seconds as f32;
}
