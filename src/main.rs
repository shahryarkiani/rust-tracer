use std::io;

use bmp::BmpCanvas;
use canvas::{Canvas, Pixel};

mod bmp;
mod canvas;
mod vec3;

fn main() -> Result<(), io::Error>{
    let height: u32 = 27 * 6;
    let width: u32 = 33 * 6;

    let mut bmp_canvas = BmpCanvas::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = Pixel {
                b: (x ^ y) as u8,
                g: ((x + y) ^ (x * y)) as u8,
                r: ((x + y) ^ (x / (y + 1))) as u8
            };
            bmp_canvas.set_pixel(x, y, pixel);
        }
    }

    bmp_canvas.set_pixel(2, 2, Pixel{b:255, g:255, r:255});

    bmp_canvas.save_image("examples/test.bmp")?;
    
    Ok(())
}
