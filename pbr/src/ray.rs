use crate::math::Float3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Float3,
    pub direction: Float3
}

impl Ray {
    pub fn new(origin: Float3, direction: Float3) -> Ray {
        return Ray {
            origin,
            direction
        };
    }
    pub fn at(self, t: f32) -> Float3 {
        return self.origin + t * self.direction;
    }
}