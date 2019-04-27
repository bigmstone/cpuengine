// pub fn color_vec(colors: Vec<[u8; 3]>, intensity: f64) -> Vec<[u8; 3]> {
//     let mut rendered_color: Vec<[u8; 3]> = Vec::new();

//     for color in colors {
//         rendered_color.push([
//             (intensity * f64::from(color[0])) as u8,
//             (intensity * f64::from(color[1])) as u8,
//             (intensity * f64::from(color[2])) as u8,
//         ]);
//     }

//     rendered_color
// }

pub fn color(color: [u8; 3], intensity: f64) -> [u8; 3] {
    [
        (intensity * f64::from(color[0])) as u8,
        (intensity * f64::from(color[1])) as u8,
        (intensity * f64::from(color[2])) as u8,
    ]
}
