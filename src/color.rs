use crate::utils::equal;


#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0)
        }
    }

    pub fn as_rgb(&self) -> [u8; 3] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        ]
    }

    pub fn from_vec(rgb: &Vec<f64>) -> Color {
        Color::new(rgb[0], rgb[1], rgb[2])
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn add(&self, other: &Color) -> Color {
        Color {
            r: (self.r + other.r).clamp(0.0, 1.0),
            g: (self.g + other.g).clamp(0.0, 1.0),
            b: (self.b + other.b).clamp(0.0, 1.0)
        }
    }

    pub fn subtract(&self, other: &Color) -> Color {
        Color {
            r: (self.r - other.r).clamp(0.0, 1.0),
            g: (self.g - other.g).clamp(0.0, 1.0),
            b: (self.b - other.b).clamp(0.0, 1.0)
        }
    }

    pub fn multiply(&self, n: f64) -> Color {
        Color {
            r: (self.r * n).clamp(0.0, 1.0),
            g: (self.g * n).clamp(0.0, 1.0),
            b: (self.b * n).clamp(0.0, 1.0)
        }
    }

    pub fn multiply_color(&self, &other: &Color) -> Color {
        Color {
            r: (self.r * other.r).clamp(0.0, 1.0),
            g: (self.g * other.g).clamp(0.0, 1.0),
            b: (self.b * other.b).clamp(0.0, 1.0)
        }
    }
}


impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        equal(self.r, other.r) &&
            equal(self.g, other.g) &&
            equal(self.b, other.b)
    }
}
impl Eq for Color {}
