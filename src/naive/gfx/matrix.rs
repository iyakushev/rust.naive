use super::point::{Point, cross_product, point_mul};
use std::ops::Mul;

const UNIVERSAL_ARRAY_SIZE: usize = 4;

pub struct Matrix {
    pub values: [[f32; UNIVERSAL_ARRAY_SIZE]; UNIVERSAL_ARRAY_SIZE] // 4x4 array
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, m: Self) -> Self {
        let mut result = Matrix::init();
        for col in 0..UNIVERSAL_ARRAY_SIZE {
            for row in 0..UNIVERSAL_ARRAY_SIZE {
                result.values[row][col] = self.values[row][0] * m.values[0][col] +
                    self.values[row][1] * m.values[1][col] +
                    self.values[row][2] * m.values[2][col] +
                    self.values[row][3] * m.values[3][col];
            }
        }
        result
    }
}

impl Matrix {
    pub fn init() -> Self {
        Matrix { values: [[0.0;4]; 4] }
    }

    pub fn init_identity() -> Self {
        let mut matrix = Matrix::init();
        matrix.values[0][0] = 1.0;
        matrix.values[1][1] = 1.0;
        matrix.values[2][2] = 1.0;
        matrix.values[3][3] = 1.0;
        matrix
    }

    pub fn init_projection(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let fov_r = 1.0 / ((fov * 0.5) * (std::f32::consts::PI/180.0)).tan(); // ~~ Transform to radians.
        let mut m = Matrix::init();
        m.values[0][0] = aspect_ratio * fov_r;
        m.values[1][1] = fov_r;
        m.values[2][2] = far/(far - near);
        m.values[3][2] = (-far * near) / (far - near);
        m.values[2][3] = 1.0;
        m
    }

    pub fn init_rotation_x(rad_angle: f32) -> Self {
        let mut rot_x = Matrix::init();
        rot_x.values[0][0] = 1.0;
        rot_x.values[1][1] = (rad_angle*0.5).cos() as f32;
        rot_x.values[1][2] = (rad_angle*0.5).sin() as f32;
        rot_x.values[2][1] = -(rad_angle*0.5).sin() as f32;
        rot_x.values[2][2] = (rad_angle*0.5).cos() as f32;
        rot_x.values[3][3] = 1.0;
        rot_x
    }

    pub fn init_rotation_y(rad_angle: f32) -> Self {
        let mut rot_y = Matrix::init();
        rot_y.values[0][0] = rad_angle.cos() as f32;
        rot_y.values[0][2] = rad_angle.sin() as f32;
        rot_y.values[2][0] = -rad_angle.sin() as f32;
        rot_y.values[1][1] = 1.0;
        rot_y.values[2][2] = rad_angle.cos() as f32;
        rot_y.values[3][3] = 1.0;
        rot_y
    }

    pub fn init_rotation_z(rad_angle: f32) -> Self {
        let mut rot_z = Matrix::init();
        rot_z.values[0][0] = rad_angle.cos() as f32;
        rot_z.values[0][1] = rad_angle.sin() as f32;
        rot_z.values[1][0] = -rad_angle.sin() as f32;
        rot_z.values[1][1] = rad_angle.cos() as f32;
        rot_z.values[2][2] = 1.0;
        rot_z.values[3][3] = 1.0;
        rot_z
    }

    pub fn init_translation(x: f32, y: f32, z: f32) -> Self {
        let mut translation = Matrix::init();
        translation.values[0][0] = 1.0;
        translation.values[1][1] = 1.0;
        translation.values[2][2] = 1.0;
        translation.values[3][3] = 1.0;
        translation.values[3][0] = x;
        translation.values[3][1] = y;
        translation.values[3][2] = z;
        translation
    }

    pub fn point_at(pos: &Point, target: &Point, up: &Point) -> Matrix {
        let mut forward: Point = *target - *pos;
        forward.normalize();

        let mut new_up: Point = *up - point_mul(forward, forward.dot_product(up));
        new_up.normalize();

        let mut right = cross_product(&new_up, &forward);

        let mut m = Matrix::init();
        m.values[0][0] = right.x;   m.values[0][1] = right.y;   m.values[0][2] = right.z;   m.values[0][3] = 0.0;
        m.values[1][0] = new_up.x;  m.values[1][1] = new_up.y;  m.values[1][2] = new_up.z;  m.values[1][3] = 0.0;
        m.values[2][0] = forward.x; m.values[2][1] = forward.y; m.values[2][2] = forward.z; m.values[2][3] = 0.0;
        m.values[3][0] = pos.x;     m.values[3][1] = pos.y;     m.values[3][2] = pos.z;     m.values[3][3] = 1.0;
        m
    }

    /// Applies a quick transpose calculation of a translation/rotation matrices
    pub fn quick_inverse(&self) -> Matrix {
        let mut m = Matrix::init();
        m.values[0][0] = self.values[0][0]; m.values[0][1] = self.values[1][0]; m.values[0][2] = self.values[2][0]; m.values[0][3] = 0.0;
        m.values[1][0] = self.values[0][1]; m.values[1][1] = self.values[1][1]; m.values[1][2] = self.values[2][1]; m.values[1][3] = 0.0;
        m.values[2][0] = self.values[0][2]; m.values[2][1] = self.values[1][2]; m.values[2][2] = self.values[2][2]; m.values[2][3] = 0.0;
        m.values[3][0] = -(self.values[3][0]*m.values[0][0] + self.values[3][1]*m.values[1][0] + self.values[3][2]*m.values[2][0]);
        m.values[3][1] = -(self.values[3][0]*m.values[0][1] + self.values[3][1]*m.values[1][1] + self.values[3][2]*m.values[2][1]);
        m.values[3][2] = -(self.values[3][0]*m.values[0][2] + self.values[3][1]*m.values[1][2] + self.values[3][2]*m.values[2][2]);
        m.values[3][3] = 1.0;
        m
    }

    pub fn apply(&self, input: &Point) -> Point {
        Point {
            x: input.x * self.values[0][0] + input.y * self.values[1][0] + input.z * self.values[2][0] + input.w * self.values[3][0],
            y: input.x * self.values[0][1] + input.y * self.values[1][1] + input.z * self.values[2][1] + input.w * self.values[3][1],
            z: input.x * self.values[0][2] + input.y * self.values[1][2] + input.z * self.values[2][2] + input.w * self.values[3][2],
            w: input.x * self.values[0][3] + input.y * self.values[1][3] + input.z * self.values[2][3] + input.w * self.values[3][3]
        }
    }
}
