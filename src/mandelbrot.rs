use std::collections::HashMap;
use std::collections::HashSet;
use nannou::image::DynamicImage;
use nannou::rand::random_range;
use nannou::wgpu::Texture;
use nannou::App;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use crate::complex::Complex;

pub struct Mandelbrot {
    pub width: u32,
    pub height: u32,
    pub max_iter: u32,
    pub center_x: i64,
    pub center_y: i64,
    pub zoom: u64,
    pub last_squares: HashMap<Square, Texture>,
    pub finished_frame: bool,
    pub new: bool,
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
            last_squares: HashMap::new(),
            finished_frame: false,
            new: true,
        }
    }

    pub fn zoom(&mut self, zoom: i32, mouse_x: i32 ,mouse_y: i32) {
        let new_zoom = u64::max((self.zoom as f64 * 1.33f64.powi(zoom)) as u64, 16);
        let x_offset = (mouse_x as i64) * (new_zoom as i64 - self.zoom as i64) / self.zoom as i64;
        let y_offset = (mouse_y as i64) * (new_zoom as i64 - self.zoom as i64) / self.zoom as i64;
        self.center_x = (self.center_x as f64 * new_zoom as f64 / self.zoom as f64) as i64 + x_offset as i64;
        self.center_y = (self.center_y as f64 * new_zoom as f64 / self.zoom as f64) as i64 + y_offset as i64;
        self.zoom = new_zoom;
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
        
        let mut squares:Vec<Square> = Vec::new();
        for x in (start_x..top_x + self.width as i64).step_by(square_size as usize) {
            for y in (start_y..top_y + self.height as i64).step_by(square_size as usize) {
                squares.push(Square::new(x, y, self.zoom, square_size, self.max_iter));
            }
        }
        self.finished_frame = squares.iter().find(|square| !self.last_squares.contains_key(square)).is_none();
        if self.finished_frame {
            self.last_squares.retain(|square, _| square.zoom == self.zoom && square.max_iter == self.max_iter);
        }
        
        let tiles_per_frame = std::thread::available_parallelism().unwrap().get() * 8;    
        let square_results:Vec<(Square,DynamicImage)> = squares.into_par_iter()
            .filter(|square| !self.last_squares.contains_key(square))
            .take_any(tiles_per_frame)
            .map(|square|(square, square.calculate_square()))
            .collect();

        square_results.into_iter().for_each(|(square, square_result)| {
            let texture = nannou::wgpu::Texture::from_image(app, &square_result);
            self.last_squares.insert(square, texture);
        });
    } 
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square {
    pub x: i64,
    pub y: i64,
    pub zoom: u64,
    pub size: u32,
    pub max_iter: u32,
}

impl Square {
    pub fn new(x: i64, y: i64, zoom: u64, size: u32, max_iter: u32) -> Self {
        Square {
            x,
            y,
            zoom,
            size,
            max_iter,
        }
    }

    fn calculate_color(color: u32) -> nannou::image::Rgba<u8> {
        let num_colors = 161;
        if color == 0 {
            return nannou::image::Rgba([0,0,0,255]);
        } 
        let limited_input = (3 * color) % num_colors + 30 as u32;
        let hue = (limited_input as f32 / num_colors as f32) * 2.0 * std::f32::consts::PI;  
        let r = ((hue.sin() * 0.5 + 0.5) * 255.0) as u8;
        let g = ((hue.cos() * 0.5 + 0.5) * 255.0) as u8;
        let b = (((hue + std::f32::consts::PI / 2.0).cos() * 0.5 + 0.5) * 255.0) as u8;
        nannou::image::Rgba([r,g,b,255])
    }
    
    pub fn calculate_square(&self) -> nannou::image::DynamicImage {
        let stepsize = 1.0 / self.zoom as f64;
        let mut colors:Vec<u8> = Vec::new();
        colors.resize((self.size * self.size * 4) as usize, 0);
        
        if (0..self.size * self.size / 12)
            .map(|_| {
                let x = random_range(0, self.size) as usize;
                let y = random_range(0, self.size) as usize;
                let c = Complex::new(
                    (self.x + x as i64) as f64 * stepsize,
                    (self.y + y as i64) as f64 * stepsize,
                );
                let iterations = c.calculate_mandelbrot_iterations(self.max_iter);
                let color =  Self::calculate_color(iterations);
                colors[x * 4 + y * self.size as usize * 4] = color[0];
                colors[x * 4 + y * self.size as usize * 4 + 1] = color[1];
                colors[x * 4 + y * self.size as usize * 4 + 2] = color[2];
                colors[x * 4 + y * self.size as usize * 4 + 3] = color[3];
                iterations
                
            })
            .all(|iterations| iterations == 0) 
            {
                return nannou::image::DynamicImage::ImageRgba8(nannou::image::RgbaImage::from_pixel(self.size, self.size, nannou::image::Rgba([0,0,0,255])));
            }
            
        for y in 0..self.size as i64 {
            for x in 0..self.size as i64 {
                if colors[x as usize * 4 + y as usize * self.size as usize * 4 + 3] == 255 {
                    continue;
                }
                let c = Complex::new(
                    (self.x + x) as f64 * stepsize,
                    (self.y + y) as f64 * stepsize,
                );
                let color =  Self::calculate_color(c.calculate_mandelbrot_iterations(self.max_iter));
                colors[x as usize * 4 + y as usize * self.size as usize * 4] = color[0];
                colors[x as usize * 4 + y as usize * self.size as usize * 4 + 1] = color[1];
                colors[x as usize * 4 + y as usize * self.size as usize * 4 + 2] = color[2];
                colors[x as usize * 4 + y as usize * self.size as usize * 4 + 3] = color[3];
            }
        }
        nannou::image::DynamicImage::ImageRgba8(nannou::image::RgbaImage::from_vec(self.size, self.size, colors).unwrap())
    }
}