// #![windows_subsystem = "windows"]
use nannou::prelude::*;

mod mandelbrot;
mod complex;
mod square;
mod inputs;

use inputs::{mouse_pressed, mouse_released, mouse_moved, mouse_zoom, window_resized, key_pressed};

struct Model{
    mandelbrot: mandelbrot::Mandelbrot,
    mouse_pressed: bool,
    mouse_start: Option<Vec2>,
}

impl Model {
    fn new(width: u32, height: u32) -> Model {
        Model {
            mandelbrot: mandelbrot::Mandelbrot::new(width, height, 1600, -100, 0, 11),
            mouse_pressed: false,
            mouse_start: None,
        }
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

    model.mandelbrot.squares
        .iter()
        .for_each(|(square, texture)| {
            let scale_factor = model.mandelbrot.zoom as f64 / square.zoom as f64;
            let x = (square.x as f64 + square.size as f64 / 2.0) * scale_factor - model.mandelbrot.center_x as f64;
            let y = (-square.y as f64 - square.size as f64 / 2.0) * scale_factor + model.mandelbrot.center_y as f64;
            let z = if square.zoom == model.mandelbrot.zoom { 2.0 } else { 1.0 };

            if !(x - square.size as f64 * scale_factor / 2.0 > app.window_rect().right() as f64 
                || x + square.size as f64 * scale_factor / 2.0 < app.window_rect().left() as f64 
                || y - square.size as f64 * scale_factor / 2.0 > app.window_rect().top() as f64 
                || y + square.size as f64 * scale_factor / 2.0 < app.window_rect().bottom() as f64)
                && (square.zoom == model.mandelbrot.zoom || !model.mandelbrot.finished_frame )
            {
                draw.texture(&texture)
                    .x_y(x as f32, y as f32)
                    .w_h((square.size as f64 * scale_factor) as f32, (square.size as f64 * scale_factor) as f32)
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
    draw.text(&format!("Zoom: {:.2}", model.mandelbrot.zoom as f64 / 10.0))
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