use std::fmt::Display;

#[derive(Debug)]
pub struct RGB {
    r: f32,
    g: f32,
    b: f32
}

impl RGB {
    pub fn new(r: f32, g: f32, b: f32) -> RGB {
        return RGB {
            r,
            g,
            b
        }
    }
}

impl Default for RGB {
    fn default() -> RGB {
        return RGB::new(0.0, 0.0, 0.0);
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", format!("{r} {g} {b}\n", r = self.r, g = self.g, b = self.b));
    }
}