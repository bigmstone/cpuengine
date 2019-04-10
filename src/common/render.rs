pub fn color(color: [u8; 3], intensity: f64) -> [u8; 3] {
    [
        (intensity * f64::from(color[0])) as u8,
        (intensity * f64::from(color[0])) as u8,
        (intensity * f64::from(color[0])) as u8,
    ]
}
