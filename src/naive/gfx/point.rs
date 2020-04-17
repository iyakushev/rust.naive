use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point {x, y, z}
    }
    pub fn as_tuple(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    /// Normalizes the point.
    pub fn normalize(&mut self) -> &mut Self {
        let l = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt(); //Vector length
        self.x /= l; self.y /= l; self.z /= l;
        self
    }

    pub fn dot_product(&self, other: &Point) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}