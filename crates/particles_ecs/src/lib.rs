#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    clippy::clone_on_ref_ptr,
    clippy::else_if_without_else,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::mem_forget,
    clippy::multiple_inherent_impl,
    clippy::unwrap_used,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::string_add,
    clippy::wildcard_enum_match_arm,
    clippy::wrong_pub_self_convention
)]
#![allow(
    clippy::missing_docs_in_private_items,
    unknown_lints,
    clippy::expect_used,
    clippy::shadow_reuse
)]

pub mod components;
pub mod systems;
use components::common::Time;
use components::physics::*;
use components::physics_obj::Object;
use legion::*;
use systems::physics::init_world;

pub fn obj_loop(amount: &usize) {
    let (mut world, mut resources, mut time, mut schedule, mut count) = init_ecs_obj(amount);
    loop {
        let fake_time = time.overall_time + 0.1;
        time = Time {
            elapsed_seconds: fake_time - time.overall_time,
            overall_time: fake_time,
        };
        resources.insert(time);

        let mut query = <&Object>::query();
        for _object in query.iter(&world) {
            count += 1;
        }

        schedule.execute(&mut world, &mut resources);
        if time.overall_time >= 30. {
            break;
        }
    }
}

pub fn components_loop(amount: &usize) {
    let (mut world, mut resources, mut time, mut schedule, mut count) = init_ecs_components(amount);
    loop {
        let fake_time = time.overall_time + 0.1;
        time = Time {
            elapsed_seconds: fake_time - time.overall_time,
            overall_time: fake_time,
        };
        resources.insert(time);

        let mut query = <&Position>::query();
        for _position in query.iter(&mut world) {
            count += 1;
        }

        schedule.execute(&mut world, &mut resources);
        if time.overall_time >= 30. {
            break;
        }
    }
}

pub fn init_ecs_obj(amount: &usize) -> (World, Resources, Time, Schedule, u64) {
    let mut count = 0u64;
    let (mut world, mut resources, mut time, mut schedule) = init_world();
    for _ in 0..*amount / 1000 {
        world.extend([(Object::default(), ()); 1_000]);
    }
    return (world, resources, time, schedule, count);
}

pub fn init_ecs_components(amount: &usize) -> (World, Resources, Time, Schedule, u64) {
    let mut count = 0u64;
    let (mut world, mut resources, mut time, mut schedule) = init_world();
    for _ in 0..*amount / 1000 {
        world.extend(
            [(
                Position::default(),
                Velocity::default(),
                Acceleration::default(),
                Mass::default(),
            ); 1_000],
        );
    }
    return (world, resources, time, schedule, count);
}

#[cfg(test)]
mod tests {
    use crate::{init_ecs_components, init_ecs_obj};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn init_components() {
        init_ecs_components(&1_000_000);
    }

    #[test]
    fn init_objects() {
        init_ecs_obj(&1_000_000);
    }
}
