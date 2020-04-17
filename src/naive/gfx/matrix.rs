use super::point::Point;

pub struct Matrix {
    pub values: [[f32; 4]; 4] // 4x4 array
}

impl Matrix {
    pub fn init() -> Self {
        Matrix { values: [[0.0;4]; 4] }
    }

    pub fn apply(&self, input: &Point) -> Point {
        let mut p = Point {
            x: input.x * self.values[0][0] + input.y * self.values[1][0] + input.z * self.values[2][0] + self.values[3][0],
            y: input.x * self.values[0][1] + input.y * self.values[1][1] + input.z * self.values[2][1] + self.values[3][1],
            z: input.x * self.values[0][2] + input.y * self.values[1][2] + input.z * self.values[2][2] + self.values[3][2],
        };
        let e = input.x * self.values[0][3] + input.y * self.values[1][3] + input.z * self.values[2][3] + self.values[3][3];
        if e != 0.0 {
            p.x /= e;
            p.y /= e;
            p.z /= e;
        }
        p
    }
}
