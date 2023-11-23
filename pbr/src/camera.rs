use crate::errors;
use crate::math::Float3;
use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    pub origin: Float3,
    pub focal_length: f32,
    pub sensor_size: f32,
    pub i: Float3,
    pub j: Float3,
    pub k: Float3
}

impl Camera {
    pub fn look_at(&mut self, v: Float3) -> Result<(), errors::Error>{


        let look = (v - self.origin).normalize()?;


        self.i = look;

        let mut j = self.i ^ Float3::k();

        if j.length() < f32::EPSILON {
            j = (self.i ^ Float3::j()).normalize()?;
        } else {
            j = j.normalize()?;
        }

        self.j = j;
        self.k =  self.j ^ self.i;

        return Ok(());
    }

    pub fn ray_at_pixel(&self, i: usize, j: usize, width: usize, height: usize) -> Ray {
        let pixel_loc = self.focal_length * self.i
                             - ((width as f32 - 1.0) / 2.0 - i as f32) * self.sensor_size / width as f32 * self.j
                             - ((height as f32 -1.0) / 2.0 - j as f32) * self.sensor_size / height as f32 * self.k ;

        return Ray::new(self.origin, (pixel_loc - self.origin).normalize().unwrap());
    }
}