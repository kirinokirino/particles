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
    clippy::option_unwrap_used,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::result_unwrap_used,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::string_add,
    clippy::wildcard_enum_match_arm,
    clippy::wrong_pub_self_convention
)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::unknown_clippy_lints,
    clippy::option_expect_used,
    clippy::result_expect_used
)]

use legion::*;
use macroquad::*;
#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    pos: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity {
    vel: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Acceleration {
    acc: Vec2,
}

struct Mass {
    mass: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Circle {
    r: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Camera {
    target: Vec2,
    zoom: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Time {
    elapsed_seconds: f64,
    overall_time: f64,
}

fn draw_ui() {
    // Screen space, render fixed ui
    set_default_camera();
    let text_color: Color = Color([100, 100, 100, 150]);
    draw_text(",aoe to move camera", 10.0, 0.0, 30.0, text_color);
    draw_text(
        "PageUp and PageDown to zoom camera",
        10.0,
        50.0,
        30.0,
        text_color,
    );
}

fn move_camera(camera: &mut Camera) {
    // scroll
    if is_key_down(KeyCode::Comma) {
        camera
            .target
            .set_y(camera.target.y() - 0.01 / camera.zoom.x())
    }
    if is_key_down(KeyCode::O) {
        camera
            .target
            .set_y(camera.target.y() + 0.01 / camera.zoom.x())
    }
    if is_key_down(KeyCode::A) {
        camera
            .target
            .set_x(camera.target.x() + 0.01 / camera.zoom.x())
    }
    if is_key_down(KeyCode::E) {
        camera
            .target
            .set_x(camera.target.x() - 0.01 / camera.zoom.x())
    }
    // zoom
    if is_key_down(KeyCode::PageUp) {
        camera.zoom.set_x(camera.zoom.x() * 0.98);
        camera.zoom.set_y(camera.zoom.y() * 0.98);
    }
    if is_key_down(KeyCode::PageDown) {
        camera.zoom.set_x(camera.zoom.x() / 0.98);
        camera.zoom.set_y(camera.zoom.y() / 0.98);
    }
}

fn get_relative_mouse_position(camera: &Camera) -> Vec2 {
    let mouse = mouse_position();
    Vec2::new(
        ((mouse.0 - screen_width() / 2.0) / (screen_width() / 2.0) / camera.zoom.x())
            + camera.target.x(),
        ((-mouse.1 + screen_height() / 2.0)
            / (screen_height() / 2.0)
            / camera.zoom.x()
            / (screen_width() / screen_height()))
            + camera.target.y(),
    )
}

fn create_particle(position: Vec2) -> (Position, Velocity, Acceleration, Mass, Circle) {
    let mass = rand::gen_range(1, 5) as f32;
    return (
        Position { pos: position },
        Velocity {
            vel: Vec2::new(0.0, 0.0),
        },
        Acceleration {
            acc: Vec2::new(
                rand::gen_range(-400, 400) as f32,
                rand::gen_range(-400, 400) as f32,
            ),
        },
        Mass { mass },
        Circle {
            r: (mass / std::f32::consts::PI).sqrt(),
        },
    );
}

#[system(for_each)]
fn apply_gravity(acc: &mut Acceleration, mass: &Mass, #[resource] time: &Time) {
    acc.acc
        .set_y(acc.acc.y() + -50.0 * mass.mass * time.elapsed_seconds as f32);
}

#[system(for_each)]
fn drop_acceleration(acc: &mut Acceleration) {
    acc.acc.set_y(0.0);
    acc.acc.set_x(0.0);
}

#[system(for_each)]
fn apply_air_drag(vel: &mut Velocity) {
    vel.vel.set_y(vel.vel.y() * 0.999);
    vel.vel.set_x(vel.vel.x() * 0.999);
}

#[system(for_each)]
fn update_velocity(vel: &mut Velocity, acc: &Acceleration, #[resource] time: &Time) {
    vel.vel
        .set_x(vel.vel.x() + acc.acc.x() * time.elapsed_seconds as f32);
    vel.vel
        .set_y(vel.vel.y() + acc.acc.y() * time.elapsed_seconds as f32);
}

#[system(for_each)]
fn update_position(pos: &mut Position, vel: &Velocity, #[resource] time: &Time) {
    pos.pos
        .set_x(pos.pos.x() + vel.vel.x() * time.elapsed_seconds as f32);
    pos.pos
        .set_y(pos.pos.y() + vel.vel.y() * time.elapsed_seconds as f32);
}

#[macroquad::main("Name")]
async fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut time = Time {
        elapsed_seconds: 0.0,
        overall_time: 0.0,
    };
    resources.insert(time);

    let starting_zoom = 0.05;
    let mut main_camera = Camera {
        target: vec2(0.0, 0.0),
        zoom: vec2(
            starting_zoom,
            starting_zoom * screen_width() / screen_height(),
        ),
    };

    let mut mouse_pressed = false;

    // construct a schedule (you should do this on init)
    let mut schedule = Schedule::builder()
        .add_system(apply_gravity_system())
        .add_system(update_velocity_system())
        .add_system(apply_air_drag_system())
        .add_system(update_position_system())
        .add_system(drop_acceleration_system())
        .build();

    loop {
        // Update
        time = Time {
            elapsed_seconds: get_time() - time.overall_time,
            overall_time: get_time(),
        };
        //info!("{}", time.elapsed_seconds);
        resources.insert(time.clone());
        let mouse_position = get_relative_mouse_position(&main_camera);
        move_camera(&mut main_camera);
        if is_key_down(KeyCode::Right) {}
        if is_key_down(KeyCode::Left) {}
        if is_key_down(KeyCode::Down) {}
        if is_key_down(KeyCode::Up) {}
        if is_mouse_button_down(MouseButton::Left) {
            if mouse_pressed == false {
                let pos = get_relative_mouse_position(&main_camera);
                let _ = world.push(create_particle(pos));
                info!("Mouse pressed at x:{} , y:{}", pos.x(), pos.y())
            }
            mouse_pressed = true;
        } else {
            mouse_pressed = false;
        }

        // Draw

        clear_background(Color([255, 255, 255, 255]));

        // Camera space, render game objects
        set_camera(Camera2D {
            target: main_camera.target,
            zoom: main_camera.zoom,
            ..Default::default()
        });

        // construct a query from a "view tuple"
        let mut query = <(&Position, &Circle)>::query();

        for (position, circle) in query.iter_mut(&mut world) {
            draw_circle(
                position.pos.x(),
                position.pos.y(),
                circle.r,
                Color([100, 150, 200, 255]),
            )
        }

        draw_circle(
            mouse_position.x(),
            mouse_position.y(),
            0.2,
            Color([200, 150, 225, 255]),
        );

        draw_ui();

        // run our schedule (you should do this each update)
        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}
