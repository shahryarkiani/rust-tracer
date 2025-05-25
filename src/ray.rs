use crate::vec3::Vec3;
use core::f32;

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

    pub fn at(&self, t: f32) -> Point3 {
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
    endpoints: [f32; 2],
}

impl Interval {
    pub fn new(begin: f32, end: f32) -> Interval {
        Interval {
            endpoints: [begin, end],
        }
    }

    pub fn contains(&self, t: f32) -> bool {
        t >= self.endpoints[0] && t <= self.endpoints[1]
    }

    pub fn clamp(&self, t: f32) -> f32 {
        if t > self.endpoints[1] {
            return self.endpoints[1];
        } else if t < self.endpoints[0] {
            return self.endpoints[0];
        }

        t
    }

    pub fn get_val(&self, i: usize) -> f32 {
        return self.endpoints[i as usize];
    }
}

pub const EMPTY: Interval = Interval {
    endpoints: [f32::INFINITY, f32::NEG_INFINITY],
};
pub const ANY: Interval = Interval {
    endpoints: [f32::NEG_INFINITY, f32::INFINITY],
};
