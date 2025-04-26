use crate::{ray::Interval, vec3::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { b, g, r }
    }
}

pub trait Canvas {
    fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel);
    fn get_pixel(&self, x: u32, y: u32) -> Pixel;
    fn height(&self) -> u32;
    fn width(&self) -> u32;
}

pub fn to_pixel(color_vec: Vec3) -> Pixel {
    let intensity = Interval::new(0.0, 0.999);
    Pixel {
        b: (256.0 * intensity.clamp(color_vec.z)) as u8,
        g: (256.0 * intensity.clamp(color_vec.y)) as u8,
        r: (256.0 * intensity.clamp(color_vec.x)) as u8,
    }
}
