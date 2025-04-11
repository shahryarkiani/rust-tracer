#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8
}

pub trait Canvas{
    fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel);
    fn get_pixel(&self, x: u32, y:u32) -> Pixel; 
    fn height(&self) -> u32;
    fn width(&self) -> u32;
}



