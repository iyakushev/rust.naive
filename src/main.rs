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
}

#[derive(Copy, Clone)]
struct Triangle {
    p: [Point; 3]
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Triangle {p: [p1, p2, p3]}
    }

    pub fn pos_at(&self, idx: usize) -> (i32, i32) {
        if idx > 2 {exit(11);}
        (self.p[idx].x as i32, self.p[idx].y as i32)
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

    pub fn multiply_by_point(&self, input: &Point) -> Point {
        let x = input.x * self.values[0][0] + input.y * self.values[1][0] + input.z * self.values[2][0] + self.values[3][0];
        let y = input.x * self.values[0][1] + input.y * self.values[1][1] + input.z * self.values[2][1] + self.values[3][1];
        let z = input.x * self.values[0][2] + input.y * self.values[1][2] + input.z * self.values[2][2] + self.values[3][2];
        let e = input.x * self.values[0][3] + input.y * self.values[1][3] + input.z * self.values[2][3] + self.values[3][3];

        if e != 0.0 {
            Point {x: x/e, y: y/e, z: z/e}
        } else { Point {x: 0.0, y: 0.0, z: 0.0} }
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
            Triangle::new(Point::new(1.0, 0.0, 1.0), Point::new(0.0, 1.0, 1.0), Point::new(0.0, 1.0, 1.0)),

            Triangle::new(Point::new(0.0, 0.0, 1.0), Point::new(0.0, 1.0, 1.0), Point::new(0.0, 1.0, 0.0)), // WEST
            Triangle::new(Point::new(0.0, 0.0, 1.0), Point::new(0.0, 1.0, 0.0), Point::new(0.0, 0.0, 0.0)),

            Triangle::new(Point::new(0.0, 1.0, 0.0), Point::new(0.0, 1.0, 1.0), Point::new(1.0, 1.0, 1.0)), // TOP
            Triangle::new(Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 1.0), Point::new(1.0, 1.0, 0.0)),

            Triangle::new(Point::new(1.0, 0.0, 1.0), Point::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, 0.0)), // BOTTOM
            Triangle::new(Point::new(1.0, 0.0, 1.0), Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)),
    ]};

    let mut f_theta: f32 = 0.0;
    let mut elapsed_time = 0.0;
    let f_near  = 0.1;
    let f_far   = 1000.0;
    let f_fov   = 90.0;
    let f_ratio = (W_HEIGHT/W_WIDTH) as f32;
    let f_fov_r = 1.0 / (f_fov * 0.5 / 180.0 * std::f32::consts::PI).tan();


    let mut m = Matrix::init();
    m.values[0][0] = f_ratio * f_fov_r;
    m.values[1][1] = f_fov_r;
    m.values[2][2] = f_far/(f_far - f_near);
    m.values[3][2] = (-f_far * f_near) / (f_far - f_near);
    m.values[2][3] = 1.0;

    let mut event = window.create_event_pump();

    'run: loop {
        f_theta += 1.0 * elapsed_time;
        window.draw_bg(Color::RGB(54,54,54));

        for e in event.poll_iter() {
            match e {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,
                _ => {}
            }
        }

        let mut rot_z = Matrix::init();
        m.values[0][0] = f_theta.cos() as f32;
        m.values[0][1] = f_theta.sin() as f32;
        m.values[1][0] = -f_theta.sin() as f32;
        m.values[1][1] = f_theta.cos() as f32;
        m.values[2][2] = 1.0;
        m.values[3][3] = 1.0;

        let mut rot_x = Matrix::init();
        m.values[0][0] = 1.0;
        m.values[1][1] = (f_theta*0.5).cos() as f32;
        m.values[1][2] = (f_theta*0.5).sin() as f32;
        m.values[2][1] = -(f_theta*0.5).sin() as f32;
        m.values[2][2] = (f_theta*0.5).cos() as f32;
        m.values[3][3] = 1.0;


        // Triangles
        for tri in mesh.tris.iter_mut() {

            let mut tri_z_rot = Triangle::new(rot_z.multiply_by_point(&tri.p[0]),
                                              rot_z.multiply_by_point(&tri.p[1]),
                                              rot_z.multiply_by_point(&tri.p[2]));
            let mut tri_x_rot = Triangle::new(rot_x.multiply_by_point(&tri_z_rot.p[0]),
                                              rot_x.multiply_by_point(&tri_z_rot.p[1]),
                                              rot_x.multiply_by_point(&tri_z_rot.p[2]));


            let mut translation = tri_x_rot.clone();
            translation.p[0].z = tri_x_rot.p[0].z + 3.0;
            translation.p[1].z = tri_x_rot.p[1].z + 3.0;
            translation.p[2].z = tri_x_rot.p[2].z + 3.0;


            let mut projection = Triangle::new(m.multiply_by_point(&translation.p[0]),
                                           m.multiply_by_point(&translation.p[1]),
                                           m.multiply_by_point(&translation.p[2]));

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
            window.draw_triangle(C_WHITE, projection.pos_at(0), projection.pos_at(1), projection.pos_at(2));
        }


        window.present();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000));
        elapsed_time += 1.0;
    }

}

fn main() {
    println!("Hello, world!");
    create();
}
