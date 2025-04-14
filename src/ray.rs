use crate::{vec3::Vec3};

#[derive(Clone, Copy, Default, Debug)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

// Alias representing 3D coordinate
pub type Point3 = Vec3;

impl Ray {

    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + t * self.dir;
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }
}