use super::{matrix::Matrix, point::Point};
use sdl2::pixels::Color;
use std::cmp::max;
use super::point::point_div;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub p: [Point; 3],
    pub base_color: Color,
    pub color: Color
}

impl Triangle {
    // Creates a new Triangle instance
    pub fn new(p1: Point, p2: Point, p3: Point, base_color: Option<Color>) -> Self {
        let color = match base_color {
            Some(c) => c,
            None => Color::RGBA(255,255,255,255)
        };
        Triangle {p: [p1, p2, p3], base_color: color, color}
    }

    // Returns an array of 2 tuples: i32 for X,Y
    pub fn get_2d_points(&self) -> [(i32, i32);3] {
        [(self.p[0].x as i32, self.p[0].y as i32),
            (self.p[1].x as i32, self.p[1].y as i32),
            (self.p[2].x as i32, self.p[2].y as i32)]
    }

    /// Returns an array of 3 tuples: i32 for X,Y,Z
    pub fn get_3d_points(&self) -> [(f32,f32,f32); 3] {
        [
            (self.p[0].x, self.p[0].y, self.p[0].z),
            (self.p[1].x, self.p[1].y, self.p[1].z),
            (self.p[2].x, self.p[2].y, self.p[2].z)]
    }

    pub fn normalize(&mut self) {
        self.p[0] = point_div(self.p[0], self.p[0].w);
        self.p[1] = point_div(self.p[1], self.p[1].w);
        self.p[2] = point_div(self.p[2], self.p[2].w);
    }

    pub fn shade(&mut self, luminance: f32) {
        self.color.r = (self.base_color.r as f32*luminance) as u8;
        self.color.g = (self.base_color.g as f32*luminance) as u8;
        self.color.b = (self.base_color.b as f32*luminance) as u8;
    }

    /// Add value to each point
    pub fn add_each_point(&mut self, p: Point) {
        self.p[0] += p;
        self.p[1] += p;
        self.p[2] += p;
    }

    /// Creates a new Triangle instance from matrix application to the given triangle
    pub fn from_matrix_application(m: &Matrix, origin: &Triangle) -> Self {
        Triangle {
            p: [m.apply(&origin.p[0]), m.apply(&origin.p[1]), m.apply(&origin.p[2])],
            base_color: origin.base_color,
            color: origin.color,
        }
    }
}