mod geometry;
mod model;
mod render;

use log::debug;

use cgmath::Vector3;

use geometry::{Line, Triangle};
use model::obj;
use render::png;

const COLOR: [u8; 3] = [255, 0, 0];

fn init() -> (Vec<Vec<[u8; 3]>>, u32, u32) {
    debug!("Starting render");
    let width = 200;
    let height = 200;
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

pub fn render_obj(args: &[String]) {
    let (mut data, width, height) = init();
    let object = obj::Object::new(args[2].clone()).unwrap();
    object.render(&mut data, width, height);
    png::write_image(&mut data, width, height);
}

pub fn render_triangle_set(_args: &[String]) {
    let (mut data, width, height) = init();

    // Triangle 1
    let (vertex0, vertex1, vertex2) = (
        Vector3::new(10., 70., 0.),
        Vector3::new(50., 160., 0.),
        Vector3::new(70., 80., 0.),
    );

    let triangle = Triangle::new(vertex0, vertex1, vertex2, COLOR).unwrap();
    triangle
        .render(&mut data)
        .expect("Error rendering triangle.");
    // triangle.fill(&mut data).expect("Error filling triangle");

    // Triangle 2
    let (vertex0, vertex1, vertex2) = (
        Vector3::new(180., 50., 0.),
        Vector3::new(150., 1., 0.),
        Vector3::new(70., 180., 0.),
    );

    let triangle = Triangle::new(vertex0, vertex1, vertex2, COLOR).unwrap();
    triangle
        .render(&mut data)
        .expect("Error rendering triangle.");
    // triangle.fill(&mut data).expect("Error filling triangle");

    // Triangle 3
    let (vertex0, vertex1, vertex2) = (
        Vector3::new(180., 150., 0.),
        Vector3::new(120., 160., 0.),
        Vector3::new(130., 180., 0.),
    );

    let triangle = Triangle::new(vertex0, vertex1, vertex2, COLOR).unwrap();
    triangle
        .render(&mut data)
        .expect("Error rendering triangle.");
    // triangle.fill(&mut data).expect("Error filling triangle");
    png::write_image(&mut data, width, height);
}

pub fn render_triangle(args: &[String]) {
    let (mut data, width, height) = init();

    let (vertex0, vertex1, vertex2) = (
        Vector3::new(
            args[2].parse::<f64>().unwrap(),
            args[3].parse::<f64>().unwrap(),
            args[4].parse::<f64>().unwrap(),
        ),
        Vector3::new(
            args[5].parse::<f64>().unwrap(),
            args[6].parse::<f64>().unwrap(),
            args[7].parse::<f64>().unwrap(),
        ),
        Vector3::new(
            args[8].parse::<f64>().unwrap(),
            args[9].parse::<f64>().unwrap(),
            args[10].parse::<f64>().unwrap(),
        ),
    );

    let triangle = Triangle::new(vertex0, vertex1, vertex2, COLOR).unwrap();
    triangle
        .render(&mut data)
        .expect("Error rendering triangle.");
    triangle.fill(&mut data).expect("Error filling triangle");
    png::write_image(&mut data, width, height);
}

pub fn render_line(args: &[String]) {
    let (mut data, width, height) = init();
    let vertex0 = Vector3::new(
        args[2].parse::<f64>().unwrap(),
        args[3].parse::<f64>().unwrap(),
        args[4].parse::<f64>().unwrap(),
    );
    let vertex1 = Vector3::new(
        args[5].parse::<f64>().unwrap(),
        args[6].parse::<f64>().unwrap(),
        args[7].parse::<f64>().unwrap(),
    );
    let line = Line::new(vertex0, vertex1, COLOR).unwrap();
    line.render(&mut data);
    png::write_image(&mut data, width, height);
}
