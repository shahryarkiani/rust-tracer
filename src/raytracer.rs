
use crate::{canvas::{to_pixel, Canvas}, hittable::{HitInfo, Hittable, HittableList}, ray::{Interval, Point3, Ray}, vec3::Vec3};



pub struct RayTracer {
    viewport_height: f64,
    viewport_width: f64,
    focal_len: f64,
    camera_pos: Point3
}

impl RayTracer {
    pub fn new(canvas: &impl Canvas, viewport_height: f64, focal_length: f64) -> RayTracer {

        let viewport_width = viewport_height * (canvas.width() as f64 / canvas.height() as f64);

        // Camera is (initially) positioned at (0, 0, 0)
        RayTracer {
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            focal_len: focal_length,
            camera_pos: Point3::new(0.0, 0.0, 0.0)
        }
    }


    pub fn draw(&self, canvas: &mut impl Canvas, world: &HittableList) {
        
        let x_viewport = Vec3::new(self.viewport_width, 0.0, 0.0);
        let y_viewport = Vec3::new(0.0, self.viewport_height, 0.0);
        let x_delta = x_viewport / canvas.width() as f64;
        let y_delta =  y_viewport / canvas.height() as f64;

        let viewport_top_left= self.camera_pos - Point3::new(0.0, 0.0, self.focal_len) - x_viewport / 2.0 -  y_viewport / 2.0 + x_delta / 2.0 + y_delta / 2.0;

        for y in 0..canvas.height() {
            for x in 0..canvas.width() {
                let dir = viewport_top_left + (x_delta * x as f64) + (y_delta * y as f64);
                let ray = Ray::new(self.camera_pos, dir);

                let color = ray_color(ray, world, 20);

                canvas.set_pixel(x, y, to_pixel(color));

            }
        }
    }
}

fn ray_color(ray: Ray, world: &HittableList, rec_depth: i16) -> Vec3 {
    if rec_depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let mut hit_info = HitInfo::default();

    if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut hit_info) {
        // Get the material based on the material id
        let material = world.material_for_id(hit_info.material_id);

        let mut scattered_ray: Ray = Ray::default();
        let mut attenuation: Vec3 = Vec3::default();

        if material.scatter(ray, &hit_info, &mut attenuation, &mut scattered_ray) {
            return attenuation * ray_color(scattered_ray, world, rec_depth - 1);
        }

        return Vec3::new(0.0, 0.0, 0.0);
    }
    
    let dir_u = ray.dir() / ray.dir().magnitude();
    let a = 0.5 * (dir_u.y + 1.0);

    return (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
}