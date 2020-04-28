mod naive;
#[allow(dead_code)]
use naive::render::Window;
use naive::gfx::{triangle, vector, Triangle, Vec3D, Matrix, Mesh};
use naive::gfx;
use std::process::exit;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color
};
use std::cmp::max;
use crate::naive::gfx::vector::vec_mul_by;
use std::collections::LinkedList;

const C_WHITE: Color = Color::RGBA(255,255,255,255);
const C_BLACK: Color = Color::RGBA(0,0,0,255);
const W_WIDTH:   u32 = 640;
const W_HEIGHT:  u32 = 480;

fn create() {
    let mut window = Window::new(W_WIDTH, W_HEIGHT);
    let mut mesh = Mesh::load_object("example_objs/landscape.obj");

    let mut f_theta: f32 = 0.0;
    let near  = 0.1;
    let far   = 1000.0;
    let fov   = 90.0;
    let ratio = W_HEIGHT as f32/ W_WIDTH as f32;

    let mut m = Matrix::init_projection(fov, ratio, near, far);

    let mut event = window.create_event_pump();
    let mut timer = std::time::Instant::now();
    let mut draw_timer = std::time::Instant::now();
    let mut fps = 0;
    let mut yaw = 0.0;

    let mut camera = Vec3D::init();
    let mut look_d = Vec3D::new(0.0, 0.0, 1.0);

    'run: loop {
        // f_theta += 0.01;
        let elapsed_time = draw_timer.elapsed().as_micros() as f32 / 1e5;
        draw_timer = std::time::Instant::now();
        // window.draw_bg(Color::RGB(54,54,54));
        window.draw_bg(C_BLACK);
        let forward = vec_mul_by(&look_d, 8.0 * elapsed_time);
        for e in event.poll_iter() {
            match e {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,
                // Tilting
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => camera.y += 10.0 * elapsed_time,
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => camera.y -= 10.0 * elapsed_time,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => camera.x += 10.0 * elapsed_time,
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => camera.x -= 10.0 * elapsed_time,
                // Movement
                Event::KeyDown { keycode: Some(Keycode::W), .. } => camera += forward,
                Event::KeyDown { keycode: Some(Keycode::S), .. } => camera -= forward,
                Event::KeyDown { keycode: Some(Keycode::A), .. } => yaw -= 3.0 * elapsed_time,
                Event::KeyDown { keycode: Some(Keycode::D), .. } => yaw += 3.0 * elapsed_time,
                _ => {}
            }
        }

        let mut raster_triangles = Vec::new();

        let mat_rz = Matrix::init_rotation_z(f_theta);
        let mat_rx = Matrix::init_rotation_x(f_theta);
        let mat_ry = Matrix::init_rotation_y(f_theta);
        let mat_trans = Matrix::init_translation(0.0, 0.0, 10.0);
        let mut mat_world = Matrix::init_identity();
        mat_world = mat_rz * mat_rx * mat_ry; // apply rotation to the world
        mat_world = mat_world * mat_trans; // apply translation to the world

        let mut target = Vec3D::new(0.0, 0.0, 1.0);
        let up_vec = Vec3D::new(0.0, -1.0, 0.0);
        let mat_crot = Matrix::init_rotation_y(yaw);
        look_d = mat_crot.apply(&target);
        target = camera + look_d;
        let mat_view = Matrix::point_at(&camera, &target, &up_vec).quick_inverse();


        // Triangles
        for tri in mesh.tris.iter() {

            let t_transformed = Triangle::from_matrix_application(&mat_world, tri);

            let line1 = t_transformed.p[1] - t_transformed.p[0];
            let line2 = t_transformed.p[2] - t_transformed.p[0];
            let mut normal = gfx::vector::cross_product(&line1, &line2);

            // a ray from triangle to camera
            let camera_ray = t_transformed.p[0] - camera;

            if normal.normalize().dot_product(&camera_ray) < 0.0 {
                let mut illumination = Vec3D::new(0.0, 1.0, -1.0); // Facing camera
                let dp = illumination.normalize().dot_product(&normal).max(0.1);

                // apply view matrix
                let t_viewed = Triangle::from_matrix_application(&mat_view, &t_transformed);

                // check clipping
                let clipped = triangle::clipping(&Vec3D::new(0.0, 0.0, 0.1),
                                                 &mut Vec3D::new(0.0, 0.0, 1.0), &t_viewed);
                for clip in clipped.iter() {
                    if clip.is_none() {continue}
                    let t_clipped = clip.unwrap();

                    // 3D -> 2D
                    let mut projection = Triangle::from_matrix_application(&m, &t_clipped);
                    projection.normalize();
                    projection.shade(dp);

                    // Scale
                    projection.add_each_point(Vec3D::new(1.0, 1.0, 0.0));

                    projection.p[0].x *= 0.5 * W_WIDTH as f32;
                    projection.p[0].y *= 0.5 * W_HEIGHT as f32;
                    projection.p[1].x *= 0.5 * W_WIDTH as f32;
                    projection.p[1].y *= 0.5 * W_HEIGHT as f32;
                    projection.p[2].x *= 0.5 * W_WIDTH as f32;
                    projection.p[2].y *= 0.5 * W_HEIGHT as f32;

                    raster_triangles.push(projection);
                }
            }
        }


        raster_triangles.sort_by(|&t1, &t2| {
            let z1 = (t1.p[0].z + t1.p[1].z + t1.p[2].z)/3.0;
            let z2 = (t2.p[0].z + t2.p[1].z + t2.p[2].z)/3.0;
            z2.partial_cmp(&z1).unwrap()
        });

        // CLIP AGAINST THE SCREEN
        for tri_raster in &raster_triangles {
            let mut alist = LinkedList::new();
            let mut count = 1;
            alist.push_back(*tri_raster);

            for p in 0..4 {
                while count > 0 {
                    let test: Triangle = alist.pop_front().unwrap();
                    count -= 1;

                    let clipped = match p {
                        0 => triangle::clipping(&Vec3D::new(0.0,0.0,0.0),
                                                &mut Vec3D::new(0.0,1.0,0.0), &test),

                        1 => triangle::clipping(&Vec3D::new(0.0, W_HEIGHT as f32 - 1.0, 0.0),
                                                &mut Vec3D::new(0.0,-1.0,0.0), &test),

                        2 => triangle::clipping(&Vec3D::new(0.0,0.0,0.0),
                                                &mut Vec3D::new(1.0,0.0,0.0), &test),

                        3 => triangle::clipping(&Vec3D::new(W_WIDTH as f32 - 1.0,0.0,0.0),
                                                &mut Vec3D::new(-1.0,0.0,0.0), &test),
                        _ => exit(4)
                    };

                    for clip in &clipped {
                        if clip.is_some() {alist.push_back(clip.unwrap())}
                    }
                }
                count = alist.len();
            }

            // DRAW
            for tri in &alist {
                tri.draw_fast(&mut window, true);
                // let points = tri.get_2d_points();
                // window.draw_triangle(C_BLACK, points[0], points[1], points[2]).unwrap(); // wireframe
            }

        }



        if timer.elapsed().as_secs() > 1 {
            window.set_title(&format!("NAIVE WINDOW. FPS: {}, E = {}", fps, elapsed_time));
            timer = std::time::Instant::now();
            fps = 0;
        }
        window.present();
        fps += 1;
    }

}

fn main() {
    create();
}
