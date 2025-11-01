use rand::Rng;

use crate::{
    canvas::{Canvas, to_pixel},
    hittable::{HitInfo, Hittable},
    ray::{Interval, Point3, Ray},
    triangle_mesh::Scene,
    vec3::Vec3,
};

pub struct RayTracer {
    viewport_height: f32,
    viewport_width: f32,
    focal_len: f32,
    camera_pos: Point3,
}

impl RayTracer {
    pub fn new(canvas: &impl Canvas, viewport_height: f32, focal_length: f32) -> RayTracer {
        let viewport_width = viewport_height * (canvas.width() as f32 / canvas.height() as f32);

        // Camera is (initially) positioned at (0, 0, 0)
        RayTracer {
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            focal_len: focal_length,
            camera_pos: Point3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn draw(&self, canvas: &mut impl Canvas, scene: &Scene, samples: i32, max_bounces: i16) {
        let x_viewport = Vec3::new(self.viewport_width, 0.0, 0.0);
        let y_viewport = Vec3::new(0.0, -self.viewport_height, 0.0);
        let x_delta = x_viewport / canvas.width() as f32;
        let y_delta = y_viewport / canvas.height() as f32;

        let viewport_top_left = self.camera_pos
            - Point3::new(0.0, 0.0, self.focal_len)
            - x_viewport / 2.0
            - y_viewport / 2.0
            + x_delta / 2.0
            + y_delta / 2.0;
        for y in 0..canvas.height() {
            for x in 0..canvas.width() {
                let base_dir = viewport_top_left + (x_delta * x as f32) + (y_delta * y as f32);

                let mut color = Vec3::new(0., 0., 0.);
                for _ in 0..samples {
                    let x_offset = rand::rng().random_range(-0.5..0.5);
                    let y_offset = rand::rng().random_range(-0.5..0.5);

                    let dir = Vec3::new(
                        base_dir.x + x_delta.x * x_offset,
                        base_dir.y + y_delta.y * y_offset,
                        base_dir.z,
                    );

                    let ray = Ray::new(self.camera_pos, dir);

                    color = color + ray_color(ray, scene, max_bounces);
                }

                color = color / (samples as f32);

                canvas.set_pixel(x, y, to_pixel(color));
            }
        }
    }
}

fn ray_color(ray: Ray, scene: &Scene, rec_depth: i16) -> Vec3 {
    let hit_interval = Interval::new(0.001, f32::INFINITY);

    let mut scattered_ray = ray;
    let mut total_attenuation = Vec3::new(1., 1., 1.);

    let mut hit_info = HitInfo::default();

    for _ in 0..rec_depth {
        if scene.hit(scattered_ray, hit_interval, &mut hit_info) {
            let material = hit_info.material;

            let mut attenuation = Vec3::default();

            let emitted = material.emission();

            if material.scatter(
                scattered_ray,
                &hit_info,
                &mut attenuation,
                &mut scattered_ray,
            ) {
                total_attenuation = total_attenuation * attenuation;
            } else {
                return emitted * total_attenuation;
            }
        } else {
            let dir_u = scattered_ray.dir() / scattered_ray.dir().magnitude();
            let a = 0.5 * (dir_u.y + 1.0);

            return total_attenuation
                * ((1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0));
        }
    }

    return Vec3::new(0., 0., 0.);
}
