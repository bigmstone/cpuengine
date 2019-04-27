use std::error;

use cgmath::Vector3;
use image::{DynamicImage, GenericImageView};
use log::{debug, warn};

use crate::geometry::common::minmax;
use crate::render::common as render_common;
use crate::render::Renderer;

pub struct Triangle<'a> {
    a: Vector3<f64>,
    b: Vector3<f64>,
    c: Vector3<f64>,
    texture: &'a DynamicImage,
    texture_vertices: &'a [Vector3<f64>],
    intensity: f64,
}

impl<'a> Triangle<'a> {
    pub fn new(
        a: Vector3<f64>,
        b: Vector3<f64>,
        c: Vector3<f64>,
        texture: &'a DynamicImage,
        texture_vertices: &'a [Vector3<f64>],
        intensity: f64,
    ) -> Result<Triangle<'a>, Box<error::Error>> {
        let triangle: Triangle<'a> = Triangle {
            a,
            b,
            c,
            texture,
            texture_vertices,
            intensity,
        };
        Ok(triangle)
    }

    pub fn get_color(&self, barycenter: Vector3<f64>) -> [u8; 3] {
        let mut coords = self.texture_vertices.to_vec();

        for coord in coords.iter_mut() {
            coord.x *= f64::from(self.texture.width());
            coord.y *= f64::from(self.texture.height());
        }

        let coord = coords[0] * barycenter.x + coords[1] * barycenter.y + coords[2] * barycenter.z;

        let x = coord.x.round() as u32;
        let y = coord.y.round() as u32;

        if self.texture.in_bounds(x, y) {
            let pixel = self.texture.get_pixel(x, y);
            debug!("Pixel: {:#?}", pixel);
            debug!("Pixel R, G, B: {}, {}, {}", pixel[0], pixel[1], pixel[2]);
            return [pixel[0], pixel[1], pixel[2]];
        }
        warn!(
            "Requested color outside texture bounds: {}, {}, {}, {}",
            x,
            y,
            self.texture.width(),
            self.texture.height(),
        );
        [255, 255, 255]
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
        let (min, max) = minmax(&[self.a, self.b, self.c]);

        for x in min.x.round() as u32..=max.x.round() as u32 {
            for y in min.y.round() as u32..=max.y.round() as u32 {
                let vertex = Vector3::new(f64::from(x), f64::from(y), 0.);
                let barycenter = self.barycentric(vertex);
                if barycenter.x < 0. || barycenter.y < 0. || barycenter.z < 0. {
                    continue;
                }
                let z = self.a.z * barycenter.x + self.b.z * barycenter.y + self.c.z * barycenter.z;

                let pixel: Vector3<f64> = Vector3::new(f64::from(x), f64::from(y), z);

                let color = render_common::color(self.get_color(barycenter), self.intensity);

                renderer.set_pixel(pixel, color);
            }
        }

        Ok(true)
    }
}
