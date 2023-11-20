use crate::math::Float3;

pub struct Ray {
    origin: Float3,
    direction: Float3
}

impl Ray {
    fn at(self, t: f32) -> Float3 {
        return self.origin + t * self.direction;
    }
}