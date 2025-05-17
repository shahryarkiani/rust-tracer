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
    begin: f32,
    end: f32,
}

impl Interval {
    pub fn new(begin: f32, end: f32) -> Interval {
        Interval { begin, end }
    }

    pub fn contains(&self, t: f32) -> bool {
        t >= self.begin && t <= self.end
    }

    pub fn clamp(&self, t: f32) -> f32 {
        if t > self.end {
            return self.end;
        } else if t < self.begin {
            return self.begin;
        }

        t
    }
}

pub const EMPTY: Interval = Interval {
    begin: f32::INFINITY,
    end: f32::NEG_INFINITY,
};
pub const ANY: Interval = Interval {
    begin: f32::NEG_INFINITY,
    end: f32::INFINITY,
};
