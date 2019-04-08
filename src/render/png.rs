use std::fs::File;

use image::{png, ColorType};

pub fn flatten(data: &mut Vec<Vec<[u8; 3]>>) -> Vec<u8> {
    let mut flat_data: Vec<u8> = Vec::new();

    data.reverse();

    for row in data {
        for column in row {
            for item in column.iter_mut() {
                flat_data.push(*item);
            }
        }
    }

    flat_data
}

pub fn write_pixel(data: &mut Vec<Vec<[u8; 3]>>, x: u32, y: u32, color: [u8; 3]) {
    data[y as usize][x as usize] = color;
}

pub fn write_image(data: &mut Vec<Vec<[u8; 3]>>, width: u32, height: u32) {
    let flat_data = flatten(data);
    let buffer = File::create("foo.png").unwrap();
    let encoder = png::PNGEncoder::new(buffer);
    encoder
        .encode(&flat_data, width, height, ColorType::RGB(8))
        .expect("Error encoding PNG");
}
