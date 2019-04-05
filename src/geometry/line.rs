use crate::render::png;

use cgmath::Vector3;

pub struct Line {
    vertex0: Vector3<u32>,
    vertex1: Vector3<u32>,
}

impl Line {
    pub fn new(
        vertex0: Vector3<u32>,
        vertex1: Vector3<u32>,
    ) -> Result<Line, Box<std::error::Error>> {
        let line = Line {
            vertex0: vertex0,
            vertex1: vertex1,
        };
        Ok(line)
    }

    pub fn render(&self, data: &mut Vec<Vec<[u8; 3]>>) {
        let (start_x, start_y, end_x, end_y) = (
            self.vertex0.x,
            self.vertex0.y,
            self.vertex1.x,
            self.vertex1.y,
        );
        let (start_x, start_y, end_x, end_y, steep) =
            if (start_x as i32 - end_x as i32) < (start_y as i32 - end_y as i32) {
                let (start_x, start_y) = (start_y, start_x);
                let (end_x, end_y) = (end_y, end_x);
                (start_x, start_y, end_x, end_y, true)
            } else {
                (start_x, start_y, end_x, end_y, false)
            };

        let (start_x, start_y, end_x, end_y) = if start_x as i32 > end_x as i32 {
            let (start_x, end_x) = (end_x, start_x);
            let (start_y, end_y) = (end_y, start_y);
            (start_x, start_y, end_x, end_y)
        } else {
            (start_x, start_y, end_x, end_y)
        };

        for x in start_x..end_x {
            let t = (x - start_x) as f64 / (end_x - start_x) as f64;
            let y = start_y as f64 * (1.0 - t) + end_y as f64 * t;
            if steep {
                png::set_pixel(y as u32, x, data);
            } else {
                png::set_pixel(x, y as u32, data);
            }
        }
    }
}
