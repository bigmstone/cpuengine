use std::error;

use cgmath::Vector3;

use crate::geometry::common::minmax;
use crate::render::Renderer;

pub struct Triangle {
    a: Vector3<f64>,
    b: Vector3<f64>,
    c: Vector3<f64>,
    color: Vec<[u8; 3]>,
}

impl Triangle {
    pub fn new(
        a: Vector3<f64>,
        b: Vector3<f64>,
        c: Vector3<f64>,
        color: Vec<[u8; 3]>,
    ) -> Result<Triangle, Box<error::Error>> {
        let triangle = Triangle { a, b, c, color };
        Ok(triangle)
    }

    pub fn sort_vertices(&self) -> Vec<Vector3<f64>> {
        let mut vertices: Vec<Vector3<f64>> = vec![self.a, self.b, self.c];
        vertices.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
        vertices
    }

    fn barycentric(&self, vertex: Vector3<f64>) -> Vector3<f64> {
        let u: Vector3<f64> = Vector3::new(
            self.c.x - self.a.x,
            self.b.x - self.a.x,
            self.a.x - vertex.x,
        )
        .cross(Vector3::new(
            self.c.y - self.a.y,
            self.b.y - self.a.y,
            self.a.y - vertex.y,
        ));

        if u.z.abs() < 1. {
            return Vector3::new(-1., 1., 1.);
        }

        Vector3::new(1. - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
    }

    pub fn render(&self, renderer: &mut impl Renderer) -> Result<bool, Box<error::Error>> {
        let vertices = self.sort_vertices();

        let (min, max) = minmax(&vertices);

        for x in min.x as i32 - 1..=max.x as i32 + 1 {
            for y in min.y as i32 - 1..=max.y as i32 + 1 {
                let vertex = Vector3::new(f64::from(x), f64::from(y), 0.);
                let barycenter = self.barycentric(vertex);
                if barycenter.x < 0. || barycenter.y < 0. || barycenter.z < 0. {
                    continue;
                }
                let z = self.a.z * barycenter.x + self.b.z * barycenter.y + self.c.z * barycenter.z;

                let pixel: Vector3<f64> = Vector3::new(f64::from(x), f64::from(y), z);

                let color = match self.color.get(
                    (((max.x - min.x) as i32 * (x - min.x as i32 - 1)) + (y - min.y as i32 - 1))
                        as usize,
                ) {
                    Some(color) => color,
                    None => &self.color[0],
                };

                renderer.set_pixel(pixel, color.clone());
            }
        }

        Ok(true)
    }
}
