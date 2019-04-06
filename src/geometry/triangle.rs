use std::error;

use cgmath::Vector3;

use crate::geometry::Line;

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

    pub fn render(&self, image: &mut Vec<Vec<[u8; 3]>>) -> Result<bool, Box<error::Error>> {
        let line1 = Line::new(self.a, self.b, self.color).unwrap();
        let line2 = Line::new(self.b, self.c, self.color).unwrap();
        let line3 = Line::new(self.c, self.a, self.color).unwrap();
        line1.render(image);
        line2.render(image);
        line3.render(image);
        Ok(true)
    }

    pub fn fill(&self, image: &mut Vec<Vec<[u8; 3]>>) -> Result<bool, Box<error::Error>> {
        let mut vectors = vec![self.a, self.b, self.c];
        println!("{:#?}", vectors);
        vectors.sort_by(|a, b| a.y.cmp(&b.y));
        println!("{:#?}", vectors);
        Ok(true)
    }
}
