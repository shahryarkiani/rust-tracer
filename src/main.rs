use std::io;

use bmp::BmpCanvas;
use material::{Material, MaterialType};
use raytracer::RayTracer;
use triangle_mesh::{Scene, TriangleMesh};
use vec3::Vec3;

mod bmp;
mod canvas;
mod hittable;
mod material;
mod ray;
mod raytracer;
mod triangle_mesh;
mod vec3;

fn main() -> Result<(), io::Error> {
    let width: u32 = 400;
    let aspect_ratio = 16.0 / 9.0;
    let height: u32 = (width as f64 / aspect_ratio) as u32;

    let mut bmp_canvas = BmpCanvas::new(width, height);
    let mut scene = Scene::default();

    let mut mesh = TriangleMesh::new(Material {
        material_type: MaterialType::Lambertian,
        albedo: Vec3::new(0.3, 0.4, 0.5),
    });

    mesh.add_vertex(Vec3::new(0., -0.8, -3.));
    mesh.add_vertex(Vec3::new(1.5, -0.8, -1.2));
    mesh.add_vertex(Vec3::new(-1.5, 1.5, -1.2));

    mesh.add_triangle(0, 1, 2);

    scene.add_mesh(mesh);

    let mut floor = TriangleMesh::new(Material {
        material_type: MaterialType::Lambertian,
        albedo: Vec3::new(0.7, 0.8, 0.5),
    });

    floor.add_vertex(Vec3::new(-5., -0.8, 5.)); // close left 0 
    floor.add_vertex(Vec3::new(5., -0.8, 5.)); // close right 1
    floor.add_vertex(Vec3::new(-5., -0.8, -15.)); // far left 2 
    floor.add_vertex(Vec3::new(5., -0.8, -15.)); // far right 3

    floor.add_triangle(0, 3, 2);
    floor.add_triangle(1, 3, 0);

    scene.add_mesh(floor);

    let raytracer = RayTracer::new(&bmp_canvas, 2.0, 1.0);

    raytracer.draw(&mut bmp_canvas, &scene, 40);

    bmp_canvas.save_image("examples/test.bmp")?;

    Ok(())
}
