use std::fs::{File, self};
use std::io::{self, Write};
use std::path;

use crate::canvas::{Canvas, Pixel};

pub struct BmpCanvas {
    width: u32,
    height: u32,
    scanline_size: u32,
    pixels: Box<[u8]>,
}

impl BmpCanvas {
    pub fn new(width: u32, height: u32) -> BmpCanvas {
        // We pre-allocate the padding, this makes it easy to save the image to a bmp file later
        let mut scanline_size = width * 3;
        let padding_size = scanline_size % 4;
        scanline_size += padding_size;

        let pixels = vec![0; scanline_size as usize * height as usize].into_boxed_slice();
        BmpCanvas {width, height, pixels, scanline_size}
    }

    pub fn save_image(&self, filename: &str) -> Result<(), io::Error>{
        // Create the directory if needed
        let path = path::Path::new(filename);
        if let Some(dir_path) = path.parent() {
            fs::create_dir_all(dir_path)?;
        }
        let mut output_file = File::create(filename)?;
    
        output_file.write_all(&create_bmp_header(self))?;
        output_file.write_all(&self.pixels)?;
    
        output_file.sync_all()?;
    
        Ok(())
    }
}

impl Canvas for BmpCanvas {
    fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        let idx = (((self.height - 1 - y) * self.scanline_size) + x * 3) as usize;

        self.pixels[idx] = pixel.b;
        self.pixels[idx + 1] = pixel.g;
        self.pixels[idx + 2] = pixel.r;
    }

    fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let idx = ((y * self.scanline_size) + x * 3) as usize;

        Pixel {
            b: self.pixels[idx], 
            g: self.pixels[idx + 1], 
            r: self.pixels[idx + 2]
        }
    }
    
    fn height(&self) -> u32 {
        self.height
    }
    
    fn width(&self) -> u32 {
        self.width
    }
}


fn create_bmp_header(bmp_canvas: &BmpCanvas) -> [u8; 54] {
    let bmp_signature: u16 = 0x4d42;
    let bmp_file_size: u32 = bmp_canvas.scanline_size * bmp_canvas.height + 54;
    let bmp_reserved: u32 = 0;
    let bmp_data_offset: u32 = 54;

    let info_header_size: u32 = 40;
    let info_header_width: u32 = bmp_canvas.width;
    let info_header_height: u32 = bmp_canvas.height;
    let info_header_planes: u16 = 1;
    let info_header_bits_per_pixel: u16 = 24;
    let info_header_compression: u32 = 0;
    let info_header_image_size: u32 = 0;
    let info_header_x_pixels_per_m: u32 = 0;
    let info_header_y_pixels_per_m: u32 = 0;
    let info_header_colors_used: u32 = 0;
    let info_header_important_colors: u32 = 0;

    let mut bmp_header = [0u8; 54];

    bmp_header[0..2].copy_from_slice(&bmp_signature.to_le_bytes());
    bmp_header[2..6].copy_from_slice(&bmp_file_size.to_le_bytes());
    bmp_header[6..10].copy_from_slice(&bmp_reserved.to_le_bytes());
    bmp_header[10..14].copy_from_slice(&bmp_data_offset.to_le_bytes());
    bmp_header[14..18].copy_from_slice(&info_header_size.to_le_bytes());
    bmp_header[18..22].copy_from_slice(&info_header_width.to_le_bytes());
    bmp_header[22..26].copy_from_slice(&info_header_height.to_le_bytes());
    bmp_header[26..28].copy_from_slice(&info_header_planes.to_le_bytes());
    bmp_header[28..30].copy_from_slice(&info_header_bits_per_pixel.to_le_bytes());
    bmp_header[30..34].copy_from_slice(&info_header_compression.to_le_bytes());
    bmp_header[34..38].copy_from_slice(&info_header_image_size.to_le_bytes());
    bmp_header[38..42].copy_from_slice(&info_header_x_pixels_per_m.to_le_bytes());
    bmp_header[42..46].copy_from_slice(&info_header_y_pixels_per_m.to_le_bytes());
    bmp_header[46..50].copy_from_slice(&info_header_colors_used.to_le_bytes());
    bmp_header[50..54].copy_from_slice(&info_header_important_colors.to_le_bytes());

    bmp_header
}