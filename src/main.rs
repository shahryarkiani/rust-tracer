use std::io;

use bmp::BmpCanvas;
use canvas::{Canvas, Pixel};
use raytracer::RayTracer;

mod bmp;
mod canvas;
mod vec3;
mod ray;
mod raytracer;
mod sphere;

fn main() -> Result<(), io::Error>{
    let height: u32 = 510;
    let width: u32 = 800;

    let mut bmp_canvas = BmpCanvas::new(width, height);

    let raytracer = RayTracer::new(&bmp_canvas, 2.0, 1.0);

    raytracer.draw(&mut bmp_canvas);

    bmp_canvas.set_pixel(2, 2, Pixel{b:255, g:255, r:255});

    bmp_canvas.save_image("examples/test.bmp")?;
    
    Ok(())
}
