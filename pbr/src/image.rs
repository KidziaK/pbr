use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Image<T> {
    width: usize,
    height: usize,
    bytes: Vec<Vec<T>>
}

pub enum ImageFormat {
    PPM
}

impl<T: Default + ToString> Image<T> {
    pub fn blank(width: usize, height: usize) -> Image<T> {
        let mut bytes = Vec::<Vec::<T>>::with_capacity(height);

        for _ in 0..height {
            let mut row = Vec::<T>::with_capacity(width);

            for _ in 0..width {
                row.push(Default::default());
            }

            bytes.push(row);
        }

        return Image::<T> {
            width,
            height,
            bytes
        };
    }

    fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {} \n255\n", self.width, self.height);

        for row in &self.bytes {
            for rgb in row {
                ppm.push_str(rgb.to_string().as_str());
            }
        }

        return ppm;
    }

    pub fn save(&self, path: &str, format: ImageFormat) {
        let content : String;

        match format {
            ImageFormat::PPM => {
                content = self.to_ppm();
            }
        }

        let mut file = File::create(path)
            .expect(format!("Couldn't create a file {}", path).as_str());
        let _ = file.write_all(content.as_bytes());
    }
    
    pub fn write_pixel(&mut self, row: usize, col: usize, val: T) {
        self.bytes[row][col] = val;
    }
}
