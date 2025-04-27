use crate::{
    material::Material, ray::{Interval, Point3, Ray}, sphere::Sphere, vec3::Vec3
};

pub trait Hittable {
    fn hit(&self, ray: Ray, interval: Interval, hit_info_out: &mut HitInfo) -> bool;
}

#[derive(Default)]
pub struct HitInfo {
    pub t: f64,
    pub normal: Vec3,
    pub point: Point3,
    pub color: Vec3,
    pub material_id: MaterialId
}

#[derive(Default)]
pub struct HittableList {
    spheres: Vec<Sphere>,
    sphere_material: Vec<MaterialId>,
    materials: Vec<Material>
}

type MaterialId = u16;

impl HittableList {

    pub fn add_material(&mut self, material: Material) -> MaterialId {
        self.materials.push(material);

        (self.materials.len() - 1) as MaterialId
    }

    pub fn add_sphere(&mut self, sphere: Sphere, material_id: MaterialId) {
        self.spheres.push(sphere);
        self.sphere_material.push(material_id);
    }

    pub fn material_for_id(&self, material_id: MaterialId) -> &Material{
        &self.materials[material_id as usize]
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, interval: Interval, hit_info_out: &mut HitInfo) -> bool {
        let mut hit_info_tmp = HitInfo::default();

        hit_info_out.t = f64::INFINITY;

        let mut hit_idx = self.spheres.len();

        for i in 0..self.spheres.len() {
            let sphere = &self.spheres[i];

            if sphere.hit(ray, interval, &mut hit_info_tmp) {
                if hit_info_tmp.t < hit_info_out.t {
                    // We find the closest hit
                    hit_info_out.t = hit_info_tmp.t;
                    hit_idx = i;
                }
            }
        }

        if hit_idx < self.spheres.len() {
            let closest_sphere = &self.spheres[hit_idx]; 

            hit_info_out.point = ray.at(hit_info_out.t); // Only want to compute these values for a single sphere
            hit_info_out.normal = closest_sphere.normal(hit_info_out.point);
            hit_info_out.color = hit_info_out.normal;
            hit_info_out.material_id = self.sphere_material[hit_idx];

            true
        } else {
            false
        }
    }
}
