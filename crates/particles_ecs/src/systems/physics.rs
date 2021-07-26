use crate::components::common::{Circle, Time};
use crate::components::physics::{Acceleration, Mass, Position, Velocity};
use crate::components::physics_obj::Object;
use legion::{system, IntoQuery, Query, Resources, Schedule, World};
use macroquad::math::Vec2;

#[must_use]
pub fn init_world() -> (World, Resources, Time, Schedule) {
    let world = world();
    let mut resources = resources();

    let time = Time {
        elapsed_seconds: 0.0,
        overall_time: 0.0,
    };
    resources.insert(time);

    let schedule = schedule();
    (world, resources, time, schedule)
}

#[must_use]
pub fn resources() -> Resources {
    Resources::default()
}

#[must_use]
pub fn world() -> World {
    World::default()
}

#[must_use]
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

#[must_use]
pub fn get_component_objects() -> Query<(&'static Position, &'static Circle)> {
    <(&Position, &Circle)>::query()
}

#[must_use]
pub fn get_objects() -> Query<(&'static Object, &'static Circle)> {
    <(&Object, &Circle)>::query()
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
