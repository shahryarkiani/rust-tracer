use crate::{ray::Interval, vec3::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

pub trait Canvas {
    fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel);
    fn get_pixel(&self, x: u32, y: u32) -> Pixel;
    fn height(&self) -> u32;
    fn width(&self) -> u32;
}

pub fn to_pixel(color_vec: Vec3) -> Pixel {
    let intensity = Interval::new(0.0, 0.999);
    let max_val = f32::max(color_vec.x, f32::max(color_vec.y, color_vec.z));

    if max_val > 1.0 {
        let r = (256.0 * intensity.clamp(linear_to_gamma(color_vec.x / max_val))) as u8;
        let g = (256.0 * intensity.clamp(linear_to_gamma(color_vec.y / max_val))) as u8;
        let b = (256.0 * intensity.clamp(linear_to_gamma(color_vec.z / max_val))) as u8;

        Pixel { r, g, b }
    } else {
        let r = (256.0 * intensity.clamp(linear_to_gamma(color_vec.x))) as u8;
        let g = (256.0 * intensity.clamp(linear_to_gamma(color_vec.y))) as u8;
        let b = (256.0 * intensity.clamp(linear_to_gamma(color_vec.z))) as u8;

        Pixel { r, g, b }
    }
}

fn linear_to_gamma(intensity: f32) -> f32 {
    if intensity > 0.0 {
        return f32::sqrt(intensity);
    }

    0.0
}
