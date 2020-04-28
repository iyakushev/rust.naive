use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};

#[derive(Copy, Clone, Debug)]
pub struct Vec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl AddAssign for Vec3D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3D {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Add for Vec3D {
    type Output = Vec3D;
    fn add(self, other: Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: 1.0,
        }
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;
    fn sub(self, other: Vec3D) -> Vec3D {
        Vec3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: 1.0,
        }
    }
}

pub fn vec_mul_by(p: &Vec3D, multiplier: f32) -> Vec3D {
    Vec3D {
        x: p.x * multiplier,
        y: p.y * multiplier,
        z: p.z * multiplier,
        w: 1.0,
    }
}

pub fn vec_div_by(p: &Vec3D, divider: f32) -> Vec3D {
    Vec3D {
        x: p.x / divider,
        y: p.y / divider,
        z: p.z / divider,
        w: 1.0,
    }
}

/// Returns a cross product between two vectors
pub fn cross_product(p: &Vec3D, other: &Vec3D) -> Vec3D {
    Vec3D {
        x: p.y * other.z - p.z * other.y,
        y: p.z * other.x - p.x * other.z,
        z: p.x * other.y - p.y * other.x,
        w: 1.0
    }
}

pub fn intersect_plane(plane_point: Vec3D, plane_normal: &mut Vec3D, ray_start: Vec3D, ray_finish: Vec3D) -> Vec3D {
    plane_normal.normalize();
    let a = ray_start.dot_product(plane_normal);
    let b = ray_finish.dot_product(plane_normal);
    let t = (plane_normal.dot_product(&plane_point) - a)/(b - a);
    ray_start + vec_mul_by(&(ray_finish - ray_start), t)
}

impl Vec3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3D {
        Vec3D {x, y, z, w: 1.0}
    }
    pub fn init() -> Self { Vec3D {x: 0.0, y: 0.0, z: 0.0, w: 1.0} }
    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }

    /// Returns a length of the vector: |Vec3D|
    pub fn len(&self) -> f32 {
        self.dot_product(self).sqrt()
    }

    /// Normalizes the vector against its length.
    pub fn normalize(&mut self) -> &mut Self {
        let l = self.len(); //Vector length
        self.x /= l; self.y /= l; self.z /= l;
        self
    }

    /// Returns a dot product between self and another Vec3D
    pub fn dot_product(&self, other: &Vec3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

}