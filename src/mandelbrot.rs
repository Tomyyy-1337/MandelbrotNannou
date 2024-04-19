use std::collections::HashMap;
use nannou::image::DynamicImage;
use nannou::wgpu::Texture;
use nannou::App;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::square::Square;

pub struct Mandelbrot {
    pub width: u32,
    pub height: u32,
    pub max_iter: u32,
    pub center_x: i64,
    pub center_y: i64,
    pub zoom: u64,
    pub squares: HashMap<Square, Texture>,
    pub finished_frame: bool,
    pub new: bool,
    just_zoomed: bool,
}

impl Mandelbrot {
    pub fn new(width: u32, height: u32, max_iter: u32, center_x: i64, center_y: i64, zoom: u64) -> Self {
        Mandelbrot {
            width,
            height,
            max_iter,
            center_x,
            center_y,
            zoom,
            squares: HashMap::new(),
            finished_frame: false,
            new: true,
            just_zoomed: false,
        }
    }

    pub fn zoom(&mut self, zoom: i32, mouse_x: i32 ,mouse_y: i32) {
        let new_zoom = u64::max((self.zoom as f64 * 1.33f64.powi(zoom)) as u64, 10);
        let x_offset = (mouse_x as i64) * (new_zoom as i64 - self.zoom as i64) / self.zoom as i64;
        let y_offset = (mouse_y as i64) * (new_zoom as i64 - self.zoom as i64) / self.zoom as i64;
        self.center_x = (self.center_x as f64 * new_zoom as f64 / self.zoom as f64) as i64 + x_offset as i64;
        self.center_y = (self.center_y as f64 * new_zoom as f64 / self.zoom as f64) as i64 + y_offset as i64;
        self.zoom = new_zoom;
        self.just_zoomed = true;
    }

    pub fn change_size(&mut self ,delta_width: u32, delta_height: u32) {
        let width = self.width + delta_width;
        let height = self.height + delta_height;
        let zoom = self.zoom as f64 * height as f64 / self.height as f64;
        self.width = width;
        self.height = height;
        self.center_x = (self.center_x as f64 * zoom / self.zoom as f64 ) as i64;
        self.center_y = (self.center_y as f64 * zoom / self.zoom as f64) as i64;
        self.zoom = zoom as u64;
    }

    pub fn move_center(&mut self, x: i64, y: i64) {
        self.center_x += x;
        self.center_y += y;
    }

    pub fn increase_max_iter(&mut self, delta: i32) {
        self.max_iter = ((self.max_iter as i32 + delta) as u32).max(100);
    }

    pub fn calculate_mandelbrot(&mut self, app: &App) {
        if self.new {
            self.new = false;
            return;
        }
        let square_size:u32 = 48;
        let top_x = self.center_x - self.width as i64 / 2;
        let top_y = self.center_y - self.height as i64 / 2;
        let start_x = top_x - top_x % square_size as i64 - square_size as i64;
        let start_y = top_y - top_y % square_size as i64 - square_size as i64;
        
        let mut squares:Vec<Square> = (start_x..top_x + self.width as i64)
            .step_by(square_size as usize)
            .flat_map(|x| (start_y..top_y + self.height as i64)
                .step_by(square_size as usize)
                .map(move |y| (x,y))
                .map(|(x,y)| Square::new(x, y, self.zoom, square_size, self.max_iter))
            )
            .filter(|square| !self.squares.contains_key(square))
            .collect();
        
        self.finished_frame = squares.len() == 0;
        if self.finished_frame {
            self.squares.retain(|square, _| square.zoom == self.zoom && square.max_iter == self.max_iter);
            return;
        }
        
        let calc_time = if self.just_zoomed { self.just_zoomed = false; 75 } else { 10 };
        let start_time = std::time::Instant::now();
        squares.shuffle(&mut thread_rng());
        let square_results:Vec<(Square,DynamicImage)> = squares.into_par_iter()
            .take_any_while(|_| start_time.elapsed().as_millis() < calc_time)
            .map(|square|(square, square.calculate_square()))
            .collect();

        square_results.into_iter().for_each(|(square, square_result)| {
            let texture = nannou::wgpu::Texture::from_image(app, &square_result);
            self.squares.insert(square, texture);
        });
    } 
}
