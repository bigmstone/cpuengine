use std::error;

use cgmath::Vector3;

use crate::geometry::line;

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

    fn build_lines(&self) -> (line::Line, line::Line, line::Line) {
        let line0 = line::Line::new(self.a, self.b, self.color).unwrap();
        let line1 = line::Line::new(self.b, self.c, self.color).unwrap();
        let line2 = line::Line::new(self.c, self.a, self.color).unwrap();

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
        // let (min_x, max_x, min_y, max_y) = minmax(&vectors);
        // for index in 0..(max_y - min_y) {}
        // println!("{:#?}", vectors);
        // vectors.sort_by(|a, b| a.y.cmp(&b.y));
        println!("{:#?}", vectors);
        let (line0, line1, line2) = self.build_lines();
        line0.intersect(line1);
        Ok(true)
    }
}
