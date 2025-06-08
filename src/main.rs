#![feature(portable_simd)]

use std::{io, time::SystemTime};

use bmp::BmpCanvas;
use material::{Material, MaterialType};
use raytracer::RayTracer;
use triangle_mesh::{Scene, TriangleMesh};
use vec3::Vec3;

mod bbox;
mod bmp;
mod bvh;
mod canvas;
mod hittable;
mod material;
mod ray;
mod raytracer;
mod triangle_mesh;
mod vec3;

fn main() -> Result<(), io::Error> {
    let width: u32 = 2560;
    let aspect_ratio = 16.0 / 9.0;
    let height: u32 = (width as f32 / aspect_ratio) as u32;

    let mut bmp_canvas = BmpCanvas::new(width, height);
    let mut scene = Scene::default();

    let mut mesh = TriangleMesh::new(Material {
        material_type: MaterialType::Lambertian,
        albedo: Vec3::new(0.3, 0.4, 0.5),
    });

    mesh.add_vertex(Vec3::new(0.1, -0.5, -0.6));
    mesh.add_vertex(Vec3::new(0.5, -0.5, -0.6));
    mesh.add_vertex(Vec3::new(0.4, -0.5, -1.));
    mesh.add_vertex(Vec3::new(0., -0.5, -1.));

    mesh.add_vertex(Vec3::new(0.1, 0.1, -0.6));
    mesh.add_vertex(Vec3::new(0.5, 0.1, -0.6));
    mesh.add_vertex(Vec3::new(0.4, 0.3, -1.));
    mesh.add_vertex(Vec3::new(0., 0.3, -1.));

    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    mesh.add_triangle(1, 6, 5);
    mesh.add_triangle(1, 2, 6);
    mesh.add_triangle(0, 1, 5);
    mesh.add_triangle(0, 5, 4);
    mesh.add_triangle(2, 3, 6);
    mesh.add_triangle(3, 7, 6);
    mesh.add_triangle(5, 7, 4);
    mesh.add_triangle(5, 6, 7);

    scene.add_mesh(mesh);

    let mut mirror = TriangleMesh::new(Material {
        material_type: MaterialType::Metal,
        albedo: Vec3::new(0.9, 0.8, 0.85),
    });

    mirror.add_vertex(Vec3::new(-1.5, -0.5, -1.5));
    mirror.add_vertex(Vec3::new(-1., -0.5, -1.));
    mirror.add_vertex(Vec3::new(-0.5, -0.5, -1.5));
    mirror.add_vertex(Vec3::new(-1., -0.5, -2.));

    mirror.add_vertex(Vec3::new(-1.5, 1., -1.5));
    mirror.add_vertex(Vec3::new(-1., 1., -1.));
    mirror.add_vertex(Vec3::new(-0.5, 1., -1.5));
    mirror.add_vertex(Vec3::new(-1., 1., -2.));

    mirror.add_triangle(0, 1, 2);
    mirror.add_triangle(0, 2, 3);
    mirror.add_triangle(1, 6, 5);
    mirror.add_triangle(1, 2, 6);

    scene.add_mesh(mirror);

    let mut floor = TriangleMesh::new(Material {
        material_type: MaterialType::Lambertian,
        albedo: Vec3::new(0.7, 0.8, 0.5),
    });

    floor.add_vertex(Vec3::new(-555., -0.51, 5.)); // close left 0 
    floor.add_vertex(Vec3::new(555., -0.51, 5.)); // close right 1
    floor.add_vertex(Vec3::new(-555., -0.51, -155.)); // far left 2 
    floor.add_vertex(Vec3::new(555., -0.51, -155.)); // far right 3

    floor.add_triangle(0, 3, 2);
    floor.add_triangle(1, 3, 0);

    scene.add_mesh(floor);

    let mut tinybox = TriangleMesh::new(Material {
        material_type: MaterialType::Emissive,
        albedo: Vec3::new(4.0, 1.0, 1.0),
    });

    tinybox.add_vertex(Vec3::new(0.3, -0.5, -0.65));
    tinybox.add_vertex(Vec3::new(0.4, -0.5, -0.65));
    tinybox.add_vertex(Vec3::new(0.4, -0.5, -0.7));
    tinybox.add_vertex(Vec3::new(0.3, -0.5, -0.7));

    tinybox.add_vertex(Vec3::new(0.3, -0.4, -0.65));
    tinybox.add_vertex(Vec3::new(0.4, -0.4, -0.65));
    tinybox.add_vertex(Vec3::new(0.4, -0.4, -0.7));
    tinybox.add_vertex(Vec3::new(0.3, -0.4, -0.7));

    tinybox.add_triangle(0, 1, 2);
    tinybox.add_triangle(0, 2, 3);
    tinybox.add_triangle(1, 6, 5);
    tinybox.add_triangle(1, 2, 6);
    tinybox.add_triangle(0, 1, 5);
    tinybox.add_triangle(0, 5, 4);
    tinybox.add_triangle(2, 3, 6);
    tinybox.add_triangle(3, 7, 6);
    tinybox.add_triangle(5, 7, 4);
    tinybox.add_triangle(5, 6, 7);
    tinybox.add_triangle(3, 4, 7);
    tinybox.add_triangle(3, 0, 4);

    scene.add_mesh(tinybox);

    let raytracer = RayTracer::new(&bmp_canvas, 2.0, 1.0);

    let start_time = SystemTime::now();

    raytracer.draw(&mut bmp_canvas, &scene, 15, 20);

    println!(
        "rendered in {} ms",
        start_time.elapsed().unwrap().as_millis()
    );

    bmp_canvas.save_image("examples/test.bmp")?;

    Ok(())
}
