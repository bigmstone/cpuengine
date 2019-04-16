use std::error;
use std::fs;

use cgmath::{InnerSpace, Vector3};
use image::{open, DynamicImage, FilterType, GenericImageView};
use log::debug;

use crate::geometry::common::minmax;
use crate::geometry::Triangle;
use crate::render::common as render_common;
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
            texture: open("./african_head_diffuse.tga").unwrap().flipv(),
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

    fn get_texture(
        &self,
        vertices: &[Vector3<f64>],
        texture_vertices: &[Vector3<f64>],
    ) -> Vec<[u8; 3]> {
        let (min, max) = minmax(vertices);
        let (texture_min, texture_max) = minmax(texture_vertices);
        let width = self.texture.width();
        let height = self.texture.height();

        let texture = self
            .texture
            .clone()
            .crop(
                (f64::from(width) * texture_min.x) as u32,
                (f64::from(height) * texture_min.y) as u32,
                ((f64::from(width) * texture_max.x) - (f64::from(width) * texture_min.x)) as u32,
                ((f64::from(height) * texture_max.y) - (f64::from(height) * texture_min.y)) as u32,
            )
            .resize_exact(
                (1024. * max.x - 1024. * min.x) as u32,
                (1024. * max.y - 1024. * min.y) as u32,
                FilterType::Nearest,
            );

        debug!("Texture Width, Height: {}, {}", width, height);
        debug!(
            "Texture Crop X, Y: {}, {}",
            f64::from(width) * texture_min.x,
            f64::from(height) * texture_min.y
        );
        debug!(
            "Texture Crop Width, Height: {}, {}",
            ((f64::from(width) * texture_max.x) - (f64::from(width) * texture_min.x)),
            ((f64::from(height) * texture_max.y) - (f64::from(height) * texture_min.y))
        );
        debug!(
            "Resize Width, Height: {}, {}",
            (1000. * max.x - 1000. * min.x) as u32,
            (1000. * max.y - 1000. * min.y) as u32,
        );
        debug!("Face Min, Max: {:#?}, {:#?}", min, max);

        let color = texture.as_rgb8().unwrap().clone().into_vec();

        //         if color.is_empty() {
        //             return vec![[0, 0, 0]];
        //         }

        debug!("Color: {:#?}", color);

        self.structure_texture(&color)
    }

    fn structure_texture(&self, texture: &[u8]) -> Vec<[u8; 3]> {
        let mut result: Vec<[u8; 3]> = Vec::new();
        for index in 0..texture.len() / 3 {
            result.push([
                texture[index * 3],
                texture[index * 3 + 1],
                texture[index * 3 + 2],
            ]);
        }
        result
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
            let color =
                render_common::color(self.get_texture(&vertices, &texture_vertices), intensity);
            // let color = self.get_texture(&vertices, &texture_vertices);

            for vertex in &mut vertices {
                vertex.x = (vertex.x + 1.) * f64::from(width) / 2.0;
                vertex.y = (vertex.y + 1.) * f64::from(height) / 2.0;
            }

            Triangle::new(vertices[0], vertices[1], vertices[2], color)?.render(renderer)?;
        }
        Ok(true)
    }
}
