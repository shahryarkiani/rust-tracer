use crate::{hittable::HitInfo, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian { albedo: Vec3},
    Metal { albedo: Vec3 },
    Emissive { emission: Vec3 }
}

impl Default for Material {
    fn default() -> Material {
        Material::Metal { albedo: Vec3::default() }
    }
}

impl Material {
    pub fn scatter(
        &self,
        ray: Ray,
        hit_info: &HitInfo,
        attenuation_out: &mut Vec3,
        scatter_out: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let bounce_dir = hit_info.normal + Vec3::random_unit();
                *scatter_out = Ray::new(hit_info.point, bounce_dir);
                *attenuation_out = *albedo;
                true
            }
            Material::Metal { albedo } => {
                let reflect_dir = ray.dir() - 2.0 * ray.dir().dot(hit_info.normal) * hit_info.normal;
                *scatter_out = Ray::new(hit_info.point, reflect_dir);
                *attenuation_out = *albedo;                
                true
            }
            Material::Emissive { emission: _ } => {
                false
            }
        }
    }

    pub fn emission(&self) -> Vec3 {
        match self {
            Material::Emissive { emission } => *emission,
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }
}