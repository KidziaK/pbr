use std::ops::{Add, Mul};

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