use crate::vec3::Vec3;
use core::f64;

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

#[derive(Clone, Copy, Default)]
pub struct Interval {
    begin: f64,
    end: f64,
}

impl Interval {
    pub fn new(begin: f64, end: f64) -> Interval {
        Interval { begin, end }
    }

    pub fn contains(&self, t: f64) -> bool {
        t >= self.begin && t <= self.end
    }
}

pub const empty: Interval = Interval {
    begin: f64::INFINITY,
    end: f64::NEG_INFINITY,
};
pub const any: Interval = Interval {
    begin: f64::NEG_INFINITY,
    end: f64::INFINITY,
};
