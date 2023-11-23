use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Sub};
use std::marker::Copy;
use crate::errors::Error;
use crate::ray::Ray;

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

    pub fn reflect(&self, n: Float3) -> Float3 {
        todo!();
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
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl fmt::Display for Float3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


#[derive(Debug)]
pub struct Sphere {
    center: Float3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Float3, radius: f32) -> Sphere {
        return Sphere {
            center,
            radius
        };
    }
    pub fn area(&self) -> f32 {
        return self.radius.powi(2) * std::f32::consts::PI;
    }

    pub fn is_hit(&self, ray: Ray) -> bool {
        let o_c = ray.origin - self.center;
        let a = ray.direction | ray.direction;
        let b = 2.0 * ray.direction | o_c;
        let c = (o_c | o_c) - self.radius.powi(2);

        let delta = b * b - 4.0 * a * c;

        if delta > 0.0 {
            return true;
        }

        return false;
    }
}