// #![windows_subsystem = "windows"]
use nannou::prelude::*;

mod mandelbrot;
mod complex;

struct Model{
    mandelbrot: mandelbrot::Mandelbrot,
    mouse_pressed: bool,
    mouse_start: Option<Vec2>,
}

impl Model {
    fn new(width: u32, height: u32) -> Model {
        Model {
            mandelbrot: mandelbrot::Mandelbrot::new(width, height, 1600, -100, 0, 200),
            mouse_pressed: false,
            mouse_start: None,
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
        }
        _ => {}
    }
}

fn window_resized(_app: &App, model: &mut Model, dim: Vec2) {
    let delta_width = dim.x as u32 - model.mandelbrot.width;
    let delta_height = dim.y as u32 - model.mandelbrot.height;
    model.mandelbrot.change_size(delta_width, delta_height);
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
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
    model.mandelbrot.calculate_mandelbrot(app);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    model.mandelbrot.last_squares
        .iter()
        .for_each(|(square, texture)| {
            let scale_factor = model.mandelbrot.zoom as f64 / square.zoom as f64;
            let x = square.x as f64 * scale_factor - model.mandelbrot.center_x as f64 + square.size as f64 / 2.0 * scale_factor;
            let y = -square.y as f64 * scale_factor + model.mandelbrot.center_y as f64 - square.size as f64 / 2.0 * scale_factor;
            let z = if square.zoom == model.mandelbrot.zoom { 2.0 } else { 1.0 };

            if !(x - square.size as f64 * scale_factor / 2.0 > app.window_rect().right() as f64 
                || x + square.size as f64 * scale_factor / 2.0 < app.window_rect().left() as f64 
                || y - square.size as f64 * scale_factor / 2.0 > app.window_rect().top() as f64 
                || y + square.size as f64 * scale_factor / 2.0 < app.window_rect().bottom() as f64)
                && (square.zoom == model.mandelbrot.zoom || !model.mandelbrot.finished_frame )
            {
                draw.texture(&texture)
                    .x_y(x as f32, y as f32)
                    .w_h(square.size as f32 * scale_factor as f32, square.size as f32 * scale_factor as f32)
                    .z(z);
            }   
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
        .left_justify()
        .z(10.0);
    y -= 20.0;
    draw.text(&format!("Max iterations: {}", model.mandelbrot.max_iter))
        .x_y(x, y)
        .w_h(line_width, 20.0)
        .font_size(20)
        .left_justify()
        .z(10.0);
    y -= 20.0;
    draw.text(&format!("Zoom: {:.2}", model.mandelbrot.zoom as f64 / 200.0))
        .x_y(x, y)
        .w_h(line_width, 20.0)
        .font_size(20)
        .left_justify()
        .z(10.0);

    draw.to_frame(app, &frame).unwrap();
}

fn format_float(n: f64) -> String {
    if n < 0.0 {
        format!("{:.16}", n)
    } else {
        format!(" {:.16}", n)
    }
}