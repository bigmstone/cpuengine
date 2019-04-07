use std::error;
use std::fmt;

use cgmath::Vector3;

pub struct Line {
    pub vertex0: Vector3<u32>,
    pub vertex1: Vector3<u32>,
    pub slope: f64,
    pub y_intercept: u32,
    color: [u8; 3],
    vertices_sorted_x: Vec<Vector3<u32>>,
    vertices_sorted_y: Vec<Vector3<u32>>,
}

impl Line {
    pub fn new(
        vertex0: Vector3<u32>,
        vertex1: Vector3<u32>,
        color: [u8; 3],
    ) -> Result<Line, Box<std::error::Error>> {
        let mut vertices_sorted_x: Vec<Vector3<u32>> = vec![vertex0, vertex1];
        vertices_sorted_x.sort_by(|a, b| a.x.cmp(&b.x));
        let mut vertices_sorted_y: Vec<Vector3<u32>> = vec![vertex0, vertex1];
        vertices_sorted_y.sort_by(|a, b| a.y.cmp(&b.y));
        let line = Line {
            vertex0: vertex0,
            vertex1: vertex1,
            slope: Line::slope(vertex0, vertex1),
            y_intercept: Line::y_intercept(vertex0, vertex1),
            vertices_sorted_x: vertices_sorted_x,
            vertices_sorted_y: vertices_sorted_y,
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

    pub fn in_line(&self, vertex: Vector3<u32>) -> bool {
        let mut result = true;
        //Check X bounds
        if vertex.x < self.vertices_sorted_x[0].x || vertex.x > self.vertices_sorted_x[1].x {
            result = false;
        }

        // Check Y bounds
        if vertex.y < self.vertices_sorted_y[0].y || vertex.y > self.vertices_sorted_y[1].y {
            result = false;
        }

        result
    }

    pub fn intersect(&self, line: &Line) -> Result<Vector3<u32>, IntersectError> {
        println!("Slope 0: {}", self.slope);
        println!("Slope 1: {}", line.slope);
        println!("Line 0 Y Intercept: {}", self.y_intercept);
        println!("Line 1 Y Intercept: {}", line.y_intercept);

        // Mx+B=y=Mx+B
        // line0.slope * x + line0.y_intercept = line1.slope * x + line1.y_intercept
        // line0.slope * x - line1.slope * x = line1.y_intercept - line0.y_intercept
        // line0.slope-line1.slope*x = line1.y_intercept-line0.y_intercept
        let x = (self.y_intercept as f64 - line.y_intercept as f64) / (line.slope - self.slope);
        let y = self.slope * x + self.y_intercept as f64;

        let x = x as u32;
        let y = y as u32;

        let intersect: Vector3<u32> = Vector3::new(x, y, 0);
        println!("Intersect: {:#?}", intersect);
        if !self.in_line(intersect) || !line.in_line(intersect) {
            return Err(IntersectError);
        }

        Ok(intersect)
    }
}

#[derive(Debug, Clone)]
pub struct IntersectError;

impl fmt::Display for IntersectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lines do not intersect")
    }
}

impl error::Error for IntersectError {
    fn description(&self) -> &str {
        "Lines do not intersect."
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_intersect() {
        let color = [255, 255, 255];
        let line0_vertex0: Vector3<u32> = Vector3::new(1, 11, 0);
        let line0_vertex1: Vector3<u32> = Vector3::new(10, 20, 0);
        let line0 = self::Line::new(line0_vertex0, line0_vertex1, color).unwrap();

        let line1_vertex0: Vector3<u32> = Vector3::new(2, 18, 0);
        let line1_vertex1: Vector3<u32> = Vector3::new(11, 9, 0);
        let line1 = self::Line::new(line1_vertex0, line1_vertex1, color).unwrap();

        let intersect = line0.intersect(&line1).unwrap();

        assert_eq!(intersect.x, 5);
        assert_eq!(intersect.y, 15);

        let intersect = line1.intersect(&line0).unwrap();

        assert_eq!(intersect.x, 5);
        assert_eq!(intersect.y, 15);
    }

    #[test]
    fn steep_line_intersect() {
        let color = [255, 255, 255];
        let line0_vertex0: Vector3<u32> = Vector3::new(0, 0, 0);
        let line0_vertex1: Vector3<u32> = Vector3::new(2, 200, 0);
        let line0 = self::Line::new(line0_vertex0, line0_vertex1, color).unwrap();

        let line1_vertex0: Vector3<u32> = Vector3::new(0, 100, 0);
        let line1_vertex1: Vector3<u32> = Vector3::new(1, 0, 0);
        let line1 = self::Line::new(line1_vertex0, line1_vertex1, color).unwrap();

        let intersect = line0.intersect(&line1).unwrap();

        assert_eq!(intersect.x, 0);
        assert_eq!(intersect.y, 50);

        let intersect = line1.intersect(&line0).unwrap();

        assert_eq!(intersect.x, 0);
        assert_eq!(intersect.y, 50);
    }

    #[test]
    fn intersect_error() {
        let color = [255, 255, 255];
        let line0_vertex0: Vector3<u32> = Vector3::new(6, 4, 0);
        let line0_vertex1: Vector3<u32> = Vector3::new(4, 6, 0);
        let line0 = self::Line::new(line0_vertex0, line0_vertex1, color).unwrap();

        let line1_vertex0: Vector3<u32> = Vector3::new(7, 7, 0);
        let line1_vertex1: Vector3<u32> = Vector3::new(6, 6, 0);
        let line1 = self::Line::new(line1_vertex0, line1_vertex1, color).unwrap();

        let intersect = line0.intersect(&line1);
        match intersect {
            Ok(_) => panic!("Should not intersect"),
            Err(err) => {}
        };

        let intersect = line1.intersect(&line0);

        match intersect {
            Ok(_) => panic!("Should not intersect"),
            Err(_) => {}
        };
    }
}
