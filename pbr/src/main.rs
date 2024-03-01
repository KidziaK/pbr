mod image;
mod color;
mod math;
mod ray;
mod camera;
mod errors;

use math::{Float3, Sphere};
use image::{Image, ImageFormat};
use color::RGB;
use crate::camera::Camera;

fn main() {


    let mut cam = Camera {
        origin: Float3::zero(),
        focal_length: 50.0,
        sensor_size: 36.0,
        i: Float3::zero(),
        j: Float3::zero(),
        k: Float3::zero()
    };

    cam.look_at(Float3::new(60.0, 0.0, 0.0));


    let width = 500;
    let height = 500;

    let sphere = Sphere::new(Float3::i(), 0.1);
    let sphere2 = Sphere::new(Float3::new(1.0, 0.1, 0.0), 0.1);

    // Image
    let mut img = Image::<RGB>::blank(width, height);

    // Render
    for i in 0..height {
        for j in 0..width {
            let ray = cam.ray_at_pixel(i, j, width, height);

            if sphere.is_hit(&ray) {
                img.write_pixel(i, j, RGB::new(255.0, 0.0, 0.0));
            }

            if sphere2.is_hit(&ray) {
                img.write_pixel(i, j, RGB::new(0.0, 255.0, 0.0));
            }
        }
    }

    img.save("test3.ppm", ImageFormat::PPM);

}
