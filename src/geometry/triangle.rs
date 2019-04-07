use std::error;

use cgmath::Vector3;

use crate::geometry::Line;

fn minmax(vectors: &Vec<Vector3<u32>>) -> (u32, u32, u32, u32) {
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
    a: Vector3<u32>,
    b: Vector3<u32>,
    c: Vector3<u32>,
    color: [u8; 3],
}

impl Triangle {
    pub fn new(
        a: Vector3<u32>,
        b: Vector3<u32>,
        c: Vector3<u32>,
        color: [u8; 3],
    ) -> Result<Triangle, Box<error::Error>> {
        let triangle = Triangle {
            a: a,
            b: b,
            c: c,
            color: color,
        };
        Ok(triangle)
    }

    fn build_lines(&self) -> (Line, Line, Line) {
        let line0 = Line::new(self.a, self.b, self.color).unwrap();
        let line1 = Line::new(self.b, self.c, self.color).unwrap();
        let line2 = Line::new(self.c, self.a, self.color).unwrap();

        (line0, line1, line2)
    }

    pub fn render(&self, image: &mut Vec<Vec<[u8; 3]>>) -> Result<bool, Box<error::Error>> {
        let (line0, line1, line2) = self.build_lines();
        line0.render(image);
        line1.render(image);
        line2.render(image);
        Ok(true)
    }

    pub fn fill(&self, image: &mut Vec<Vec<[u8; 3]>>) -> Result<bool, Box<error::Error>> {
        let mut vectors = vec![self.a, self.b, self.c];
        let (line0, line1, line2) = self.build_lines();
        let (_, _, min_y, max_y) = minmax(&vectors);
        vectors.sort_by(|a, b| a.y.cmp(&b.y));
        for index in 0..(max_y - min_y) {
            let intersects = vec![line0.intersect(&line1)?, line0.intersect(&line2)?];
            let (_, _, min_y, max_y) = minmax(&intersects);

            let vertex0: Vector3<u32> = Vector3::new(line0.vertex0.x + index, min_y, 0);
            let vertex1: Vector3<u32> = Vector3::new(line0.vertex0.x + index, max_y, 0);

            println!("Vertices: {:#?} {:#?}", vertex0, vertex1);

            let line = Line::new(vertex0, vertex1, self.color)?;
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
        let vectors: Vec<Vector3<u32>> = vec![
            Vector3::new(30, 17, 0),
            Vector3::new(20, 42, 0),
            Vector3::new(50, 93, 0),
            Vector3::new(8, 6, 0),
            Vector3::new(10, 15, 0),
        ];

        let (min_x, max_x, min_y, max_y) = self::minmax(&vectors);

        assert_eq!(min_x, 8);
        assert_eq!(max_x, 50);
        assert_eq!(min_y, 6);
        assert_eq!(max_y, 93);
    }
}
