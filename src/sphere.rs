use crate::{
    canvas::Color,
    hittable::{HitInfo, Hittable},
    ray::{Interval, Point3, Ray}, vec3::Vec3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    color: Color,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, color: Color) -> Sphere {
        Sphere {
            center,
            radius,
            color,
        }
    }

    pub fn intersects(&self, ray: Ray) -> f64 {
        let diff = self.center - ray.origin();
        let a = ray.dir().dot(ray.dir());
        let b = -2.0 * ray.dir().dot(diff);
        let c = diff.dot(diff) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            (-b - f64::sqrt(discriminant)) / (2.0 * a)
        } else {
            -1.0
        }
    }

    pub fn normal(&self, point: Point3) -> Vec3 {
        (point - self.center) / self.radius
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, interval: Interval, hit_info_out: &mut HitInfo) -> bool {
        let diff = self.center - ray.origin();
        let a = ray.dir().dot(ray.dir());
        let b = -2.0 * ray.dir().dot(diff);
        let c = diff.dot(diff) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t = (-b - f64::sqrt(discriminant)) / (2.0 * a);
            if interval.contains(t) {
                hit_info_out.t = t;
                return true
            }
        }
            
        false 
    }
}
