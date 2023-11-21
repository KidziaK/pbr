use crate::math::Float3;

#[derive(Debug)]
pub struct Camera {
    pub origin: Float3,
    pub focal_length: f32,
    pub i: Float3,
    pub j: Float3,
    pub k: Float3
}

impl Camera {
    pub(crate) fn look_at(&mut self, v: Float3) -> Result<(), ()>{
        if self.origin == v {
            return Err(());
        }

        let look = (v - self.origin).normalize().unwrap();


        self.i = look;

        todo!();
        // let mut j = (self.i ^ Float3::k()).normalize();
        //
        // if j.length() < f32::EPSILON {
        //     self.k = j;
        //     self.j = self.i ^ j;
        // } else {
        //     self.j = j;
        //     self.k = self.i ^ j;
        // }

        return Ok(());
    }
}