use std::fs::File;

use cgmath::Vector3;
use image::{png, ColorType};
use log::debug;

use crate::render::Renderer;

pub struct PNG {
    pub width: u32,
    pub height: u32,
    zindex: Vec<f64>,
    image: Vec<Vec<[u8; 3]>>,
}

impl Renderer for PNG {
    fn new(width: u32, height: u32) -> PNG {
        PNG {
            width,
            height,
            zindex: vec![std::f64::NEG_INFINITY; (width * height) as usize],
            image: vec![vec![[0, 0, 0]; width as usize]; height as usize],
        }
    }

    fn set_pixel(&mut self, pixel: Vector3<f64>, color: [u8; 3]) {
        if self.zindex[(pixel.x + pixel.y * f64::from(self.width)) as usize] < pixel.z {
            self.zindex[(pixel.x + pixel.y * f64::from(self.width)) as usize] = pixel.z;
            self.image[pixel.y as usize][pixel.x as usize] = color;
        }
    }

    fn render(&mut self) {
        debug!("Writing image.");
        let flat_data = flatten(&mut self.image);
        let buffer = File::create("foo.png").unwrap();
        let encoder = png::PNGEncoder::new(buffer);
        encoder
            .encode(&flat_data, self.width, self.height, ColorType::RGB(8))
            .expect("Error encoding PNG");
    }

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

fn flatten(data: &mut Vec<Vec<[u8; 3]>>) -> Vec<u8> {
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
