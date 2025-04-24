use crate::{ray::Ray, sphere::{self, Sphere}, vec3::Vec3};

pub trait Hittable{
    fn hit(&self, ray: Ray, hit_info_out : &mut HitInfo) -> bool;
}

#[derive(Default)]
pub struct HitInfo {
    t: f64,
    normal: Vec3,
}


pub struct HittableList {
    spheres : Vec<Sphere>
}


impl Hittable for HittableList {
    fn hit(&self, ray: Ray, hit_info_out : &mut HitInfo) -> bool {
        let mut hit_info_tmp = HitInfo::default();
        
        for sphere in &self.spheres {
            todo!()
        }
        
        false
    }
}