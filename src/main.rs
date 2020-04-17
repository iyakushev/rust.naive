mod naive;
use naive::render::Window;
use naive::gfx::{Triangle, Point, Matrix, Mesh};
use std::process::exit;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color
};

const C_WHITE: Color = Color::RGBA(255,255,255,255);
const C_GREYT: Color = Color::RGBA(50,50,50,128);
const C_BLACK: Color = Color::RGBA(0,0,0,255);
const W_WIDTH:   u32 = 640;
const W_HEIGHT:  u32 = 480;

fn create() {
    let mut window = Window::new(W_WIDTH, W_HEIGHT);
    let mut mesh = Mesh::load_object("example_objs/space_ship.obj");

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
    let mut timer = std::time::Instant::now();
    let mut fps = 0;

    let camera = Point::new(0.0,0.0,0.0);

    'run: loop {
        f_theta += 0.01;
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

        let mut raster_triangles = Vec::new();

        // Triangles
        for tri in mesh.tris.iter() {
            // Rotate along z-axis
            let mut tri_z_rot = Triangle::from_matrix_application(&rot_z, &tri);

            // Rotate along x-axis as well
            let mut tri_zx_rot = Triangle::from_matrix_application(&rot_x, &tri_z_rot);


            // MOVE BACK THE MESH ALONG THE Z-AXIS
            let mut translation = tri_zx_rot.clone();
            translation.p[0].z += 8.0;
            translation.p[1].z += 8.0;
            translation.p[2].z += 8.0;

            let line1 = Point {
                x: translation.p[1].x - translation.p[0].x,
                y: translation.p[1].y - translation.p[0].y,
                z: translation.p[1].z - translation.p[0].z
            };
            let line2 = Point {
                x: translation.p[2].x - translation.p[0].x,
                y: translation.p[2].y - translation.p[0].y,
                z: translation.p[2].z - translation.p[0].z
            };
            let mut normal = Point {
                x: line1.y * line2.z - line1.z * line2.y,
                y: line1.z * line2.x - line1.x * line2.z,
                z: line1.x * line2.y - line1.y * line2.x,
            };

            normal.normalize();

            if normal.x * (translation.p[0].x - camera.x) +
               normal.y * (translation.p[0].y - camera.y) +
               normal.z * (translation.p[0].z - camera.z) < 0.0 {

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

                raster_triangles.push(projection);
            }
        }

        raster_triangles.sort_by(|&t1, &t2| {
            let z1 = (t1.p[0].z + t1.p[1].z + t1.p[2].z)/3.0;
            let z2 = (t2.p[0].z + t2.p[1].z + t2.p[2].z)/3.0;
            z2.partial_cmp(&z1).unwrap()
        });
        // DRAW
        for tri in &raster_triangles {
            let points = tri.get_2d_points();
            window.fill_triangle(C_WHITE, points[0], points[1], points[2]).unwrap();
            window.draw_triangle(C_BLACK, points[0], points[1], points[2]).unwrap(); // wireframe
        }



        if timer.elapsed().as_secs() > 1 {
            window.set_title(&format!("NAIVE WINDOW. FPS: {}", fps));
            timer = std::time::Instant::now();
            fps = 0;
        }
        window.present();
        fps += 1;
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000/100));

    }

}

fn main() {
    create();
}
