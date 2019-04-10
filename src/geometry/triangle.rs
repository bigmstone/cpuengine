use std::error;

use cgmath::Vector3;

use crate::geometry::Line;
use crate::render::png;

fn minmax(vectors: &[Vector3<f64>]) -> (f64, f64, f64, f64) {
    let mut min_x = vectors[0].x;
    let mut max_x = vectors[0].y;
    let mut min_y = vectors[0].x;
    let mut max_y = vectors[0].y;

    for vector in vectors {
        if vector.x < min_x {
            min_x = vector.x
        }
        if vector.x > max_x {
            max_x = vector.x
        }
        if vector.y < min_y {
            min_y = vector.y
        }
        if vector.y > max_y {
            max_y = vector.y
        }
    }

    (min_x, max_x, min_y, max_y)
}

pub struct Triangle {
    a: Vector3<f64>,
    b: Vector3<f64>,
    c: Vector3<f64>,
    color: [u8; 3],
}

impl Triangle {
    pub fn new(
        a: Vector3<f64>,
        b: Vector3<f64>,
        c: Vector3<f64>,
        color: [u8; 3],
    ) -> Result<Triangle, Box<error::Error>> {
        let triangle = Triangle { a, b, c, color };
        Ok(triangle)
    }

    fn build_lines(&self) -> (Line, Line, Line) {
        let vertices = self.sort_vertices();
        (
            Line::new(vertices[0], vertices[1], self.color).unwrap(),
            Line::new(vertices[1], vertices[2], self.color).unwrap(),
            Line::new(vertices[2], vertices[0], self.color).unwrap(),
        )
    }

    pub fn sort_vertices(&self) -> Vec<Vector3<f64>> {
        let mut vertices: Vec<Vector3<f64>> = vec![self.a, self.b, self.c];
        vertices.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
        vertices
    }

    pub fn render(&self, image: &mut Vec<Vec<[u8; 3]>>) -> Result<bool, Box<error::Error>> {
        let (line0, line1, line2) = self.build_lines();
        line0.render(image);
        line1.render(image);
        line2.render(image);
        Ok(true)
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

        if u.y.abs() < 1. {
            return Vector3::new(-1., 1., 1.);
        }

        Vector3::new(1. - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
    }

    pub fn fill(&self, image: &mut Vec<Vec<[u8; 3]>>) -> Result<bool, Box<error::Error>> {
        let vertices = self.sort_vertices();

        let (min_x, max_x, min_y, max_y) = self::minmax(&vertices);

        for x in min_x as i32 - 1..=max_x as i32 + 1 {
            for y in min_y as i32 - 1..=max_y as i32 + 1 {
                let vertex = Vector3::new(f64::from(x), f64::from(y), 0.);
                let barycenter = self.barycentric(vertex);
                if barycenter.x < 0. || barycenter.y < 0. || barycenter.z < 0. {
                    continue;
                }

                png::write_pixel(image, x as u32, y as u32, self.color);
            }
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minmax() {
        let vectors: Vec<Vector3<f64>> = vec![
            Vector3::new(30., 17., 0.),
            Vector3::new(20., 42., 0.),
            Vector3::new(50., 93., 0.),
            Vector3::new(8., 6., 0.),
            Vector3::new(10., 15., 0.),
        ];

        let (min_x, max_x, min_y, max_y) = self::minmax(&vectors);

        assert_eq!(min_x.abs() as i32, 8);
        assert_eq!(max_x.abs() as i32, 50);
        assert_eq!(min_y.abs() as i32, 6);
        assert_eq!(max_y.abs() as i32, 93);
    }
}
