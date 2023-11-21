use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Sub};
use std::marker::Copy;
use crate::errors::Error;

#[derive(Debug)]
pub struct Float3 {
    x: f32,
    y: f32,
    z: f32
}

impl Float3 {
    pub fn new(x: f32, y: f32, z: f32) -> Float3 {
        return Float3 {
            x,
            y,
            z
        }
    }

    pub fn length(&self) -> f32 {
        return f32::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2));
    }

    pub fn normalize(mut self) -> Result<Float3, Error>{
        let len = self.length();

        if len < f32::EPSILON {
            return Err(Error::LengthZeroNormalization);
        }

        self.x /= len;
        self.y /= len;
        self.z /= len;

        return Ok(self);
    }

    pub fn i() -> Float3 {
        return Float3::new(1.0, 0.0, 0.0);
    }

    pub fn j() -> Float3 {
        return Float3::new(0.0, 1.0, 0.0);
    }

    pub fn k() -> Float3 {
        return Float3::new(0.0, 0.0, 1.0);
    }

    pub fn zero() -> Float3 {
        return Float3::new(0.0, 0.0, 0.0);
    }
}

impl Add for Float3 {
    type Output = Float3;
    fn add(self, rhs: Float3) -> Self::Output {
        return Float3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        };
    }
}

impl Add<f32> for Float3 {
    type Output = Float3;

    fn add(self, rhs: f32) -> Self::Output {
        return Float3 {
            x: self.x +rhs,
            y: self.y + rhs,
            z: self.z + rhs
        }
    }
}

impl Add<Float3> for f32 {
    type Output = Float3;

    fn add(self, rhs: Float3) -> Self::Output {
        return rhs + self;
    }
}

impl Sub<Float3> for Float3 {
    type Output = Float3;

    fn sub(self, rhs: Float3) -> Self::Output {
        return Float3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul<f32> for Float3 {
    type Output = Float3;

    fn mul(self, rhs: f32) -> Self::Output {
        return Float3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Float3> for f32 {
    type Output = Float3;

    fn mul(self, rhs: Float3) -> Self::Output {
        return rhs * self;
    }
}

impl PartialEq<Self> for Float3 {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}

impl Clone for Float3 {
    fn clone(&self) -> Self {
        return Float3 {
            x: self.x,
            y: self.y,
            z: self.z
        };
    }
}

impl Copy for Float3 {

}

impl Eq for Float3 {

}

impl BitOr for Float3 {
    type Output = f32;

    fn bitor(self, rhs: Self) -> Self::Output {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }
}

impl BitXor for Float3 {
    type Output = Float3;

    fn bitxor(self, rhs: Self) -> Self::Output {
        return Float3 {
            x: self.x * rhs.y - self.y * rhs.x,
            y: self.y * rhs.z - self.z * rhs.y,
            z: self.z * rhs.x - self.x * rhs.z,
        }
    }
}


#[derive(Debug)]
struct Sphere {
    o: Float3,
    r: f32
}

impl Sphere {
    fn area(&self) -> f32 {
        return self.r.powi(2) * std::f32::consts::PI;
    }

}