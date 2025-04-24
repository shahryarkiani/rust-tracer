use std::f64::consts::PI;

use crate::{canvas::{Canvas, Color}, hittable::{self, Hittable}, ray::{Point3, Ray}, sphere::Sphere, vec3::Vec3};



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


    pub fn draw(&self, canvas: &mut impl Canvas, world: &impl Hittable) {
        
        let x_viewport = Vec3::new(self.viewport_width, 0.0, 0.0);
        let y_viewport = Vec3::new(0.0, self.viewport_height, 0.0);
        let x_delta = x_viewport / canvas.width() as f64;
        let y_delta =  y_viewport / canvas.height() as f64;

        let viewport_top_left= self.camera_pos - Point3::new(0.0, 0.0, self.focal_len) - x_viewport / 2.0 -  y_viewport / 2.0 + x_delta / 2.0 + y_delta / 2.0;
        
        let sphere_center = Point3::new(0.0, 0.0, -1.0);
        let sphere = Sphere::new(sphere_center, 0.5, Color::new(125,125,0));

        let point_light = Point3::new(0.0, 0.0, 0.0);

        for y in 0..canvas.height() {
            for x in 0..canvas.width() {
                let dir = viewport_top_left + (x_delta * x as f64) + (y_delta * y as f64);
                let ray = Ray::new(self.camera_pos, dir);
                let intersection_t = sphere.intersects(ray);
                if intersection_t >= 0.0 {
                    let intersection_point = ray.at(intersection_t);
                    let norm = intersection_point - sphere_center;
                    let norm_u = norm / norm.magnitude();

                    let light_path = intersection_point - point_light;
                    let light_u = light_path / light_path.magnitude();

                    let cosine = norm_u.dot(light_u);

                    let angle_proportion = (f64::acos(cosine)) / PI;
                    let color = Color::new((255.0 * angle_proportion) as u8, (255.0 * angle_proportion) as u8, 0);

                    canvas.set_pixel(x, y, color);
                } else {
                    canvas.set_pixel(x, y, Color::new(135 , 200,255));
                }

            }
        }
    }
}