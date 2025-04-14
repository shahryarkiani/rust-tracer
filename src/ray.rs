use crate::{canvas::Pixel, vec3::Vec3};

#[derive(Clone, Copy, Default, Debug)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

// Alias representing 3D coordinate
pub type Point3 = Vec3;

impl Ray {

    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + t * self.dir;
    }

}