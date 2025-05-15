use crate::{
    material::Material,
    ray::{Interval, Point3, Ray},
    vec3::Vec3,
};

pub trait Hittable {
    fn hit(&self, ray: Ray, interval: Interval, hit_info_out: &mut HitInfo) -> bool;
}

#[derive(Default)]
pub struct HitInfo {
    pub t: f64,
    pub normal: Vec3,
    pub point: Point3,
    pub material: Material,
}
