use std::error;
use std::fs;

use cgmath::{InnerSpace, Vector3};
use log::debug;
use regex::Regex;

use crate::common::render;
use crate::geometry::Triangle;
use crate::render::Renderer;

pub struct Object {
    pub faces: Vec<[[u32; 3]; 3]>,
    pub vertices: Vec<Vector3<f64>>,
}

impl Object {
    pub fn new(path: String) -> Result<Object, Box<error::Error>> {
        debug!("Loading object: {}", path);
        let file_contents = fs::read_to_string(path)?;

        debug!("Parsing Faces");
        let faces = Object::parse_faces(&file_contents)?;
        debug!("Parsing vertices");
        let vertices = Object::parse_vertices(&file_contents)?;

        Ok(Object { faces, vertices })
    }

    fn parse_faces(file_contents: &str) -> Result<Vec<[[u32; 3]; 3]>, Box<error::Error>> {
        let mut faces: Vec<[[u32; 3]; 3]> = Vec::new();
        let face_rex =
            Regex::new(r"f\s+(\d+)/(\d+)/(\d+)\s+(\d+)/(\d+)/(\d+)\s+(\d+)/(\d+)/(\d+)")?;
        for line in file_contents.lines() {
            //Extracts obj face line: f 266/1335/266 679/696/679 27/8/27
            let face = face_rex.captures(&line);
            let face = match face {
                None => continue,
                Some(result) => result,
            };

            faces.push([
                [
                    face[1].parse::<u32>()?,
                    face[2].parse::<u32>()?,
                    face[3].parse::<u32>()?,
                ],
                [
                    face[4].parse::<u32>()?,
                    face[5].parse::<u32>()?,
                    face[6].parse::<u32>()?,
                ],
                [
                    face[7].parse::<u32>()?,
                    face[8].parse::<u32>()?,
                    face[9].parse::<u32>()?,
                ],
            ]);
        }

        Ok(faces)
    }

    fn parse_vertices(file_contents: &str) -> Result<Vec<Vector3<f64>>, Box<error::Error>> {
        let mut vertices: Vec<Vector3<f64>> = Vec::new();
        let vertex_rex = Regex::new(r"v\s+([0-9e\.-]+)\s+([0-9e\.-]+)\s+([0-9e\.-]+)")?;
        for line in file_contents.lines() {
            //Extracts vertex line: v -0.000581696 -0.734665 -0.623267
            let vertex = vertex_rex.captures(&line);
            let vertex = match vertex {
                None => {
                    continue;
                }
                Some(result) => result,
            };

            vertices.push(Vector3::new(
                vertex[1].parse::<f64>()?,
                vertex[2].parse::<f64>()?,
                vertex[3].parse::<f64>()?,
            ));
        }

        Ok(vertices)
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

            for vertex in face {
                vertices.push(self.vertices[(vertex[0] - 1) as usize]);
            }

            let intensity = Object::calc_intensity(&vertices);

            if intensity <= 0. {
                continue;
            }

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
