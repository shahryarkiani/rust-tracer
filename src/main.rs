use std::io;

use bmp::BmpCanvas;
use hittable::HittableList;
use material::{Material, MaterialType};
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
mod material;

fn main() -> Result<(), io::Error>{
    let width: u32 = 400;
    let aspect_ratio = 16.0 / 9.0;
    let height: u32 = (width as f64 / aspect_ratio) as u32;

    let mut bmp_canvas = BmpCanvas::new(width, height);
    let mut world = HittableList::default();

    let ground = Material { material_type: MaterialType::Lambertian, albedo: Vec3::new(0.8, 0.8, 0.0)};
    let center = Material { material_type: MaterialType::Lambertian, albedo: Vec3::new(0.1, 0.2, 0.5)};
    let left = Material { material_type: MaterialType::Metal, albedo: Vec3::new(0.8, 0.8, 0.8)};
    let right = Material { material_type: MaterialType::Metal, albedo: Vec3::new(0.8, 0.6, 0.2)};

    let ground_id = world.add_material(ground);
    let center_id = world.add_material(center);
    let left_id = world.add_material(left);
    let right_id = world.add_material(right);

    let mut sphere = Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5);
    world.add_sphere(sphere, center_id);

    sphere = Sphere::new(Point3::new(0., -100.5, -1.), 100.);
    world.add_sphere(sphere, ground_id);

    sphere = Sphere::new(Point3::new(-1., 0., -1.), 0.5);
    world.add_sphere(sphere, left_id);

    sphere = Sphere::new(Point3::new(1., 0., -1.), 0.5);
    world.add_sphere(sphere, right_id);

    let raytracer = RayTracer::new(&bmp_canvas, 2.0, 1.0);

    raytracer.draw(&mut bmp_canvas, &world, 100);

    bmp_canvas.save_image("examples/test.bmp")?;
    
    Ok(())
}
