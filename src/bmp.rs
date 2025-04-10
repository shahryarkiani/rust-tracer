use std::fs::{File, self};
use std::io::{self, Write};
use std::path;

pub fn save_image(filename: &str, width: u32, height: u32, data: Vec<u8>) -> Result<(), io::Error>{

    let path = path::Path::new(filename);
    if let Some(dir_path) = path.parent() {
        fs::create_dir_all(dir_path)?;
    }
    let mut output_file = File::create(filename)?;

    output_file.write_all(&create_bmp_header(width, height))?;

    output_file.write_all(&data)?;

    output_file.sync_all()?;

    Ok(())
}

fn create_bmp_header(width: u32, height: u32) -> [u8; 54] {
    let bmp_signature: u16 = 0x4d42;
    let bmp_file_size: u32;
    let bmp_reserved: u32 = 0;
    let bmp_data_offset: u32 = 54;

    let info_header_size: u32 = 40;
    let info_header_width: u32 = width;
    let info_header_height: u32 = height;
    let info_header_planes: u16 = 1;
    let info_header_bits_per_pixel: u16 = 24;
    let info_header_compression: u32 = 0;
    let info_header_image_size: u32 = 0;
    let info_header_x_pixels_per_m: u32 = 0;
    let info_header_y_pixels_per_m: u32 = 0;
    let info_header_colors_used: u32 = 0;
    let info_header_important_colors: u32 = 0;

    // Calculate file size, accounting for padding
    let mut scanline_size: u32 = info_header_width * 3;
    scanline_size += scanline_size % 4;
    bmp_file_size = scanline_size * info_header_height;

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