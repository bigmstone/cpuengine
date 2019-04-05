mod geometry;
mod model;
mod render;

use log::debug;

use cgmath::Vector3;

use geometry::{Line, Triangle};
use model::obj;
use render::png;

fn init() -> (Vec<Vec<[u8; 3]>>, u32, u32) {
    debug!("Starting render");
    let width = 2000;
    let height = 2000;
    let mut data: Vec<Vec<[u8; 3]>> = Vec::new();
    for _ in 0..height {
        let mut row: Vec<[u8; 3]> = Vec::new();
        for _ in 0..(width) {
            row.push([0, 0, 0]);
        }
        data.push(row);
    }
    (data, width, height)
}

pub fn render_obj(args: &Vec<String>) {
    let (mut data, width, height) = init();
    let object = obj::Object::new(args[2].clone()).unwrap();
    object.render(&mut data, width, height);
    png::write_image(&mut data, width, height);
}

pub fn render_triangle() {
    let (mut data, width, height) = init();
    let triangle = Triangle::new().unwrap();
    triangle.render(&mut data, width, height);
    png::write_image(&mut data, width, height);
}

pub fn render_line(args: &Vec<String>) {
    let (mut data, width, height) = init();
    let vertex0 = Vector3::new(
        args[2].parse::<u32>().unwrap(),
        args[3].parse::<u32>().unwrap(),
        args[4].parse::<u32>().unwrap(),
    );
    let vertex1 = Vector3::new(
        args[5].parse::<u32>().unwrap(),
        args[6].parse::<u32>().unwrap(),
        args[7].parse::<u32>().unwrap(),
    );
    let line = Line::new(vertex0, vertex1).unwrap();
    line.render(&mut data);
    png::write_image(&mut data, width, height);
}
