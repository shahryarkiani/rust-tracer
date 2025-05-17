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
        albedo: Vec3::new(1.0, 0.8, 0.0),
    });

    mesh.add_vertex(Vec3::new(0., 0.25, -1.5));
    mesh.add_vertex(Vec3::new(0.75, 0., -1.0));
    mesh.add_vertex(Vec3::new(-0.75, 0., -1.0));

    mesh.add_triangle(0, 1, 2);

    scene.add_mesh(mesh);

    let mut mirror = TriangleMesh::new(Material {
        material_type: MaterialType::Metal,
        albedo: Vec3::new(0.9, 0.8, 0.85),
    });

    mirror.add_vertex(Vec3::new(-0.5, -0.5, -1.)); // close left 0 
    mirror.add_vertex(Vec3::new(0.5, -0.5, -1.)); // close right 1
    mirror.add_vertex(Vec3::new(-0.5, -0.5, -2.)); // far left 2 
    mirror.add_vertex(Vec3::new(0.5, -0.5, -2.)); // far right 3

    mirror.add_vertex(Vec3::new(0.5, 1.5, -2.)); // top right 4
    mirror.add_vertex(Vec3::new(-0.5, 1.5, -2.)); // top left 5

    mirror.add_triangle(2, 3, 0);
    mirror.add_triangle(0, 3, 1);
    mirror.add_triangle(5, 4, 2);
    mirror.add_triangle(4, 3, 2);

    scene.add_mesh(mirror);

    let raytracer = RayTracer::new(&bmp_canvas, 2.0, 1.0);

    raytracer.draw(&mut bmp_canvas, &scene, 100);

    bmp_canvas.save_image("examples/test.bmp")?;

    Ok(())
}
