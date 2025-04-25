use std::io;

use bmp::BmpCanvas;
use hittable::HittableList;
use ray::Point3;
use raytracer::RayTracer;
use sphere::Sphere;
use vec3::Vec3;

mod bmp;
mod canvas;
mod vec3;
mod ray;
mod raytracer;
mod sphere;
mod hittable;

fn main() -> Result<(), io::Error>{
    let height: u32 = 510;
    let width: u32 = 800;

    let mut bmp_canvas = BmpCanvas::new(width, height);
    let mut world = HittableList::default();

    let mut sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Vec3::new(1.0, 1.0,0.0));

    world.add(sphere);

    sphere = Sphere::new(Point3::new(1.35, 1.0, -2.0), 0.75, Vec3::new(0.5, 0.22, 0.5));
    world.add(sphere);


    let raytracer = RayTracer::new(&bmp_canvas, 2.0, 1.0);

    raytracer.draw(&mut bmp_canvas, &world);

    bmp_canvas.save_image("examples/test.bmp")?;
    
    Ok(())
}
