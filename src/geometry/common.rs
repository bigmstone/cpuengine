use cgmath::Vector3;

pub fn minmax(vectors: &[Vector3<f64>]) -> (Vector3<f64>, Vector3<f64>) {
    let mut max: Vector3<f64> = Vector3::new(vectors[0].x, vectors[0].y, vectors[0].z);
    let mut min: Vector3<f64> = Vector3::new(vectors[0].x, vectors[0].y, vectors[0].z);

    for vector in vectors {
        if vector.x < min.x {
            min.x = vector.x
        }
        if vector.x > max.x {
            max.x = vector.x
        }
        if vector.y < min.y {
            min.y = vector.y
        }
        if vector.y > max.y {
            max.y = vector.y
        }
        if vector.z < min.z {
            min.z = vector.z
        }
        if vector.z > max.z {
            max.z = vector.z
        }
    }

    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minmax() {
        let vectors: Vec<Vector3<f64>> = vec![
            Vector3::new(30., 17., 44.),
            Vector3::new(20., 42., 22.),
            Vector3::new(50., 93., 7.),
            Vector3::new(8., 6., 11.),
            Vector3::new(10., 15., 64.),
        ];

        let (min, max) = self::minmax(&vectors);

        assert_eq!(min.x.abs() as i32, 8);
        assert_eq!(max.x.abs() as i32, 50);
        assert_eq!(min.y.abs() as i32, 6);
        assert_eq!(max.y.abs() as i32, 93);
        assert_eq!(min.z.abs() as i32, 7);
        assert_eq!(max.z.abs() as i32, 64);
    }
}
