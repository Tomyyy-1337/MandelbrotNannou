use nannou::prelude::*;
use crate::mandelbrot;
use crate::Model;

pub fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    model.mouse_pressed = true;
    model.mouse_start = Some(app.mouse.position());
}

pub fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.mouse_pressed = false;
}

pub fn mouse_moved(_app: &App, model: &mut Model, pos: Vec2) {
    if model.mouse_pressed {
        let delta = pos - model.mouse_start.unwrap();
        model.mandelbrot.move_center(-delta.x as i64, delta.y as i64);
        model.mouse_start = Some(pos);
    }
}

pub fn mouse_zoom(app: &App, model: &mut Model, delta: MouseScrollDelta, _phase: TouchPhase) {
    match delta {
        MouseScrollDelta::LineDelta(_x, y) => {
            model.mandelbrot.zoom(y as i32, app.mouse.x as i32, -app.mouse.y as i32);
        }
        _ => {}
    }
}

pub fn window_resized(_app: &App, model: &mut Model, dim: Vec2) {
    let delta_width = dim.x as u32 - model.mandelbrot.width;
    let delta_height = dim.y as u32 - model.mandelbrot.height;
    model.mandelbrot.change_size(delta_width, delta_height);
}

pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::F11 => {
            app.main_window().set_fullscreen(!app.main_window().is_fullscreen());
        }
        Key::Up => {
            model.mandelbrot.increase_max_iter(model.mandelbrot.max_iter as i32);
        }
        Key::Down => {
            model.mandelbrot.increase_max_iter(-(model.mandelbrot.max_iter as i32 / 2));
        }
        Key::R => {
            model.mandelbrot = mandelbrot::Mandelbrot::new(
                model.mandelbrot.width,
                model.mandelbrot.height,
                1600,
                -100,
                0,
                200,
            );
        }
        _ => {}
    }
}