use std::error;
use std::fs;

use cgmath::{InnerSpace, Vector3};
use image::DynamicImage;
use log::debug;

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
    pub texture: DynamicImage,
}

impl Object {
    pub fn new(path: String, texture: DynamicImage) -> Result<Object, Box<error::Error>> {
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
                "vt" => match line.len() {
                    1 => textures.push(Vector3::new(line[0].parse::<f64>().unwrap(), 0., 0.)),
                    2 => textures.push(Vector3::new(
                        line[0].parse::<f64>().unwrap(),
                        line[1].parse::<f64>().unwrap(),
                        0.,
                    )),
                    3 => textures.push(Vector3::new(
                        line[0].parse::<f64>().unwrap(),
                        line[1].parse::<f64>().unwrap(),
                        line[2].parse::<f64>().unwrap(),
                    )),
                    _ => {}
                },
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
            texture,
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

    pub fn render(&self, renderer: &mut impl Renderer) -> Result<bool, Box<error::Error>> {
        let (width, height) = renderer.get_size();
        for face in &self.faces {
            let mut vertices: Vec<Vector3<f64>> = Vec::new();
            let mut texture_vertices: Vec<Vector3<f64>> = Vec::new();

            for vertex in face {
                vertices.push(self.vertices[(vertex.vertex - 1) as usize]);
                texture_vertices.push(self.textures[(vertex.texture - 1) as usize]);
            }

            let intensity = Object::calc_intensity(&vertices);

            if intensity <= 0. {
                continue;
            }

            for vertex in &mut vertices {
                vertex.x = (vertex.x + 1.) * f64::from(width) / 2.0;
                vertex.y = (vertex.y + 1.) * f64::from(height) / 2.0;
            }

            for (index, vertex) in &mut vertices.iter().enumerate() {
                if vertices.get(index + 2).is_none() {
                    break;
                }

                Triangle::new(
                    *vertex,
                    vertices[index + 1],
                    vertices[index + 2],
                    &self.texture,
                    &texture_vertices,
                    intensity,
                )?
                .render(renderer)?;
            }
        }
        Ok(true)
    }
}
