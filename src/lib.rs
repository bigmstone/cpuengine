mod common;
mod geometry;
mod model;
mod render;

use log::debug;

use cgmath::Vector3;

use geometry::{Line, Triangle};
use model::obj;
use render::png::PNG;
use render::Renderer;

const COLOR: [u8; 3] = [255, 255, 255];

fn init() -> PNG {
    debug!("Starting render");
    let width = 1000;
    let height = 1000;
    Renderer::new(width, height)
}

pub fn render_obj(args: &[String]) {
    let mut renderer = init();
    let object = obj::Object::new(args[2].clone()).unwrap();
    object
        .render(&mut renderer)
        .expect("Error rendering object.");
    renderer.render();
}

pub fn render_triangle(args: &[String]) {
    let mut renderer = init();

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
        .render(&mut renderer)
        .expect("Error rendering triangle.");
    triangle
        .fill(&mut renderer)
        .expect("Error filling triangle");
    renderer.render();
}

pub fn render_line(args: &[String]) {
    let mut renderer = init();
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
    line.render(&mut renderer);
    renderer.render();
}
