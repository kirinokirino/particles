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

use macroquad::camera::{set_camera, set_default_camera, Camera2D};
use macroquad::color::Color;
use macroquad::color_u8;
use macroquad::input::{is_key_down, is_mouse_button_down, mouse_position, KeyCode, MouseButton};
use macroquad::logging::{info, warn};
use macroquad::math::{vec2, Vec2};
use macroquad::rand;
use macroquad::shapes::draw_circle;
use macroquad::text::draw_text;
use macroquad::time::get_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use particles_ecs::components::common::{Camera, Circle, Time};
use particles_ecs::components::physics::{Acceleration, Mass, Position, Velocity};
use particles_ecs::components::physics_obj::Object;
use particles_ecs::systems::physics::{get_component_objects, get_objects, init_world};

fn draw_ui() {
    // Screen space, render fixed ui
    set_default_camera();
    let text_color = color_u8!(100, 100, 100, 150);
    draw_text("\",aoe\" to move camera", 10.0, 0.0, 30.0, text_color);
    draw_text(
        "PageUp and PageDown / \"'\" \".\" to zoom camera",
        10.0,
        50.0,
        30.0,
        text_color,
    );
}

fn move_camera(camera: &mut Camera) {
    // scroll
    if is_key_down(KeyCode::Comma) {
        camera.target.y += 0.01 / camera.zoom.x
    }
    if is_key_down(KeyCode::O) {
        camera.target.y -= 0.01 / camera.zoom.x
    }
    if is_key_down(KeyCode::A) {
        camera.target.x -= 0.01 / camera.zoom.x
    }
    if is_key_down(KeyCode::E) {
        camera.target.x += 0.01 / camera.zoom.x
    }
    // zoom
    if is_key_down(KeyCode::PageUp) || is_key_down(KeyCode::Apostrophe) {
        camera.zoom.x *= 0.98;
        camera.zoom.y *= 0.98;
    }
    if is_key_down(KeyCode::PageDown) || is_key_down(KeyCode::Period) {
        camera.zoom.x /= 0.98;
        camera.zoom.y /= 0.98;
    }
}

fn get_relative_mouse_position(camera: &Camera) -> Vec2 {
    let mouse = mouse_position();
    Vec2::new(
        ((mouse.0 - screen_width() / 2.0) / (screen_width() / 2.0) / camera.zoom.x)
            + camera.target.x,
        ((-mouse.1 + screen_height() / 2.0)
            / (screen_height() / 2.0)
            / camera.zoom.x
            / (screen_width() / screen_height()))
            + camera.target.y,
    )
}

fn create_obj(position: Vec2) -> (Object, Circle) {
    let mass = rand::gen_range(1., 5.);
    (
        Object {
            pos: position,
            vel: Vec2::new(0.0, 0.0),
            acc: Vec2::new(rand::gen_range(-400., 400.), rand::gen_range(-400., 400.)),
            mass,
        },
        Circle {
            r: (mass / std::f32::consts::PI).sqrt(),
            color: color_u8!(255, 180, 220, 255),
        },
    )
}
fn create_particle(position: Vec2) -> (Position, Velocity, Acceleration, Mass, Circle) {
    let mass = rand::gen_range(1., 5.);
    (
        Position { pos: position },
        Velocity {
            vel: Vec2::new(0.0, 0.0),
        },
        Acceleration {
            acc: Vec2::new(rand::gen_range(-400., 400.), rand::gen_range(-400., 400.)),
        },
        Mass { mass },
        Circle {
            r: (mass / std::f32::consts::PI).sqrt(),
            color: color_u8!(120, 220, 255, 255),
        },
    )
}

// I don't know how to apply this line.
#[allow(clippy::future_not_send, clippy::too_many_lines)]
#[macroquad::main("Name")]
async fn main() {
    let (mut world, mut resources, mut time, mut schedule) = init_world();
    let starting_zoom = 0.05;
    let mut main_camera = Camera {
        target: vec2(0.0, 0.0),
        zoom: vec2(
            starting_zoom,
            starting_zoom * screen_width() / screen_height(),
        ),
    };

    let mut mouse_pressed = false;

    loop {
        time = Time {
            elapsed_seconds: get_time() - time.overall_time,
            overall_time: get_time(),
        };
        //info!("{}", time.elapsed_seconds);
        resources.insert(time);

        let mouse_position = get_relative_mouse_position(&main_camera);
        move_camera(&mut main_camera);
        if is_key_down(KeyCode::Right) {}
        if is_key_down(KeyCode::Left) {}
        if is_key_down(KeyCode::Down) {}
        if is_key_down(KeyCode::Up) {}
        if is_mouse_button_down(MouseButton::Left) {
            if !mouse_pressed {
                let pos = get_relative_mouse_position(&main_camera);
                info!("Mouse pressed at x:{} , y:{}", pos.x, pos.y);
                for _ in 0..10 {
                    let _ = world.push(create_particle(pos));
                    let _ = world.push(create_obj(pos));
                }
            }
            mouse_pressed = true;
        } else {
            mouse_pressed = false;
        }

        // Draw

        clear_background(color_u8!(255, 255, 255, 255));

        // Camera space, render game objects
        set_camera(&Camera2D {
            target: main_camera.target,
            zoom: main_camera.zoom,
            ..Camera2D::default()
        });

        // construct a query from a "view tuple"
        let mut query = get_component_objects();

        for (position, circle) in query.iter_mut(&mut world) {
            draw_circle(position.pos.x, position.pos.y, circle.r, circle.color)
        }

        let mut query = get_objects();
        for (object, circle) in query.iter(&world) {
            draw_circle(object.pos.x, object.pos.y, circle.r, circle.color)
        }

        draw_circle(
            mouse_position.x,
            mouse_position.y,
            0.2,
            color_u8!(200, 150, 225, 255),
        );

        draw_ui();

        // run our schedule (you should do this each update)
        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}
