use crate::{
    ray::{Interval, Point3, Ray}, sphere::Sphere, vec3::Vec3
};

pub trait Hittable {
    fn hit(&self, ray: Ray, interval: Interval, hit_info_out: &mut HitInfo) -> bool;
}

#[derive(Default)]
pub struct HitInfo {
    pub t: f64,
    pub normal: Vec3,
    pub point: Point3,
    pub color: Vec3
}

#[derive(Default)]
pub struct HittableList {
    spheres: Vec<Sphere>,
}

impl HittableList {
    pub fn add(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, interval: Interval, hit_info_out: &mut HitInfo) -> bool {
        let mut hit_info_tmp = HitInfo::default();

        hit_info_out.t = f64::INFINITY;

        let mut closest_sphere: Option<&Sphere> = None;

        for sphere in &self.spheres {
            if sphere.hit(ray, interval, &mut hit_info_tmp) {
                if hit_info_tmp.t < hit_info_out.t {
                    // We find the closest hit
                    closest_sphere = Some(sphere);
                    hit_info_out.t = hit_info_tmp.t;
                }
            }
        }

        if let Some(closest_sphere) = closest_sphere {
            hit_info_out.point = ray.at(hit_info_out.t); // Only want to compute these values for a single sphere
            hit_info_out.normal = closest_sphere.normal(hit_info_out.point);
            hit_info_out.color = hit_info_out.normal;
            true
        } else {
            false
        }
    }
}
