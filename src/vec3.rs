use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Default, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: rand::rng().random(),
            y: rand::rng().random(),
            z: rand::rng().random(),
        }
    }

    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: rand::rng().random_range(min..max),
            y: rand::rng().random_range(min..max),
            z: rand::rng().random_range(min..max),
        }
    }

    pub fn random_unit() -> Vec3 {
        let vec = Vec3::random_range(-1., 1.);
        let mag_squared = vec.dot(vec);

        if 1e-160 < mag_squared && mag_squared <= 1. {
            return vec / f32::sqrt(mag_squared);
        }

        vec
    }

    pub fn axis_val(&self, i: usize) -> f32 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.dot(*self))
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, c: f32) -> Vec3 {
        Vec3 {
            x: self.x * c,
            y: self.y * c,
            z: self.z * c,
        }
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * vec.x,
            y: self.y * vec.y,
            z: self.z * vec.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, c: f32) -> Vec3 {
        Vec3 {
            x: self.x / c,
            y: self.y / c,
            z: self.z / c,
        }
    }
}
