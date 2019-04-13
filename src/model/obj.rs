use std::error;
use std::fs;

use cgmath::{InnerSpace, Vector3};
use image::open;
use log::debug;

use crate::common::render;
use crate::geometry::Triangle;
use crate::render::Renderer;

pub struct Face {
    pub vertex: u32,
    pub texture: u32,
    pub normal: u32,
}

pub struct Object {
    pub faces: Vec<Vec<Face>>,
    pub vertices: Vec<Vector3<f64>>,
    pub normals: Vec<Vector3<f64>>,
    pub textures: Vec<Vector3<f64>>,
    pub texture: image::RgbImage,
}

impl Object {
    pub fn new(path: String) -> Result<Object, Box<error::Error>> {
        debug!("Loading object: {}", path);
        let file_contents = fs::read_to_string(path)?;

        let mut faces: Vec<Vec<Face>> = Vec::new();
        let mut vertices: Vec<Vector3<f64>> = Vec::new();
        let mut textures: Vec<Vector3<f64>> = Vec::new();
        let mut normals: Vec<Vector3<f64>> = Vec::new();

        for line in file_contents.lines() {
            let mut line: Vec<&str> = line.split_whitespace().collect();

            if line.is_empty() {
                continue;
            }

            let line_type = line.remove(0);

            match line_type {
                "f" => faces.push(Object::parse_face(&line)),
                "vt" => textures.push(Vector3::new(
                    line[0].parse::<f64>().unwrap(),
                    line[1].parse::<f64>().unwrap(),
                    line[2].parse::<f64>().unwrap(),
                )),
                "vn" => normals.push(Vector3::new(
                    line[0].parse::<f64>().unwrap(),
                    line[1].parse::<f64>().unwrap(),
                    line[2].parse::<f64>().unwrap(),
                )),
                "v" => vertices.push(Vector3::new(
                    line[0].parse::<f64>().unwrap(),
                    line[1].parse::<f64>().unwrap(),
                    line[2].parse::<f64>().unwrap(),
                )),
                _ => {}
            }
        }

        Ok(Object {
            faces,
            vertices,
            normals,
            textures,
            texture: open("./african_head_diffuse.tga").unwrap().to_rgb(),
        })
    }

    fn parse_face(line: &[&str]) -> Vec<Face> {
        let mut face: Vec<Face> = Vec::new();
        for reference in line {
            let reference: Vec<&str> = reference.split('/').collect();
            match reference.len() {
                1 => {
                    face.push(Face {
                        vertex: reference[0].parse::<u32>().unwrap(),
                        texture: 0 as u32,
                        normal: 0 as u32,
                    });
                }
                2 => {
                    face.push(Face {
                        vertex: reference[0].parse::<u32>().unwrap(),
                        texture: reference[1].parse::<u32>().unwrap(),
                        normal: 0 as u32,
                    });
                }
                3 => {
                    face.push(Face {
                        vertex: reference[0].parse::<u32>().unwrap(),
                        texture: reference[1].parse::<u32>().unwrap(),
                        normal: reference[2].parse::<u32>().unwrap(),
                    });
                }
                _ => {}
            }
        }
        face
    }

    fn calc_intensity(vertices: &[Vector3<f64>]) -> f64 {
        let n: Vector3<f64> = (vertices[2] - vertices[0]).cross(vertices[1] - vertices[0]);
        let n = n.normalize();

        let light_direction: Vector3<f64> = Vector3::new(0., 0., -1.);

        n.dot(light_direction)
    }

    fn get_texture(&self, face: &[Face]) {
        let mut texture: Vec<Vector3<f64>> = Vec::new();
        for vertex in face {
            texture.push(self.textures[(vertex.texture - 1) as usize]);
        }
        debug!("Textures: {:#?}", texture);
    }

    pub fn render(&self, renderer: &mut impl Renderer) -> Result<bool, Box<error::Error>> {
        let (width, height) = renderer.get_size();
        for face in &self.faces {
            let mut vertices: Vec<Vector3<f64>> = Vec::new();

            for vertex in face {
                vertices.push(self.vertices[(vertex.vertex - 1) as usize]);
            }

            let intensity = Object::calc_intensity(&vertices);

            if intensity <= 0. {
                continue;
            }
            self.get_texture(face);
            let color = render::color([255, 255, 255], intensity);

            for vertex in &mut vertices {
                vertex.x = (vertex.x + 1.) * f64::from(width) / 2.0;
                vertex.y = (vertex.y + 1.) * f64::from(height) / 2.0;
            }

            Triangle::new(vertices[0], vertices[1], vertices[2], color)?.fill(renderer)?;
        }
        Ok(true)
    }
}
