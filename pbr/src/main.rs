mod image;
mod color;
mod math;
mod ray;
mod camera;
mod errors;

use math::Float3;
use image::{Image, ImageFormat};
use color::RGB;
use crate::camera::Camera;

fn main() {
    // let width = 256;
    // let height = 256;
    //
    // // Image
    // let mut img = Image::<RGB>::blank(width, height);
    //
    // // Render
    // for j in 0..height {
    //     for i in 0..width {
    //         img.write_pixel(i, j, RGB::new(i as f32, j as f32, 0.0));
    //     }
    // }
    //
    // img.save("test2.ppm", ImageFormat::PPM);

    let mut cam = Camera {
        origin: Float3::zero(),
        focal_length: 0.0,
        i: Float3::zero(),
        j: Float3::zero(),
        k: Float3::zero()
    };

    cam.look_at(Float3::new(1.0, 1.0, 1.0));

    println!("{:?}", cam);

}
