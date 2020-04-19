use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: 1.0,
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
            w: 1.0,
        }
    }
}

pub fn point_mul(p: Point, multiplier: f32) -> Point {
    Point {
        x: p.x * multiplier,
        y: p.y * multiplier,
        z: p.z * multiplier,
        w: 1.0,
    }
}

pub fn point_div(p: Point, divider: f32) -> Point {
    Point {
        x: p.x / divider,
        y: p.y / divider,
        z: p.z / divider,
        w: 1.0,
    }
}


pub fn cross_product(p: &Point, other: &Point) -> Point {
    Point {
        x: p.y * other.z - p.z * other.y,
        y: p.z * other.x - p.x * other.z,
        z: p.x * other.y - p.y * other.x,
        w: 1.0
    }
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point {x, y, z, w: 1.0}
    }
    pub fn init() -> Self { Point {x: 0.0, y: 0.0, z: 0.0, w: 1.0} }
    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }

    pub fn len(&self) -> f32 {
        self.dot_product(self).sqrt()
    }

    /// Normalizes the point.
    pub fn normalize(&mut self) -> &mut Self {
        let l = self.len(); //Vector length
        self.x /= l; self.y /= l; self.z /= l;
        self
    }

    pub fn dot_product(&self, other: &Point) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

}