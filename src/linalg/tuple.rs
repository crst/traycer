use crate::utils::{equal};
use crate::linalg::matrix::Matrix;


#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x: x, y: y, z: z, w: w, }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x: x, y: y, z: z, w: 1.0 }
    }

    pub fn point_from_vec(xyz: &Vec<f64>) -> Tuple {
        Tuple::new(xyz[0], xyz[1], xyz[2], 1.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x: x, y: y, z: z, w: 0.0}
    }

    pub fn vector_from_vec(xyz: &Vec<f64>) -> Tuple {
        Tuple::new(xyz[0], xyz[1], xyz[2], 0.0)
    }

    pub fn add(&self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn subtract(&self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    pub fn negate(&self) -> Tuple {
        Tuple { x: -self.x, y: -self.y, z: -self.z, w: -self.w, }
    }

    pub fn multiply(&self, n: f64) -> Tuple {
        Tuple { x: self.x * n, y: self.y * n, z: self.z * n, w: self.w * n, }
    }

    pub fn divide(&self, n: f64) -> Tuple {
        Tuple { x: self.x / n, y: self.y / n, z: self.z / n, w: self.w / n, }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        self.divide(self.magnitude())
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Tuple {
        Matrix::translation(x, y, z).multiply_tuple(self)
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Tuple {
        Matrix::scaling(x, y, z).multiply_tuple(self)
    }

    pub fn rotate_x(&self, deg: f64) -> Tuple {
        let rad = (deg * std::f64::consts::PI) / 180.0;
        Matrix::rotation_x(rad).multiply_tuple(self)
    }

    pub fn rotate_y(&self, deg: f64) -> Tuple {
        let rad = (deg * std::f64::consts::PI) / 180.0;
        Matrix::rotation_y(rad).multiply_tuple(self)
    }

    pub fn rotate_z(&self, deg: f64) -> Tuple {
        let rad = (deg * std::f64::consts::PI) / 180.0;
        Matrix::rotation_z(rad).multiply_tuple(self)
    }

    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Tuple {
        Matrix::shearing(xy, xz, yx, yz, zx, zy).multiply_tuple(self)
    }

}


impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x) &&
            equal(self.y, other.y) &&
            equal(self.z, other.z) &&
            equal(self.w, other.w)
    }
}
impl Eq for Tuple {}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_add_subtract() {
        let n: f64 = 123_f64;
        let mut rng = rand::thread_rng();
        for _ in 1..10 {
            let a: Tuple = Tuple::new(rng.gen::<f64>() * n, rng.gen::<f64>() * n, rng.gen::<f64>() * n, rng.gen::<f64>() * n);
            let b: Tuple = Tuple::new(rng.gen::<f64>() * n, rng.gen::<f64>() * n, rng.gen::<f64>() * n, rng.gen::<f64>() * n);
            assert_eq!(a, a.add(&b).subtract(&b));
        }
    }
}
