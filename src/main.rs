mod naive;

use naive::render::Window;
use naive::gfx::{Triangle, Point, Matrix, Mesh};
use naive::gfx;
use std::process::exit;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color
};
use std::cmp::max;

const C_WHITE: Color = Color::RGBA(255,255,255,255);
const C_BLACK: Color = Color::RGBA(0,0,0,255);
const W_WIDTH:   u32 = 640;
const W_HEIGHT:  u32 = 480;

fn create() {
    let mut window = Window::new(W_WIDTH, W_HEIGHT);
    let mut mesh = Mesh::load_object("example_objs/axis.obj");

    let mut f_theta: f32 = 0.0;
    let near  = 0.1;
    let far   = 1000.0;
    let fov   = 90.0;
    let ratio = W_HEIGHT as f32/ W_WIDTH as f32;

    let mut m = Matrix::init_projection(fov, ratio, near, far);

    let mut event = window.create_event_pump();
    let mut timer = std::time::Instant::now();
    let mut fps = 0;

    let mut camera = Point::init();
    let look_d = Point::new(0.0,0.0,1.0);

    'run: loop {
        // f_theta += 0.01;
        window.draw_bg(Color::RGB(54,54,54));

        for e in event.poll_iter() {
            match e {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => camera.y += 0.8,
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => camera.y -= 0.8,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => camera.x -= 0.8,
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => camera.x += 0.8,
                _ => {}
            }
        }

        let mut raster_triangles = Vec::new();

        let mat_rz = Matrix::init_rotation_z(f_theta * 0.5);
        let mat_rx = Matrix::init_rotation_x(f_theta);
        let mat_trans = Matrix::init_translation(0.0, 0.0, 10.0); // move back along z-axis 16 units
        let mut mat_world = Matrix::init_identity();
        mat_world = mat_rz * mat_rx; // apply rotation to the world
        mat_world = mat_world * mat_trans; // apply translation to the world

        let up_vec = Point::new(0.0,1.0,0.0);
        let target = camera + look_d;
        let mat_view = Matrix::point_at(&camera, &target, &up_vec).quick_inverse();


        // Triangles
        for tri in mesh.tris.iter() {

            let t_transformed = Triangle::from_matrix_application(&mat_world, tri);

            let line1 = t_transformed.p[1] - t_transformed.p[0];
            let line2 = t_transformed.p[2] - t_transformed.p[0];
            let mut normal = gfx::point::cross_product(&line1, &line2);

            // a ray from triangle to camera
            let camera_ray = t_transformed.p[0] - camera;

            if normal.normalize().dot_product(&camera_ray) < 0.0 {
                let mut illumination = Point::new(0.0, 1.0, -1.0); // Facing camera
                let dp = illumination.normalize().dot_product(&normal).max(0.1);

                // apply view matrix
                let t_viewed = Triangle::from_matrix_application(&mat_view, &t_transformed);

                // 3D -> 2D
                let mut projection = Triangle::from_matrix_application(&m, &t_viewed);
                projection.normalize();
                projection.shade(dp);

                // Scale
                projection.add_each_point(Point::new(1.0,1.0,0.0));

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
            window.fill_triangle(tri.color, points[0], points[1], points[2]).unwrap();
            window.draw_triangle(tri.color, points[0], points[1], points[2]).unwrap(); // wireframe
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
