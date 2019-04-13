use std::error;
use std::fmt;

use cgmath::Vector3;

use crate::render::Renderer;

pub struct Line {
    pub vertex0: Vector3<f64>,
    pub vertex1: Vector3<f64>,
    pub slope: f64,
    pub y_intercept: f64,
    color: [u8; 3],
    vertices_sorted_x: Vec<Vector3<f64>>,
    vertices_sorted_y: Vec<Vector3<f64>>,
}

impl Line {
    pub fn new(
        vertex0: Vector3<f64>,
        vertex1: Vector3<f64>,
        color: [u8; 3],
    ) -> Result<Line, Box<std::error::Error>> {
        let mut vertices_sorted_x: Vec<Vector3<f64>> = vec![vertex0, vertex1];
        vertices_sorted_x.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        let mut vertices_sorted_y: Vec<Vector3<f64>> = vec![vertex0, vertex1];
        vertices_sorted_y.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
        let line = Line {
            vertex0,
            vertex1,
            slope: Line::slope(vertex0, vertex1),
            y_intercept: Line::y_intercept(vertex0, vertex1),
            vertices_sorted_x,
            vertices_sorted_y,
            color,
        };
        Ok(line)
    }

    pub fn render(&self, renderer: &mut impl Renderer) {
        let steps: f64 = self.vertex0.x - self.vertex1.x;
        for index in 0..steps.abs() as u32 {
            let (x, y) = if self.vertex0.x < self.vertex1.x {
                (
                    self.vertex0.x + f64::from(index),
                    self.vertex0.y + (f64::from(index) * self.slope),
                )
            } else {
                (
                    self.vertex0.x - f64::from(index),
                    self.vertex1.y + ((steps - f64::from(index)) * self.slope),
                )
            };

            let pixel: Vector3<f64> = Vector3::new(x, y, 0.0);

            renderer.set_pixel(pixel, self.color);
        }
    }

    fn slope(vertex0: Vector3<f64>, vertex1: Vector3<f64>) -> f64 {
        (vertex0.y - vertex1.y) / (vertex0.x - vertex1.x)
    }

    fn y_intercept(vertex0: Vector3<f64>, vertex1: Vector3<f64>) -> f64 {
        (vertex0.y - (vertex0.x * Line::slope(vertex0, vertex1)))
    }

    pub fn in_line(&self, vertex: Vector3<f64>) -> bool {
        if vertex.x < self.vertices_sorted_x[0].x || vertex.x > self.vertices_sorted_x[1].x {
            // Check X bounds
            false
        } else {
            !(vertex.y < self.vertices_sorted_y[0].y || vertex.y > self.vertices_sorted_y[1].y)
        }
    }

    pub fn intersect(&self, line: &Line) -> Result<Vector3<f64>, IntersectError> {
        println!("Slope 0: {}", self.slope);
        println!("Slope 1: {}", line.slope);
        println!("Line 0 Y Intercept: {}", self.y_intercept);
        println!("Line 1 Y Intercept: {}", line.y_intercept);

        // Mx+B=y=Mx+B
        // line0.slope * x + line0.y_intercept = line1.slope * x + line1.y_intercept
        // line0.slope * x - line1.slope * x = line1.y_intercept - line0.y_intercept
        // line0.slope-line1.slope*x = line1.y_intercept-line0.y_intercept
        let x = (self.y_intercept - line.y_intercept) / (line.slope - self.slope);
        let y = self.slope * x + self.y_intercept;

        let intersect: Vector3<f64> = Vector3::new(x, y, 0.);
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
        let line0_vertex0: Vector3<f64> = Vector3::new(1., 11., 0.);
        let line0_vertex1: Vector3<f64> = Vector3::new(10., 20., 0.);
        let line0 = self::Line::new(line0_vertex0, line0_vertex1, color).unwrap();

        let line1_vertex0: Vector3<f64> = Vector3::new(2., 18., 0.);
        let line1_vertex1: Vector3<f64> = Vector3::new(11., 9., 0.);
        let line1 = self::Line::new(line1_vertex0, line1_vertex1, color).unwrap();

        let intersect = line0.intersect(&line1).unwrap();

        assert_eq!(intersect.x as i32, 5);
        assert_eq!(intersect.y as i32, 15);

        let intersect = line1.intersect(&line0).unwrap();

        assert_eq!(intersect.x as i32, 5);
        assert_eq!(intersect.y as i32, 15);
    }

    #[test]
    fn steep_line_intersect() {
        let color = [255, 255, 255];
        let line0_vertex0: Vector3<f64> = Vector3::new(0., 0., 0.);
        let line0_vertex1: Vector3<f64> = Vector3::new(2., 200., 0.);
        let line0 = self::Line::new(line0_vertex0, line0_vertex1, color).unwrap();

        let line1_vertex0: Vector3<f64> = Vector3::new(0., 100., 0.);
        let line1_vertex1: Vector3<f64> = Vector3::new(1., 0., 0.);
        let line1 = self::Line::new(line1_vertex0, line1_vertex1, color).unwrap();

        let intersect = line0.intersect(&line1).unwrap();

        assert_eq!(intersect.x as i32, 0);
        assert_eq!(intersect.y as i32, 50);

        let intersect = line1.intersect(&line0).unwrap();

        assert_eq!(intersect.x as i32, 0);
        assert_eq!(intersect.y as i32, 50);
    }

    #[test]
    fn intersect_error() {
        let color = [255, 255, 255];
        let line0_vertex0: Vector3<f64> = Vector3::new(6., 4., 0.);
        let line0_vertex1: Vector3<f64> = Vector3::new(4., 6., 0.);
        let line0 = self::Line::new(line0_vertex0, line0_vertex1, color).unwrap();

        let line1_vertex0: Vector3<f64> = Vector3::new(7., 7., 0.);
        let line1_vertex1: Vector3<f64> = Vector3::new(6., 6., 0.);
        let line1 = self::Line::new(line1_vertex0, line1_vertex1, color).unwrap();

        let intersect = line0.intersect(&line1);
        if intersect.is_ok() {
            panic!("Should not intersect")
        }

        let intersect = line1.intersect(&line0);
        if intersect.is_ok() {
            panic!("Should not intersect")
        }
    }
}
