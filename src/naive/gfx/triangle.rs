use super::{matrix::Matrix, point::Point};

#[derive(Copy, Clone)]
pub struct Triangle {
    pub p: [Point; 3]
}

impl Triangle {
    // Creates a new Triangle instance
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Triangle {p: [p1, p2, p3]}
    }

    // Returns an array of 2 tuples: i32 for X,Y
    pub fn get_2d_points(&self) -> [(i32, i32);3] {
        [(self.p[0].x as i32, self.p[0].y as i32),
            (self.p[1].x as i32, self.p[1].y as i32),
            (self.p[2].x as i32, self.p[2].y as i32)]
    }

    // Returns an array of 3 tuples: i32 for X,Y,Z
    pub fn get_3d_points(&self) -> [(f32,f32,f32); 3] {
        [(self.p[0].x, self.p[0].y, self.p[0].z),
            (self.p[1].x, self.p[1].y, self.p[1].z),
            (self.p[2].x, self.p[2].y, self.p[2].z)]
    }

    // Creates a new Triangle instance from matrix application to the given triangle
    pub fn from_matrix_application(m: &Matrix, origin: &Triangle) -> Self {
        Triangle {
            p: [m.apply(&origin.p[0]), m.apply(&origin.p[1]), m.apply(&origin.p[2])]
        }
    }
}