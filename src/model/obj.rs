use std::error;
use std::fs;

use cgmath::Vector3;
use log::debug;
use regex::Regex;

use crate::geometry::Line;

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

        Ok(Object {
            faces: faces,
            vertices: vertices,
        })
    }

    fn parse_faces(file_contents: &String) -> Result<Vec<[[u32; 3]; 3]>, Box<error::Error>> {
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

    fn parse_vertices(file_contents: &String) -> Result<Vec<Vector3<f64>>, Box<error::Error>> {
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

    pub fn render(&self, image: &mut Vec<Vec<[u8; 3]>>, width: u32, height: u32) {
        let height = height as f64 - 1.0;
        let width = width as f64 - 1.0;
        for face in &self.faces {
            for index in 0..3 {
                let v0 = self.vertices[(face[index][0] - 1) as usize];
                let v1 = self.vertices[(face[(index + 1) % 3][0] - 1) as usize];
                // let x0 = width / 2.0 + (v0.x * width / 2.0);
                // let y0 = height / 2.0 + (v0.y * height / 2.0);
                // let x1 = width / 2.0 + (v1.x * width / 2.0);
                // let y1 = height / 2.0 + (v1.y * height / 2.0);
                let x0 = (v0.x as f64 + 1.0) * width as f64 / 2.0;
                let y0 = (v0.y as f64 + 1.0) * height as f64 / 2.0;
                let x1 = (v1.x as f64 + 1.0) * width as f64 / 2.0;
                let y1 = (v1.y as f64 + 1.0) * height as f64 / 2.0;
                let vertex0 = Vector3::new(x0 as u32, y0 as u32, 0 as u32);
                let vertex1 = Vector3::new(x1 as u32, y1 as u32, 0 as u32);
                let line = Line::new(vertex0, vertex1, [255, 255, 255]).unwrap();
                line.render(image);
            }
        }
    }
}
