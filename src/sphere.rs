use crate::{canvas::Color, ray::{Point3, Ray}};



pub struct Sphere {
    center: Point3,
    radius: f64,
    color: Color
}

impl Sphere {

    pub fn new(center: Point3, radius: f64, color: Color) -> Sphere {
        Sphere {center, radius, color}
    }

    pub fn intersects(&self, ray: Ray) -> bool {
        let diff = self.center - ray.origin();
        let a = ray.dir().dot(ray.dir());
        let b = -2.0 * ray.dir().dot(diff);
        let c= diff.dot(diff) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

