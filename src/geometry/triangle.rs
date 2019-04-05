use std::error;

use cgmath::Vector3;

pub struct Triangle {
    a: Vector3<u32>,
    b: Vector3<u32>,
    c: Vector3<u32>,
}

impl Triangle {
    pub fn new() -> Result<Triangle, Box<error::Error>> {
        let triangle = Triangle {
            a: Vector3::new(0, 0, 0),
            b: Vector3::new(0, 0, 0),
            c: Vector3::new(0, 0, 0),
        };
        Ok(triangle)
    }

    pub fn render(
        &self,
        image: &mut Vec<Vec<[u8; 3]>>,
        width: u32,
        height: u32,
    ) -> Result<bool, Box<error::Error>> {
        Ok(true)
    }
}
