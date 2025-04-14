use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Default, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}


impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.dot(*self))
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

}




impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        } 
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        } 
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, c: f64) -> Vec3 {
        Vec3 {
            x: self.x * c,
            y: self.y * c,
            z: self.z * c
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, c: f64) -> Vec3 {
        Vec3 {
            x: self.x / c,
            y: self.y / c,
            z: self.z / c
        }
    } 
}

