mod image;
mod color;
mod math;
mod ray;

use math::Float3;
use image::{Image, ImageFormat};
use color::RGB;

fn main() {
    let width = 256;
    let height = 256;

    // Image
    let mut img = Image::<RGB>::blank(width, height);

    // Render
    for j in 0..height {
        for i in 0..width {
            img.write_pixel(i, j, RGB::new(i as f32, j as f32, 0.0));
        }
    }

    img.save("test2.ppm", ImageFormat::PPM);

}
