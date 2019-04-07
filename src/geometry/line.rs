use crate::render::png;

use cgmath::Vector3;

pub struct Line {
    pub vertex0: Vector3<u32>,
    pub vertex1: Vector3<u32>,
    pub slope: f64,
    pub y_intercept: u32,
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
            slope: Line::slope(vertex0, vertex1),
            y_intercept: Line::y_intercept(vertex0, vertex1),
            color: color,
        };
        Ok(line)
    }

    pub fn render(&self, data: &mut Vec<Vec<[u8; 3]>>) {
        let steps: i32 = self.vertex0.x as i32 - self.vertex1.x as i32;
        for index in 0..steps.abs() {
            let x;
            let y;
            if self.vertex0.x < self.vertex1.x {
                x = self.vertex0.x as i32 + index;
                y = self.vertex0.y as f64 + (index as f64 * self.slope);
            } else {
                x = self.vertex0.x as i32 - index;
                y = self.vertex1.y as f64 + ((steps - index) as f64 * self.slope);
            }

            data[x as usize][y as usize] = self.color;
        }
    }

    fn slope(vertex0: Vector3<u32>, vertex1: Vector3<u32>) -> f64 {
        (vertex0.y as i32 - vertex1.y as i32) as f64 / (vertex0.x as i32 - vertex1.x as i32) as f64
    }

    fn y_intercept(vertex0: Vector3<u32>, vertex1: Vector3<u32>) -> u32 {
        (vertex0.y as f64 - (vertex0.x as f64 * Line::slope(vertex0, vertex1))) as u32
    }

    pub fn intersect(&self, line: Line) {
        println!("Slope 0: {}", self.slope);
        println!("Slope 1: {}", line.slope);
        println!("Line 0 Y Intercept: {}", self.y_intercept);
        println!("Line 1 Y Intercept: {}", line.y_intercept);
    }
}
