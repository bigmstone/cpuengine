use std::error;

use cgmath::Vector3;

use crate::geometry::Line;

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
            // Line::new(vertices[0], vertices[1], self.color).unwrap(),
            // Line::new(vertices[1], vertices[2], self.color).unwrap(),
            // Line::new(vertices[2], vertices[0], self.color).unwrap(),
            Line::new(vertices[0], vertices[1], [0, 255, 0]).unwrap(),
            Line::new(vertices[1], vertices[2], [0, 255, 0]).unwrap(),
            Line::new(vertices[2], vertices[0], [255, 0, 0]).unwrap(),
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

    pub fn fill(&self, image: &mut Vec<Vec<[u8; 3]>>) -> Result<bool, Box<error::Error>> {
        let vertices = self.sort_vertices();
        let (line0, _line1, line2) = self.build_lines();

        for index in 0..(vertices[2].y - vertices[0].y) as i32 {
            let x0 = vertices[0].x + (line0.slope * f64::from(index));
            let x1 = vertices[2].x - (line2.slope * f64::from(index));
            let line = Line::new(
                Vector3::new(x0, vertices[0].y + f64::from(index), 0.),
                Vector3::new(x1, vertices[0].y + f64::from(index), 0.),
                self.color,
            )?;

            line.render(image);
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
