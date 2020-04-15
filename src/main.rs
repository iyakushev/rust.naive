mod engine;
use engine::render::Window;
use std::process::exit;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color
};

const C_WHITE: Color = Color::RGBA(255,255,255,255);
const W_WIDTH:   u32 = 640;
const W_HEIGHT:  u32 = 480;

#[derive(Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point {x, y, z}
    }

    pub fn as_tuple(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}

#[derive(Copy, Clone)]
struct Triangle {
    p: [Point; 3]
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

struct Mesh {
    tris: Vec<Triangle>
}


struct Matrix {
    values: [[f32; 4]; 4] // 4x4 array
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


fn create() {
    let mut window = Window::new(W_WIDTH, W_HEIGHT);

    // UNIT-CUBE
     let mut mesh = Mesh{
        tris: vec![
            Triangle::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0)), // SOUTH
            Triangle::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0), Point::new(1.0, 0.0, 0.0)),

            Triangle::new(Point::new(1.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0), Point::new(1.0, 1.0, 1.0)), // EAST
            Triangle::new(Point::new(1.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0), Point::new(1.0, 0.0, 1.0)),

            Triangle::new(Point::new(1.0, 0.0, 1.0), Point::new(1.0, 1.0, 1.0), Point::new(0.0, 1.0, 1.0)), // NORTH
            Triangle::new(Point::new(1.0, 0.0, 1.0), Point::new(0.0, 1.0, 1.0), Point::new(0.0, 0.0, 1.0)),

            Triangle::new(Point::new(0.0, 0.0, 1.0), Point::new(0.0, 1.0, 1.0), Point::new(0.0, 1.0, 0.0)), // WEST
            Triangle::new(Point::new(0.0, 0.0, 1.0), Point::new(0.0, 1.0, 0.0), Point::new(0.0, 0.0, 0.0)),

            Triangle::new(Point::new(0.0, 1.0, 0.0), Point::new(0.0, 1.0, 1.0), Point::new(1.0, 1.0, 1.0)), // TOP
            Triangle::new(Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 1.0), Point::new(1.0, 1.0, 0.0)),

            Triangle::new(Point::new(1.0, 0.0, 1.0), Point::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, 0.0)), // BOTTOM
            Triangle::new(Point::new(1.0, 0.0, 1.0), Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)),
    ]};

    let mut f_theta: f32 = 0.0;
    let near  = 0.1;
    let far   = 1000.0;
    let fov   = 90.0;
    let ratio = W_HEIGHT as f32/ W_WIDTH as f32;
    let fov_r = 1.0 / ((fov * 0.5) * (std::f32::consts::PI/180.0)).tan(); // ~~ Transform to radians.


    let mut m = Matrix::init();
    m.values[0][0] = ratio * fov_r;
    m.values[1][1] = fov_r;
    m.values[2][2] = far/(far - near);
    m.values[3][2] = (-far * near) / (far - near);
    m.values[2][3] = 1.0;

    let mut event = window.create_event_pump();

    'run: loop {
        f_theta += 0.05;
        window.draw_bg(Color::RGB(54,54,54));

        for e in event.poll_iter() {
            match e {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,
                _ => {}
            }
        }

        let mut rot_z = Matrix::init();
        rot_z.values[0][0] = f_theta.cos() as f32;
        rot_z.values[0][1] = f_theta.sin() as f32;
        rot_z.values[1][0] = -f_theta.sin() as f32;
        rot_z.values[1][1] = f_theta.cos() as f32;
        rot_z.values[2][2] = 1.0;
        rot_z.values[3][3] = 1.0;

        let mut rot_x = Matrix::init();
        rot_x.values[0][0] = 1.0;
        rot_x.values[1][1] = (f_theta*0.5).cos() as f32;
        rot_x.values[1][2] = (f_theta*0.5).sin() as f32;
        rot_x.values[2][1] = -(f_theta*0.5).sin() as f32;
        rot_x.values[2][2] = (f_theta*0.5).cos() as f32;
        rot_x.values[3][3] = 1.0;


        // Triangles
        for tri in mesh.tris.iter() {
            // Rotate along z-axis
            let mut tri_z_rot = Triangle::from_matrix_application(&rot_z, &tri);

            // Rotate along x-axis as well
            let mut tri_zx_rot = Triangle::from_matrix_application(&rot_x, &tri_z_rot);


            // MOVE BACK THE MESH ALONG THE Z-AXIS
            let mut translation = tri_zx_rot.clone();
            translation.p[0].z += 3.0;
            translation.p[1].z += 3.0;
            translation.p[2].z += 3.0;

            // 3D -> 2D
            let mut projection = Triangle::from_matrix_application(&m, &translation);

            // Scale
            projection.p[0].x += 1.0;
            projection.p[0].y += 1.0;
            projection.p[1].x += 1.0;
            projection.p[1].y += 1.0;
            projection.p[2].x += 1.0;
            projection.p[2].y += 1.0;

            projection.p[0].x *= 0.5 * W_WIDTH as f32;
            projection.p[0].y *= 0.5 * W_HEIGHT as f32;
            projection.p[1].x *= 0.5 * W_WIDTH as f32;
            projection.p[1].y *= 0.5 * W_HEIGHT as f32;
            projection.p[2].x *= 0.5 * W_WIDTH as f32;
            projection.p[2].y *= 0.5 * W_HEIGHT as f32;


            // DRAW
            let points = projection.get_2d_points();
            window.draw_triangle(C_WHITE, points[0], points[1], points[2]);
        }


        window.present();
        // std::thread::sleep(std::time::Duration::new(0, 1_000_000_000/1000));
    }

}

fn main() {
    create();
}
