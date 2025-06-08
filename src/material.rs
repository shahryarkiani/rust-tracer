use crate::{hittable::HitInfo, ray::Ray, vec3::Vec3};

#[derive(Default, Clone, Copy)]
pub struct Material {
    pub material_type: MaterialType,
    pub albedo: Vec3,
}

impl Material {
    pub fn scatter(
        &self,
        ray: Ray,
        hit_info: &HitInfo,
        attenuation_out: &mut Vec3,
        scatter_out: &mut Ray,
    ) -> bool {
        return match self.material_type {
            MaterialType::Lambertian => {
                self.scatter_lambertian(hit_info, attenuation_out, scatter_out)
            }
            MaterialType::Metal => self.scatter_metal(ray, hit_info, attenuation_out, scatter_out),
            MaterialType::Emissive => self.scatter_emissive(),
        };
    }

    pub fn emission(&self) -> Vec3 {
        return match self.material_type {
            MaterialType::Emissive => self.albedo,
            _ => Vec3::new(0., 0., 0.),
        };
    }

    fn scatter_lambertian(
        &self,
        hit_info: &HitInfo,
        attenuation_out: &mut Vec3,
        scatter_out: &mut Ray,
    ) -> bool {
        let bounce_dir = hit_info.normal + Vec3::random_unit();
        *scatter_out = Ray::new(hit_info.point, bounce_dir);
        *attenuation_out = self.albedo;
        true
    }

    fn scatter_metal(
        &self,
        ray: Ray,
        hit_info: &HitInfo,
        attenuation_out: &mut Vec3,
        scatter_out: &mut Ray,
    ) -> bool {
        let reflect_dir = ray.dir() - 2.0 * ray.dir().dot(hit_info.normal) * hit_info.normal;
        *scatter_out = Ray::new(hit_info.point, reflect_dir);
        *attenuation_out = self.albedo;
        true
    }

    fn scatter_emissive(&self) -> bool {
        false
    }
}

#[derive(Default, Clone, Copy)]
pub enum MaterialType {
    Lambertian,
    #[default]
    Metal,
    Emissive,
}
