use std::io;

use bmp::BmpCanvas;
use material::{Material, MaterialType};
use raytracer::RayTracer;
use triangle_mesh::TriangleMesh;
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
    let mut mesh = TriangleMesh::new(Material {
        material_type: MaterialType::Lambertian,
        albedo: Vec3::new(1.0, 0.5, 0.0),
    });

    mesh.add_vertex(Vec3::new(-0.25, 0.25, -1.0));
    mesh.add_vertex(Vec3::new(0.75, 0.25, -1.0));
    mesh.add_vertex(Vec3::new(0.5, 0.5, -1.0));

    mesh.add_triangle(2, 1, 0);

    let raytracer = RayTracer::new(&bmp_canvas, 2.0, 1.0);

    raytracer.draw(&mut bmp_canvas, &mesh, 100);

    bmp_canvas.save_image("examples/test.bmp")?;

    Ok(())
}
