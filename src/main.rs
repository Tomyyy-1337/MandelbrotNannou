// #![windows_subsystem = "windows"]
use nannou::prelude::*;

mod mandelbrot;
mod complex;

struct Model{
    mandelbrot: mandelbrot::Mandelbrot,
    mouse_pressed: bool,
    mouse_start: Option<Vec2>,
    changed: bool,
}

impl Model {
    fn new(width: u32, height: u32) -> Model {
        Model {
            mandelbrot: mandelbrot::Mandelbrot::new(width, height, 1600, -100, 0, 200),
            mouse_pressed: false,
            mouse_start: None,
            changed: false,
        }
    }
}   

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    model.mouse_pressed = true;
    model.mouse_start = Some(app.mouse.position());
}

fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.mouse_pressed = false;
}

fn mouse_moved(_app: &App, model: &mut Model, pos: Vec2) {
    if model.mouse_pressed {
        let delta = pos - model.mouse_start.unwrap();
        model.mandelbrot.move_center(-delta.x as i64, delta.y as i64);
        model.mouse_start = Some(pos);
    }
}

fn mouse_zoom(app: &App, model: &mut Model, delta: MouseScrollDelta, _phase: TouchPhase) {
    match delta {
        MouseScrollDelta::LineDelta(_x, y) => {
            model.mandelbrot.zoom(y as i32, app.mouse.x as i32, -app.mouse.y as i32);
            model.changed = true;
        }
        _ => {}
    }
}

fn window_resized(_app: &App, model: &mut Model, dim: Vec2) {
    let delta_width = dim.x as u32 - model.mandelbrot.width;
    let delta_height = dim.y as u32 - model.mandelbrot.height;
    model.mandelbrot.change_size(delta_width, delta_height);
    model.changed = true;
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::F11 => {
            app.main_window().set_fullscreen(!app.main_window().is_fullscreen());
            model.changed = true;
        }
        Key::Up => {
            model.mandelbrot.increase_max_iter(model.mandelbrot.max_iter as i32);
            model.changed = true;
        }
        Key::Down => {
            model.mandelbrot.increase_max_iter(-(model.mandelbrot.max_iter as i32 / 2));
            model.changed = true;
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
            model.changed = true;
        }
        _ => {}
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let width = 1000;
    let height = 800;
    app.new_window()
        .size(width, height)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_moved(mouse_moved)
        .mouse_wheel(mouse_zoom)
        .resized(window_resized)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model::new(width, height)
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.mandelbrot.calculate_mandelbrot(app, &mut model.changed);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    model.mandelbrot.last_squares
        .iter()
        .filter(|(square, _)| {
            let top_x = model.mandelbrot.center_x - model.mandelbrot.width as i64 / 2;
            let top_y = model.mandelbrot.center_y - model.mandelbrot.height as i64 / 2;
            let bottom_x = top_x + model.mandelbrot.width as i64;
            let bottom_y = top_y + model.mandelbrot.height as i64;
            let square_bottom_x = square.x + square.size as i64;
            let square_bottom_y = square.y + square.size as i64;
            square.x < bottom_x && square_bottom_x > top_x && square.y < bottom_y && square_bottom_y > top_y
        }).for_each(|(square, texture)| {
            let x = square.x - model.mandelbrot.center_x + square.size as i64 / 2;
            let y = -square.y + model.mandelbrot.center_y - square.size as i64 / 2;
            
            draw.texture(&texture)
                .x_y(x as f32, y as f32)
                .w_h(square.size as f32, square.size as f32);
        });

    let line_width = 500.0;
    let x = (line_width - app.window_rect().w()) / 2.0 + 10.0;
    let mut y = app.window_rect().h() / 2.0 - 20.0;
    let stepsize = 1.0 / model.mandelbrot.zoom as f64;
    let real = format_float((model.mandelbrot.center_x + app.mouse.x as i64) as f64 * stepsize);
    let imag = format_float(-(model.mandelbrot.center_y - app.mouse.y as i64) as f64 * stepsize);
    draw.text(&format!("C: {} + {}i", real, imag))
        .x_y(x, y)
        .w_h(line_width, 20.0)
        .font_size(20)
        .left_justify();
    y -= 20.0;
    draw.text(&format!("Max iterations: {}", model.mandelbrot.max_iter))
        .x_y(x, y)
        .w_h(line_width, 20.0)
        .font_size(20)
        .left_justify();
    y -= 20.0;
    draw.text(&format!("Zoom: {:.2}", model.mandelbrot.zoom as f64 / 200.0))
        .x_y(x, y)
        .w_h(line_width, 20.0)
        .font_size(20)
        .left_justify();

    draw.to_frame(app, &frame).unwrap();
}

fn format_float(n: f64) -> String {
    if n < 0.0 {
        format!("{:.16}", n)
    } else {
        format!(" {:.16}", n)
    }
}