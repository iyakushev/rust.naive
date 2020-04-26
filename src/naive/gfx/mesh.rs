use super::triangle::Triangle;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::naive::gfx::Vec3D;
use sdl2::pixels::Color;

pub struct Mesh {
    pub tris: Vec<Triangle>
}

impl Mesh {
    pub fn load_object(file_name: &str) -> Self {
        let file = File::open(file_name).unwrap();
        let mut vert: Vec<Vec3D> = Vec::new();
        let mut tris: Vec<Triangle> = Vec::new();

        for l in BufReader::new(file).lines() {
            let line = l.unwrap();
            match line.get(0..1) {
                Some("v") => {
                    let coords: Vec<f32> = line.get(2..).unwrap().split(' ').map(|x| x.parse::<f32>().unwrap()).collect();
                    vert.push(Vec3D::new(coords[0], coords[1], coords[2]));
                },
                Some("f") => {
                    let idxs: Vec<usize> =  line.get(2..).unwrap().split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
                    tris.push(Triangle::new(vert[idxs[0]-1], vert[idxs[1]-1], vert[idxs[2]-1], None));
                },
                _ => ()
            }
        }

        Mesh {tris}
    }
}