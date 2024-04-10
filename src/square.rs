use std::collections::HashSet;

use nannou::rand::random_range;
use crate::complex::Complex;

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

    #[inline]
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
            .map(|_| (random_range(0, self.size) as usize ,random_range(0, self.size) as usize))
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|(x, y)| {
                let c = Complex::new(
                    (self.x + x as i64) as f64 * stepsize,
                    (self.y + y as i64) as f64 * stepsize,
                );
                let iterations = c.calculate_mandelbrot_iterations(self.max_iter);
                let color =  Self::calculate_color(iterations);
                colors[x * 4 + y * self.size as usize * 4..x * 4 + y * self.size as usize * 4 + 4].copy_from_slice(&color.0);
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
                colors[x as usize * 4 + y as usize * self.size as usize * 4..x as usize * 4 + y as usize * self.size as usize * 4 + 4].copy_from_slice(&color.0);
            }
        }
        nannou::image::DynamicImage::ImageRgba8(nannou::image::RgbaImage::from_vec(self.size, self.size, colors).unwrap())
    }
}