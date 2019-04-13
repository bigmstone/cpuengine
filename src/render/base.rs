use cgmath::Vector3;

pub trait Renderer {
    fn new(width: u32, height: u32) -> Self;
    fn get_size(&self) -> (u32, u32);
    fn set_pixel(&mut self, pixel: Vector3<f64>, color: [u8; 3]);
    fn render(&mut self);
}
