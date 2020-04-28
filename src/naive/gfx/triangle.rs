use super::{matrix::Matrix, vector::Vec3D};
use sdl2::pixels::Color;
use std::cmp::max;
use super::vector;
use crate::naive::render::Window;

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub p: [Vec3D; 3],
    pub base_color: Color,
    pub color: Color
}

impl Triangle {
    // Creates a new Triangle instance
    pub fn new(p1: Vec3D, p2: Vec3D, p3: Vec3D, base_color: Option<Color>) -> Self {
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
        self.p[0] = vector::vec_div_by(&self.p[0], self.p[0].w);
        self.p[1] = vector::vec_div_by(&self.p[1], self.p[1].w);
        self.p[2] = vector::vec_div_by(&self.p[2], self.p[2].w);
    }

    pub fn shade(&mut self, luminance: f32) {
        self.color.r = (self.base_color.r as f32*luminance) as u8;
        self.color.g = (self.base_color.g as f32*luminance) as u8;
        self.color.b = (self.base_color.b as f32*luminance) as u8;
    }

    /// Add value to each point
    pub fn add_each_point(&mut self, p: Vec3D) {
        self.p[0] += p;
        self.p[1] += p;
        self.p[2] += p;
    }

    pub fn draw_fast(&self, window: &mut Window, fill: bool) {
        let points = self.get_2d_points();
        if fill {
            window.fill_triangle(self.color, points[0], points[1], points[2]).unwrap();
        }
        window.draw_triangle_fast(self.color,
                                  (self.p[0].x as i16, self.p[0].y as i16),
                                  (self.p[1].x as i16, self.p[1].y as i16),
                                  (self.p[2].x as i16, self.p[2].y as i16)).unwrap();
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

/// Checks if a triangle is clipping against the plane and returns its decomposition
pub fn clipping(plane_point: &Vec3D, plane_normal: &mut Vec3D, triangle: &Triangle) -> [Option<Triangle>;2] {
    plane_normal.normalize();
    let dist = |mut p: Vec3D| { // a helping lambda function to acquire distances
        plane_normal.dot_product(&p) - plane_normal.dot_product(plane_point)
    };

    let mut in_points: [Vec3D;3] = [Vec3D::init();3];
    let mut out_points:[Vec3D;3] = [Vec3D::init();3];
    let mut in_count = 0;
    let mut out_count = 0;
    let distances = [dist(triangle.p[0]), dist(triangle.p[1]), dist(triangle.p[2])];

    for i in 0..distances.len() {
        if distances[i] > 0.0 {
            in_points[in_count] = triangle.p[i];
            in_count += 1;
        } else {
            out_points[out_count] = triangle.p[i];
            out_count += 1;
        }
    }

    // Classification
    return if in_count == 3 {
        // All points lie on the inside of plane, so do nothing
        // and allow the triangle to simply pass through
        [Some(*triangle), None]
    } else if in_count == 1 && out_count == 2 {
        let out_triangle =
            Triangle::new(in_points[0],
                          vector::intersect_plane(*plane_point, plane_normal, in_points[0], out_points[0]),
                          vector::intersect_plane(*plane_point, plane_normal, in_points[0], out_points[1]),
                          Some(triangle.color));

        return [Some(out_triangle), None]
    } else if in_count == 2 && out_count == 1 {
        let out_triangle1 =
            Triangle::new(in_points[0],
                          in_points[1],
                          vector::intersect_plane(*plane_point, plane_normal, in_points[0], out_points[0]),
                          Some(triangle.color));

        let out_triangle2 =
            Triangle::new(in_points[1],
                          out_triangle1.p[2],
                          vector::intersect_plane(*plane_point, plane_normal, in_points[1], out_points[0]),
                          Some(triangle.color));
        return [Some(out_triangle1), Some(out_triangle2)]
    } else {  // in_count == 0
        // Triangle is beyond the plane, so cut it entirely
        return [None, None]
    }
}