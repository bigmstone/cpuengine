use crate::render::png;

use cgmath::Vector3;

pub struct Line {
    vertex0: Vector3<u32>,
    vertex1: Vector3<u32>,
    color: [u8; 3],
}

impl Line {
    pub fn new(
        vertex0: Vector3<u32>,
        vertex1: Vector3<u32>,
        color: [u8; 3],
    ) -> Result<Line, Box<std::error::Error>> {
        let line = Line {
            vertex0: vertex0,
            vertex1: vertex1,
            color: color,
        };
        Ok(line)
    }

    pub fn render(&self, data: &mut Vec<Vec<[u8; 3]>>) {
        let slope = (self.vertex0.y as i32 - self.vertex1.y as i32) as f64
            / (self.vertex0.x as i32 - self.vertex1.x as i32) as f64;
        let steps: i32 = self.vertex0.x as i32 - self.vertex1.x as i32;
        for index in 0..steps.abs() {
            let x;
            let y;
            if self.vertex0.x < self.vertex1.x {
                x = self.vertex0.x as i32 + index;
                y = self.vertex0.y as f64 + (index as f64 * slope);
            } else {
                x = self.vertex0.x as i32 - index;
                y = self.vertex1.y as f64 + ((steps - index) as f64 * slope);
            }
            png::set_pixel(x as u32, y as u32, data, self.color);
        }
    }
}
